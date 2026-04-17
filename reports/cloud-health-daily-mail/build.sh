#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE="$(cd "$ROOT/.." && pwd)"
BINARY="cloud-health-daily-mail"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cargo/target}"

build() {
  if command -v "$BINARY" >/dev/null 2>&1; then
    echo "═══ $BINARY found in PATH — skipping build ═══"
    return 0
  fi
  echo "═══ Building $BINARY ═══"
  cargo build --release --manifest-path "$WORKSPACE/Cargo.toml" -p "$BINARY"

  rm -rf "$ROOT/dist"
  mkdir -p "$ROOT/dist"
  ln -sf "$CARGO_TARGET_DIR/release/$BINARY" "$ROOT/dist/$BINARY"
  echo "→ dist/$BINARY"
}

html() {
  echo "═══ Generating HTML report → dist/cloud_health_daily.html ═══"
  if command -v "$BINARY" >/dev/null 2>&1; then
    (cd "$ROOT" && "$BINARY")
  else
    (cd "$ROOT" && "$ROOT/dist/$BINARY")
  fi
}

send() {
  echo "═══ Sending report via SMTP ═══"
  sh "$ROOT/src/send.sh" "$ROOT/dist/cloud_health_daily.html"
}

case "${1:-all}" in
  build) build ;;
  html)  html ;;
  send)  send ;;
  run)   build; html ;;
  all)   build; html ;;
  ship)  build; html; send ;;
  *)     echo "Usage: $0 [build|html|send|run|all|ship]"; exit 1 ;;
esac
