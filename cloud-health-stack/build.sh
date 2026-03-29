#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

run_one() {
  dir="$1"; bin="$2"
  echo ""
  echo "═══ Running $bin ═══"
  (cd "$ROOT/$dir" && "$ROOT/target/release/$bin")
}

target="${1:-all}"
cargo build --release

case "$target" in
  all)
    run_one cloud-stack-report       health-reporter
    run_one cloud-health-full-report cloud-health-full
    run_one cloud-mail-full-report   cloud-mail-full
    ;;
  stack) run_one cloud-stack-report       health-reporter ;;
  cloud) run_one cloud-health-full-report cloud-health-full ;;
  mail)  run_one cloud-mail-full-report   cloud-mail-full ;;
  *)
    echo "Usage: $0 [all|stack|cloud|mail]"
    echo ""
    echo "  all    Run all 3 reports (default)"
    echo "  stack  Cloud stack report"
    echo "  cloud  Cloud health full (10-layer)"
    echo "  mail   Cloud mail full (6-phase)"
    exit 1
    ;;
esac
