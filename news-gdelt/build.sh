#!/usr/bin/env bash
# ╔══════════════════════════════════════════════════════════════════╗
# ║  fin-api endpoint snapshot — universal engine                    ║
# ║                                                                  ║
# ║  Reads src/endpoints.json (data-driven), fetches every GET       ║
# ║  endpoint, writes the response to dist/<name>.json. Validates    ║
# ║  each output is parseable JSON (FIRE rule 4 tester).             ║
# ║                                                                  ║
# ║  Adding an endpoint = add an entry to src/endpoints.json.        ║
# ║  NEVER hardcode endpoints in this script.                        ║
# ║                                                                  ║
# ║  Overrides:                                                      ║
# ║    BASE_URL=http://oci-apps:8340  ./build.sh   # bypass Caddy    ║
# ║    BEARER_TOKEN=$(jq -r .access_token ...)  ./build.sh           ║
# ╚══════════════════════════════════════════════════════════════════╝

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPEC="$SCRIPT_DIR/src/endpoints.json"
DIST="$SCRIPT_DIR/dist"
GIT_ROOT="${GIT_ROOT:-$HOME/git}"

# ── colours ────────────────────────────────────────────────────────────
BLUE='\033[0;34m'; GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[0;33m'; NC='\033[0m'
log()       { printf "%b[%s]%b %s\n"   "$BLUE"   "$(date +%H:%M:%S)" "$NC" "$*"; }
log_ok()    { printf "%b[%s] ✓%b %s\n" "$GREEN"  "$(date +%H:%M:%S)" "$NC" "$*"; }
log_warn()  { printf "%b[%s] ⚠%b %s\n" "$YELLOW" "$(date +%H:%M:%S)" "$NC" "$*"; }
log_error() { printf "%b[%s] ✗%b %s\n" "$RED"    "$(date +%H:%M:%S)" "$NC" "$*"; } >&2

# ── preflight ──────────────────────────────────────────────────────────
[ -f "$SPEC" ] || { log_error "spec not found: $SPEC"; exit 1; }
command -v jq   >/dev/null || { log_error "jq required";   exit 1; }
command -v curl >/dev/null || { log_error "curl required"; exit 1; }

mkdir -p "$DIST"

# ── load spec ──────────────────────────────────────────────────────────
SERVICE=$(jq -r '.service'  "$SPEC")
BASE_URL="${BASE_URL:-$(jq -r '.base_url' "$SPEC")}"
DEFAULT_AUTH=$(jq -r '.auth.default // "none"' "$SPEC")
TOKEN_FILE_REL=$(jq -r '.auth.bearer_token_file // empty' "$SPEC")
TOKEN_FIELD=$(jq -r '.auth.bearer_token_field // "access_token"' "$SPEC")

# ── resolve bearer token (env wins over file) ──────────────────────────
TOKEN="${BEARER_TOKEN:-}"
if [ -z "$TOKEN" ] && [ -n "$TOKEN_FILE_REL" ] && [ -f "$GIT_ROOT/$TOKEN_FILE_REL" ]; then
    TOKEN=$(jq -r ".${TOKEN_FIELD} // empty" "$GIT_ROOT/$TOKEN_FILE_REL" 2>/dev/null || echo "")
fi
if [ -z "$TOKEN" ]; then
    log_warn "no bearer token (env BEARER_TOKEN unset, $TOKEN_FILE_REL absent) — auth=bearer endpoints will fail"
fi

DEFAULT_TIMEOUT=$(jq -r '.defaults.timeout_seconds // 10' "$SPEC")
log "service=$SERVICE  base_url=$BASE_URL  auth_default=$DEFAULT_AUTH  default_timeout=${DEFAULT_TIMEOUT}s"
log "dist=$DIST"

# ── slug helper: "climate change" → "climate_change" ───────────────────
slugify() { echo "$1" | tr ' ' '_' | tr -cd '[:alnum:]_-'; }

# ── fetch one URL with given output name + timeout (single helper) ─────
fetch_one() {
    local name="$1" url="$2" method="$3" auth="$4" timeout="$5" body_format="${6:-json}"
    local out="$DIST/${name}.json" meta="$DIST/${name}.meta"
    TOTAL=$((TOTAL+1))

    local auth_args=()
    if [ "$auth" = "bearer" ] && [ -n "$TOKEN" ]; then
        auth_args=(-H "Authorization: Bearer $TOKEN")
    fi

    local http_code
    http_code=$(curl -sS -o "$out" -w "%{http_code}" \
        -X "$method" \
        -H "Accept: application/json" \
        "${auth_args[@]}" \
        --max-time "$timeout" \
        "$url" || echo "000")

    # ── normalise NDJSON → JSON array (so the standard jq tester applies) ─
    # ntfy and other streaming endpoints return one JSON object per line.
    # Spec opts in via "body_format":"ndjson"; engine slurps with jq -s.
    if [ "$body_format" = "ndjson" ] && [ -s "$out" ]; then
        if jq -s . "$out" > "${out}.tmp" 2>/dev/null; then
            mv "${out}.tmp" "$out"
        else
            rm -f "${out}.tmp"
        fi
    fi

    # ── tester: 2xx + valid JSON (FIRE rule 5) ─────────────────────────
    if [[ "$http_code" =~ ^2 ]] && jq empty "$out" 2>/dev/null; then
        log_ok "$name → HTTP $http_code  ($(wc -c <"$out" | tr -d ' ')b, $(jq -c 'if type=="array" then length elif type=="object" then (keys|length) else 1 end' "$out" 2>/dev/null) items/keys)"
        printf '{"name":"%s","url":"%s","http_code":%s,"fetched_at":"%s","timeout_s":%s,"body_format":"%s"}\n' \
            "$name" "$url" "$http_code" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$timeout" "$body_format" > "$meta"
        OK=$((OK+1))
    else
        log_error "$name → HTTP $http_code  ($url)  body=$(head -c 200 "$out" 2>/dev/null | tr '\n' ' ')"
        FAIL=$((FAIL+1))
    fi
}

# ── resolve q_values_from: read source-of-truth file via jq path ───────
# Looks up `file` in dist/ first (lets one fetch feed the next, e.g.
# news-gdelt fetches /topics then expands per-topic), then falls through
# to src/ (lets an upstream-not-running service declare its topic list
# locally — the truly declarative pattern when no API exposes it).
resolve_q_values_from() {
    local file="$1" jqp="$2"
    for prefix in "$DIST" "$SCRIPT_DIR/src" "$SCRIPT_DIR"; do
        if [ -f "$prefix/$file" ]; then
            jq -r "$jqp" "$prefix/$file"
            return 0
        fi
    done
    log_warn "q_values_from file not found in dist/ or src/: $file — skipping expansion"
    return 1
}

# ── fetch loop ─────────────────────────────────────────────────────────
TOTAL=0; OK=0; FAIL=0
while IFS= read -r endpoint; do
    name=$(jq -r '.name'                     <<<"$endpoint")
    method=$(jq -r '.method // "GET"'         <<<"$endpoint")
    path=$(jq -r '.path'                      <<<"$endpoint")
    auth=$(jq -r ".auth // \"$DEFAULT_AUTH\"" <<<"$endpoint")
    timeout=$(jq -r ".timeout_seconds // $DEFAULT_TIMEOUT" <<<"$endpoint")
    body_format=$(jq -r '.body_format // "json"' <<<"$endpoint")

    # Either explicit q_values list, or q_values_from { file, jq } reference
    q_values=$(jq -r '.q_values // empty | if type=="array" then .[] else empty end' <<<"$endpoint")
    if [ -z "$q_values" ]; then
        qvf_file=$(jq -r '.q_values_from.file // empty' <<<"$endpoint")
        qvf_jq=$(jq -r '.q_values_from.jq   // empty'   <<<"$endpoint")
        if [ -n "$qvf_file" ] && [ -n "$qvf_jq" ]; then
            q_values=$(resolve_q_values_from "$qvf_file" "$qvf_jq" || echo "")
        fi
    fi

    if [ -z "$q_values" ]; then
        # Single fetch — no expansion
        fetch_one "$name" "${BASE_URL}${path}" "$method" "$auth" "$timeout" "$body_format"
    else
        # Expanded fetch — one request per q value, output dist/<name>__<slug>.json
        while IFS= read -r qv; do
            [ -z "$qv" ] && continue
            slug=$(slugify "$qv")
            qv_enc=$(jq -rn --arg v "$qv" '$v|@uri')
            expanded_path="${path//\{q\}/$qv_enc}"
            fetch_one "${name}__${slug}" "${BASE_URL}${expanded_path}" "$method" "$auth" "$timeout" "$body_format"
        done <<< "$q_values"
    fi
done < <(jq -c '.endpoints[]' "$SPEC")

# ── summary + exit ─────────────────────────────────────────────────────
echo
if [ "$FAIL" -eq 0 ]; then
    log_ok "fetched $OK/$TOTAL endpoints → $DIST"
    exit 0
else
    log_error "fetched $OK/$TOTAL — $FAIL failed"
    exit 1
fi
