#!/bin/sh
# ╔══════════════════════════════════════════════════════════════════╗
# ║ cloud-data-reports entrypoint — UNIVERSAL                       ║
# ║                                                                  ║
# ║ Same pattern as cloud-builder-x entrypoint.                     ║
# ║ Sets up SSH, WG, SOPS, then dispatches report commands.         ║
# ║                                                                  ║
# ║ Usage:                                                           ║
# ║   docker compose run --rm reports all                            ║
# ║   docker compose run --rm reports stack                          ║
# ║   docker compose run --rm reports cloud                          ║
# ║   docker compose run --rm reports mail                           ║
# ║   docker compose run --rm reports bash                           ║
# ╚══════════════════════════════════════════════════════════════════╝
set -e

# ── 1. Help / passthrough ─────────────────────────────────────────
case "${1:-}" in
  ""|--help|-h)
    echo "cloud-data-reports — health report runner"
    echo ""
    echo "Commands:"
    echo "  all            Run all 5 reports (master + 4 derives)"
    echo "  daily          Master: cloud-health-full-daily (build only — NO email)"
    echo "  daily-mail     Master: cloud-health-full-daily — build + send via SMTP"
    echo "  mail           Derive: cloud-mail-health-full"
    echo "  url            Derive: cloud-url-health-report (fast)"
    echo "  sec-network    Derive: cloud-sec-network-report"
    echo "  sec-data       Derive: cloud-sec-data-report"
    echo "  bash           Interactive shell"
    exit 0
    ;;
  bash|sh)
    # Interactive shell — setup env but don't dispatch
    ;;
  all|daily|daily-mail|mail|url|sec-network|sec-data)
    # Handled below after setup
    ;;
  *)
    exec "$@"
    ;;
esac

# ── 2. SSH setup (env vars OR mounted files) ──────────────────────
mkdir -p ~/.ssh 2>/dev/null || true
chmod 700 ~/.ssh 2>/dev/null || true
ssh-keyscan github.com >> ~/.ssh/known_hosts 2>/dev/null || true

# Materialise per-VM SSH keys from env vars. Source of truth is the consolidated
# JSON (vms[*].gha.ssh_secret) — the same env-var name CI exposes. We never
# hardcode VM names here.
CONSOLIDATED=""
for cand in /root/git/cloud/2_configs/dist/_cloud-data-consolidated.json \
            /root/git/cloud-data/_cloud-data-consolidated.json \
            /home/diego/git/cloud/2_configs/dist/_cloud-data-consolidated.json; do
  if [ -f "$cand" ]; then
    CONSOLIDATED="$cand"
    echo "[setup]   probe HIT: $cand"
    break
  else
    echo "[setup]   probe miss: $cand"
  fi
done

if [ -z "$CONSOLIDATED" ]; then
  echo "[setup]   skip SSH-config derive: CONSOLIDATED unset"
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "[setup]   skip SSH-config derive: jq not in PATH (got: $(command -v jq 2>&1 || echo none))"
fi
if [ -n "$CONSOLIDATED" ] && command -v jq >/dev/null 2>&1; then
  echo "[setup] SSH config: deriving from $CONSOLIDATED"
  : > ~/.ssh/config.d.tmp
  # jq emits TSV: ssh_alias \t ip \t user \t ssh_secret
  jq -r '
    .vms
    | to_entries[]
    | select(.value.gha.ssh_secret != null and .value.ip != null and .value.ip != "TBD")
    | [.value.ssh_alias, .value.ip, (.value.user // "ubuntu"), .value.gha.ssh_secret]
    | @tsv
  ' "$CONSOLIDATED" | while IFS="$(printf '\t')" read -r alias ip user secret; do
    [ -z "$alias" ] && continue
    eval "key_val=\${$secret:-}"
    if [ -z "$key_val" ]; then
      echo "[setup]   skip $alias — env \$$secret unset"
      continue
    fi
    key_file="$HOME/.ssh/id_${secret}"
    printf '%s\n' "$key_val" > "$key_file"
    chmod 600 "$key_file"
    cat >> ~/.ssh/config.d.tmp <<EOF
Host ${alias}
  HostName ${ip}
  User ${user}
  IdentityFile ${key_file}
  StrictHostKeyChecking no
  UserKnownHostsFile /dev/null
  ServerAliveInterval 30
  ServerAliveCountMax 10

EOF
    echo "[setup]   $alias → ${user}@${ip} (key=\$$secret)"
  done
  if [ -s ~/.ssh/config.d.tmp ]; then
    mv ~/.ssh/config.d.tmp ~/.ssh/config
    chmod 600 ~/.ssh/config
  else
    rm -f ~/.ssh/config.d.tmp
    echo "[setup] SSH config: no VMs matched (no secrets set?)"
  fi
elif [ -n "${SSH_KEY:-}" ]; then
  echo "$SSH_KEY" > ~/.ssh/id_deploy
  chmod 600 ~/.ssh/id_deploy
  echo "[setup] SSH key from \$SSH_KEY (single-host fallback)"
elif [ -f ~/.ssh/id_rsa ] || [ -f ~/.ssh/vault_id_rsa ] \
  || [ -f ~/.ssh/id_ed25519 ] || [ -f ~/.ssh/id_deploy ]; then
  echo "[setup] SSH keys from mounted ~/.ssh"
else
  echo "[setup] WARNING: no SSH keys found"
fi

# Single-host override (legacy SSH_ALIAS/SSH_HOST/SSH_KEY path) — append only
# if the data-driven config block isn't already covering it.
if [ -n "${SSH_ALIAS:-}" ] && [ -n "${SSH_HOST:-}" ] && \
   ! grep -q "^Host ${SSH_ALIAS}\$" ~/.ssh/config 2>/dev/null; then
  cat >> ~/.ssh/config <<EOF

Host ${SSH_ALIAS}
  HostName ${SSH_HOST}
  User ${SSH_USER:-ubuntu}
  IdentityFile ~/.ssh/id_deploy
  StrictHostKeyChecking no
  ServerAliveInterval 30
  ServerAliveCountMax 10
EOF
  chmod 600 ~/.ssh/config
fi

# ── 3. SOPS setup (mounted file takes precedence; env var only if dir writable) ──
if [ -f ~/.config/sops/age/keys.txt ]; then
  export SOPS_AGE_KEY_FILE=~/.config/sops/age/keys.txt
  echo "[setup] SOPS age key from mounted file"
elif [ -n "${SOPS_AGE_KEY:-}" ] && mkdir -p ~/.config/sops/age 2>/dev/null && [ -w ~/.config/sops/age ]; then
  printf '%s' "$SOPS_AGE_KEY" > ~/.config/sops/age/keys.txt
  export SOPS_AGE_KEY_FILE=~/.config/sops/age/keys.txt
  echo "[setup] SOPS age key from env var"
else
  echo "[setup] WARNING: no SOPS age key (not mounted, not in env, or config dir RO)" >&2
fi

# ── 4. GHCR login (optional — only if docker client is in image) ──
if ! command -v docker >/dev/null 2>&1; then
  echo "[setup] docker client not in image — skipping GHCR login (reports don't need it)"
elif [ -n "${GITHUB_TOKEN:-}" ]; then
  echo "$GITHUB_TOKEN" | docker login ghcr.io -u "${GITHUB_ACTOR:-diegonmarcos}" --password-stdin 2>/dev/null \
    && echo "[setup] GHCR authenticated" \
    || echo "[setup] GHCR login failed (non-fatal)"
elif command -v gh >/dev/null 2>&1 && gh auth token >/dev/null 2>&1; then
  gh auth token 2>/dev/null | docker login ghcr.io -u "$(gh api user --jq .login 2>/dev/null || echo diegonmarcos)" --password-stdin 2>/dev/null \
    && echo "[setup] GHCR authenticated via gh CLI" \
    || echo "[setup] GHCR login failed (non-fatal)"
fi

# ── 5. WireGuard (if key provided) ────────────────────────────────
# Peer endpoint+pubkey come from cloud-data (vault provider mapping). We pin
# to the gcp-proxy hub which routes to all internal VMs via AllowedIPs.
if [ -n "${WG_PRIVATE_KEY:-}" ]; then
  if ! command -v wg-quick >/dev/null 2>&1; then
    echo "[setup] WireGuard SKIPPED — wg-quick not in image (fix Dockerfile)" >&2
  else
    SUDO=""; command -v sudo >/dev/null 2>&1 && SUDO="sudo"
    umask 077
    # Peer details: hub = gcp-E2-f_0 (gcp-proxy). Resolved from consolidated.
    WG_PEER_PUBKEY=""
    WG_PEER_ENDPOINT=""
    if [ -n "$CONSOLIDATED" ] && command -v jq >/dev/null 2>&1; then
      WG_PEER_PUBKEY=$(jq -r '.vms["gcp-E2-f_0"].wg_public_key // empty' "$CONSOLIDATED")
      WG_PEER_IP=$(jq -r '.vms["gcp-E2-f_0"].ip // empty' "$CONSOLIDATED")
      WG_PEER_PORT=$(jq -r '.vms["gcp-E2-f_0"].wg_port // 51820' "$CONSOLIDATED")
      [ -n "$WG_PEER_IP" ] && WG_PEER_ENDPOINT="${WG_PEER_IP}:${WG_PEER_PORT}"
    fi
    : "${WG_PEER_PUBKEY:=vV/phXUwnCjxACQ5Df11Uw47BzJaK4r85jPYMu2HmDc=}"
    : "${WG_PEER_ENDPOINT:=35.226.147.64:51820}"
    cat > /tmp/wg0.conf << WGEOF
[Interface]
PrivateKey = ${WG_PRIVATE_KEY}
Address = 10.0.0.200/24

[Peer]
PublicKey = ${WG_PEER_PUBKEY}
Endpoint = ${WG_PEER_ENDPOINT}
AllowedIPs = 10.0.0.0/24
PersistentKeepalive = 25
WGEOF
    $SUDO mkdir -p /etc/wireguard
    $SUDO cp /tmp/wg0.conf /etc/wireguard/wg0.conf
    rm /tmp/wg0.conf
    if $SUDO wg-quick up wg0; then
      echo "[setup] WireGuard up (peer=${WG_PEER_ENDPOINT})"
    else
      echo "[setup] WireGuard FAILED — VM probes over 10.0.0.0/24 will fail" >&2
    fi
  fi
fi

echo "[setup] Ready"

# ── 6. Dispatch ───────────────────────────────────────────────────
REPORTS_DIR="/root/git/cloud-data/reports"
CMD="$1"; shift

case "$CMD" in
  all)           exec bash "$REPORTS_DIR/build.sh" all ;;
  daily)         exec bash "$REPORTS_DIR/build.sh" health-full-daily ;;
  daily-mail)    exec bash "$REPORTS_DIR/src/cloud-health-full-daily/build.sh" ship ;;
  mail)          exec bash "$REPORTS_DIR/build.sh" mail-health-full ;;
  url)           exec bash "$REPORTS_DIR/build.sh" url-health ;;
  sec-network)   exec bash "$REPORTS_DIR/build.sh" sec-network ;;
  sec-data)      exec bash "$REPORTS_DIR/build.sh" sec-data ;;
  bash|sh)       exec bash "$@" ;;
esac
