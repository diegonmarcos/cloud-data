#!/usr/bin/env bash
# GCP VM operations — run from Surface (requires gcloud)
# Usage: ./alias.sh                # interactive
#        ./alias.sh <cmd> <vm>     # direct
set -euo pipefail

declare -A VM_MAP=(
  [gcp-proxy]="arch-1:us-central1-a"
  [gcp-t4]="ollama-spot-gpu:us-central1-a"
)
PROJECT="diegonmarcos-infra-prod"

COMMANDS=(serial ssh rescue reset setup dev-install kill-watchdog)

resolve_vm() {
  [[ -n "${VM_MAP[$1]:-}" ]] || { echo "Unknown VM: $1 (available: ${!VM_MAP[*]})"; exit 1; }
  IFS=: read -r INSTANCE ZONE <<< "${VM_MAP[$1]}"
}

pick() {
  local label="$1"; shift; local -a items=("$@")
  echo "$label"
  for i in "${!items[@]}"; do printf "  %d) %s\n" $((i+1)) "${items[$i]}"; done
  read -rp "> " idx
  ((idx--)); [[ $idx -ge 0 && $idx -lt ${#items[@]} ]] || { echo "Invalid"; exit 1; }
  PICK="${items[$idx]}"
}

run_cmd() {
  local cmd="$1" vm="$2"
  resolve_vm "$vm"
  case "$cmd" in
    serial)         gcloud compute connect-to-serial-port "$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    ssh)            gcloud compute ssh root@"$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    rescue)         gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='sudo iptables -F INPUT; sudo iptables -P INPUT ACCEPT; sudo systemctl restart sshd 2>/dev/null || sudo systemctl restart ssh 2>/dev/null; echo done' ;;
    reset)          gcloud compute instances reset "$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    setup)          gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='command -v pacman >/dev/null && sudo pacman -Sy --noconfirm fish || { sudo apt-get update -qq && sudo apt-get install -y -qq fish; }; fish --version' ;;
    dev-install)    gcloud compute ssh root@"$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='
set -e
if command -v pacman >/dev/null 2>&1; then
  pacman -Syu --noconfirm
  pacman -S --noconfirm --needed \
    fish git curl wget htop btop vim nano neovim \
    base-devel gcc make cmake rust cargo go \
    python python-pip python-virtualenv \
    nodejs npm yarn typescript \
    docker docker-compose docker-buildx \
    jq yq ripgrep fd bat eza tree fzf zoxide \
    rsync openssh wireguard-tools \
    tmux screen strace lsof bind-tools net-tools iproute2 nmap \
    zip unzip p7zip tar gzip \
    man-db less which file \
    sops age gnupg \
    sqlite postgresql-libs \
    starship github-cli
elif command -v apt-get >/dev/null 2>&1; then
  apt-get update -qq
  apt-get install -y -qq \
    fish git curl wget htop vim nano neovim \
    build-essential gcc make cmake rustc cargo golang \
    python3 python3-pip python3-venv \
    nodejs npm \
    docker.io docker-compose docker-buildx-plugin \
    jq ripgrep fd-find bat eza tree fzf \
    rsync openssh-server wireguard-tools \
    tmux screen strace lsof dnsutils net-tools iproute2 nmap \
    zip unzip p7zip-full tar gzip \
    man-db less file \
    sops age gnupg \
    sqlite3 libpq-dev \
    gh
fi
# Set fish as default shell
chsh -s "$(command -v fish)" "$(logname 2>/dev/null || echo diego)" 2>/dev/null || true
chsh -s "$(command -v fish)" root 2>/dev/null || true
# Claude Code
npm install -g @anthropic-ai/claude-code 2>/dev/null || true
# Nix (if not installed)
if ! command -v nix >/dev/null 2>&1; then
  curl -L https://nixos.org/nix/install | sh -s -- --daemon --yes 2>/dev/null || true
fi
echo "dev-install done"
' ;;
    kill-watchdog)  gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='sudo systemctl stop watchdog-petter.timer watchdog-petter.service 2>/dev/null; sudo systemctl disable watchdog-petter.timer 2>/dev/null; echo done' ;;
    *)              echo "Unknown: $cmd"; exit 1 ;;
  esac
}

if [[ $# -ge 2 ]]; then
  run_cmd "$1" "$2"
elif [[ $# -eq 0 ]]; then
  mapfile -t vms < <(printf '%s\n' "${!VM_MAP[@]}" | sort)
  pick "Command:" "${COMMANDS[@]}"; cmd="$PICK"
  pick "VM:" "${vms[@]}"; vm="$PICK"
  run_cmd "$cmd" "$vm"
else
  echo "Usage: $0 [command] [vm]"
  echo "Commands: ${COMMANDS[*]}"
  echo "VMs: ${!VM_MAP[*]}"
fi
