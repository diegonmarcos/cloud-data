# Plan — Ingest `reports-logs/dist/index.json` into `kg-graph` via `cloud-cgc-mcp`

> **Status**: PROPOSED — not implemented. Phase 2 of reports-logs work.
> **Dependencies**: `reports-logs/dist/index.json` (already built), `cloud-cgc-mcp` (running on oci-apps), `ca-dat_kg-graph` SurrealDB instance.

## Goal

Unlock ad-hoc multi-hop queries across **infra evidence + code provenance** without spinning up a new daemon. Reuse `kg-graph` (SurrealDB backend behind `cloud-cgc-mcp`) as the optional query accelerator. Local `jq` against `index.json` remains the fast path; kg-graph is opt-in for complex traversals (blast radius, cross-repo ownership, multi-hop dependencies).

## Hard invariants (do NOT break)

1. **`dist/index.json` is authoritative.** kg-graph is a derived view. If kg-graph dies, nothing in `reports-logs` breaks.
2. **Idempotent ingestion.** Re-pushing the same snapshot must not duplicate nodes/edges.
3. **Schema is declarative.** Lives in `cloud-data-kg-schema.json`, referenced by both the ingest script and anything that queries.
4. **Zero hardcoded VM/service/container names.** All node populations come from `cloud-data-topology.json` + `dist/index.json`.
5. **No secrets in graph.** Filter out `inspect.json` env values at ingest time — schema has no fields for them.

## Schema (Graph model)

### Node types

| Label | Key | Source | Sample properties |
|---|---|---|---|
| `VM`              | alias (`oci-apps`) | topology.vms[] | `role, wg_ip, public_ip, arch, region` |
| `Service`         | name (`photos-webhook`) | topology.services | `category, public, ports.app, domain` |
| `Container`       | (service,name) | index.json `kind:container` | `state, exit_code, arch, healthy, image, last_seen` |
| `Image`           | digest or tag | inspect.json `.Config.Image` | `registry, name, tag, arch` |
| `Domain`          | FQDN | cloud-data-configs + cf_records | `zone, type, proxy, wildcard_parent` |
| `TLSEndpoint`     | (host,port) | config.modules.tls.endpoints | `cipher, sig_alg, protocol, san[], expires` |
| `SystemdUnit`     | (vm,unit) | config.modules.systemd.units_per_vm | `active_state, sub_state` |
| `CloudflareRecord`| (zone,name,type) | cloudflare/all_records.json | `value, ttl, proxied` |
| `Repo`            | path (`~/git/cloud`) | knowledge_inventory (existing) | `visibility, default_branch` |
| `SecretExposure`  | (repo,commit,path,pattern) | sec-data repo_findings | `severity, literal_hash` |
| `HealthCheck`     | (report,name) | reports' json output | `severity, passed, category` |

### Edge types

| Edge | From | To | Source of truth | Notes |
|---|---|---|---|---|
| `RUNS_ON`         | Container | VM | index tag `vm:<alias>` | |
| `BELONGS_TO`      | Container | Service | path `containers/<svc>/<ctr>/` | |
| `BUILDS`          | Image | Container | inspect `.Config.Image` | |
| `DECLARED_IN`     | Service | Repo | knowledge_service_spec | |
| `DEPENDS_ON`      | Service | Service | cloud-data-deps.json | already materialized in kg |
| `ROUTES_TO`       | Domain | Service | cloud-data-dns-services.json + caddy | |
| `RESOLVES_TO`     | Domain | `IP` literal | dns/<domain>.txt | IP as string property, not node |
| `TERMINATES_AT`   | TLSEndpoint | Container | (port match via topology) | |
| `MANAGED_BY`      | Container | SystemdUnit | docker.service edge | |
| `AUTHORITATIVE_FOR`| CloudflareRecord | Domain | cloudflare/all_records | |
| `EXPOSES`         | SecretExposure | Repo | repo-scan findings | |
| `FLAGS`           | HealthCheck | (VM\|Service\|Container\|Domain) | reports' failed checks | polymorphic |

### Facet-indexed properties (for fast filter)

Tag-style properties mirror `index.json` facets so graph queries can join with evidence:
- `state` (running/exited/created/unreachable)
- `failed` (bool)
- `arch` (arm64/amd64)
- `severity` (critical/warning/info)
- `public` (bool)

## Ingestion module — `reports-logs/src/modules/kg_ingest.sh`

### Inputs
```
reports-logs/dist/index.json           # tag index (authoritative)
reports-logs/dist/vms/<vm>/...          # content for property hydration
reports-logs/dist/containers/.../inspect.json
cloud-data/cloud-data-topology.json
cloud-data/cloud-data-configs.json
cloud-data/cloud-data-dns-services.json
cloud-data/cloud-data-kg-schema.json   # NEW — node+edge type declarations
```

### Outputs
```
reports-logs/dist/kg_delta.json         # nodes+edges to upsert (auditable)
```
Then POSTs `kg_delta.json` to the kg-graph HTTP endpoint.

### Pipeline steps
1. **Load** schema from `cloud-data-kg-schema.json` — validates ingest won't emit unknown node/edge types.
2. **Build node set** by walking `index.json.files` and grouping by facet. Each unique `vm:X`, `service:Y`, `container:Z`, etc. becomes a node entry.
3. **Hydrate properties** by reading the small subset of evidence files referenced by each node (via the `evidence` back-pointer in index).
4. **Build edge set** from path structure + topology cross-ref (same logic already in `index.sh` — DRY-able into a shared helper).
5. **Emit `kg_delta.json`** in the schema kg-graph accepts (likely JSON with `{nodes:[], edges:[]}` per SurrealDB Surrealist format — VERIFY).
6. **POST to kg-graph** idempotent upsert endpoint (one HTTP call, retry with backoff).
7. **Tag the snapshot** with `generated_at` so queries can filter by freshness.

### Idempotency strategy
- Every node has a deterministic key derived from its identity (e.g. `vm:oci-apps`, `container:photos-webhook/photos-db`).
- Upsert = `UPSERT node:<key> MERGE {...props}` in SurrealDB. Repeated ingest is a no-op if nothing changed.
- Edges use composite keys `(from, label, to)` — same idempotency.
- Stale nodes (present in kg-graph but not in latest `kg_delta.json`) get marked `state:stale` with `last_seen_ts`; never hard-deleted (preserves history for diffs).

## Queries unlocked

### Blast-radius traversal
```
-- "If OCI S3 key rotates, which services break?"
SELECT ->DEPENDS_ON->Service FROM Service
  WHERE ->USES->Credential:oci_s3_key
  GROUP BY service
```
(Requires a `Credential` node type — additive extension.)

### Cross-repo ownership
```
-- "Which git repo owns the code behind the failing container?"
SELECT <-BELONGS_TO<-Container.name AS ctr,
       ->DECLARED_IN->Repo.path AS repo
  FROM Service WHERE ->ROUTES_TO<-Domain.fqdn = 'mail.diegonmarcos.com'
```

### Impact of arch-mismatch
```
-- "Which services still have amd64-only images pulled onto ARM VMs?"
SELECT Container.name, Image.name FROM Container
  WHERE Container.arch = 'amd64' AND ->RUNS_ON->VM.arch = 'arm64'
```

### Secret leak propagation
```
-- "If credential X is in git history of repo Y, which live services deployed from that repo are still in use?"
SELECT SecretExposure, Repo, ->DECLARED_IN<-Service.name, ->ROUTES_TO<-Domain.fqdn
```

These are what `jq` struggles with because they need 3+ relationship hops.

## Schema file — `cloud-data-kg-schema.json` (NEW)

```json
{
  "_comment": "Declares kg-graph schema for reports-logs ingestion. Source of truth for node/edge types. Ingest MUST fail loudly on unknown types.",
  "version": 1,
  "nodes": {
    "VM":        { "key": ["alias"],             "properties": ["role","wg_ip","public_ip","arch","region"] },
    "Service":   { "key": ["name"],              "properties": ["category","public","ports","domain"] },
    "Container": { "key": ["service","name"],    "properties": ["state","exit_code","arch","healthy","image","last_seen"] },
    "Image":     { "key": ["ref"],               "properties": ["registry","name","tag","digest","arch"] },
    "Domain":    { "key": ["fqdn"],              "properties": ["zone","type","proxied","wildcard_parent"] },
    "TLSEndpoint":{"key": ["host","port"],       "properties": ["cipher","sig_alg","protocol","san","expires_at"] },
    "SystemdUnit":{"key": ["vm","unit"],         "properties": ["active_state","sub_state"] },
    "CloudflareRecord":{"key":["zone","name","type"], "properties":["value","ttl","proxied"] },
    "Repo":       { "key": ["path"],             "properties": ["visibility","default_branch"] },
    "SecretExposure":{"key":["repo","commit","path","pattern"], "properties":["severity","literal_hash"] },
    "HealthCheck":{ "key": ["report","name"],    "properties": ["severity","passed","category","details"] }
  },
  "edges": {
    "RUNS_ON":   { "from": "Container",       "to": "VM" },
    "BELONGS_TO":{ "from": "Container",       "to": "Service" },
    "BUILDS":    { "from": "Image",           "to": "Container" },
    "DECLARED_IN":{"from": "Service",         "to": "Repo" },
    "DEPENDS_ON":{ "from": "Service",         "to": "Service" },
    "ROUTES_TO": { "from": "Domain",          "to": "Service" },
    "TERMINATES_AT":{"from":"TLSEndpoint",    "to": "Container" },
    "MANAGED_BY":{ "from": "Container",       "to": "SystemdUnit" },
    "AUTHORITATIVE_FOR":{"from":"CloudflareRecord","to":"Domain" },
    "EXPOSES":   { "from": "SecretExposure",  "to": "Repo" },
    "FLAGS":     { "from": "HealthCheck",     "to": ["VM","Service","Container","Domain"] }
  }
}
```

## Wiring

### Build.sh dispatcher addition
```
  kg_ingest)
      ensure_dist
      [ -f "${DIST}/index.json" ] || sh "${MODULES_DIR}/index.sh"
      CONFIG="${CONFIG}" TOPOLOGY="${TOPOLOGY}" DIST="${DIST}" \
        KG_SCHEMA="${REPO_ROOT}/cloud-data-kg-schema.json" \
        sh "${MODULES_DIR}/kg_ingest.sh"
      ;;
```

### Optional: chain after `all`
Add `kg_ingest` as the very last step of `all` — runs only if `cloud-data-kg-schema.json` exists (opt-in by file presence).

## Testers

Required before this is "done":

1. **Ingest idempotency**
   ```
   sh build.sh all && sh build.sh kg_ingest
   sh build.sh kg_ingest  # second run
   # Expect: second run produces same node/edge count; no duplicates in kg-graph
   ```

2. **Schema enforcement**
   ```
   # Mutate cloud-data-kg-schema.json to REMOVE "Container"
   sh build.sh kg_ingest
   # Expect: exit 1, stderr "unknown node type: Container"
   ```

3. **Round-trip via graphrag MCP tool**
   ```
   # After ingest, query via cloud-cgc-mcp:
   cgc_octocode_graphrag query="containers with state=exited on vm=oci-apps"
   # Expect: same 3 rows as jq tester in index.sh commit message
   ```

4. **Offline degradation**
   ```
   # Stop kg-graph
   docker stop kg-graph
   sh build.sh all     # must succeed
   sh build.sh kg_ingest  # must exit non-zero with clear error, NOT hang
   # reports-logs/build.sh all without kg_ingest remains fully functional
   ```

5. **Stale detection**
   ```
   # Delete a container in reality
   # Re-run: sh build.sh all && sh build.sh kg_ingest
   # Query kg-graph for that container → expect state=stale, last_seen < now
   ```

## Phased rollout

| Phase | Deliverable | Gate |
|---|---|---|
| 2a | `cloud-data-kg-schema.json` committed | schema reviewed |
| 2b | `kg_ingest.sh` emits `kg_delta.json` only (no HTTP push) | delta JSON looks sane |
| 2c | HTTP push to kg-graph dev endpoint | idempotency tester passes |
| 2d | Wire into `build.sh all` as optional final step | offline-degradation tester passes |
| 2e | Add 3 cgc_octocode_graphrag example queries to README | round-trip tester passes |

Ship Phase 2a–2b first (local delta only, no HTTP) — let the delta file accumulate a week of snapshots to validate schema completeness before touching kg-graph.

## Out of scope (explicit)

- **Edge property updates** — only node properties mutate; edges are "exists or not". Simplifies upsert semantics.
- **Historical time-series** — kg-graph stores LATEST state only. For time-series, keep using `index.json` snapshots + S3 archival (separate concern).
- **Writing to kg-graph from Rust reports** — reports stay readers. Only `kg_ingest.sh` writes.
- **Extending `cloud-cgc-mcp` tool surface** — we use existing `octocode_graphrag` for queries. No new MCP tools required for MVP.
- **Multiple kg-graph instances** — single deployment at `ca-dat_kg-graph`. Multi-env split is out of scope.

## Risks

| Risk | Mitigation |
|---|---|
| Schema drift between `cloud-data-topology.json` and kg-graph | schema file declares the bridge; CI can validate |
| SurrealDB schema evolution over time | nodes keyed by deterministic tuples; additive changes only |
| Confusion "is kg-graph the source of truth?" (no — index.json is) | README makes this explicit; kg is a view |
| Blob size — inspect.json can be 100 KB+ per container × 60 containers | only hydrate 6–8 key properties per node, not whole file |
| Secret accidentally ingested | schema whitelists properties; env vars never referenced |

## Open questions (answer before 2c)

1. kg-graph HTTP API — does it expose `/upsert` or only SurrealQL? May need a thin HTTP adapter in `cloud-cgc-mcp`.
2. Authentication — does kg-graph run auth'd over WG? If yes, bearer or basic?
3. Does octocode_graphrag already support the node types we want to add, or do we need to extend it?

Need 15 min of exploration against the running kg-graph to answer these — trivial, just not done yet.
