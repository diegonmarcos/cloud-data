#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")" && pwd)"

target="${1:-all}"

case "$target" in
  all)
    sh "$ROOT/cloud-stack-report/build.sh" all
    sh "$ROOT/cloud-health-full-report/build.sh" all
    sh "$ROOT/cloud-mail-full-report/build.sh" all
    ;;
  stack)  sh "$ROOT/cloud-stack-report/build.sh" all ;;
  cloud)  sh "$ROOT/cloud-health-full-report/build.sh" all ;;
  mail)   sh "$ROOT/cloud-mail-full-report/build.sh" all ;;
  build)
    sh "$ROOT/cloud-stack-report/build.sh" build
    sh "$ROOT/cloud-health-full-report/build.sh" build
    sh "$ROOT/cloud-mail-full-report/build.sh" build
    ;;
  *)
    echo "Usage: $0 [all|stack|cloud|mail|build]"
    echo ""
    echo "  all    Build + run all 3 reports (default)"
    echo "  stack  Cloud stack report"
    echo "  cloud  Cloud health full (10-layer)"
    echo "  mail   Cloud mail full (6-phase)"
    echo "  build  Build all without running"
    exit 1
    ;;
esac
