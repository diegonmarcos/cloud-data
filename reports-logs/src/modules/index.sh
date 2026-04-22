#!/bin/sh
# index module — builds dist/index.json.
#
# Scans dist/ after other modules have run, derives tags from:
#   1. Path structure   (vms/<vm>/, containers/<svc>/<ctr>/, dns/, tls/, ...)
#   2. File content     (docker inspect → state, exit_code, arch, image)
#   3. Topology ref     (cloud-data-topology.json → public, category)
#   4. Config ref       (known_ephemeral_containers → ephemeral:true)
#
# Output format:
#   {
#     "generated": "<ISO-8601>",
#     "files":  { "<rel_path>": { "tags": ["vm:X","service:Y",...], "size": N } },
#     "tags":   { "vm:X": ["<path>", ...], ... }
#   }
#
# Usage:  jq '.tags["state:exited"]' dist/index.json
#         jq '.files | to_entries[] | select(.value.tags | contains(["service:photos-webhook"]))' dist/index.json
set -eu

: "${CONFIG:?}" "${TOPOLOGY:?}" "${DIST:?}"

OUT="${DIST}/index.json"
TMP=$(mktemp)
ENTRIES=$(mktemp)
: > "${ENTRIES}"

# --- Precompute topology lookups (service → {public,category,vm}) ---
TOP_SVC=$(mktemp)
if [ -f "${TOPOLOGY}" ]; then
    jq -r '
        .services // {} | to_entries[] as $s |
        [$s.key, ($s.value.public // false | tostring), ($s.value.category // ""), ($s.value.vm // $s.value.vm_alias // "") ] | @tsv
    ' "${TOPOLOGY}" > "${TOP_SVC}" 2>/dev/null || true
fi

# --- Precompute container → vm from collected docker_ps.json files ---
CTR_VM=$(mktemp)
for m in "${DIST}"/vms/*/meta.json; do
    [ -f "${m}" ] || continue
    vm=$(jq -r '.vm' "${m}" 2>/dev/null)
    ps="${DIST}/vms/${vm}/docker_ps.json"
    [ -f "${ps}" ] && jq -r --arg vm "${vm}" '.[] | [.Names, $vm] | @tsv' "${ps}" >> "${CTR_VM}"
done

# --- Precompute ephemeral containers ---
EPHEM=$(jq -r '.known_ephemeral_containers[]? // empty' "${CONFIG}" | tr '\n' '|' | sed 's/|$//')

tag_of_topology_service() {
    svc="$1"
    [ -f "${TOP_SVC}" ] || return
    line=$(awk -F'\t' -v s="${svc}" '$1==s {print; exit}' "${TOP_SVC}" 2>/dev/null || true)
    [ -z "${line}" ] && return
    pub=$(printf '%s\n' "${line}" | cut -f2)
    cat=$(printf '%s\n' "${line}" | cut -f3)
    tvm=$(printf '%s\n' "${line}" | cut -f4)
    [ -n "${pub}" ] && printf 'public:%s\n' "${pub}"
    [ -n "${cat}" ] && printf 'category:%s\n' "${cat}"
    [ -n "${tvm}" ] && printf 'vm:%s\n' "${tvm}"
}

vm_of_container() {
    ctr="$1"
    [ -f "${CTR_VM}" ] || return
    awk -F'\t' -v c="${ctr}" '$1==c {print $2; exit}' "${CTR_VM}" 2>/dev/null || true
}

# --- Walk files and emit {path, tags[], size} JSON per line ---
find "${DIST}" -type f ! -name 'index.json' ! -name 'latest.json' | sort | while IFS= read -r abs; do
    rel=${abs#${DIST}/}
    size=$(stat -c%s "${abs}" 2>/dev/null || echo 0)
    tags=""

    case "${rel}" in
        vms/*/docker_ps.json)
            vm=$(echo "${rel}" | cut -d/ -f2)
            tags="vm:${vm} kind:vm source:docker"
            ;;
        vms/*/meta.json)
            vm=$(echo "${rel}" | cut -d/ -f2)
            tags="vm:${vm} kind:vm source:docker"
            reach=$(jq -r '.reachable // true' "${abs}" 2>/dev/null)
            [ "${reach}" = "false" ] && tags="${tags} state:unreachable"
            ;;
        vms/*/systemd_failed.txt)
            vm=$(echo "${rel}" | cut -d/ -f2)
            tags="vm:${vm} kind:vm source:systemd"
            [ -s "${abs}" ] && tags="${tags} failed:true"
            ;;
        vms/*/journal_*.log)
            vm=$(echo "${rel}" | cut -d/ -f2)
            tags="vm:${vm} kind:vm source:systemd"
            ;;
        vms/*/network.txt)
            vm=$(echo "${rel}" | cut -d/ -f2)
            tags="vm:${vm} kind:vm source:network"
            ;;
        containers/*/*/inspect.json)
            svc=$(echo "${rel}" | cut -d/ -f2)
            ctr=$(echo "${rel}" | cut -d/ -f3)
            tags="service:${svc} container:${ctr} kind:container source:docker"
            cvm=$(vm_of_container "${ctr}")
            [ -n "${cvm}" ] && tags="${tags} vm:${cvm}"
            # Content-derived tags
            if [ -s "${abs}" ]; then
                state=$(jq -r '.[0].State.Status // ""' "${abs}" 2>/dev/null)
                exit_code=$(jq -r '.[0].State.ExitCode // 0' "${abs}" 2>/dev/null)
                arch=$(jq -r '.[0].Architecture // .[0].Platform // ""' "${abs}" 2>/dev/null)
                vm_host=$(jq -r '.[0].Config.Hostname // ""' "${abs}" 2>/dev/null)
                health=$(jq -r '.[0].State.Health.Status // ""' "${abs}" 2>/dev/null)
                image=$(jq -r '.[0].Config.Image // ""' "${abs}" 2>/dev/null)
                [ -n "${state}" ] && tags="${tags} state:${state}"
                [ "${exit_code}" != "0" ] && tags="${tags} exit_code:${exit_code} failed:true"
                [ -n "${arch}" ] && [ "${arch}" != "linux" ] && tags="${tags} arch:${arch}"
                [ -n "${health}" ] && tags="${tags} healthy:${health}"
                [ -n "${image}" ] && tags="${tags} image:${image}"
            fi
            # Ephemeral?
            if [ -n "${EPHEM}" ] && echo "${ctr}" | grep -qE "^(${EPHEM})$"; then
                tags="${tags} ephemeral:true"
            fi
            # Topology cross-ref
            for t in $(tag_of_topology_service "${svc}"); do
                tags="${tags} ${t}"
            done
            ;;
        containers/*/*/logs.txt)
            svc=$(echo "${rel}" | cut -d/ -f2)
            ctr=$(echo "${rel}" | cut -d/ -f3)
            tags="service:${svc} container:${ctr} kind:container source:docker"
            cvm=$(vm_of_container "${ctr}")
            [ -n "${cvm}" ] && tags="${tags} vm:${cvm}"
            ;;
        dns/*)
            dom=$(basename "${rel}" .txt)
            tags="kind:dns source:dns domain:${dom}"
            ;;
        tls/*)
            stem=$(basename "${rel}" .txt)
            tags="kind:tls source:tls endpoint:${stem}"
            ;;
        mail/*)
            stem=$(basename "${rel}" .txt)
            tags="kind:mail source:mail probe:${stem}"
            ;;
        cloudflare/*)
            stem=$(basename "${rel}" .json)
            tags="kind:cloudflare source:cloudflare record_type:${stem}"
            ;;
        *)
            tags="kind:other"
            ;;
    esac

    # Emit one JSON record per file
    printf '%s\n' "${tags}" | tr ' ' '\n' | grep -v '^$' | jq -R . | jq -s \
        --arg path "${rel}" --argjson size "${size}" \
        '{path: $path, tags: ., size: $size}' >> "${ENTRIES}"
done

# --- Build final index.json: files{} + inverted tags{} ---
jq -s --arg ts "$(date -u +%FT%TZ)" '
    {
        generated: $ts,
        files:  (map({ (.path): { tags: .tags, size: .size } }) | add // {}),
        tags:   (
            reduce .[] as $e ({};
                reduce $e.tags[] as $t (.;
                    .[$t] = ((.[$t] // []) + [$e.path])
                )
            )
        )
    }
' "${ENTRIES}" > "${OUT}"

rm -f "${TMP}" "${ENTRIES}" "${TOP_SVC}" "${CTR_VM}"

files_count=$(jq '.files | length' "${OUT}")
tags_count=$(jq '.tags | length' "${OUT}")
echo "[index] done — ${files_count} files, ${tags_count} distinct tags → ${OUT}"
