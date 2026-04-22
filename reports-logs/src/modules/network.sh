#!/bin/sh
# network module — snapshot of listeners, interfaces, routes, WG, firewall
# per VM. Commands come from cloud-data-reports-logs.json
# `modules.network.commands`.
set -eu

: "${CONFIG:?}" "${DIST:?}"

vms=$(jq -r '.modules.systemd.units_per_vm | keys[]' "${CONFIG}")

# Commands to run — each run in isolation so one failure doesn't abort the rest.
cmds=$(jq -r '.modules.network.commands[]' "${CONFIG}")

for vm in ${vms}; do
    out="${DIST}/vms/${vm}"
    mkdir -p "${out}"
    if ! ssh -o BatchMode=yes -o ConnectTimeout=3 "${vm}" true 2>/dev/null; then
        echo "  ${vm}: unreachable"
        continue
    fi
    : > "${out}/network.txt"
    echo "${cmds}" | while IFS= read -r c; do
        [ -z "${c}" ] && continue
        {
            printf '\n# === %s ===\n' "${c}"
            ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
                "sudo -n ${c} 2>/dev/null || ${c} 2>/dev/null || echo '(command failed)'"
        } >> "${out}/network.txt"
    done
    echo "  ${vm}: network snapshot"
done

echo "[network] done — outputs in ${DIST}/vms/*/network.txt"
