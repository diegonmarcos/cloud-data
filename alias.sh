#!/usr/bin/env bash
# Diego's VM toolkit — install, ssh, clone, info
# Usage: ./alias.sh                # interactive
#        ./alias.sh <cmd> [args]   # direct
set -euo pipefail
# Force real system binaries FIRST (bypass nix guardrail wrappers)
export PATH="/usr/bin:/usr/sbin:/usr/local/bin:/bin:/sbin:/nix/var/nix/profiles/default/bin:${HOME:-/root}/.nix-profile/bin:/run/current-system/sw/bin:$PATH"

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

MAIN_COMMANDS=(docker-start install ssh git-clone info)

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
# 0) DOCKER START — pull & run dev environment container
# ═══════════════════════════════════════════════════════════════════

do_docker_start() {
  local IMG="ghcr.io/diegonmarcos/diego-user-env:latest"
  local HOME_DIR="${HOME:-/root}"

  # Check docker + git, install if missing
  if ! command -v docker >/dev/null 2>&1 || ! command -v git >/dev/null 2>&1; then
    echo "Docker or git not found — installing..."
    if command -v dnf >/dev/null 2>&1; then
      dnf install -y --skip-unavailable docker git curl
      systemctl enable --now docker 2>/dev/null || true
    elif command -v apt-get >/dev/null 2>&1; then
      apt-get update -qq && apt-get install -y -qq docker.io git curl
      systemctl enable --now docker 2>/dev/null || true
    elif command -v pacman >/dev/null 2>&1; then
      pacman -Sy --noconfirm docker git curl
      systemctl enable --now docker 2>/dev/null || true
    elif command -v nix-env >/dev/null 2>&1; then
      nix-env -iA nixpkgs.docker nixpkgs.git nixpkgs.curl
    else
      echo "No package manager found — install docker and git manually"
      exit 1
    fi
  fi

  # Find docker binary (may be in Nix store)
  DOCKER="$(command -v docker 2>/dev/null || find /nix/store -maxdepth 3 -name docker -type f 2>/dev/null | head -1 || echo docker)"

  # Ensure Docker daemon is running
  if ! "$DOCKER" info >/dev/null 2>&1; then
    echo "Docker daemon not running — starting..."
    systemctl enable --now docker 2>/dev/null || service docker start 2>/dev/null || true
    sleep 3
    if ! "$DOCKER" info >/dev/null 2>&1; then
      echo "ERROR: Docker daemon failed to start"
      exit 1
    fi
  fi

  echo "=== Docker Start: $IMG ==="
  echo "Pulling latest image..."
  "$DOCKER" pull "$IMG"
  echo "Starting container (root, home mounted at $HOME_DIR)..."
  exec "$DOCKER" run -it --rm \
    --name diego-env \
    --hostname "$(hostname)-dev" \
    --privileged \
    --network host \
    --pid host \
    -v "$HOME_DIR":"$HOME_DIR" \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v /etc/wireguard:/etc/wireguard:ro \
    -v /opt:/opt \
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
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "$ID" in
      fedora|rhel|centos|rocky|alma) echo "fedora" ;;
      arch|manjaro)                  echo "arch" ;;
      debian|ubuntu|pop|mint)        echo "debian" ;;
      nixos)                         echo "nix" ;;
      *)                             echo "" ;;
    esac
  else
    echo ""
  fi
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
    docker-start)   do_docker_start ;;
    install)        do_install ;;
    ssh)            do_ssh ;;
    git-clone)      do_git_clone "${2:-$HOME/git}" ;;
    info)           do_info ;;
    *)              echo "Usage: $0 {docker-start|install|ssh|git-clone|info}"; exit 1 ;;
  esac
elif [[ $# -eq 0 ]]; then
  pick "What do you need?" "${MAIN_COMMANDS[@]}"
  case "$PICK" in
    docker-start)   do_docker_start ;;
    install)        do_install ;;
    ssh)            do_ssh ;;
    git-clone)      do_git_clone ;;
    info)           do_info ;;
  esac
fi
