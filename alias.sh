#!/usr/bin/env bash
# VM operations — works everywhere (gcloud commands auto-skipped if missing)
# Usage: ./alias.sh                # interactive
#        ./alias.sh <cmd> <vm>     # direct
set -euo pipefail

declare -A VM_MAP=(
  [gcp-proxy]="arch-1:us-central1-a"
  [gcp-t4]="ollama-spot-gpu:us-central1-a"
)
PROJECT="diegonmarcos-infra-prod"

COMMANDS=(install serial ssh rescue reset kill-watchdog)

pick() {
  local label="$1"; shift; local -a items=("$@")
  echo "$label"
  for i in "${!items[@]}"; do printf "  %d) %s\n" $((i+1)) "${items[$i]}"; done
  read -rp "> " idx
  ((idx--)); [[ $idx -ge 0 && $idx -lt ${#items[@]} ]] || { echo "Invalid"; exit 1; }
  PICK="${items[$idx]}"
}

resolve_vm() {
  [[ -n "${VM_MAP[$1]:-}" ]] || { echo "Unknown VM: $1 (available: ${!VM_MAP[*]})"; exit 1; }
  IFS=: read -r INSTANCE ZONE <<< "${VM_MAP[$1]}"
}

# ── Install packages (runs locally, no gcloud needed) ───────────────

install_dev_arch() {
  echo "=== Arch: Full Dev Toolchain ==="
  sudo pacman -Syu --noconfirm
  sudo pacman -S --noconfirm --needed \
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
  install_extras
}

install_dev_debian() {
  echo "=== Debian/Ubuntu: Full Dev Toolchain ==="
  sudo apt-get update -qq
  sudo apt-get install -y -qq \
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
  install_extras
}

install_dev_nix() {
  echo "=== Nix: Full Dev Toolchain ==="
  if ! command -v nix >/dev/null 2>&1; then
    echo "Installing Nix..."
    curl -L https://nixos.org/nix/install | sh -s -- --daemon --yes
    . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh 2>/dev/null || true
  fi
  nix-env -iA \
    nixpkgs.fish nixpkgs.git nixpkgs.curl nixpkgs.wget nixpkgs.htop nixpkgs.btop \
    nixpkgs.neovim nixpkgs.gcc nixpkgs.gnumake nixpkgs.cmake \
    nixpkgs.rustc nixpkgs.cargo nixpkgs.go \
    nixpkgs.python3 nixpkgs.nodejs nixpkgs.yarn nixpkgs.typescript \
    nixpkgs.docker-compose \
    nixpkgs.jq nixpkgs.yq nixpkgs.ripgrep nixpkgs.fd nixpkgs.bat nixpkgs.eza \
    nixpkgs.tree nixpkgs.fzf nixpkgs.zoxide \
    nixpkgs.rsync nixpkgs.wireguard-tools \
    nixpkgs.tmux nixpkgs.strace nixpkgs.nmap \
    nixpkgs.unzip nixpkgs.p7zip \
    nixpkgs.sops nixpkgs.age nixpkgs.gnupg \
    nixpkgs.sqlite nixpkgs.starship nixpkgs.gh
  install_extras
}

install_extras() {
  # Claude Code
  echo "Installing Claude Code..."
  npm install -g @anthropic-ai/claude-code 2>/dev/null || true
  # Fish as default shell
  if command -v fish >/dev/null 2>&1; then
    sudo chsh -s "$(command -v fish)" "$(logname 2>/dev/null || whoami)" 2>/dev/null || true
    sudo chsh -s "$(command -v fish)" root 2>/dev/null || true
    echo "Fish set as default shell"
  fi
  echo "=== Install complete ==="
}

do_install() {
  local distros=(arch debian nix)
  pick "Distro:" "${distros[@]}"
  case "$PICK" in
    arch)   install_dev_arch ;;
    debian) install_dev_debian ;;
    nix)    install_dev_nix ;;
  esac
}

# ── Remote commands (require gcloud) ────────────────────────────────

run_cmd() {
  local cmd="$1" vm="$2"
  case "$cmd" in
    install) do_install; return ;;
  esac
  resolve_vm "$vm"
  if ! command -v gcloud >/dev/null 2>&1; then
    echo "gcloud not found — remote commands need gcloud CLI"
    exit 1
  fi
  case "$cmd" in
    serial)         gcloud compute connect-to-serial-port "$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    ssh)            gcloud compute ssh root@"$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    rescue)         gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='sudo iptables -F INPUT; sudo iptables -P INPUT ACCEPT; sudo systemctl restart sshd 2>/dev/null || sudo systemctl restart ssh 2>/dev/null; echo done' ;;
    reset)          gcloud compute instances reset "$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    kill-watchdog)  gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='sudo systemctl stop watchdog-petter.timer watchdog-petter.service 2>/dev/null; sudo systemctl disable watchdog-petter.timer 2>/dev/null; echo done' ;;
    *)              echo "Unknown: $cmd"; exit 1 ;;
  esac
}

# ── Entry point ─────────────────────────────────────────────────────

if [[ $# -ge 2 ]]; then
  run_cmd "$1" "$2"
elif [[ $# -eq 1 && "$1" == "install" ]]; then
  do_install
elif [[ $# -eq 0 ]]; then
  pick "Command:" "${COMMANDS[@]}"; cmd="$PICK"
  if [[ "$cmd" == "install" ]]; then
    do_install
  else
    mapfile -t vms < <(printf '%s\n' "${!VM_MAP[@]}" | sort)
    pick "VM:" "${vms[@]}"; vm="$PICK"
    run_cmd "$cmd" "$vm"
  fi
else
  echo "Usage: $0 [command] [vm]"
  echo "Commands: ${COMMANDS[*]}"
  echo "VMs: ${!VM_MAP[*]}"
fi
