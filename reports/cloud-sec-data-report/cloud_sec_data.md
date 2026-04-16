# Cloud Security: Data Scan Report

> Generated: 2026-04-16 12:00:01 UTC

---

## Issues Summary

  13 issues: 10 critical, 3 warnings, 0 info

  CRITICAL:
    ❌ YARA gcp-proxy:hickory-dns: 1 matches found
    ❌ YARA gcp-proxy:ntfy: 1 matches found
    ❌ YARA oci-apps:rig-agentic: 2 matches found
    ❌ YARA oci-apps:lgtm_grafana: 1 matches found
    ❌ YARA oci-apps:lgtm_loki: 2 matches found
    ❌ YARA oci-apps:lgtm_tempo: 1 matches found
    ❌ YARA oci-mail:maddy: 1 matches found
    ❌ YARA oci-mail:smtp-proxy: 1 matches found
    ❌ YARA oci-mail:snappymail: 1 matches found
    ❌ YARA oci-analytics:dozzle: 1 matches found
  WARNINGS:
    ⚠️  Export gcp-t4: Failed: SSH to gcp-t4 failed: ssh: connect to host 10.0.0.8 port 22: Connection timed out
    ⚠️  URLhaus feed: HTTP 404
    ⚠️  Cross-correlation: 1 correlations: 0 critical, 1 warning

---

## Container Export Status

  ✅ Export gcp-proxy               15 containers exported (1040 files) (71.5s)
  ✅ gcp-proxy:introspect-proxy     120 files, 454675 bytes (7.3s)
  ✅ gcp-proxy:authelia             0 files, 0 bytes (2.0s)
  ✅ gcp-proxy:authelia-redis       Skipped (database container)
  ✅ gcp-proxy:hickory-dns          49 files, 284490 bytes (3.3s)
  ✅ gcp-proxy:syslog-bridge        116 files, 456843 bytes (3.9s)
  ✅ gcp-proxy:github-rss           116 files, 460623 bytes (3.4s)
  ✅ gcp-proxy:ntfy                 44 files, 285408 bytes (2.2s)
  ✅ gcp-proxy:redis                Skipped (database container)
  ✅ gcp-proxy:sqlite-vaultwarden   17 files, 220411 bytes (2.2s)
  ✅ gcp-proxy:sqlite-ntfy          17 files, 220411 bytes (2.1s)
  ✅ gcp-proxy:sqlite-npm           17 files, 220411 bytes (1.8s)
  ✅ gcp-proxy:sqlite-authelia      17 files, 220411 bytes (1.9s)
  ✅ gcp-proxy:postlite-vaultwarden 106 files, 10975965 bytes (8.2s)
  ✅ gcp-proxy:postlite-ntfy        106 files, 10975965 bytes (8.2s)
  ✅ gcp-proxy:postlite-npm         106 files, 10975965 bytes (11.2s)
  ✅ gcp-proxy:postlite-authelia    106 files, 10975965 bytes (10.5s)
  ✅ gcp-proxy:vaultwarden          103 files, 409143 bytes (2.4s)
  ⚠️  Export gcp-t4                  Failed: SSH to gcp-t4 failed: ssh: connect to host 10.0.0.8 port 22: Connection timed out (5.0s) [WARNING]
  ✅ Export oci-apps                6 containers exported (418 files) (117.7s)
  ✅ oci-apps:rig-agentic           113 files, 9626809 bytes (24.8s)
  ✅ oci-apps:nocodb                Skipped (database container)
  ✅ oci-apps:nocodb-db             Skipped (database container)
  ✅ oci-apps:lgtm_grafana          93 files, 490477 bytes (4.3s)
  ✅ oci-apps:lgtm_loki             21 files, 1990529 bytes (6.7s)
  ✅ oci-apps:lgtm_mimir            20 files, 315947 bytes (3.4s)
  ✅ oci-apps:lgtm_tempo            53 files, 294345 bytes (3.5s)
  ✅ oci-apps:dbgate                Skipped (database container)
  ✅ oci-apps:news-gdelt            118 files, 5517545 bytes (71.4s)
  ✅ Export oci-mail                4 containers exported (430 files) (131.8s)
  ✅ oci-mail:maddy                 53 files, 291104 bytes (8.2s)
  ✅ oci-mail:smtp-proxy            106 files, 6508080 bytes (21.3s)
  ✅ oci-mail:dagu                  146 files, 473553 bytes (35.2s)
  ✅ oci-mail:snappymail            125 files, 661989 bytes (63.5s)
  ✅ Export oci-analytics           3 containers exported (254 files) (46.1s)
  ✅ oci-analytics:dagu             146 files, 473505 bytes (35.6s)
  ✅ oci-analytics:sauron-forwarder 103 files, 175700 bytes (3.5s)
  ✅ oci-analytics:dozzle           5 files, 224669 bytes (3.3s)

  Summary: 37/38 passed, 1 failed

---

## YARA Scan Summary

  ✅ YARA rules loaded              7 rule files from /usr/local/share/yara-rules
  ✅ YARA gcp-proxy:introspect-proxy Clean — no matches (0.1s)
  ✅ YARA gcp-proxy:authelia        Clean — no matches (0.0s)
  ❌ YARA gcp-proxy:hickory-dns     1 matches found (0.1s) [CRITICAL]
  ✅ YARA gcp-proxy:syslog-bridge   Clean — no matches (0.1s)
  ✅ YARA gcp-proxy:github-rss      Clean — no matches (0.1s)
  ❌ YARA gcp-proxy:ntfy            1 matches found (0.1s) [CRITICAL]
  ✅ YARA gcp-proxy:sqlite-vaultwarden Clean — no matches (0.0s)
  ✅ YARA gcp-proxy:sqlite-ntfy     Clean — no matches (0.0s)
  ✅ YARA gcp-proxy:sqlite-npm      Clean — no matches (0.0s)
  ✅ YARA gcp-proxy:sqlite-authelia Clean — no matches (0.0s)
  ✅ YARA gcp-proxy:postlite-vaultwarden Clean — no matches (0.3s)
  ✅ YARA gcp-proxy:postlite-ntfy   Clean — no matches (0.3s)
  ✅ YARA gcp-proxy:postlite-npm    Clean — no matches (0.3s)
  ✅ YARA gcp-proxy:postlite-authelia Clean — no matches (0.3s)
  ✅ YARA gcp-proxy:vaultwarden     Clean — no matches (0.1s)
  ❌ YARA oci-apps:rig-agentic      2 matches found (0.3s) [CRITICAL]
  ❌ YARA oci-apps:lgtm_grafana     1 matches found (0.1s) [CRITICAL]
  ❌ YARA oci-apps:lgtm_loki        2 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-apps:lgtm_mimir       Clean — no matches (0.1s)
  ❌ YARA oci-apps:lgtm_tempo       1 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-apps:news-gdelt       Clean — no matches (0.2s)
  ❌ YARA oci-mail:maddy            1 matches found (0.1s) [CRITICAL]
  ❌ YARA oci-mail:smtp-proxy       1 matches found (0.2s) [CRITICAL]
  ✅ YARA oci-mail:dagu             Clean — no matches (0.1s)
  ❌ YARA oci-mail:snappymail       1 matches found (0.1s) [CRITICAL]
  ✅ YARA oci-analytics:dagu        Clean — no matches (0.1s)
  ✅ YARA oci-analytics:sauron-forwarder Clean — no matches (0.0s)
  ❌ YARA oci-analytics:dozzle      1 matches found (0.0s) [CRITICAL]

  Summary: 19/29 passed, 10 failed

### YARA Hits Detail

  - YARA gcp-proxy:hickory-dns: 1 matches found
  - YARA gcp-proxy:ntfy: 1 matches found
  - YARA oci-apps:rig-agentic: 2 matches found
  - YARA oci-apps:lgtm_grafana: 1 matches found
  - YARA oci-apps:lgtm_loki: 2 matches found
  - YARA oci-apps:lgtm_tempo: 1 matches found
  - YARA oci-mail:maddy: 1 matches found
  - YARA oci-mail:smtp-proxy: 1 matches found
  - YARA oci-mail:snappymail: 1 matches found
  - YARA oci-analytics:dozzle: 1 matches found

---

## SIEM Alerts Summary

  ✅ SIEM API auth                  Skipped — no bearer token available

  Summary: 1/1 passed, 0 failed

### SIEM Alert Details

  No critical SIEM alerts.

---

## Threat Intelligence

  ⚠️  URLhaus feed                   HTTP 404 (0.3s) [WARNING]

  Summary: 0/1 passed, 1 failed

---

## Cross-Correlation Analysis

  ⚠️  Cross-correlation              1 correlations: 0 critical, 1 warning [WARNING]

  Summary: 0/1 passed, 1 failed

---

## Performance

  Container export         372.2s
  YARA+SIEM+ThreatIntel    3.4s
  Correlation              0.0s

  Total: 375.9s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

---

## Result

**CRITICAL -- 57/70 passed, 10 critical, 3 warnings**
