#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

usage() {
  echo "Usage: $0 [command] [project]"
  echo ""
  echo "Commands:"
  echo "  build       Build all (release)"
  echo "  run         Run all 3 reports"
  echo "  run-stack   Run cloud-stack-report only"
  echo "  run-cloud   Run cloud-health-full-report only"
  echo "  run-mail    Run cloud-mail-full-report only"
  echo "  clean       Remove target/"
  echo ""
  echo "No args = build + run all"
}

build() {
  echo "═══ Building all (release) ═══"
  cargo build --release
  echo "═══ Build done ═══"
}

run_one() {
  local dir="$1" bin="$2"
  echo ""
  echo "═══ Running $bin ═══"
  (cd "$ROOT/$dir" && "$ROOT/target/release/$bin")
}

run_all() {
  run_one cloud-stack-report       health-reporter
  run_one cloud-health-full-report cloud-health-full
  run_one cloud-mail-full-report   cloud-mail-full
}

CMD="${1:-all}"

case "$CMD" in
  build)     build ;;
  run)       run_all ;;
  run-stack) build; run_one cloud-stack-report       health-reporter ;;
  run-cloud) build; run_one cloud-health-full-report cloud-health-full ;;
  run-mail)  build; run_one cloud-mail-full-report   cloud-mail-full ;;
  clean)     rm -rf target/ */target/ ; echo "Cleaned." ;;
  all)       build; run_all ;;
  *)         usage; exit 1 ;;
esac
