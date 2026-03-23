#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════
# Decrypt all sops-encrypted *.yaml → .secrets + .json.secrets
# Outputs:
#   {name}.secrets       — KEY=VALUE env file
#   {name}.json.secrets  — JSON object { "KEY": "VALUE", ... }
#   cloud-secrets.json.secrets — consolidated all services
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

if ! command -v jq >/dev/null 2>&1; then
  echo "ERROR: jq not found in PATH" >&2
  exit 1
fi

export SOPS_AGE_KEY_FILE="$AGE_KEY"

count=0
failed=0

# Consolidated JSON: { "service-name": { "KEY": "VAL", ... }, ... }
CONSOLIDATED="{}"

for yaml in "$SCRIPT_DIR"/*.yaml; do
  [ -f "$yaml" ] || continue
  base=$(basename "$yaml" .yaml)
  out_env="$SCRIPT_DIR/${base}.secrets"
  out_json="$SCRIPT_DIR/${base}.json.secrets"

  if decrypted=$(sops -d "$yaml" 2>/dev/null); then
    # 1. KEY=VALUE env file
    echo "$decrypted" | awk '
      /^sops:/ { stop=1 }
      !stop && /^[A-Za-z_][A-Za-z0-9_]*:/ {
        key = $0
        sub(/:.*/, "", key)
        val = $0
        sub(/^[^:]*: */, "", val)
        if (val ~ /^".*"$/) { val = substr(val, 2, length(val)-2) }
        if (val ~ /^'"'"'.*'"'"'$/) { val = substr(val, 2, length(val)-2) }
        print key "=" val
      }
    ' > "$out_env"

    # 2. JSON object
    SERVICE_JSON=$(echo "$decrypted" | awk '
      /^sops:/ { stop=1 }
      !stop && /^[A-Za-z_][A-Za-z0-9_]*:/ {
        key = $0
        sub(/:.*/, "", key)
        val = $0
        sub(/^[^:]*: */, "", val)
        if (val ~ /^".*"$/) { val = substr(val, 2, length(val)-2) }
        if (val ~ /^'"'"'.*'"'"'$/) { val = substr(val, 2, length(val)-2) }
        printf "%s\t%s\n", key, val
      }
    ' | jq -Rn '[inputs | split("\t") | {(.[0]): .[1]}] | add // {}')

    echo "$SERVICE_JSON" > "$out_json"

    # 3. Add to consolidated
    CONSOLIDATED=$(echo "$CONSOLIDATED" | jq --arg svc "$base" --argjson data "$SERVICE_JSON" '. + {($svc): $data}')

    count=$((count + 1))
    lines=$(wc -l < "$out_env")
    echo "  OK  ${base} (${lines} keys)"
  else
    failed=$((failed + 1))
    echo "  FAIL ${base}" >&2
    rm -f "$out_env" "$out_json"
  fi
done

# Write consolidated
echo "$CONSOLIDATED" | jq '.' > "$SCRIPT_DIR/cloud-secrets.json.secrets"

echo ""
echo "Done: ${count} decrypted | ${failed} failed"
echo "Consolidated: cloud-secrets.json.secrets ($(echo "$CONSOLIDATED" | jq 'keys | length') services)"
