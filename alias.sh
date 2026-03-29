#!/usr/bin/env bash
# VM operations + dev environment installer
# Usage: ./alias.sh                # interactive
#        ./alias.sh <cmd> [vm]     # direct
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

# ═══════════════════════════════════════════════════════════════════
# INSTALL — runs locally on any machine
# ═══════════════════════════════════════════════════════════════════

install_dev_arch() {
  echo "=== Arch Linux: Full Dev Toolchain ==="
  sudo pacman -Syu --noconfirm
  sudo pacman -S --noconfirm --needed \
    fish git curl wget htop btop vim nano neovim \
    base-devel gcc make cmake rust cargo go \
    python python-pip python-virtualenv \
    nodejs npm yarn typescript \
    docker docker-compose docker-buildx \
    jq yq ripgrep fd bat eza tree fzf zoxide duf ncdu \
    rsync openssh wireguard-tools \
    tmux screen strace lsof bind-tools net-tools iproute2 nmap ncat \
    zip unzip p7zip tar gzip \
    man-db less which file \
    sops age gnupg openssl \
    sqlite postgresql-libs \
    starship github-cli terraform \
    rclone unison
  # gcloud
  if ! command -v gcloud >/dev/null 2>&1; then
    echo "Installing Google Cloud SDK..."
    curl -sL https://sdk.cloud.google.com | bash -s -- --disable-prompts --install-dir=/opt 2>/dev/null || true
    ln -sf /opt/google-cloud-sdk/bin/gcloud /usr/local/bin/gcloud 2>/dev/null || true
  fi
  # OCI CLI
  if ! command -v oci >/dev/null 2>&1; then
    echo "Installing OCI CLI..."
    pip install oci-cli 2>/dev/null || true
  fi
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
    jq ripgrep fd-find bat eza tree fzf duf ncdu \
    rsync openssh-server wireguard-tools \
    tmux screen strace lsof dnsutils net-tools iproute2 nmap ncat \
    zip unzip p7zip-full tar gzip \
    man-db less file \
    sops age gnupg openssl \
    sqlite3 libpq-dev \
    gh terraform \
    rclone
  # gcloud
  if ! command -v gcloud >/dev/null 2>&1; then
    echo "Installing Google Cloud SDK..."
    curl -sL https://sdk.cloud.google.com | bash -s -- --disable-prompts --install-dir=/opt 2>/dev/null || true
    ln -sf /opt/google-cloud-sdk/bin/gcloud /usr/local/bin/gcloud 2>/dev/null || true
  fi
  # OCI CLI
  if ! command -v oci >/dev/null 2>&1; then
    echo "Installing OCI CLI..."
    pip3 install oci-cli 2>/dev/null || true
  fi
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
    nixpkgs.python3 nixpkgs.nodejs_22 nixpkgs.yarn nixpkgs.typescript \
    nixpkgs.docker-compose \
    nixpkgs.jq nixpkgs.yq-go nixpkgs.ripgrep nixpkgs.fd nixpkgs.bat nixpkgs.eza \
    nixpkgs.tree nixpkgs.fzf nixpkgs.zoxide nixpkgs.duf nixpkgs.ncdu \
    nixpkgs.rsync nixpkgs.wireguard-tools nixpkgs.openssh \
    nixpkgs.tmux nixpkgs.strace nixpkgs.nmap \
    nixpkgs.unzip nixpkgs.p7zip \
    nixpkgs.sops nixpkgs.age nixpkgs.gnupg nixpkgs.openssl \
    nixpkgs.sqlite nixpkgs.starship nixpkgs.gh nixpkgs.terraform \
    nixpkgs.google-cloud-sdk nixpkgs.oci-cli nixpkgs.awscli2 \
    nixpkgs.flarectl nixpkgs.cloudflared nixpkgs.rclone
  install_extras
}

install_extras() {
  echo ""
  echo "=== Installing extras (Claude Code, npm globals, fish config) ==="

  # Claude Code
  npm install -g @anthropic-ai/claude-code 2>/dev/null || true

  # Cloudflare Wrangler
  npm install -g wrangler 2>/dev/null || true

  # Fish as default shell
  if command -v fish >/dev/null 2>&1; then
    FISH_PATH="$(command -v fish)"
    grep -qxF "$FISH_PATH" /etc/shells 2>/dev/null || echo "$FISH_PATH" | sudo tee -a /etc/shells >/dev/null
    sudo chsh -s "$FISH_PATH" "$(logname 2>/dev/null || whoami)" 2>/dev/null || true
    sudo chsh -s "$FISH_PATH" root 2>/dev/null || true
    echo "Fish set as default shell"
  fi

  # Fish config (aliases, abbrs, starship)
  setup_fish_config

  # Starship prompt
  if command -v starship >/dev/null 2>&1; then
    mkdir -p ~/.config
    [[ -f ~/.config/starship.toml ]] || cat > ~/.config/starship.toml << 'STAR'
format = "$username$hostname$directory$git_branch$git_status$cmd_duration$line_break$character"
[character]
success_symbol = "[❯](green)"
error_symbol = "[❯](red)"
[directory]
truncation_length = 3
[git_branch]
format = "[$branch]($style) "
[cmd_duration]
min_time = 2000
STAR
  fi

  echo ""
  echo "=== Install complete ==="
}

setup_fish_config() {
  local FISH_DIR="${HOME}/.config/fish"
  mkdir -p "$FISH_DIR"
  cat > "$FISH_DIR/config.fish" << 'FISHCONF'
# Fish config — generated by alias.sh dev-install
if status is-interactive
    # Modern CLI replacements
    alias ls="eza --color=auto --icons 2>/dev/null || command ls --color=auto"
    alias ll="eza -alF --icons 2>/dev/null || command ls -alF"
    alias la="eza -A --icons 2>/dev/null || command ls -A"
    alias lt="eza --tree --level=2 --icons 2>/dev/null || tree -L 2"
    alias cat="bat --paging=never 2>/dev/null || command cat"
    alias grep="rg 2>/dev/null || command grep --color=auto"
    alias find="fd 2>/dev/null || command find"
    alias df="duf 2>/dev/null || command df -h"
    alias du="ncdu 2>/dev/null || command du -sh"

    # Navigation
    alias ..="cd .."
    alias ...="cd ../.."
    alias ....="cd ../../.."

    # Safety
    alias rm="rm -i"
    alias cp="cp -i"
    alias mv="mv -i"

    # Git abbreviations
    abbr -a gs "git status -sb"
    abbr -a ga "git add"
    abbr -a gaa "git add --all"
    abbr -a gc "git commit"
    abbr -a gcm "git commit -m"
    abbr -a gp "git push"
    abbr -a gl "git log --oneline --graph --decorate -20"
    abbr -a gd "git diff"
    abbr -a gco "git checkout"
    abbr -a gpl "git pull"

    # Docker abbreviations
    abbr -a dps "docker ps"
    abbr -a dpsa "docker ps -a"
    abbr -a dcu "docker compose up"
    abbr -a dcd "docker compose down"
    abbr -a dcl "docker compose logs -f"

    # Quick
    alias c="clear"
    alias h="history"
    alias ports="ss -tulanp"
    alias myip="curl -s ifconfig.me"
    alias py="python3"
    alias reload="source ~/.config/fish/config.fish"

    # Claude
    alias cc="claude"

    # PATH
    fish_add_path -m ~/.cargo/bin ~/.npm-global/bin ~/go/bin ~/.local/bin ~/.nix-profile/bin

    # Starship prompt
    if command -q starship
        starship init fish | source
    end

    # Zoxide (smart cd)
    if command -q zoxide
        zoxide init fish | source
    end
end
FISHCONF
  echo "Fish config written to $FISH_DIR/config.fish"
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

# ═══════════════════════════════════════════════════════════════════
# REMOTE COMMANDS (require gcloud)
# ═══════════════════════════════════════════════════════════════════

run_cmd() {
  local cmd="$1" vm="${2:-}"
  case "$cmd" in
    install) do_install; return ;;
  esac
  [[ -n "$vm" ]] || { echo "VM required"; exit 1; }
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

# ═══════════════════════════════════════════════════════════════════
# ENTRY POINT
# ═══════════════════════════════════════════════════════════════════

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
