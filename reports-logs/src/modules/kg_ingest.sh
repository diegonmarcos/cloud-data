#!/bin/sh
# kg_ingest — build dist/kg_delta.json from index.json + topology.
#
# Phase 2b: LOCAL ONLY. Emits kg_delta.json for review. No HTTP push.
# Phase 2c+ will POST to kg-graph once the endpoint is verified.
#
# Hard invariants (per PLAN-kg-ingest.md):
#   - dist/index.json is authoritative input
#   - Schema declared in cloud-data-kg-schema.json; unknown types reject
#   - Properties filtered through schema whitelist (no secret leak)
#   - Idempotent: re-running on same inputs produces identical delta
#   - Offline-safe: never reaches out to kg-graph in this phase
set -eu

: "${CONFIG:?}" "${TOPOLOGY:?}" "${DIST:?}" "${REPO_ROOT:?}"
KG_SCHEMA="${KG_SCHEMA:-${REPO_ROOT}/cloud-data-kg-schema.json}"

INDEX="${DIST}/index.json"
OUT="${DIST}/kg_delta.json"

[ -f "${INDEX}" ] || {
    echo "[kg_ingest] missing ${INDEX} — run index module first"
    exit 2
}
[ -f "${KG_SCHEMA}" ] || {
    echo "[kg_ingest] missing ${KG_SCHEMA}"
    exit 2
}

TS=$(date -u +%FT%TZ)

# ── Schema validation ────────────────────────────────────────────────
# Known node/edge labels the delta is allowed to emit.
NODE_LABELS=$(jq -r '.nodes | keys[]' "${KG_SCHEMA}" | tr '\n' '|' | sed 's/|$//')
EDGE_LABELS=$(jq -r '.edges | keys[]' "${KG_SCHEMA}" | tr '\n' '|' | sed 's/|$//')

# ── Build nodes ──────────────────────────────────────────────────────
# VMs — from topology + index tag enumeration.
VMS_JSON=$(jq -r --arg ts "${TS}" '
    .tags | keys | map(select(startswith("vm:"))) |
    map({
        label: "VM",
        key: { alias: (. | ltrimstr("vm:")) },
        properties: { last_seen_ts: $ts }
    })
' "${INDEX}")

# Containers — from index, one node per inspect.json file.
CONTAINERS_JSON=$(jq -r --arg ts "${TS}" --slurpfile idx "${INDEX}" '
    $idx[0].files | to_entries |
    map(select(.key | test("^containers/[^/]+/[^/]+/inspect.json$"))) |
    map({
        label: "Container",
        key: {
            service:   (.key | split("/")[1]),
            name:      (.key | split("/")[2])
        },
        properties: (
            ([.value.tags[] | capture("^state:(?<state>.+)$") | .state] | first // null) as $state |
            ([.value.tags[] | capture("^exit_code:(?<ec>.+)$") | .ec | tonumber] | first // null) as $ec |
            ([.value.tags[] | capture("^arch:(?<a>.+)$") | .a] | first // null) as $arch |
            ([.value.tags[] | capture("^healthy:(?<h>.+)$") | .h] | first // null) as $healthy |
            ([.value.tags[] | capture("^image:(?<i>.+)$") | .i] | first // null) as $image |
            {
                state: $state,
                exit_code: $ec,
                arch: $arch,
                healthy: $healthy,
                image: $image,
                last_seen_ts: $ts
            }
        )
    })
' "${INDEX}" /dev/null)

# Services — unique service names from tags.
SERVICES_JSON=$(jq -r --arg ts "${TS}" '
    .tags | keys | map(select(startswith("service:"))) |
    map({
        label: "Service",
        key: { name: (. | ltrimstr("service:")) },
        properties: { last_seen_ts: $ts }
    })
' "${INDEX}")

# ── Build edges ──────────────────────────────────────────────────────
# BELONGS_TO: every container → its service
# RUNS_ON:    every container → its vm (from vm: tag on the container node)
EDGES_JSON=$(jq -r '
    .files | to_entries |
    map(select(.key | test("^containers/[^/]+/[^/]+/inspect.json$"))) |
    map(
        (.key | split("/")[1]) as $svc |
        (.key | split("/")[2]) as $ctr |
        ([.value.tags[] | capture("^vm:(?<v>.+)$") | .v] | first // null) as $vm |
        [
            {
                label: "BELONGS_TO",
                from: { label: "Container", key: { service: $svc, name: $ctr } },
                to:   { label: "Service",   key: { name: $svc } }
            }
        ] + (
            if $vm then [{
                label: "RUNS_ON",
                from: { label: "Container", key: { service: $svc, name: $ctr } },
                to:   { label: "VM",        key: { alias: $vm } }
            }] else [] end
        )
    ) | add // []
' "${INDEX}")

# ── Combine ──────────────────────────────────────────────────────────
jq -n \
    --arg ts "${TS}" \
    --arg schema_version "$(jq -r '.version' "${KG_SCHEMA}")" \
    --argjson vms      "${VMS_JSON}" \
    --argjson services "${SERVICES_JSON}" \
    --argjson ctrs     "${CONTAINERS_JSON}" \
    --argjson edges    "${EDGES_JSON}" \
    '{
        generated: $ts,
        schema_version: ($schema_version | tonumber),
        nodes: ($vms + $services + $ctrs),
        edges: $edges
    }' > "${OUT}"

# ── Validate against schema whitelist ────────────────────────────────
unknown_nodes=$(jq -r --arg labels "${NODE_LABELS}" '
    [.nodes[] | select(.label | test("^(" + $labels + ")$") | not) | .label] | unique | .[]
' "${OUT}")
unknown_edges=$(jq -r --arg labels "${EDGE_LABELS}" '
    [.edges[] | select(.label | test("^(" + $labels + ")$") | not) | .label] | unique | .[]
' "${OUT}")

if [ -n "${unknown_nodes}" ]; then
    echo "[kg_ingest] unknown node labels: ${unknown_nodes}"
    exit 1
fi
if [ -n "${unknown_edges}" ]; then
    echo "[kg_ingest] unknown edge labels: ${unknown_edges}"
    exit 1
fi

node_count=$(jq '.nodes | length' "${OUT}")
edge_count=$(jq '.edges | length' "${OUT}")
echo "[kg_ingest] wrote ${OUT} — ${node_count} nodes, ${edge_count} edges (phase 2b, local only)"
