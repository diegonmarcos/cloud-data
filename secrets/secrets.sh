#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════
# Decrypt all sops-encrypted *-secrets*.yaml → *.secrets (KEY=VALUE)
# Uses age key from ~/.config/sops/age/keys.txt
# ══════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AGE_KEY="${SOPS_AGE_KEY_FILE:-$HOME/.config/sops/age/keys.txt}"

if [ ! -f "$AGE_KEY" ]; then
  echo "ERROR: Age key not found at $AGE_KEY" >&2
  exit 1
fi

if ! command -v sops >/dev/null 2>&1; then
  echo "ERROR: sops not found in PATH" >&2
  exit 1
fi

export SOPS_AGE_KEY_FILE="$AGE_KEY"

count=0
failed=0
skipped=0

for yaml in "$SCRIPT_DIR"/*.yaml; do
  [ -f "$yaml" ] || continue
  base=$(basename "$yaml" .yaml)
  out="$SCRIPT_DIR/${base}.secrets"

  # Decrypt and convert YAML to KEY=VALUE, stripping the sops: metadata block
  if decrypted=$(sops -d "$yaml" 2>/dev/null); then
    echo "$decrypted" | awk '
      /^sops:/ { stop=1 }
      !stop && /^[A-Za-z_][A-Za-z0-9_]*:/ {
        key = $0
        sub(/:.*/, "", key)
        val = $0
        sub(/^[^:]*: */, "", val)
        # Strip surrounding quotes if present
        if (val ~ /^".*"$/) { val = substr(val, 2, length(val)-2) }
        if (val ~ /^'"'"'.*'"'"'$/) { val = substr(val, 2, length(val)-2) }
        print key "=" val
      }
    ' > "$out"
    count=$((count + 1))
    lines=$(wc -l < "$out")
    echo "  OK  ${base} (${lines} keys)"
  else
    failed=$((failed + 1))
    echo "  FAIL ${base}" >&2
    rm -f "$out"
  fi
done

echo ""
echo "Done: ${count} decrypted | ${failed} failed"
