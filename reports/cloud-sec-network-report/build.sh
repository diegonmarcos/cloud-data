#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE="$(cd "$ROOT/.." && pwd)"
BINARY="cloud-sec-network"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cargo/target}"

build() {
  echo "═══ Building $BINARY ═══"
  cargo build --release --manifest-path "$WORKSPACE/Cargo.toml" -p "$BINARY"

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
  (cd "$ROOT" && "$ROOT/dist/$BINARY")
}

case "${1:-all}" in
  build) build ;;
  run)   run ;;
  all)   build; run ;;
  *)     echo "Usage: $0 [build|run|all]"; exit 1 ;;
esac
