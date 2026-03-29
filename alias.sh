#!/usr/bin/env bash
# Usage: ./alias.sh serial <vm-alias>
# Maps VM aliases to GCP instance names for serial console access

set -euo pipefail

declare -A VM_MAP=(
  [gcp-proxy]="arch-1:us-central1-a"
  [gcp-t4]="ollama-spot-gpu:us-central1-a"
)

PROJECT="diegonmarcos-infra-prod"

cmd="${1:-}"
vm="${2:-}"

case "$cmd" in
  serial)
    if [[ -z "$vm" || -z "${VM_MAP[$vm]:-}" ]]; then
      echo "Unknown VM: $vm"
      echo "Available: ${!VM_MAP[*]}"
      exit 1
    fi
    IFS=: read -r instance zone <<< "${VM_MAP[$vm]}"
    exec gcloud compute connect-to-serial-port "$instance" --zone="$zone" --project="$PROJECT"
    ;;
  rescue)
    if [[ -z "$vm" || -z "${VM_MAP[$vm]:-}" ]]; then
      echo "Unknown VM: $vm"
      echo "Available: ${!VM_MAP[*]}"
      exit 1
    fi
    echo "=== Rescue $vm — flush iptables + restart sshd ==="
    IFS=: read -r instance zone <<< "${VM_MAP[$vm]}"
    gcloud compute ssh "$instance" --zone="$zone" --project="$PROJECT" --command='
      sudo iptables -F INPUT
      sudo iptables -P INPUT ACCEPT
      sudo systemctl restart sshd 2>/dev/null || sudo systemctl restart ssh 2>/dev/null
      echo "iptables flushed, sshd restarted"
    '
    ;;
  *)
    echo "Usage: $0 {serial|rescue} <vm-alias>"
    echo "VMs: ${!VM_MAP[*]}"
    exit 1
    ;;
esac
