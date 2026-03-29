#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"

build_one() {
  echo "═══ Building $2 ═══"
  cargo build --release --manifest-path "$ROOT/$1/Cargo.toml"
}

run_one() {
  echo ""
  echo "═══ Running $2 ═══"
  (cd "$ROOT/$1" && "$ROOT/$1/target/release/$2")
}

do_project() {
  build_one "$1" "$2"
  run_one "$1" "$2"
}

target="${1:-all}"

case "$target" in
  all)
    do_project cloud-stack-report       health-reporter
    do_project cloud-health-full-report cloud-health-full
    do_project cloud-mail-full-report   cloud-mail-full
    ;;
  stack) do_project cloud-stack-report       health-reporter ;;
  cloud) do_project cloud-health-full-report cloud-health-full ;;
  mail)  do_project cloud-mail-full-report   cloud-mail-full ;;
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
