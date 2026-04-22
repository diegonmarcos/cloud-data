#!/bin/sh
# cloudflare module — zone + DNS records snapshot via `cloudflare-cli`
# OR `gh api cloudflare.com`. Requires CF_API_TOKEN from the vault.
# Purely data-driven: zone + record types come from the config.
set -eu

: "${CONFIG:?}" "${DIST:?}"

zone=$(jq -r '.modules.cloudflare.zone // "diegonmarcos.com"' "${CONFIG}")
types=$(jq -r '.modules.cloudflare.records_of_interest[]' "${CONFIG}")

out="${DIST}/cloudflare"
mkdir -p "${out}"

# Token lookup: env → vault fallback
CF_TOKEN="${CF_API_TOKEN:-}"
if [ -z "${CF_TOKEN}" ] && [ -f "${HOME}/git/vault/A0_keys/providers/cloudflare/api_token" ]; then
    CF_TOKEN=$(cat "${HOME}/git/vault/A0_keys/providers/cloudflare/api_token" 2>/dev/null || true)
fi

if [ -z "${CF_TOKEN}" ]; then
    echo "[cloudflare] CF_API_TOKEN not set and vault file not found — skip"
    printf '{"skipped":true,"reason":"CF_API_TOKEN missing"}\n' > "${out}/status.json"
    exit 0
fi

# Zone ID
zone_id=$(curl -sS -H "Authorization: Bearer ${CF_TOKEN}" \
    "https://api.cloudflare.com/client/v4/zones?name=${zone}" \
    | jq -r '.result[0].id // empty')

if [ -z "${zone_id}" ]; then
    echo "[cloudflare] zone ${zone} not found"
    exit 1
fi

# All records (one fetch, then filter locally by type)
curl -sS -H "Authorization: Bearer ${CF_TOKEN}" \
    "https://api.cloudflare.com/client/v4/zones/${zone_id}/dns_records?per_page=200" \
    > "${out}/all_records.json"

for t in ${types}; do
    jq --arg t "${t}" '.result | map(select(.type == $t))' "${out}/all_records.json" \
        > "${out}/${t}.json"
done

echo "[cloudflare] zone=${zone} records → ${out}/"
