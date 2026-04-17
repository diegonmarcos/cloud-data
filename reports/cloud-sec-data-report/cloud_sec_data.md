# Cloud Security: Data Scan Report

> Generated: 2026-04-17 07:56:27 UTC

---

## Issues Summary

  22 issues: 18 critical, 4 warnings, 0 info

  CRITICAL:
    ❌ YARA oci-apps:crawlee_dashboard: 1 matches found
    ❌ YARA oci-apps:crawlee_api: 1 matches found
    ❌ YARA oci-apps:crawlee_scheduler: 1 matches found
    ❌ YARA oci-apps:rig-agentic-sonn-14bq8: 2 matches found
    ❌ YARA oci-apps:lgtm_grafana: 1 matches found
    ❌ YARA oci-apps:lgtm_tempo: 1 matches found
    ❌ YARA oci-apps:lgtm_loki: 2 matches found
    ❌ YARA oci-apps:etherpad_app: 1 matches found
    ❌ YARA oci-apps:gitea: 1 matches found
    ❌ YARA oci-apps:radicale: 1 matches found
    ❌ YARA oci-apps:ntfy: 1 matches found
    ❌ YARA oci-apps:photoprism_rclone: 1 matches found
    ❌ YARA oci-apps:c3-infra-api: 3 matches found
    ❌ YARA oci-mail:smtp-proxy: 1 matches found
    ❌ YARA oci-mail:snappymail: 1 matches found
    ❌ YARA oci-mail:maddy: 1 matches found
    ❌ YARA oci-analytics:umami: 1 matches found
    ❌ YARA oci-analytics:dozzle: 1 matches found
  WARNINGS:
    ⚠️  Export gcp-proxy: Failed: SSH to gcp-proxy failed: fish: Unknown command: docker
fish: 
docker ps --format '{{.Names}}'
^~~~~^
    ⚠️  Export gcp-t4: Failed: SSH to gcp-t4 failed: ssh: connect to host 10.0.0.8 port 22: Connection timed out
    ⚠️  YARA oci-apps:photoprism_app: 2 matches found
    ⚠️  Cross-correlation: 1 correlations: 0 critical, 1 warning

---

## Container Export Status

  ✅ Evidence gcp-proxy             No evidence vault snapshot — using docker cp fallback (2.4s)
  ⚠️  Export gcp-proxy               Failed: SSH to gcp-proxy failed: fish: Unknown command: docker
fish: 
docker ps --format '{{.Names}}'
^~~~~^ (0.4s) [WARNING]
  ✅ Evidence gcp-t4                No evidence vault snapshot — using docker cp fallback (5.0s)
  ⚠️  Export gcp-t4                  Failed: SSH to gcp-t4 failed: ssh: connect to host 10.0.0.8 port 22: Connection timed out (5.0s) [WARNING]
  ✅ Evidence oci-apps              No evidence vault snapshot — using docker cp fallback (3.3s)
  ✅ Export oci-apps                40 containers exported (8448 files) (1600.0s)
  ✅ oci-apps:cloud-cgc-mcp         144 files, 459998 bytes (64.5s)
  ✅ oci-apps:c3-infra-mcp          132 files, 5840822 bytes (69.3s)
  ✅ oci-apps:crawlee_runner        125 files, 5538478 bytes (76.9s)
  ✅ oci-apps:crawlee_dashboard     55 files, 5616660 bytes (79.0s)
  ✅ oci-apps:crawlee_api           55 files, 5616839 bytes (76.8s)
  ✅ oci-apps:crawlee_scheduler     55 files, 5616839 bytes (76.6s)
  ✅ oci-apps:crawlee_redis         Skipped (database container)
  ✅ oci-apps:crawlee_minio         52 files, 958270 bytes (5.0s)
  ✅ oci-apps:crawlee_db            Skipped (database container)
  ✅ oci-apps:rig-agentic-sonn-14bq8 113 files, 9626881 bytes (22.6s)
  ✅ oci-apps:windmill-worker       173 files, 568582 bytes (71.3s)
  ✅ oci-apps:windmill-server       173 files, 568582 bytes (70.9s)
  ✅ oci-apps:windmill-db           Skipped (database container)
  ✅ oci-apps:ollama-hai            123 files, 350424 bytes (4.4s)
  ✅ oci-apps:dbgate                Skipped (database container)
  ✅ oci-apps:grist_app             115 files, 5512520 bytes (45.3s)
  ✅ oci-apps:cloud-spec            9 files, 3921 bytes (2.9s)
  ✅ oci-apps:lgtm_grafana          93 files, 493296 bytes (4.4s)
  ✅ oci-apps:lgtm_tempo            53 files, 294345 bytes (7.3s)
  ✅ oci-apps:lgtm_mimir            20 files, 315947 bytes (3.2s)
  ✅ oci-apps:lgtm_loki             21 files, 1990529 bytes (6.5s)
  ✅ oci-apps:code-server           241 files, 449839 bytes (37.6s)
  ✅ oci-apps:etherpad_app          103 files, 413440 bytes (70.3s)
  ✅ oci-apps:etherpad_postgres     Skipped (database container)
  ✅ oci-apps:filebrowser_app       19 files, 333583 bytes (3.5s)
  ✅ oci-apps:quant_light_engine    116 files, 502166 bytes (4.1s)
  ✅ oci-apps:quant_light_research  200 files, 655258 bytes (37.9s)
  ✅ oci-apps:quant_light_db        Skipped (database container)
  ✅ oci-apps:gitea                 127 files, 1018452 bytes (65.1s)
  ✅ oci-apps:hedgedoc_app          120 files, 5500678 bytes (50.4s)
  ✅ oci-apps:radicale              105 files, 999217 bytes (5.5s)
  ✅ oci-apps:hedgedoc_postgres     Skipped (database container)
  ✅ oci-apps:vaultwarden           103 files, 409048 bytes (4.4s)
  ✅ oci-apps:syslog-bridge         116 files, 509907 bytes (4.1s)
  ✅ oci-apps:github-rss            116 files, 513683 bytes (4.1s)
  ✅ oci-apps:ntfy                  43 files, 284678 bytes (3.5s)
  ✅ oci-apps:mattermost-bots       118 files, 572456 bytes (4.0s)
  ✅ oci-apps:mattermost            20 files, 321432 bytes (3.9s)
  ✅ oci-apps:mattermost-postgres   Skipped (database container)
  ✅ oci-apps:photoprism_app        237 files, 1321962 bytes (71.3s)
  ✅ oci-apps:photoprism_rclone     49 files, 294046 bytes (38.3s)
  ✅ oci-apps:photoprism_mariadb    Skipped (database container)
  ✅ oci-apps:google-workspace-mcp  115 files, 452341 bytes (68.6s)
  ✅ oci-apps:mattermost-mcp        118 files, 5515246 bytes (76.3s)
  ✅ oci-apps:mail-mcp              119 files, 5519266 bytes (76.9s)
  ✅ oci-apps:c3-services-mcp       121 files, 5751827 bytes (81.9s)
  ✅ oci-apps:c3-infra-api          4392 files, 64617324 bytes (48.6s)
  ✅ oci-apps:news-gdelt            118 files, 5517545 bytes (75.3s)
  ✅ oci-apps:c3-services-api       121 files, 5751811 bytes (76.9s)
  ✅ Evidence oci-mail              No evidence vault snapshot — using docker cp fallback (5.2s)
  ✅ Export oci-mail                4 containers exported (430 files) (122.6s)
  ✅ oci-mail:smtp-proxy            106 files, 6512616 bytes (18.1s)
  ✅ oci-mail:snappymail            125 files, 661930 bytes (63.6s)
  ✅ oci-mail:maddy                 53 files, 291104 bytes (5.0s)
  ✅ oci-mail:dagu                  146 files, 473553 bytes (35.3s)
  ✅ Evidence oci-analytics         No evidence vault snapshot — using docker cp fallback (3.9s)
  ✅ Export oci-analytics           5 containers exported (558 files) (122.2s)
  ✅ oci-analytics:matomo-hybrid    247 files, 779826 bytes (5.0s)
  ✅ oci-analytics:umami            57 files, 5626050 bytes (73.1s)
  ✅ oci-analytics:umami-db         Skipped (database container)
  ✅ oci-analytics:dagu             146 files, 473505 bytes (36.2s)
  ✅ oci-analytics:sauron-forwarder 103 files, 175700 bytes (3.5s)
  ✅ oci-analytics:dozzle           5 files, 224669 bytes (3.2s)

  Summary: 67/69 passed, 2 failed

---

## YARA Scan Summary

  ✅ YARA rules loaded              6 rule files from yara-rules
  ✅ YARA oci-apps:cloud-cgc-mcp    Clean — no matches (0.2s)
  ✅ YARA oci-apps:c3-infra-mcp     Clean — no matches (0.7s)
  ✅ YARA oci-apps:crawlee_runner   Clean — no matches (0.4s)
  ❌ YARA oci-apps:crawlee_dashboard 1 matches found (0.4s) [CRITICAL]
  ❌ YARA oci-apps:crawlee_api      1 matches found (0.5s) [CRITICAL]
  ❌ YARA oci-apps:crawlee_scheduler 1 matches found (0.5s) [CRITICAL]
  ✅ YARA oci-apps:crawlee_minio    Clean — no matches (0.2s)
  ❌ YARA oci-apps:rig-agentic-sonn-14bq8 2 matches found (0.6s) [CRITICAL]
  ✅ YARA oci-apps:windmill-worker  Clean — no matches (0.2s)
  ✅ YARA oci-apps:windmill-server  Clean — no matches (0.2s)
  ✅ YARA oci-apps:ollama-hai       Clean — no matches (0.2s)
  ✅ YARA oci-apps:grist_app        Clean — no matches (0.4s)
  ✅ YARA oci-apps:cloud-spec       Clean — no matches (0.0s)
  ❌ YARA oci-apps:lgtm_grafana     1 matches found (0.1s) [CRITICAL]
  ❌ YARA oci-apps:lgtm_tempo       1 matches found (0.2s) [CRITICAL]
  ✅ YARA oci-apps:lgtm_mimir       Clean — no matches (0.1s)
  ❌ YARA oci-apps:lgtm_loki        2 matches found (0.2s) [CRITICAL]
  ✅ YARA oci-apps:code-server      Clean — no matches (0.2s)
  ❌ YARA oci-apps:etherpad_app     1 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-apps:filebrowser_app  Clean — no matches (0.1s)
  ✅ YARA oci-apps:quant_light_engine Clean — no matches (0.1s)
  ✅ YARA oci-apps:quant_light_research Clean — no matches (0.1s)
  ❌ YARA oci-apps:gitea            1 matches found (0.2s) [CRITICAL]
  ✅ YARA oci-apps:hedgedoc_app     Clean — no matches (0.4s)
  ❌ YARA oci-apps:radicale         1 matches found (0.2s) [CRITICAL]
  ✅ YARA oci-apps:vaultwarden      Clean — no matches (0.2s)
  ✅ YARA oci-apps:syslog-bridge    Clean — no matches (0.2s)
  ✅ YARA oci-apps:github-rss       Clean — no matches (0.2s)
  ❌ YARA oci-apps:ntfy             1 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-apps:mattermost-bots  Clean — no matches (0.1s)
  ✅ YARA oci-apps:mattermost       Clean — no matches (0.1s)
  ⚠️  YARA oci-apps:photoprism_app   2 matches found (0.1s) [WARNING]
  ❌ YARA oci-apps:photoprism_rclone 1 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-apps:google-workspace-mcp Clean — no matches (0.1s)
  ✅ YARA oci-apps:mattermost-mcp   Clean — no matches (0.4s)
  ✅ YARA oci-apps:mail-mcp         Clean — no matches (0.4s)
  ✅ YARA oci-apps:c3-services-mcp  Clean — no matches (0.4s)
  ❌ YARA oci-apps:c3-infra-api     3 matches found (1.6s) [CRITICAL]
  ✅ YARA oci-apps:news-gdelt       Clean — no matches (0.4s)
  ✅ YARA oci-apps:c3-services-api  Clean — no matches (0.4s)
  ❌ YARA oci-mail:smtp-proxy       1 matches found (0.4s) [CRITICAL]
  ❌ YARA oci-mail:snappymail       1 matches found (0.1s) [CRITICAL]
  ❌ YARA oci-mail:maddy            1 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-mail:dagu             Clean — no matches (0.1s)
  ✅ YARA oci-analytics:matomo-hybrid Clean — no matches (0.2s)
  ❌ YARA oci-analytics:umami       1 matches found (0.4s) [CRITICAL]
  ✅ YARA oci-analytics:dagu        Clean — no matches (0.1s)
  ✅ YARA oci-analytics:sauron-forwarder Clean — no matches (0.1s)
  ❌ YARA oci-analytics:dozzle      1 matches found (0.1s) [CRITICAL]

  Summary: 31/50 passed, 19 failed

### YARA Hits Detail

  - YARA oci-apps:crawlee_dashboard: 1 matches found
  - YARA oci-apps:crawlee_api: 1 matches found
  - YARA oci-apps:crawlee_scheduler: 1 matches found
  - YARA oci-apps:rig-agentic-sonn-14bq8: 2 matches found
  - YARA oci-apps:lgtm_grafana: 1 matches found
  - YARA oci-apps:lgtm_tempo: 1 matches found
  - YARA oci-apps:lgtm_loki: 2 matches found
  - YARA oci-apps:etherpad_app: 1 matches found
  - YARA oci-apps:gitea: 1 matches found
  - YARA oci-apps:radicale: 1 matches found
  - YARA oci-apps:ntfy: 1 matches found
  - YARA oci-apps:photoprism_app: 2 matches found
  - YARA oci-apps:photoprism_rclone: 1 matches found
  - YARA oci-apps:c3-infra-api: 3 matches found
  - YARA oci-mail:smtp-proxy: 1 matches found
  - YARA oci-mail:snappymail: 1 matches found
  - YARA oci-mail:maddy: 1 matches found
  - YARA oci-analytics:umami: 1 matches found
  - YARA oci-analytics:dozzle: 1 matches found

---

## SIEM Alerts Summary

  ✅ SIEM API auth                  Skipped — no bearer token available

  Summary: 1/1 passed, 0 failed

### SIEM Alert Details

  No critical SIEM alerts.

---

## Threat Intelligence

  ✅ URLhaus feed                   200 indicators fetched (0.3s)

  Summary: 1/1 passed, 0 failed

---

## Journal Analysis (24h)

  ✅ Journal analysis               No evidence directories available

  Summary: 1/1 passed, 0 failed

  No suspicious journal entries detected.

---

## Runtime Security Analysis

  ✅ Runtime analysis               No evidence directories available

  Summary: 1/1 passed, 0 failed

  No runtime security issues detected.

---

## Container Diff Analysis

  ✅ Diff analysis                  No evidence directories available

  Summary: 1/1 passed, 0 failed

  No significant container changes detected.

---

## Cross-Correlation Analysis

  ⚠️  Cross-correlation              1 correlations: 0 critical, 1 warning [WARNING]

  Summary: 0/1 passed, 1 failed

---

## Performance

  Container export + evidence 1870.1s
  YARA+SIEM+ThreatIntel+Journal+Runtime+Diff 13.1s
  Correlation              0.0s

  Total: 1883.5s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

---

## Result

**CRITICAL -- 103/125 passed, 18 critical, 4 warnings**
