#!/bin/sh
# _crate_engine.sh — universal engine for report crates.
#
# Per-crate build.sh must set (before sourcing):
#   ROOT=<absolute crate root>
#   BINARY=<cargo bin name>
# Optional:
#   SUPPORT_DIRS="yara-rules cache"   (space-separated; symlinked into the shared dist)
#   TEMPLATES_GLOB="*.md.tpl"         (default)
#
# Then call: engine_dispatch "$@"
#
# Single shared output dir — reports/dist/:
#   bin/<BINARY>            (symlink to release binary)
#   bin/deps/*.rlib         (shared dep rlibs)
#   <name>.md.tpl           (template symlinks → ../src/<crate>/<name>.md.tpl)
#   <name>.md / .json / .html   (binary output — cwd=dist when running)
#
# Support dirs are symlinked flat (yara-rules) or exposed via env vars
# (CACHE_DIR) to avoid collisions between crates.

set -eu

: "${ROOT:?ROOT must be set to crate root}"
: "${BINARY:?BINARY must be set to cargo bin name}"

# ROOT = reports/src/<crate>
# WORKSPACE = reports/src   (Cargo.toml lives here)
# REPORTS_ROOT = reports/
# REPORTS_DIST = reports/dist   (SHARED output dir for all crates)
ENGINE_WORKSPACE="$(cd "$ROOT/.." && pwd)"
REPORTS_ROOT="$(cd "$ENGINE_WORKSPACE/.." && pwd)"
REPORTS_DIST="$REPORTS_ROOT/dist"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cargo/target}"
: "${SUPPORT_DIRS:=}"
: "${TEMPLATES_GLOB:=*.md.tpl}"

engine_build() {
    if [ -f /opt/reports/entrypoint.sh ]; then
        echo "═══ Docker image — using pre-built $BINARY ═══"
    else
        echo "═══ Building $BINARY ═══"
        cargo build --release --manifest-path "$ENGINE_WORKSPACE/Cargo.toml" -p "$BINARY"
    fi

    mkdir -p "$REPORTS_DIST/bin/deps"

    if [ -e "$CARGO_TARGET_DIR/release/$BINARY" ]; then
        ln -sf "$CARGO_TARGET_DIR/release/$BINARY" "$REPORTS_DIST/bin/$BINARY"
        for f in "$CARGO_TARGET_DIR/release/deps"/lib*.rlib; do
            [ -f "$f" ] && ln -sf "$f" "$REPORTS_DIST/bin/deps/"
        done
    fi

    # Templates stay in src/ — resolved at run time via $TEMPLATE_DIR (see engine_run).
    # No symlinks in dist/ for source files.

    # Support dirs — yara-rules is unique; symlink flat.
    # cache/ is exposed via CACHE_DIR env var at run time (see engine_run).
    for sd in $SUPPORT_DIRS; do
        [ -d "$ROOT/$sd" ] || continue
        case "$sd" in
            cache) : ;;  # exposed via env var, not symlinked
            *)     ln -sfn "../src/$(basename "$ROOT")/$sd" "$REPORTS_DIST/$sd" ;;
        esac
    done

    echo "→ dist/bin/$BINARY"
}

engine_run() {
    mkdir -p "$REPORTS_DIST"
    # Source-side paths exposed via env vars so the binary (cwd=dist) can resolve them.
    export TEMPLATE_DIR="$ROOT"
    if [ -d "$ROOT/cache" ]; then
        export CACHE_DIR="$ROOT/cache"
    fi
    echo ""
    echo "═══ Running $BINARY (cwd=reports/dist/) ═══"
    if [ -x "$REPORTS_DIST/bin/$BINARY" ]; then
        (cd "$REPORTS_DIST" && "$REPORTS_DIST/bin/$BINARY")
    elif command -v "$BINARY" >/dev/null 2>&1; then
        (cd "$REPORTS_DIST" && "$BINARY")
    elif [ -x "$CARGO_TARGET_DIR/release/$BINARY" ]; then
        (cd "$REPORTS_DIST" && "$CARGO_TARGET_DIR/release/$BINARY")
    else
        echo "ERROR: $BINARY not found (dist/bin / PATH / cargo target)" >&2
        exit 1
    fi
}

engine_dispatch() {
    case "${1:-all}" in
        build) engine_build ;;
        run)   engine_run ;;
        all)   engine_build; engine_run ;;
        *)
            if [ -n "${ENGINE_EXTRA_DISPATCH:-}" ] && command -v "$ENGINE_EXTRA_DISPATCH" >/dev/null 2>&1; then
                "$ENGINE_EXTRA_DISPATCH" "$@"
            else
                echo "Usage: $(basename "$0") [build|run|all]"; exit 1
            fi
            ;;
    esac
}
