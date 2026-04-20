#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"

# Generate reports/manifest.json from .md symlinks
generate_manifest() {
    printf '[\n' > "$ROOT/manifest.json"
    first=true
    for md in "$ROOT"/*.md; do
        [ -f "$md" ] || continue
        name=$(basename "$md" .md | tr '_' ' ')
        file="reports/$(basename "$md")"
        if [ "$first" = true ]; then first=false; else printf ',\n' >> "$ROOT/manifest.json"; fi
        printf '  {"file": "%s", "name": "%s"}' "$file" "$name" >> "$ROOT/manifest.json"
    done
    printf '\n]\n' >> "$ROOT/manifest.json"
    echo "Generated reports/manifest.json"
}

# Symlink each report's .md output to reports/ root (for manifest + consumers)
link_reports() {
    for dir in "$ROOT"/cloud-*-report; do
        [ -d "$dir" ] || continue
        for md in "$dir"/*.md; do
            [ -f "$md" ] || continue
            ln -sf "$(basename "$dir")/$(basename "$md")" "$ROOT/$(basename "$md")"
        done
    done
}

# Aggregate per-crate dist/ dirs under reports/dist/<crate> (symlinks)
link_dists() {
    mkdir -p "$ROOT/dist"
    for dir in "$ROOT"/cloud-*/; do
        [ -d "$dir" ] || continue
        name=$(basename "$dir")
        [ -d "$dir/dist" ] || continue
        ln -sfn "../$name/dist" "$ROOT/dist/$name"
    done
}

# Assert every crate with dist/ has a corresponding symlink under reports/dist/
test_dists() {
    fail=0
    for dir in "$ROOT"/cloud-*/; do
        [ -d "$dir" ] || continue
        name=$(basename "$dir")
        [ -d "$dir/dist" ] || continue
        link="$ROOT/dist/$name"
        want="../$name/dist"
        if [ ! -L "$link" ]; then
            echo "FAIL $name: $link is not a symlink"; fail=1; continue
        fi
        got=$(readlink "$link")
        if [ "$got" != "$want" ]; then
            echo "FAIL $name: target=$got want=$want"; fail=1; continue
        fi
        if [ ! -d "$link/" ]; then
            echo "FAIL $name: symlink does not resolve to a directory"; fail=1; continue
        fi
        echo "OK   $name -> $got"
    done
    [ "$fail" -eq 0 ] && echo "test-dists: PASS" || { echo "test-dists: FAIL"; exit 1; }
}

target="${1:-all}"

case "$target" in
  all)
    sh "$ROOT/cloud-stack-report/build.sh" all
    sh "$ROOT/cloud-health-full-report/build.sh" all
    sh "$ROOT/cloud-mail-full-report/build.sh" all
    sh "$ROOT/cloud-url-health-report/build.sh" all
    sh "$ROOT/cloud-sec-network-report/build.sh" all
    sh "$ROOT/cloud-sec-data-report/build.sh" all
    sh "$ROOT/cloud-health-daily-mail/build.sh" all
    link_reports
    link_dists
    generate_manifest
    ;;
  stack)       sh "$ROOT/cloud-stack-report/build.sh" all; link_reports; link_dists ;;
  cloud)       sh "$ROOT/cloud-health-full-report/build.sh" all; link_reports; link_dists ;;
  mail)        sh "$ROOT/cloud-mail-full-report/build.sh" all; link_reports; link_dists ;;
  daily-mail)  sh "$ROOT/cloud-health-daily-mail/build.sh" all; link_dists ;;
  url)      sh "$ROOT/cloud-url-health-report/build.sh" all; link_reports; link_dists ;;
  sec-net)  sh "$ROOT/cloud-sec-network-report/build.sh" all; link_reports; link_dists ;;
  sec-data) sh "$ROOT/cloud-sec-data-report/build.sh" all; link_reports; link_dists ;;
  sec)
    sh "$ROOT/cloud-sec-network-report/build.sh" all
    sh "$ROOT/cloud-sec-data-report/build.sh" all
    link_reports
    link_dists
    generate_manifest
    ;;
  build)
    sh "$ROOT/cloud-stack-report/build.sh" build
    sh "$ROOT/cloud-health-full-report/build.sh" build
    sh "$ROOT/cloud-mail-full-report/build.sh" build
    sh "$ROOT/cloud-url-health-report/build.sh" build
    sh "$ROOT/cloud-sec-network-report/build.sh" build
    sh "$ROOT/cloud-sec-data-report/build.sh" build
    link_dists
    generate_manifest
    ;;
  dists)      link_dists ;;
  test-dists) test_dists ;;
  manifest)   generate_manifest ;;
  *)
    echo "Usage: $0 [all|stack|cloud|mail|url|sec-net|sec-data|sec|build|dists|test-dists|manifest]"
    echo ""
    echo "  all         Build + run all 6 reports (default)"
    echo "  stack       Cloud stack report"
    echo "  cloud       Cloud health full (10-layer)"
    echo "  mail        Cloud mail full (6-phase)"
    echo "  url         URL health (4-layer probe)"
    echo "  sec-net     Security: network scan"
    echo "  sec-data    Security: data scan (YARA + SIEM + threat intel)"
    echo "  sec         Both security reports"
    echo "  build       Build all without running"
    echo "  dists       Aggregate per-crate dist/ dirs under reports/dist/"
    echo "  test-dists  Verify reports/dist/ symlinks match expected layout"
    echo "  manifest    Regenerate reports/manifest.json"
    exit 1
    ;;
esac
