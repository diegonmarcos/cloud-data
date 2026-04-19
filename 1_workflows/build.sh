#!/bin/sh
# ╔══════════════════════════════════════════════════════════════════╗
# ║ cloud-data workflows — build + deploy                          ║
# ║                                                                  ║
# ║ build:  src/ → dist/ (1:1 copy)                                 ║
# ║ deploy: dist/ → repo targets (.github/, .gitconfig, etc.)       ║
# ║                                                                  ║
# ║ Usage: ./build.sh [build|deploy|all]                             ║
# ╚══════════════════════════════════════════════════════════════════╝
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC_DIR="$SCRIPT_DIR/src"
DIST_DIR="$SCRIPT_DIR/dist"

log() { printf "[%s] %s\n" "$(date '+%H:%M:%S')" "$1"; }

build() {
    rm -rf "$DIST_DIR"
    cp -a "$SRC_DIR" "$DIST_DIR"
    log "Built: src/ → dist/"
}

test_stage() {
    # Auto-discover and run every src/scripts/test-*.sh.
    # Each tester is self-contained, read-only, exits non-zero on failure.
    [ -d "$SRC_DIR/scripts" ] || { log "No src/scripts/ — skipping tests"; return 0; }
    set +e
    fails=0
    for t in "$SRC_DIR/scripts/"test-*.sh; do
        [ -f "$t" ] || continue
        log "TEST: $(basename "$t")"
        bash "$t"
        rc=$?
        if [ $rc -ne 0 ]; then
            log "FAIL: $(basename "$t") (exit $rc)"
            fails=$((fails + 1))
        fi
    done
    set -e
    [ $fails -eq 0 ] || { log "Tests failed: $fails"; exit 1; }
    log "All tests passed"
}

deploy() {
    [ -d "$DIST_DIR" ] || { log "No dist/ — run build first"; exit 1; }

    # cicd/*.yml → .github/workflows/
    TARGET="$REPO_ROOT/.github/workflows"
    mkdir -p "$TARGET"
    for f in "$DIST_DIR/cicd/"*.yml; do
        [ -f "$f" ] || continue
        cp "$f" "$TARGET/"
    done
    log "Deployed $(ls "$DIST_DIR/cicd/"*.yml 2>/dev/null | wc -l) workflow(s) → .github/workflows/"

    # scripts/ → .github/workflows/scripts/
    if [ -d "$DIST_DIR/scripts" ]; then
        mkdir -p "$TARGET/scripts"
        cp -r "$DIST_DIR/scripts/"* "$TARGET/scripts/"
        chmod +x "$TARGET/scripts/"* 2>/dev/null || true
        log "Deployed $(ls "$DIST_DIR/scripts/"* 2>/dev/null | wc -l) script(s) → .github/workflows/scripts/"
    fi

    # hooks/ → .github/workflows/hooks/
    if [ -d "$DIST_DIR/hooks" ]; then
        mkdir -p "$TARGET/hooks"
        cp -r "$DIST_DIR/hooks/"* "$TARGET/hooks/"
        chmod +x "$TARGET/hooks/"* 2>/dev/null || true
        log "Deployed hooks → .github/workflows/hooks/"
    fi

    # actions/ → .github/actions/
    if [ -d "$DIST_DIR/actions" ]; then
        mkdir -p "$REPO_ROOT/.github/actions"
        cp -r "$DIST_DIR/actions/"* "$REPO_ROOT/.github/actions/"
        log "Deployed actions → .github/actions/"
    fi

    # gitconfig → .gitconfig
    if [ -f "$DIST_DIR/gitconfig" ]; then
        cp "$DIST_DIR/gitconfig" "$REPO_ROOT/.gitconfig"
        log "Deployed gitconfig → .gitconfig"
    fi

    # modules/gitmodules → .gitmodules
    if [ -f "$DIST_DIR/modules/gitmodules" ]; then
        cp "$DIST_DIR/modules/gitmodules" "$REPO_ROOT/.gitmodules"
        log "Deployed gitmodules → .gitmodules"
    fi

    log "Done"
}

case "${1:-all}" in
    build)  build ;;
    test)   test_stage ;;
    deploy) deploy ;;
    all)    build; test_stage; deploy ;;
    *)      echo "Usage: $0 [build|test|deploy|all]" ;;
esac
