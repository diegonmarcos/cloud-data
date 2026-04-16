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

target="${1:-all}"

case "$target" in
  all)
    sh "$ROOT/cloud-stack-report/build.sh" all
    sh "$ROOT/cloud-health-full-report/build.sh" all
    sh "$ROOT/cloud-mail-full-report/build.sh" all
    sh "$ROOT/cloud-url-health-report/build.sh" all
    sh "$ROOT/cloud-sec-network-report/build.sh" all
    sh "$ROOT/cloud-sec-data-report/build.sh" all
    link_reports
    generate_manifest
    ;;
  stack)    sh "$ROOT/cloud-stack-report/build.sh" all; link_reports ;;
  cloud)    sh "$ROOT/cloud-health-full-report/build.sh" all; link_reports ;;
  mail)     sh "$ROOT/cloud-mail-full-report/build.sh" all; link_reports ;;
  url)      sh "$ROOT/cloud-url-health-report/build.sh" all; link_reports ;;
  sec-net)  sh "$ROOT/cloud-sec-network-report/build.sh" all; link_reports ;;
  sec-data) sh "$ROOT/cloud-sec-data-report/build.sh" all; link_reports ;;
  sec)
    sh "$ROOT/cloud-sec-network-report/build.sh" all
    sh "$ROOT/cloud-sec-data-report/build.sh" all
    link_reports
    generate_manifest
    ;;
  build)
    sh "$ROOT/cloud-stack-report/build.sh" build
    sh "$ROOT/cloud-health-full-report/build.sh" build
    sh "$ROOT/cloud-mail-full-report/build.sh" build
    sh "$ROOT/cloud-url-health-report/build.sh" build
    sh "$ROOT/cloud-sec-network-report/build.sh" build
    sh "$ROOT/cloud-sec-data-report/build.sh" build
    generate_manifest
    ;;
  manifest) generate_manifest ;;
  *)
    echo "Usage: $0 [all|stack|cloud|mail|url|sec-net|sec-data|sec|build|manifest]"
    echo ""
    echo "  all       Build + run all 6 reports (default)"
    echo "  stack     Cloud stack report"
    echo "  cloud     Cloud health full (10-layer)"
    echo "  mail      Cloud mail full (6-phase)"
    echo "  url       URL health (4-layer probe)"
    echo "  sec-net   Security: network scan"
    echo "  sec-data  Security: data scan (YARA + SIEM + threat intel)"
    echo "  sec       Both security reports"
    echo "  build     Build all without running"
    echo "  manifest  Regenerate reports/manifest.json"
    exit 1
    ;;
esac
