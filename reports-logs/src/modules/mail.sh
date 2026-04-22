#!/bin/sh
# mail module — Maddy + Stalwart quick diagnostics. Runs only on the
# declared mail VM (modules.mail.maddy_vm).
set -eu

: "${CONFIG:?}" "${DIST:?}"

vm=$(jq -r '.modules.mail.maddy_vm // "oci-mail"' "${CONFIG}")
maddy=$(jq -r '.modules.mail.maddy_container // "maddy"' "${CONFIG}")
stalwart=$(jq -r '.modules.mail.stalwart_container // "stalwart"' "${CONFIG}")

out="${DIST}/mail"
mkdir -p "${out}"

if ! ssh -o BatchMode=yes -o ConnectTimeout=3 "${vm}" true 2>/dev/null; then
    echo "  ${vm}: unreachable"
    exit 0
fi

# Maddy — TLS cert details + bind config
ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
    "docker exec ${maddy} sh -c 'openssl x509 -in /data/tls/fullchain.pem -noout -text 2>/dev/null | head -50 || echo \"(cert not at /data/tls)\"'" \
    > "${out}/maddy_cert.txt" 2>/dev/null || true

ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
    "docker exec ${maddy} sh -c 'ss -lntp 2>/dev/null | head -40 || netstat -lnt 2>/dev/null | head -40'" \
    > "${out}/maddy_listeners.txt" 2>/dev/null || true

ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
    "docker exec ${maddy} maddy creds list 2>&1 | head -40" \
    > "${out}/maddy_creds.txt" 2>/dev/null || true

# Stalwart — admin status
ssh -n -o BatchMode=yes -o ConnectTimeout=10 "${vm}" \
    "docker exec ${stalwart} ss -lntp 2>/dev/null | head -40 || true" \
    > "${out}/stalwart_listeners.txt" 2>/dev/null || true

echo "[mail] done — ${out}/"
