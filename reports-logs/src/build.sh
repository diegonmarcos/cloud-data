#!/bin/sh
# ══════════════════════════════════════════════════════════════════════
#  reports-logs engine
# ══════════════════════════════════════════════════════════════════════
#  Universal collector for all debug evidence the reports need.
#  Declarative — all VMs, containers, units, domains, endpoints are
#  read from /home/diego/git/cloud-data/cloud-data-reports-logs.json
#  (with VM list cross-referenced against cloud-data-topology.json).
#
#  Usage:
#    build.sh all            # run every enabled module
#    build.sh <module>       # docker | systemd | network | dns | tls |
#                            # mail | cloudflare
#    build.sh list           # show modules + latest dist/ manifest
#    build.sh clean          # wipe dist/
#
#  Output tree:
#    dist/vms/<vm>/<kind>.{json,txt,log}
#    dist/containers/<svc>/<ctr>/{inspect.json,logs.txt}
#    dist/dns/<domain>.txt
#    dist/tls/<host>_<port>.txt
#    dist/mail/<command>.txt
#    dist/latest.json           (top-level manifest with timestamps)
# ══════════════════════════════════════════════════════════════════════

set -eu

ENGINE_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$(cd "${ENGINE_DIR}/.." && pwd)"
REPO_ROOT="$(cd "${CRATE_DIR}/.." && pwd)"
MODULES_DIR="${ENGINE_DIR}/modules"
CONFIG="${REPO_ROOT}/cloud-data-reports-logs.json"
TOPOLOGY="${REPO_ROOT}/cloud-data-topology.json"
DIST="${CRATE_DIR}/dist"

command -v jq >/dev/null 2>&1 || { echo "[reports-logs] jq required"; exit 2; }
[ -f "${CONFIG}" ] || { echo "[reports-logs] missing ${CONFIG}"; exit 2; }

cmd="${1:-all}"

# Resolve VM list once — source of truth is cloud-data-topology.json.
# A VM is eligible if it has an ssh alias AND is not explicitly disabled.
list_vms() {
    if [ -f "${TOPOLOGY}" ]; then
        jq -r '.vms // [] | map(select(.ssh_alias? != null)) | .[] | .alias // .id // empty' "${TOPOLOGY}" 2>/dev/null
    else
        jq -r '.modules.systemd.units_per_vm | keys[]' "${CONFIG}"
    fi
}

# Parallelism
VM_PAR=$(jq -r '.parallelism.vm_parallel // 4' "${CONFIG}")

ensure_dist() {
    mkdir -p "${DIST}/vms" "${DIST}/containers" "${DIST}/dns" "${DIST}/tls" "${DIST}/mail"
}

module_enabled() {
    jq -r ".modules.\"$1\".enabled // false" "${CONFIG}"
}

run_module() {
    mod="$1"
    script="${MODULES_DIR}/${mod}.sh"
    if [ ! -x "${script}" ]; then
        echo "[reports-logs] module ${mod} not found at ${script}"
        return 1
    fi
    if [ "$(module_enabled "${mod}")" != "true" ]; then
        echo "[reports-logs] ${mod}: disabled in ${CONFIG##*/}, skipping"
        return 0
    fi
    echo "══ ${mod} ══"
    CONFIG="${CONFIG}" TOPOLOGY="${TOPOLOGY}" DIST="${DIST}" \
        VM_PAR="${VM_PAR}" REPO_ROOT="${REPO_ROOT}" \
        sh "${script}"
}

write_manifest() {
    ensure_dist
    ts=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    jq -n \
        --arg ts "${ts}" \
        --arg cfg "${CONFIG##*/}" \
        --argjson vms "$(list_vms | jq -R . | jq -s .)" \
        '{
            generated: $ts,
            config: $cfg,
            vms: $vms,
            modules_enabled: (input | .modules | to_entries | map(select(.value.enabled == true)) | map(.key))
        }' "${CONFIG}" > "${DIST}/latest.json"
    echo "[reports-logs] wrote ${DIST}/latest.json"
}

case "${cmd}" in
    list)
        echo "== modules =="
        jq -r '.modules | to_entries[] | "  \(.key)\t enabled=\(.value.enabled)"' "${CONFIG}"
        echo ""
        echo "== VMs (from topology) =="
        list_vms | sed 's/^/  /'
        [ -f "${DIST}/latest.json" ] && echo "" && echo "== last manifest ==" && cat "${DIST}/latest.json"
        ;;
    clean)
        rm -rf "${DIST}"
        echo "[reports-logs] cleaned ${DIST}"
        ;;
    all)
        ensure_dist
        for m in docker systemd network dns tls mail cloudflare; do
            run_module "${m}" || echo "[reports-logs] ${m} reported errors (non-fatal)"
        done
        # index runs last — it scans everything else produced.
        sh "${MODULES_DIR}/index.sh"
        write_manifest
        ;;
    docker|systemd|network|dns|tls|mail|cloudflare)
        ensure_dist
        run_module "${cmd}"
        # Keep index fresh after any single module too.
        sh "${MODULES_DIR}/index.sh" 2>/dev/null || true
        write_manifest
        ;;
    index)
        ensure_dist
        CONFIG="${CONFIG}" TOPOLOGY="${TOPOLOGY}" DIST="${DIST}" sh "${MODULES_DIR}/index.sh"
        ;;
    kg_ingest)
        ensure_dist
        [ -f "${DIST}/index.json" ] || sh "${MODULES_DIR}/index.sh"
        KG_SCHEMA="${REPO_ROOT}/cloud-data-kg-schema.json"
        [ -f "${KG_SCHEMA}" ] || { echo "[kg_ingest] schema missing: ${KG_SCHEMA}"; exit 2; }
        CONFIG="${CONFIG}" TOPOLOGY="${TOPOLOGY}" DIST="${DIST}" \
            REPO_ROOT="${REPO_ROOT}" KG_SCHEMA="${KG_SCHEMA}" \
            sh "${MODULES_DIR}/kg_ingest.sh"
        ;;
    *)
        echo "usage: $0 [all|docker|systemd|network|dns|tls|mail|cloudflare|index|kg_ingest|list|clean]"
        exit 1
        ;;
esac
