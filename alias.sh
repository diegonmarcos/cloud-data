#!/usr/bin/env bash
# Diego's VM toolkit — install, ssh, clone, info
# Usage: ./alias.sh                # interactive
#        ./alias.sh <cmd> [args]   # direct
# OS-agnostic: NixOS, Arch, Debian, Fedora, macOS, Termux
set -euo pipefail

# Logging — verbose trace to file, clean output to console
LOGFILE="${HOME:-/tmp}/alias.log"
echo "=== $(date) === alias.sh $* ===" >> "$LOGFILE"
exec 3>> "$LOGFILE"
BASH_XTRACEFD=3
set -x

# Force real system binaries FIRST (bypass nix guardrail wrappers)
export PATH="/usr/bin:/usr/sbin:/usr/local/bin:/bin:/sbin:/nix/var/nix/profiles/default/bin:${HOME:-/root}/.nix-profile/bin:/run/current-system/sw/bin:$PATH"

# Stop systemd journal from flooding the terminal
if [ "$(id -u)" = "0" ] 2>/dev/null; then
  dmesg -n 1 2>/dev/null || true
  systemctl stop systemd-journald-audit.socket 2>/dev/null || true
  echo 0 > /proc/sys/kernel/printk 2>/dev/null || true
fi

# ═══════════════════════════════════════════════════════════════════
# SYSTEM DETECTION — populated once, used by all commands
# ═══════════════════════════════════════════════════════════════════

detect_system() {
  # OS / Distro
  SYS_OS="unknown"; SYS_DISTRO="unknown"; SYS_PKG="none"
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    SYS_OS="${ID:-unknown}"
    SYS_DISTRO="${PRETTY_NAME:-$ID}"
    case "$ID" in
      nixos)                         SYS_PKG="nix" ;;
      arch|manjaro)                  SYS_PKG="pacman" ;;
      debian|ubuntu|pop|mint)        SYS_PKG="apt" ;;
      fedora|rhel|centos|rocky|alma) SYS_PKG="dnf" ;;
    esac
  elif [ -d /data/data/com.termux ]; then
    SYS_OS="termux"; SYS_DISTRO="Termux (Android)"; SYS_PKG="pkg"
  elif command -v sw_vers >/dev/null 2>&1; then
    SYS_OS="macos"; SYS_DISTRO="macOS $(sw_vers -productVersion 2>/dev/null)"; SYS_PKG="brew"
  fi
  # Has nix? (overlay — works on any OS)
  if command -v nix >/dev/null 2>&1; then
    SYS_HAS_NIX=true
    [ "$SYS_PKG" = "none" ] && SYS_PKG="nix"
  else
    SYS_HAS_NIX=false
  fi

  # Architecture
  SYS_ARCH=$(uname -m 2>/dev/null || echo "unknown")
  case "$SYS_ARCH" in
    x86_64|amd64)   SYS_ARCH_SHORT="x86" ;;
    aarch64|arm64)   SYS_ARCH_SHORT="arm64" ;;
    armv7l|armhf)    SYS_ARCH_SHORT="arm32" ;;
    *)               SYS_ARCH_SHORT="$SYS_ARCH" ;;
  esac

  # Hostname
  SYS_HOSTNAME=$(hostname -s 2>/dev/null || cat /etc/hostname 2>/dev/null || echo "unknown")

  # CPU / RAM
  SYS_CPUS=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "?")
  if [ -f /proc/meminfo ]; then
    SYS_RAM_MB=$(awk '/MemTotal/{printf "%d", $2/1024}' /proc/meminfo)
  elif command -v sysctl >/dev/null 2>&1; then
    SYS_RAM_MB=$(( $(sysctl -n hw.memsize 2>/dev/null || echo 0) / 1024 / 1024 ))
  else
    SYS_RAM_MB="?"
  fi

  # Kernel
  SYS_KERNEL=$(uname -r 2>/dev/null || echo "?")

  # Docker
  if command -v docker >/dev/null 2>&1; then
    SYS_HAS_DOCKER=true
    SYS_DOCKER_PATH=$(command -v docker)
  else
    SYS_HAS_DOCKER=false
    SYS_DOCKER_PATH=""
  fi

  # Init system
  if command -v systemctl >/dev/null 2>&1; then
    SYS_INIT="systemd"
  elif [ -f /sbin/openrc ]; then
    SYS_INIT="openrc"
  else
    SYS_INIT="other"
  fi
}

show_banner() {
  echo "╔══════════════════════════════════════════════════════════╗"
  echo "║  Diego's Toolkit                                        ║"
  echo "╠══════════════════════════════════════════════════════════╣"
  printf "║  %-54s  ║\n" "Host: $SYS_HOSTNAME"
  printf "║  %-54s  ║\n" "OS:   $SYS_DISTRO ($SYS_ARCH)"
  printf "║  %-54s  ║\n" "CPU:  ${SYS_CPUS} cores  RAM: ${SYS_RAM_MB}MB  Kernel: ${SYS_KERNEL%%[-+]*}"
  printf "║  %-54s  ║\n" "Pkg:  $SYS_PKG  Nix: $SYS_HAS_NIX  Docker: $SYS_HAS_DOCKER  Init: $SYS_INIT"
  echo "╚══════════════════════════════════════════════════════════╝"
  echo ""
}

detect_system

declare -A VM_MAP=(
  [gcp-proxy]="arch-1:us-central1-a"
  [gcp-t4]="ollama-spot-gpu:us-central1-a"
)
PROJECT="diegonmarcos-infra-prod"

REPOS=(
  "cloud:https://github.com/diegonmarcos/cloud.git"
  "cloud-data:https://github.com/diegonmarcos/cloud-data.git"
  "unix:https://github.com/diegonmarcos/unix.git"
  "front:https://github.com/diegonmarcos/front.git"
  "vault:https://github.com/diegonmarcos/vault.git"
)

MAIN_COMMANDS=(commands fix-journal docker-start install ssh git-clone info)

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
# 0) COMMANDS — quick commands to run on this machine
# ═══════════════════════════════════════════════════════════════════

do_commands() {
  local cmds=(
    "flush-iptables:iptables -F INPUT; iptables -P INPUT ACCEPT; echo iptables flushed"
    "restart-sshd:systemctl restart sshd 2>/dev/null || systemctl restart ssh 2>/dev/null; echo sshd restarted"
    "restart-wg:systemctl restart wg-quick@wg0; echo wg restarted"
    "restart-docker:systemctl restart docker; echo docker restarted"
    "stop-docker:systemctl stop docker; echo docker stopped"
    "start-docker:systemctl start docker; echo docker started"
    "docker-ps:docker ps --format '{{.Names}}: {{.Status}}' | sort"
    "wg-status:wg show wg0"
    "iptables-show:iptables -L INPUT -n --line-numbers"
    "free-mem:free -m"
    "disk-usage:df -h / /var /opt 2>/dev/null"
    "kill-watchdog:systemctl stop watchdog-petter.timer watchdog-petter.service 2>/dev/null; systemctl disable watchdog-petter.timer 2>/dev/null; echo watchdog killed"
    "journal-silence:echo 0 > /proc/sys/kernel/printk; dmesg -n 1; echo journal silenced"
    "full-rescue:iptables -F INPUT; iptables -P INPUT ACCEPT; systemctl restart sshd 2>/dev/null || systemctl restart ssh 2>/dev/null; systemctl restart wg-quick@wg0 2>/dev/null; echo full rescue done"
  )

  echo "Commands (runs locally on this machine):"
  for i in "${!cmds[@]}"; do
    local name="${cmds[$i]%%:*}"
    printf "  %d) %s\n" $((i+1)) "$name"
  done
  read -rp "> " idx
  ((idx--))
  [[ $idx -ge 0 && $idx -lt ${#cmds[@]} ]] || { echo "Invalid"; exit 1; }

  local selected="${cmds[$idx]}"
  local name="${selected%%:*}"
  local cmd="${selected#*:}"
  echo "=== Running: $name ==="
  echo "[CMD] $cmd"
  eval "$cmd"
  echo "[DONE] $name (exit $?)"
}

# ═══════════════════════════════════════════════════════════════════
# 0b) FIX JOURNAL — stop systemd spam on console
# ═══════════════════════════════════════════════════════════════════

do_fix_journal() {
  echo "=== Silencing journal console spam ==="
  echo 0 > /proc/sys/kernel/printk 2>/dev/null || true
  dmesg -n 1 2>/dev/null || true
  systemctl stop systemd-journald-audit.socket 2>/dev/null || true
  # Persist across reboots
  mkdir -p /etc/sysctl.d
  echo "kernel.printk = 0 0 0 0" > /etc/sysctl.d/99-silence-console.conf 2>/dev/null || true
  echo "Done — console spam silenced"
}

# ═══════════════════════════════════════════════════════════════════
# 0b) DOCKER START — pull & run dev environment container
# ═══════════════════════════════════════════════════════════════════

do_docker_start() {
  local IMG="ghcr.io/diegonmarcos/diego-user-env:latest"
  local HOME_DIR="${HOME:-/root}"

  # ── Step 1: Ensure docker CLI is available ─────────────────────────────
  if [ "$SYS_HAS_DOCKER" = false ]; then
    echo "Docker not found — installing for $SYS_PKG..."
    case "$SYS_PKG" in
      apt)    apt-get update -qq && apt-get install -y -qq docker.io ;;
      dnf)    dnf install -y --skip-unavailable docker ;;
      pacman) pacman -Sy --noconfirm docker ;;
      nix)
        # NixOS: docker must be in system config. Non-NixOS nix: use nix-shell.
        if [ "$SYS_OS" = "nixos" ]; then
          echo "On NixOS, Docker must be enabled in your system flake:"
          echo "  virtualisation.docker.enable = true;"
          echo "Then rebuild: sudo nixos-rebuild switch"
          echo ""
          echo "Trying nix-shell fallback for docker CLI..."
          exec nix-shell -p docker --run "bash $0 docker-start"
        else
          exec nix-shell -p docker --run "bash $0 docker-start"
        fi ;;
      brew)   brew install --cask docker ;;
      pkg)    echo "Docker not available on Termux"; exit 1 ;;
      *)      echo "No package manager found — install docker manually"; exit 1 ;;
    esac
    # Re-detect after install
    command -v docker >/dev/null 2>&1 || { echo "ERROR: docker still not found after install"; exit 1; }
  fi

  # ── Step 2: Find container runtime (docker preferred, podman fallback) ─
  DOCKER=""
  RUNTIME="docker"
  # From systemd service (most reliable on Nix)
  if [ -f /etc/systemd/system/docker.service ]; then
    DOCKERD_PATH=$(grep -oP '(?<=ExecStart=)\S+' /etc/systemd/system/docker.service 2>/dev/null || true)
    if [ -n "$DOCKERD_PATH" ]; then
      DOCKER_DIR=$(dirname "$DOCKERD_PATH" 2>/dev/null)
      [ -x "${DOCKER_DIR}/docker" ] && DOCKER="${DOCKER_DIR}/docker"
    fi
  fi
  # Known docker paths
  if [ -z "$DOCKER" ]; then
    for p in /run/current-system/sw/bin/docker /usr/bin/docker /usr/local/bin/docker \
             /nix/var/nix/profiles/default/bin/docker "${HOME}/.nix-profile/bin/docker"; do
      [ -x "$p" ] && DOCKER="$p" && break
    done
  fi
  # command -v docker
  [ -z "$DOCKER" ] && DOCKER=$(command -v docker 2>/dev/null || true)
  # Last resort: podman as drop-in replacement
  if [ -z "$DOCKER" ] || ! "$DOCKER" info >/dev/null 2>&1; then
    PODMAN=$(command -v podman 2>/dev/null || true)
    if [ -n "$PODMAN" ]; then
      echo "Docker unavailable — falling back to podman"
      DOCKER="$PODMAN"
      RUNTIME="podman"
    fi
  fi
  [ -z "$DOCKER" ] && { echo "ERROR: neither docker nor podman found"; exit 1; }

  # ── Step 3: Ensure container runtime is ready ───────────────────────────
  if ! "$DOCKER" info >/dev/null 2>&1; then
    if [ "$RUNTIME" = "podman" ]; then
      # Podman is daemonless — if info fails, it's a config issue
      echo "Podman check failed — trying to initialize..."
      "$DOCKER" system migrate 2>/dev/null || true
    else
      echo "Docker daemon not running — starting..."
      if [ "$SYS_INIT" = "systemd" ]; then
        systemctl start docker 2>/dev/null || true
        for i in $(seq 1 15); do "$DOCKER" info >/dev/null 2>&1 && break; sleep 1; done
      elif command -v service >/dev/null 2>&1; then
        service docker start 2>/dev/null || true; sleep 3
      elif [ "$SYS_OS" = "macos" ]; then
        open -a Docker 2>/dev/null || true
        echo "Waiting for Docker Desktop..."
        for i in $(seq 1 30); do "$DOCKER" info >/dev/null 2>&1 && break; sleep 1; done
      fi
    fi
    if ! "$DOCKER" info >/dev/null 2>&1; then
      # Final fallback: if docker failed, try podman
      PODMAN=$(command -v podman 2>/dev/null || true)
      if [ -n "$PODMAN" ] && [ "$RUNTIME" != "podman" ]; then
        echo "Docker daemon failed — falling back to podman"
        DOCKER="$PODMAN"; RUNTIME="podman"
      else
        echo "ERROR: Container runtime failed to start"
        echo "  Runtime=$RUNTIME OS=$SYS_OS PKG=$SYS_PKG INIT=$SYS_INIT"
        echo "  Try: systemctl status docker / journalctl -u docker"
        exit 1
      fi
    fi
  fi
  echo "Using: $RUNTIME ($DOCKER)"

  # ── Step 4: Pull and run ───────────────────────────────────────────────
  echo "=== Docker Start: $IMG ==="
  echo "Pulling latest image..."
  "$DOCKER" pull "$IMG"

  # Build mount args (only mount paths that exist)
  local MOUNTS="-v $HOME_DIR:$HOME_DIR"
  [ -S /var/run/docker.sock ] && MOUNTS="$MOUNTS -v /var/run/docker.sock:/var/run/docker.sock"
  [ -d /etc/wireguard ]       && MOUNTS="$MOUNTS -v /etc/wireguard:/etc/wireguard:ro"
  [ -d /opt ]                 && MOUNTS="$MOUNTS -v /opt:/opt"

  echo "Starting container ($RUNTIME, home=$HOME_DIR, arch=$SYS_ARCH)..."
  local EXTRA_FLAGS="--privileged --network host --pid host"
  [ "$RUNTIME" = "podman" ] && EXTRA_FLAGS="--privileged --network host"

  exec "$DOCKER" run -it --rm \
    --name diego-env \
    --hostname "${SYS_HOSTNAME}-dev" \
    $EXTRA_FLAGS \
    $MOUNTS \
    -w "$HOME_DIR" \
    -e HOME="$HOME_DIR" \
    -e USER="${USER:-root}" \
    -e TERM="${TERM:-xterm-256color}" \
    "$IMG"
}

# ═══════════════════════════════════════════════════════════════════
# 1) INSTALL
# ═══════════════════════════════════════════════════════════════════

install_dev_fedora() {
  echo "=== Fedora/RHEL: Full Dev Toolchain ==="
  dnf install -y --skip-unavailable \
    fish git curl wget htop btop vim nano neovim \
    gcc gcc-c++ make cmake rust cargo golang \
    python3 python3-pip python3-virtualenv \
    nodejs npm \
    docker docker-compose \
    jq ripgrep fd-find bat tree fzf zoxide duf ncdu \
    rsync openssh-server wireguard-tools \
    tmux screen strace lsof bind-utils net-tools iproute nmap ncat \
    zip unzip p7zip tar gzip \
    man-db less which file \
    gnupg2 openssl \
    sqlite sqlite-devel postgresql-devel \
    gh rclone
  # Extras not in Fedora repos — install via cargo/curl
  echo "Installing extras (eza, starship, terraform)..."
  command -v eza >/dev/null 2>&1 || cargo install eza 2>/dev/null || true
  command -v starship >/dev/null 2>&1 || curl -sS https://starship.rs/install.sh | sh -s -- -y 2>/dev/null || true
  if ! command -v terraform >/dev/null 2>&1; then
    dnf config-manager addrepo --from-repofile=https://rpm.releases.hashicorp.com/fedora/hashicorp.repo 2>/dev/null || true
    dnf install -y terraform 2>/dev/null || true
  fi
  # sops/age
  command -v sops >/dev/null 2>&1 || { curl -sLo /usr/local/bin/sops https://github.com/getsops/sops/releases/latest/download/sops-v3.9.4.linux.amd64 && chmod +x /usr/local/bin/sops; } 2>/dev/null || true
  command -v age >/dev/null 2>&1 || dnf install -y age 2>/dev/null || true
  install_cloud_clis
  install_extras
}

install_dev_arch() {
  echo "=== Arch Linux: Full Dev Toolchain ==="
  /usr/bin/pacman -Syu --noconfirm
  /usr/bin/pacman -S --noconfirm --needed \
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
  install_cloud_clis
  install_extras
}

install_dev_debian() {
  echo "=== Debian/Ubuntu: Full Dev Toolchain ==="
  apt-get update -qq
  apt-get install -y -qq \
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
  install_cloud_clis
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

install_cloud_clis() {
  if ! command -v gcloud >/dev/null 2>&1; then
    echo "Installing Google Cloud SDK..."
    curl -sL https://sdk.cloud.google.com | bash -s -- --disable-prompts --install-dir=/opt 2>/dev/null || true
    ln -sf /opt/google-cloud-sdk/bin/gcloud /usr/local/bin/gcloud 2>/dev/null || true
  fi
  if ! command -v oci >/dev/null 2>&1; then
    echo "Installing OCI CLI..."
    pip3 install oci-cli 2>/dev/null || pip install oci-cli 2>/dev/null || true
  fi
  if ! command -v aws >/dev/null 2>&1; then
    echo "Installing AWS CLI..."
    pip3 install awscli 2>/dev/null || true
  fi
}

install_extras() {
  echo ""
  echo "=== Extras: Claude Code, Wrangler, Fish config ==="
  npm install -g @anthropic-ai/claude-code 2>/dev/null || true
  npm install -g wrangler 2>/dev/null || true

  # Fish as default shell
  if command -v fish >/dev/null 2>&1; then
    FISH_PATH="$(command -v fish)"
    grep -qxF "$FISH_PATH" /etc/shells 2>/dev/null || echo "$FISH_PATH" | tee -a /etc/shells >/dev/null
    chsh -s "$FISH_PATH" "$(logname 2>/dev/null || whoami)" 2>/dev/null || true
    chsh -s "$FISH_PATH" root 2>/dev/null || true
  fi

  setup_fish_config
  setup_starship

  echo ""
  echo "=== Install complete ==="
}

setup_starship() {
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
}

setup_fish_config() {
  local FISH_DIR="${HOME}/.config/fish"
  mkdir -p "$FISH_DIR"
  cat > "$FISH_DIR/config.fish" << 'FISHCONF'
if status is-interactive
    # Modern CLI
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
    alias ..="cd .."; alias ...="cd ../.."; alias ....="cd ../../.."

    # Safety
    alias rm="rm -i"; alias cp="cp -i"; alias mv="mv -i"

    # Git
    abbr -a gs "git status -sb"
    abbr -a ga "git add"; abbr -a gaa "git add --all"
    abbr -a gc "git commit"; abbr -a gcm "git commit -m"
    abbr -a gp "git push"; abbr -a gpl "git pull"
    abbr -a gl "git log --oneline --graph --decorate -20"
    abbr -a gd "git diff"; abbr -a gco "git checkout"

    # Docker
    abbr -a dps "docker ps"; abbr -a dpsa "docker ps -a"
    abbr -a dcu "docker compose up"; abbr -a dcd "docker compose down"
    abbr -a dcl "docker compose logs -f"

    # Quick
    alias c="clear"; alias h="history"
    alias ports="ss -tulanp"; alias myip="curl -s ifconfig.me"
    alias py="python3"; alias cc="claude"
    alias reload="source ~/.config/fish/config.fish"

    # PATH
    fish_add_path -m ~/.cargo/bin ~/.npm-global/bin ~/go/bin ~/.local/bin ~/.nix-profile/bin

    # Starship prompt
    if command -q starship; starship init fish | source; end

    # Zoxide (smart cd)
    if command -q zoxide; zoxide init fish | source; end
end
FISHCONF
  echo "Fish config written to $FISH_DIR/config.fish"
}

detect_distro() {
  # Reuse system detection (already ran at startup)
  case "$SYS_PKG" in
    dnf)    echo "fedora" ;;
    pacman) echo "arch" ;;
    apt)    echo "debian" ;;
    nix)    echo "nix" ;;
    brew)   echo "macos" ;;
    *)      echo "" ;;
  esac
}

do_install() {
  local detected
  detected=$(detect_distro)
  if [[ -n "$detected" ]]; then
    echo "Detected: $detected"
    read -rp "Use $detected? [Y/n] " yn
    if [[ "${yn:-y}" =~ ^[Yy]?$ ]]; then
      PICK="$detected"
    else
      local distros=(fedora arch debian nix)
      pick "Distro:" "${distros[@]}"
    fi
  else
    local distros=(fedora arch debian nix)
    pick "Distro:" "${distros[@]}"
  fi
  case "$PICK" in
    fedora) install_dev_fedora ;;
    arch)   install_dev_arch ;;
    debian) install_dev_debian ;;
    nix)    install_dev_nix ;;
  esac
}

# ═══════════════════════════════════════════════════════════════════
# 2) SSH
# ═══════════════════════════════════════════════════════════════════

do_ssh() {
  local modes=(serial ssh rescue reset kill-watchdog)
  pick "SSH Mode:" "${modes[@]}"; local mode="$PICK"
  mapfile -t vms < <(printf '%s\n' "${!VM_MAP[@]}" | sort)
  pick "VM:" "${vms[@]}"; local vm="$PICK"
  resolve_vm "$vm"

  if ! command -v gcloud >/dev/null 2>&1; then
    echo "gcloud not found — install first (alias.sh install)"
    exit 1
  fi

  case "$mode" in
    serial)         gcloud compute connect-to-serial-port "$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    ssh)            gcloud compute ssh root@"$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    rescue)         gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='sudo iptables -F INPUT; sudo iptables -P INPUT ACCEPT; sudo systemctl restart sshd 2>/dev/null || sudo systemctl restart ssh 2>/dev/null; echo done' ;;
    reset)          gcloud compute instances reset "$INSTANCE" --zone="$ZONE" --project="$PROJECT" ;;
    kill-watchdog)  gcloud compute ssh "$INSTANCE" --zone="$ZONE" --project="$PROJECT" --command='sudo systemctl stop watchdog-petter.timer watchdog-petter.service 2>/dev/null; sudo systemctl disable watchdog-petter.timer 2>/dev/null; echo done' ;;
  esac
}

# ═══════════════════════════════════════════════════════════════════
# 3) GIT CLONE
# ═══════════════════════════════════════════════════════════════════

do_git_clone() {
  local target="${1:-$HOME/git}"
  mkdir -p "$target"
  echo "=== Cloning all repos to $target ==="
  for entry in "${REPOS[@]}"; do
    IFS=: read -r name url <<< "$entry"
    url="${entry#*:}"
    name="${entry%%:*}"
    if [[ -d "$target/$name" ]]; then
      echo "  $name — exists, pulling..."
      git -C "$target/$name" pull --ff-only 2>&1 | head -1
    else
      echo "  $name — cloning..."
      git clone "$url" "$target/$name" 2>&1 | tail -1
    fi
  done
  echo "=== Done: $(ls -d "$target"/*/ 2>/dev/null | wc -l) repos in $target ==="
}

# ═══════════════════════════════════════════════════════════════════
# 4) INFO — show installed tools and aliases
# ═══════════════════════════════════════════════════════════════════

do_info() {
  echo "=== Installed Tools ==="
  local tools=(fish git node npm python3 rust cargo go docker gcloud oci aws terraform claude wrangler gh
               jq yq rg fd bat eza fzf zoxide tmux starship sops age nix rsync curl wget)
  for t in "${tools[@]}"; do
    if command -v "$t" >/dev/null 2>&1; then
      ver=$("$t" --version 2>/dev/null | head -1 || echo "ok")
      printf "  ✓ %-12s %s\n" "$t" "$ver"
    else
      printf "  ✗ %-12s not installed\n" "$t"
    fi
  done

  echo ""
  echo "=== Fish Aliases ==="
  echo "  ls→eza  ll→eza -alF  cat→bat  grep→rg  find→fd  df→duf  du→ncdu"
  echo "  cc→claude  py→python3  c→clear  h→history  ports→ss  myip→curl"
  echo ""
  echo "=== Git Abbreviations ==="
  echo "  gs gaa gc gcm gp gpl gl gd gco"
  echo ""
  echo "=== Docker Abbreviations ==="
  echo "  dps dpsa dcu dcd dcl"
  echo ""
  echo "=== Repos ==="
  for entry in "${REPOS[@]}"; do
    name="${entry%%:*}"
    echo "  $name"
  done
}

# ═══════════════════════════════════════════════════════════════════
# ENTRY POINT
# ═══════════════════════════════════════════════════════════════════

if [[ $# -ge 1 ]]; then
  case "$1" in
    commands)       do_commands ;;
    fix-journal)    do_fix_journal ;;
    docker-start)   do_docker_start ;;
    install)        do_install ;;
    ssh)            do_ssh ;;
    git-clone)      do_git_clone "${2:-$HOME/git}" ;;
    info)           do_info ;;
    *)              echo "Usage: $0 {fix-journal|docker-start|install|ssh|git-clone|info}"; exit 1 ;;
  esac
elif [[ $# -eq 0 ]]; then
  show_banner
  pick "What do you need?" "${MAIN_COMMANDS[@]}"
  case "$PICK" in
    commands)       do_commands ;;
    fix-journal)    do_fix_journal ;;
    docker-start)   do_docker_start ;;
    install)        do_install ;;
    ssh)            do_ssh ;;
    git-clone)      do_git_clone ;;
    info)           do_info ;;
  esac
fi
