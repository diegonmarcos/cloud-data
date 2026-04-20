#!/bin/sh
# reports orchestrator — data-driven dispatch.
# Entry point: reports/build.sh (thin shim) → reports/src/build.sh (this file).
#
# Usage:
#   build.sh all                  run every crate's build+run
#   build.sh build                build every crate, no run
#   build.sh list                 list discovered crates
#   build.sh test-dists           verify dist/ layout invariants
#   build.sh manifest             regenerate reports/manifest.json
#   build.sh <short>              run single crate (e.g. sec-data, health-full)
#   build.sh <full>               same, by full folder name (cloud-sec-data-report)
#   build.sh report<N>            same, by 1-based index in discovered list

set -eu

SELF_DIR="$(cd "$(dirname "$0")" && pwd)"
REPORTS_ROOT="$(cd "$SELF_DIR/.." && pwd)"
SRC_DIR="$SELF_DIR"
DIST_DIR="$REPORTS_ROOT/dist"
MANIFEST="$REPORTS_ROOT/manifest.json"

# Discover crates — every cloud-* dir under src/ that has a build.sh.
list_crate_dirs() {
    for d in "$SRC_DIR"/cloud-*/; do
        [ -d "$d" ] || continue
        [ -x "$d/build.sh" ] || continue
        printf '%s\n' "$(basename "$d")"
    done
}

# Map: short name → full folder name.
# Strip leading "cloud-" and trailing "-report"; fall back to full name.
short_name() {
    name="$1"
    name="${name#cloud-}"
    name="${name%-report}"
    printf '%s\n' "$name"
}

# Resolve a user-supplied target to a crate folder name.
# Accepts: full ("cloud-X-report"), short ("X"), or index ("reportN").
resolve_target() {
    target="$1"
    # Direct full-name match
    for c in $(list_crate_dirs); do
        [ "$c" = "$target" ] && { printf '%s\n' "$c"; return 0; }
    done
    # Short name match
    for c in $(list_crate_dirs); do
        [ "$(short_name "$c")" = "$target" ] && { printf '%s\n' "$c"; return 0; }
    done
    # report<N> — 1-based index
    case "$target" in
        report[0-9]*)
            idx="${target#report}"
            i=0
            for c in $(list_crate_dirs); do
                i=$((i + 1))
                [ "$i" = "$idx" ] && { printf '%s\n' "$c"; return 0; }
            done
            ;;
    esac
    return 1
}

cmd_list() {
    i=0
    for c in $(list_crate_dirs); do
        i=$((i + 1))
        printf '  report%-2d  %-32s (%s)\n' "$i" "$c" "$(short_name "$c")"
    done
}

cmd_all() {
    action="${1:-all}"  # all | build
    for c in $(list_crate_dirs); do
        echo ""
        echo "══════════════════════════════════════════"
        echo "  $c ($action)"
        echo "══════════════════════════════════════════"
        sh "$SRC_DIR/$c/build.sh" "$action"
    done
    generate_manifest
}

cmd_one() {
    crate="$1"
    action="${2:-all}"
    sh "$SRC_DIR/$crate/build.sh" "$action"
    generate_manifest
}

# Regenerate reports/manifest.json from *.md files in dist/.
generate_manifest() {
    [ -d "$DIST_DIR" ] || return 0
    printf '[\n' > "$MANIFEST"
    first=true
    for md in "$DIST_DIR"/*.md; do
        [ -f "$md" ] || continue
        name=$(basename "$md" .md | tr '_' ' ')
        file="reports/dist/$(basename "$md")"
        if [ "$first" = true ]; then first=false; else printf ',\n' >> "$MANIFEST"; fi
        printf '  {"file": "%s", "name": "%s"}' "$file" "$name" >> "$MANIFEST"
    done
    printf '\n]\n' >> "$MANIFEST"
    echo "Generated manifest.json ($(grep -c '{"file' "$MANIFEST") entries)"
}

# Verify the shared dist/ layout.
test_dists() {
    fail=0
    [ -d "$DIST_DIR" ] || { echo "FAIL: reports/dist/ missing"; exit 1; }
    [ -d "$DIST_DIR/bin" ] || { echo "FAIL: reports/dist/bin/ missing"; exit 1; }

    for c in $(list_crate_dirs); do
        # Per-crate binary lookup: main.rs crate name matches folder OR BINARY in build.sh
        binary=$(awk -F= '/^BINARY=/ { gsub(/"/,"",$2); print $2; exit }' "$SRC_DIR/$c/build.sh")
        [ -z "$binary" ] && binary="$c"
        if [ ! -e "$DIST_DIR/bin/$binary" ]; then
            echo "FAIL $c: dist/bin/$binary missing"
            fail=1
            continue
        fi
        # Templates live under src/ (resolved via TEMPLATE_DIR at run time).
        # Assert templates ARE NOT copied or symlinked into dist/.
        for tpl in "$SRC_DIR/$c"/*.md.tpl; do
            [ -f "$tpl" ] || continue
            t_name=$(basename "$tpl")
            if [ -e "$DIST_DIR/$t_name" ]; then
                echo "FAIL $c: dist/$t_name should not exist (templates stay in src/)"
                fail=1
            fi
        done
        echo "OK   $c (bin: $binary)"
    done
    [ "$fail" -eq 0 ] && echo "test-dists: PASS" || { echo "test-dists: FAIL"; exit 1; }
}

target="${1:-all}"

case "$target" in
    all)        cmd_all all ;;
    build)      cmd_all build ;;
    list)       cmd_list ;;
    manifest)   generate_manifest ;;
    test-dists) test_dists ;;
    help|-h|--help)
        sed -n '2,/^set -eu/p' "$0" | sed 's/^# \?//' | head -n -1 ;;
    *)
        crate=$(resolve_target "$target" || true)
        if [ -z "$crate" ]; then
            echo "Unknown target: $target" >&2
            echo "Run: $0 list" >&2
            exit 1
        fi
        cmd_one "$crate" all
        ;;
esac
