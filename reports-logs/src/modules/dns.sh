#!/bin/sh
# dns module — resolves every declared domain against every declared
# resolver and writes per-domain snapshots.
set -eu

: "${CONFIG:?}" "${DIST:?}"

resolvers=$(jq -r '.modules.dns.resolvers[]' "${CONFIG}")
domains=$(jq -r '.modules.dns.domains[]' "${CONFIG}")
types=$(jq -r '.modules.dns.record_types[]' "${CONFIG}")

command -v dig >/dev/null 2>&1 || { echo "[dns] dig not found — skip"; exit 0; }

for d in ${domains}; do
    safe=$(echo "${d}" | tr '/' '_')
    f="${DIST}/dns/${safe}.txt"
    : > "${f}"
    printf '# %s\n' "${d}" >> "${f}"
    for r in ${resolvers}; do
        for t in ${types}; do
            {
                printf '\n## @%s %s %s\n' "${r}" "${d}" "${t}"
                dig +short +time=3 +tries=1 "@${r}" "${d}" "${t}" 2>/dev/null || echo "(timeout)"
            } >> "${f}"
        done
    done
done

echo "[dns] done — $(echo "${domains}" | wc -w) domains × $(echo "${resolvers}" | wc -w) resolvers → ${DIST}/dns/"
