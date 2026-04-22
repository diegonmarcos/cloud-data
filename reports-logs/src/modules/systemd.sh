#!/bin/sh
# systemd module — collects journalctl per declared unit + list of failed
# units per VM. Units to collect come from cloud-data-reports-logs.json
# `modules.systemd.units_per_vm`, keyed by SSH alias.
set -eu

: "${CONFIG:?}" "${DIST:?}"

LINES=$(jq -r '.timeouts.systemd_log_lines // 200' "${CONFIG}")
INCL_FAILED=$(jq -r '.modules.systemd.include_failed_units // true' "${CONFIG}")

vms=$(jq -r '.modules.systemd.units_per_vm | keys[]' "${CONFIG}")

for vm in ${vms}; do
    out="${DIST}/vms/${vm}"
    mkdir -p "${out}"
    if ! ssh -o BatchMode=yes -o ConnectTimeout=3 "${vm}" true 2>/dev/null; then
        echo "  ${vm}: unreachable"
        continue
    fi

    if [ "${INCL_FAILED}" = "true" ]; then
        ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
            "systemctl --failed --no-pager --plain 2>/dev/null | head -60" \
            > "${out}/systemd_failed.txt" 2>/dev/null || true
    fi

    units=$(jq -r --arg vm "${vm}" '.modules.systemd.units_per_vm[$vm][]?' "${CONFIG}")
    for u in ${units}; do
        safe=$(echo "${u}" | tr '/@.:' '_')
        ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
            "journalctl -u ${u} -n ${LINES} --no-pager 2>/dev/null" \
            > "${out}/journal_${safe}.log" 2>/dev/null || true
    done
    echo "  ${vm}: $(echo "${units}" | wc -w) units"
done

echo "[systemd] done — outputs in ${DIST}/vms/*/journal_*.log"
