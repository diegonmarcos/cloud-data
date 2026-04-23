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
    echo "  daily          Master: cloud-health-full-daily (consolidated monster)"
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
  all|daily|mail|url|sec-network|sec-data)
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

if [ -n "${SSH_KEY:-}" ]; then
  echo "$SSH_KEY" > ~/.ssh/id_deploy
  chmod 600 ~/.ssh/id_deploy
  echo "[setup] SSH key from env var"
elif [ -f ~/.ssh/id_rsa ] || [ -f ~/.ssh/vault_id_rsa ] || [ -f ~/.ssh/id_ed25519 ]; then
  echo "[setup] SSH keys from mounted ~/.ssh"
else
  echo "[setup] WARNING: no SSH keys found"
fi

if [ -n "${SSH_ALIAS:-}" ] && [ -n "${SSH_HOST:-}" ]; then
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
elif [ -f ~/.ssh/config ]; then
  echo "[setup] SSH config from mounted ~/.ssh/config"
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
if [ -n "${WG_PRIVATE_KEY:-}" ]; then
  SUDO=""; command -v sudo >/dev/null 2>&1 && SUDO="sudo"
  umask 077
  cat > /tmp/wg0.conf << WGEOF
[Interface]
PrivateKey = ${WG_PRIVATE_KEY}
Address = 10.0.0.200/24

[Peer]
PublicKey = vV/phXUwnCjxACQ5Df11Uw47BzJaK4r85jPYMu2HmDc=
Endpoint = 35.226.147.64:51820
AllowedIPs = 10.0.0.0/24
PersistentKeepalive = 25
WGEOF
  $SUDO mkdir -p /etc/wireguard
  $SUDO cp /tmp/wg0.conf /etc/wireguard/wg0.conf
  rm /tmp/wg0.conf
  $SUDO wg-quick up wg0 2>/dev/null && echo "[setup] WireGuard up" || echo "[setup] WireGuard failed (non-fatal)"
fi

echo "[setup] Ready"

# ── 6. Dispatch ───────────────────────────────────────────────────
REPORTS_DIR="/root/git/cloud-data/reports"
CMD="$1"; shift

case "$CMD" in
  all)           exec bash "$REPORTS_DIR/build.sh" all ;;
  daily)         exec bash "$REPORTS_DIR/build.sh" health-full-daily ;;
  mail)          exec bash "$REPORTS_DIR/build.sh" mail-health-full ;;
  url)           exec bash "$REPORTS_DIR/build.sh" url-health ;;
  sec-network)   exec bash "$REPORTS_DIR/build.sh" sec-network ;;
  sec-data)      exec bash "$REPORTS_DIR/build.sh" sec-data ;;
  bash|sh)       exec bash "$@" ;;
esac
