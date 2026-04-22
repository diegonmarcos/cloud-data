#!/bin/sh
# tls module — openssl s_client snapshot per declared endpoint.
# Captures cert chain, cipher, protocol version, subject, SAN, issuer.
# Declared endpoints come from cloud-data-reports-logs.json
# `modules.tls.endpoints`.
set -eu

: "${CONFIG:?}" "${DIST:?}"

command -v openssl >/dev/null 2>&1 || { echo "[tls] openssl not found — skip"; exit 0; }

n=$(jq -r '.modules.tls.endpoints | length' "${CONFIG}")
i=0
while [ "${i}" -lt "${n}" ]; do
    host=$(jq -r ".modules.tls.endpoints[${i}].host" "${CONFIG}")
    port=$(jq -r ".modules.tls.endpoints[${i}].port" "${CONFIG}")
    sni=$(jq -r ".modules.tls.endpoints[${i}].sni // \"\"" "${CONFIG}")
    starttls=$(jq -r ".modules.tls.endpoints[${i}].starttls // \"\"" "${CONFIG}")

    f="${DIST}/tls/${host}_${port}.txt"
    safe_host=$(echo "${host}" | tr '/' '_')
    f="${DIST}/tls/${safe_host}_${port}.txt"
    : > "${f}"

    args="-connect ${host}:${port} -showcerts -servername ${sni:-$host}"
    [ -n "${starttls}" ] && args="${args} -starttls ${starttls}"

    {
        printf '# %s:%s  sni=%s starttls=%s\n\n' "${host}" "${port}" "${sni}" "${starttls:-none}"
        # Stream "" to close after handshake; 5s timeout.
        echo "" | timeout 8 openssl s_client ${args} 2>&1 | head -200 || echo "(timeout or refused)"
    } >> "${f}"
    i=$((i + 1))
done

echo "[tls] done — ${n} endpoints → ${DIST}/tls/"
