#!/bin/bash
# ╔══════════════════════════════════════════════════════════════════╗
# ║  ship-reports.sh — Build + push cloud-data-reports image         ║
# ║                                                                  ║
# ║  Runs INSIDE the builder container. GHA only passes env vars.    ║
# ║  Uses ~/git/cloud-data (cloned by entrypoint step 7).           ║
# ║                                                                  ║
# ║  Required env: GITHUB_TOKEN, GITHUB_ACTOR                       ║
# ╚══════════════════════════════════════════════════════════════════╝
set -euo pipefail

# Source nix profile + devShell PATH
[ -f /etc/devshell-path.txt ] && export PATH="$(cat /etc/devshell-path.txt)"
[ -f /root/.nix-profile/etc/profile.d/hm-session-vars.sh ] && . /root/.nix-profile/etc/profile.d/hm-session-vars.sh 2>/dev/null
export PATH="$HOME/.nix-profile/bin:$HOME/.node_modules/node_modules/.bin:${PATH:-/usr/bin:/bin}"

CLOUD_DATA="$HOME/git/cloud-data"

echo "══════════════════════════════════════════"
echo "  Ship Reports Image"
echo "══════════════════════════════════════════"

# ── 1. Verify cloud-data repo ─────────────────────────────────────
# Entrypoint step 7 already cloned all repos under ~/git/
if [ ! -d "$CLOUD_DATA/reports" ]; then
  echo "FATAL: $CLOUD_DATA/reports not found (entrypoint should have cloned it)"
  exit 1
fi
echo "[1/3] Using $CLOUD_DATA @ $(git -C "$CLOUD_DATA" rev-parse --short HEAD 2>/dev/null || echo unknown)"

# ── 2. GHCR login ─────────────────────────────────────────────────
echo "[2/3] GHCR login"
if [ -n "${GITHUB_TOKEN:-}" ]; then
  echo "$GITHUB_TOKEN" | docker login ghcr.io -u "${GITHUB_ACTOR:-diegonmarcos}" --password-stdin 2>/dev/null
elif command -v gh >/dev/null 2>&1 && gh auth token >/dev/null 2>&1; then
  gh auth token 2>/dev/null | docker login ghcr.io -u "$(gh api user --jq .login 2>/dev/null || echo diegonmarcos)" --password-stdin 2>/dev/null
else
  echo "FATAL: No GHCR credentials (set GITHUB_TOKEN or gh auth login)"
  exit 1
fi

# ── 3. Build + push ───────────────────────────────────────────────
echo "[3/3] Building and pushing cloud-data-reports image"
cd "$CLOUD_DATA/reports"
docker compose build
docker compose push

echo ""
echo "══════════════════════════════════════════"
echo "  Ship Reports: done"
echo "══════════════════════════════════════════"
