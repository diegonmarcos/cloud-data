#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE="$(cd "$ROOT/.." && pwd)"
BINARY="health-reporter"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cargo/target}"

build() {
  # In Docker: binary is baked into image at /usr/local/bin — skip cargo
  if [ -f /opt/reports/entrypoint.sh ]; then
    echo "═══ Docker image — using pre-built $BINARY ═══"
    return 0
  fi
  echo "═══ Building $BINARY ═══"
  cargo build --release --manifest-path "$WORKSPACE/Cargo.toml" -p "$BINARY"

  # Create dist/ with symlinks to binary and its .rlib deps
  rm -rf "$ROOT/dist"
  mkdir -p "$ROOT/dist/deps"
  ln -sf "$CARGO_TARGET_DIR/release/$BINARY" "$ROOT/dist/$BINARY"
  for f in "$CARGO_TARGET_DIR/release/deps"/lib*.rlib; do
    [ -f "$f" ] && ln -sf "$f" "$ROOT/dist/deps/"
  done
  echo "→ dist/$BINARY + dist/deps/"
}

run() {
  echo ""
  echo "═══ Running $BINARY ═══"
  # Prefer: PATH (container-installed) > dist/ resolved symlink > cargo target.
  # -x dereferences symlinks so a dangling dist/$BINARY symlink falls through
  # instead of being executed (which just yields "not found").
  if command -v "$BINARY" >/dev/null 2>&1; then
    (cd "$ROOT" && "$BINARY")
  elif [ -x "$ROOT/dist/$BINARY" ]; then
    (cd "$ROOT" && "$ROOT/dist/$BINARY")
  elif [ -x "$CARGO_TARGET_DIR/release/$BINARY" ]; then
    (cd "$ROOT" && "$CARGO_TARGET_DIR/release/$BINARY")
  else
    echo "ERROR: $BINARY not found" >&2; exit 1
  fi
}

case "${1:-all}" in
  build) build ;;
  run)   run ;;
  all)   build; run ;;
  *)     echo "Usage: $0 [build|run|all]"; exit 1 ;;
esac
