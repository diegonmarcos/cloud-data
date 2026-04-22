#!/bin/sh
# docker module — per-VM docker ps + per-container inspect + logs.
# Reads list of VMs from cloud-data-topology.json, list of containers
# from `docker ps -a` on each VM (not hardcoded).
set -eu

: "${CONFIG:?}" "${TOPOLOGY:?}" "${DIST:?}" "${VM_PAR:?}"

LOG_LINES=$(jq -r '.timeouts.docker_log_lines // 500' "${CONFIG}")
PS_ALL=$(jq -r '.modules.docker.ps_all // true' "${CONFIG}")
COLLECT_INSPECT=$(jq -r '.modules.docker.collect_inspect // true' "${CONFIG}")
COLLECT_LOGS=$(jq -r '.modules.docker.collect_logs // true' "${CONFIG}")

# VM list from topology, fall back to systemd.units_per_vm keys.
vms=$(jq -r '.vms // [] | map(select(.ssh_alias? != null)) | .[] | .ssh_alias // .alias // .id // empty' "${TOPOLOGY}" 2>/dev/null \
    || jq -r '.modules.systemd.units_per_vm | keys[]' "${CONFIG}")

collect_vm() {
    vm="$1"
    out="${DIST}/vms/${vm}"
    mkdir -p "${out}"
    # Test reachability first (1-second SSH probe); if unreachable, mark and skip.
    if ! ssh -o BatchMode=yes -o ConnectTimeout=3 "${vm}" true 2>/dev/null; then
        printf '{"vm":"%s","reachable":false,"ts":"%s"}\n' "${vm}" "$(date -u +%FT%TZ)" > "${out}/meta.json"
        echo "  ${vm}: unreachable — skipped"
        return
    fi
    ps_flag=""
    [ "${PS_ALL}" = "true" ] && ps_flag="--all"
    # docker ps -a in JSON
    ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
        "docker ps ${ps_flag} --format '{{json .}}'" 2>/dev/null | jq -s . > "${out}/docker_ps.json" || \
        echo "[]" > "${out}/docker_ps.json"

    printf '{"vm":"%s","reachable":true,"ts":"%s","container_count":%d}\n' \
        "${vm}" "$(date -u +%FT%TZ)" "$(jq 'length' "${out}/docker_ps.json")" > "${out}/meta.json"
    echo "  ${vm}: $(jq 'length' "${out}/docker_ps.json") containers"

    # Per-container inspect + logs
    if [ "${COLLECT_INSPECT}" = "true" ] || [ "${COLLECT_LOGS}" = "true" ]; then
        jq -r '.[].Names' "${out}/docker_ps.json" | while IFS= read -r ctr; do
            [ -z "${ctr}" ] && continue
            # Pick compose project label as service; fall back to container name.
            svc=$(jq -r --arg n "${ctr}" '.[] | select(.Names==$n) | .Labels' "${out}/docker_ps.json" \
                | tr ',' '\n' | awk -F= '/com.docker.compose.project=/{print $2}' | head -1)
            [ -z "${svc}" ] && svc="${ctr}"
            cdir="${DIST}/containers/${svc}/${ctr}"
            mkdir -p "${cdir}"
            if [ "${COLLECT_INSPECT}" = "true" ]; then
                ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
                    "docker inspect ${ctr}" 2>/dev/null > "${cdir}/inspect.json" || true
            fi
            if [ "${COLLECT_LOGS}" = "true" ]; then
                ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
                    "docker logs --tail ${LOG_LINES} ${ctr} 2>&1" > "${cdir}/logs.txt" 2>/dev/null || true
            fi
        done
    fi
}

# Parallel per-VM collection (bounded by VM_PAR).
pids=""
count=0
for vm in ${vms}; do
    collect_vm "${vm}" &
    pids="${pids} $!"
    count=$((count + 1))
    if [ "${count}" -ge "${VM_PAR}" ]; then
        wait
        count=0
    fi
done
wait

echo "[docker] done — outputs in ${DIST}/{vms,containers}/"
