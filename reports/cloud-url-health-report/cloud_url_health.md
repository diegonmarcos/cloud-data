```

  ██╗   ██╗██████╗ ██╗          ██╗  ██╗███████╗ █████╗ ██╗  ████████╗██╗  ██╗
  ██║   ██║██╔══██╗██║          ██║  ██║██╔════╝██╔══██╗██║  ╚══██╔══╝██║  ██║
  ██║   ██║██████╔╝██║          ███████║█████╗  ███████║██║     ██║   ███████║
  ██║   ██║██╔══██╗██║          ██╔══██║██╔══╝  ██╔══██║██║     ██║   ██╔══██║
  ╚██████╔╝██║  ██║███████╗     ██║  ██║███████╗██║  ██║███████╗██║   ██║  ██║
   ╚═════╝ ╚═╝  ╚═╝╚══════╝     ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝╚═╝   ╚═╝  ╚═╝
  CLOUD URL HEALTH — 2026-04-20T02:52:59.538275273+00:00
══════════════════════════════════════════════════════════════

  SUMMARY
══════════════════════════════════════════════════════════════
  Public:  15/15 healthy
  Private: 0/45 healthy
  Duration: 5.0s

  ISSUES
══════════════════════════════════════════════════════════════
  ❌  authelia (gcp-E2-f_0)  TCP: timeout 5s
  ❌  backup-borg (oci-A1-f_0)  TCP: timeout 5s
  ❌  backup-bup (oci-A1-f_0)  TCP: timeout 5s
  ❌  c3-infra-api (oci-A1-f_0)  TCP: timeout 5s
  ❌  c3-infra-mcp (oci-A1-f_0)  TCP: timeout 5s
  ❌  c3-services-api (oci-A1-f_0)  TCP: timeout 5s
  ❌  c3-services-mcp (oci-A1-f_0)  TCP: timeout 5s
  ❌  caddy (gcp-E2-f_0)  TCP: timeout 5s
  ❌  cloud-cgc-mcp (oci-A1-f_0)  TCP: timeout 5s
  ❌  cloud-spec (oci-A1-f_0)  TCP: timeout 5s
  ❌  code-server (oci-A1-f_0)  TCP: timeout 5s
  ❌  crawlee-cloud (oci-A1-f_0)  TCP: timeout 5s
  ❌  dagu (oci-E2-f_1)  TCP: timeout 5s
  ❌  dbgate (oci-A1-f_0)  TCP: timeout 5s
  ❌  dozzle (oci-E2-f_1)  TCP: timeout 5s
  ❌  etherpad (oci-A1-f_0)  TCP: timeout 5s
  ❌  filebrowser (oci-A1-f_0)  TCP: timeout 5s
  ❌  fluent-bit (oci-E2-f_1)  TCP: timeout 5s
  ❌  gitea (oci-A1-f_0)  TCP: timeout 5s
  ❌  google-workspace-mcp (oci-A1-f_0)  TCP: timeout 5s
  ❌  grist (oci-A1-f_0)  TCP: timeout 5s
  ❌  hedgedoc (oci-A1-f_0)  TCP: timeout 5s
  ❌  hickory-dns (gcp-E2-f_0)  TCP: timeout 5s
  ❌  introspect-proxy (gcp-E2-f_0)  TCP: timeout 5s
  ❌  kg-graph (oci-A1-f_0)  TCP: timeout 5s
  ❌  lgtm (oci-A1-f_0)  TCP: timeout 5s
  ❌  mail-mcp (oci-A1-f_0)  TCP: timeout 5s
  ❌  matomo (oci-A1-f_0)  TCP: timeout 5s
  ❌  mattermost-bots (oci-A1-f_0)  TCP: timeout 5s
  ❌  mattermost-mcp (oci-A1-f_0)  TCP: timeout 5s
  ❌  ntfy (oci-A1-f_0)  TCP: timeout 5s
  ❌  ollama (gcp-T4-p_0)  TCP: timeout 5s
  ❌  ollama-hai (oci-A1-f_0)  TCP: timeout 5s
  ❌  photoprism (oci-A1-f_0)  TCP: timeout 5s
  ❌  photos-webhook (oci-A1-f_0)  TCP: timeout 5s
  ❌  radicale (oci-A1-f_0)  TCP: timeout 5s
  ❌  redis (gcp-E2-f_0)  TCP: timeout 5s
  ❌  revealmd (oci-A1-f_0)  TCP: timeout 5s
  ❌  rig-agentic-hai-1.5bq4 (oci-A1-f_0)  TCP: timeout 5s
  ❌  rig-agentic-sonn-14bq8 (oci-A1-f_0)  TCP: timeout 5s
  ❌  smtp-proxy (oci-E2-f_0)  TCP: timeout 5s
  ❌  snappymail (oci-E2-f_0)  TCP: timeout 5s
  ❌  stalwart (oci-E2-f_0)  TCP: timeout 5s
  ❌  umami (oci-A1-f_0)  TCP: timeout 5s
  ❌  vaultwarden (oci-A1-f_0)  TCP: timeout 5s


1. PUBLIC URLS (Cloudflare -> Caddy -> Backend)
──────────────────────────────────────────────────────────────
  DOMAIN                                    DNS  TCP  TLS  HTTP    ms  DETAIL
  ------------------------------------------------------------------------------------------
  auth.diegonmarcos.com                       ✅    ✅    ✅     ✅ 165ms  200
  cal.diegonmarcos.com                        ✅    ✅    ✅     ✅ 273ms  200
  chat.diegonmarcos.com                       ✅    ✅    ✅     ✅ 294ms  200
  db.diegonmarcos.com                         ✅    ✅    ✅     ✅ 316ms  200
  diegonmarcos.com                            ✅    ✅    ✅     ✅ 276ms  200
  git.diegonmarcos.com                        ✅    ✅    ✅     ✅ 284ms  200
  grafana.diegonmarcos.com                    ✅    ✅    ✅     ✅ 281ms  200
  ide.diegonmarcos.com                        ✅    ✅    ✅     ✅ 287ms  200
  mail-stalwart.diegonmarcos.com              ✅    ✅    ✅     ✅ 314ms  200
  photos.diegonmarcos.com                     ✅    ✅    ✅     ✅ 237ms  200
  sheets.diegonmarcos.com                     ✅    ✅    ✅     ✅ 274ms  200
  smtp.diegonmarcos.com                       ✅    ✅    ✅     ✅ 311ms  405
  vault.diegonmarcos.com                      ✅    ✅    ✅     ✅ 374ms  200
  webmail.diegonmarcos.com                    ✅    ✅    ✅     ✅ 289ms  200
  workflows.diegonmarcos.com                  ✅    ✅    ✅     ✅ 274ms  200


2. PRIVATE URLS (WireGuard -> Container)
──────────────────────────────────────────────────────────────
  SERVICE                        UPSTREAM                TCP  HTTP    ms  DETAIL
  ------------------------------------------------------------------------------------------
  authelia (gcp-E2-f_0)          10.0.0.1:9091             ❌    ⬚  5001ms  TCP: timeout 5s
  backup-borg (oci-A1-f_0)       10.0.0.6:2224             ❌    ⬚  5001ms  TCP: timeout 5s
  backup-bup (oci-A1-f_0)        10.0.0.6:2223             ❌    ⬚  5001ms  TCP: timeout 5s
  c3-infra-api (oci-A1-f_0)      10.0.0.6:8081             ❌    ⬚  5001ms  TCP: timeout 5s
  c3-infra-mcp (oci-A1-f_0)      10.0.0.6:3100             ❌    ⬚  5001ms  TCP: timeout 5s
  c3-services-api (oci-A1-f_0)   10.0.0.6:8082             ❌    ⬚  5001ms  TCP: timeout 5s
  c3-services-mcp (oci-A1-f_0)   10.0.0.6:3101             ❌    ⬚  5001ms  TCP: timeout 5s
  caddy (gcp-E2-f_0)             10.0.0.1:443              ❌    ⬚  5001ms  TCP: timeout 5s
  cloud-cgc-mcp (oci-A1-f_0)     10.0.0.6:3105             ❌    ⬚  5001ms  TCP: timeout 5s
  cloud-spec (oci-A1-f_0)        10.0.0.6:3080             ❌    ⬚  5001ms  TCP: timeout 5s
  code-server (oci-A1-f_0)       10.0.0.6:8443             ❌    ⬚  5001ms  TCP: timeout 5s
  crawlee-cloud (oci-A1-f_0)     10.0.0.6:3000             ❌    ⬚  5001ms  TCP: timeout 5s
  dagu (oci-E2-f_1)              10.0.0.4:8070             ❌    ⬚  5001ms  TCP: timeout 5s
  dbgate (oci-A1-f_0)            10.0.0.6:8086             ❌    ⬚  5001ms  TCP: timeout 5s
  dozzle (oci-E2-f_1)            10.0.0.4:9999             ❌    ⬚  5001ms  TCP: timeout 5s
  etherpad (oci-A1-f_0)          10.0.0.6:3012             ❌    ⬚  5001ms  TCP: timeout 5s
  filebrowser (oci-A1-f_0)       10.0.0.6:3015             ❌    ⬚  5001ms  TCP: timeout 5s
  fluent-bit (oci-E2-f_1)        10.0.0.4:2020             ❌    ⬚  5001ms  TCP: timeout 5s
  gitea (oci-A1-f_0)             10.0.0.6:3002             ❌    ⬚  5001ms  TCP: timeout 5s
  google-workspace-mcp (oci-A1-f_0) 10.0.0.6:3104             ❌    ⬚  5001ms  TCP: timeout 5s
  grist (oci-A1-f_0)             10.0.0.6:3011             ❌    ⬚  5001ms  TCP: timeout 5s
  hedgedoc (oci-A1-f_0)          10.0.0.6:3018             ❌    ⬚  5001ms  TCP: timeout 5s
  hickory-dns (gcp-E2-f_0)       10.0.0.1:53               ❌    ⬚  5001ms  TCP: timeout 5s
  introspect-proxy (gcp-E2-f_0)  10.0.0.1:4182             ❌    ⬚  5001ms  TCP: timeout 5s
  kg-graph (oci-A1-f_0)          10.0.0.6:8001             ❌    ⬚  5001ms  TCP: timeout 5s
  lgtm (oci-A1-f_0)              10.0.0.6:3200             ❌    ⬚  5001ms  TCP: timeout 5s
  mail-mcp (oci-A1-f_0)          10.0.0.6:3103             ❌    ⬚  5001ms  TCP: timeout 5s
  matomo (oci-A1-f_0)            10.0.0.6:8084             ❌    ⬚  5001ms  TCP: timeout 5s
  mattermost-bots (oci-A1-f_0)   10.0.0.6:8065             ❌    ⬚  5001ms  TCP: timeout 5s
  mattermost-mcp (oci-A1-f_0)    10.0.0.6:3102             ❌    ⬚  5001ms  TCP: timeout 5s
  ntfy (oci-A1-f_0)              10.0.0.6:8090             ❌    ⬚  5001ms  TCP: timeout 5s
  ollama (gcp-T4-p_0)            10.0.0.8:11434            ❌    ⬚  5001ms  TCP: timeout 5s
  ollama-hai (oci-A1-f_0)        10.0.0.6:11435            ❌    ⬚  5001ms  TCP: timeout 5s
  photoprism (oci-A1-f_0)        10.0.0.6:3013             ❌    ⬚  5001ms  TCP: timeout 5s
  photos-webhook (oci-A1-f_0)    10.0.0.6:5002             ❌    ⬚  5001ms  TCP: timeout 5s
  radicale (oci-A1-f_0)          10.0.0.6:5232             ❌    ⬚  5001ms  TCP: timeout 5s
  redis (gcp-E2-f_0)             10.0.0.1:6379             ❌    ⬚  5001ms  TCP: timeout 5s
  revealmd (oci-A1-f_0)          10.0.0.6:3014             ❌    ⬚  5001ms  TCP: timeout 5s
  rig-agentic-hai-1.5bq4 (oci-A1-f_0) 10.0.0.6:8091             ❌    ⬚  5001ms  TCP: timeout 5s
  rig-agentic-sonn-14bq8 (oci-A1-f_0) 10.0.0.6:8091             ❌    ⬚  5001ms  TCP: timeout 5s
  smtp-proxy (oci-E2-f_0)        10.0.0.3:8080             ❌    ⬚  5001ms  TCP: timeout 5s
  snappymail (oci-E2-f_0)        10.0.0.3:8888             ❌    ⬚  5001ms  TCP: timeout 5s
  stalwart (oci-E2-f_0)          10.0.0.3:2443             ❌    ⬚  5001ms  TCP: timeout 5s
  umami (oci-A1-f_0)             10.0.0.6:3006             ❌    ⬚  5001ms  TCP: timeout 5s
  vaultwarden (oci-A1-f_0)       10.0.0.6:8880             ❌    ⬚  5001ms  TCP: timeout 5s


══════════════════════════════════════════════════════════════
RESULT: 15/60 healthy (45 failed) in 5.0s
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-url-health-report
Run: build.sh all
```
