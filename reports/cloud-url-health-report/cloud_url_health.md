```

  ██╗   ██╗██████╗ ██╗          ██╗  ██╗███████╗ █████╗ ██╗  ████████╗██╗  ██╗
  ██║   ██║██╔══██╗██║          ██║  ██║██╔════╝██╔══██╗██║  ╚══██╔══╝██║  ██║
  ██║   ██║██████╔╝██║          ███████║█████╗  ███████║██║     ██║   ███████║
  ██║   ██║██╔══██╗██║          ██╔══██║██╔══╝  ██╔══██║██║     ██║   ██╔══██║
  ╚██████╔╝██║  ██║███████╗     ██║  ██║███████╗██║  ██║███████╗██║   ██║  ██║
   ╚═════╝ ╚═╝  ╚═╝╚══════╝     ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝╚═╝   ╚═╝  ╚═╝
  CLOUD URL HEALTH — 2026-04-16T21:16:36.760489964+00:00
══════════════════════════════════════════════════════════════

  SUMMARY
══════════════════════════════════════════════════════════════
  Public:  15/16 healthy
  Private: 34/43 healthy
  Duration: 5.4s

  ISSUES
══════════════════════════════════════════════════════════════
  ❌  db.diegonmarcos.com  HTTP 502
  ❌  cloud-cgc-mcp (oci-A1-f_0)  TCP: Connection refused (os error 111)
  ❌  fluent-bit (oci-E2-f_1)  TCP: Connection refused (os error 111)
  ❌  hickory-dns (gcp-E2-f_0)  HTTP: error sending request for url (http://10.0.0.1:53/)
  ❌  maddy (oci-E2-f_0)  TCP: Connection refused (os error 111)
  ❌  matomo (oci-E2-f_1)  TCP: Connection refused (os error 111)
  ❌  ollama (gcp-T4-p_0)  TCP: timeout 5s
  ❌  photos-webhook (oci-A1-f_0)  TCP: Connection refused (os error 111)
  ❌  redis (gcp-E2-f_0)  HTTP: error sending request for url (http://10.0.0.1:6379/)
  ❌  revealmd (oci-A1-f_0)  TCP: Connection refused (os error 111)


1. PUBLIC URLS (Cloudflare -> Caddy -> Backend)
──────────────────────────────────────────────────────────────
  DOMAIN                                    DNS  TCP  TLS  HTTP    ms  DETAIL
  ------------------------------------------------------------------------------------------
  auth.diegonmarcos.com                       ✅    ✅    ✅     ✅ 683ms  200
  cal.diegonmarcos.com                        ✅    ✅    ✅     ✅ 3122ms  200
  chat.diegonmarcos.com                       ✅    ✅    ✅     ✅ 945ms  200
  db.diegonmarcos.com                         ✅    ✅    ✅     ❌ 1202ms  HTTP 502
  diegonmarcos.com                            ✅    ✅    ✅     ✅ 807ms  200
  git.diegonmarcos.com                        ✅    ✅    ✅     ✅ 809ms  200
  grafana.diegonmarcos.com                    ✅    ✅    ✅     ✅ 961ms  200
  ide.diegonmarcos.com                        ✅    ✅    ✅     ✅ 2879ms  200
  mail.diegonmarcos.com                       ✅    ✅    ✅     ✅ 690ms  200
  photos.diegonmarcos.com                     ✅    ✅    ✅     ✅ 937ms  200
  sheets.diegonmarcos.com                     ✅    ✅    ✅     ✅ 1058ms  401
  smtp.diegonmarcos.com                       ✅    ✅    ✅     ✅ 927ms  405
  vault.diegonmarcos.com                      ✅    ✅    ✅     ✅ 1183ms  200
  webmail.diegonmarcos.com                    ✅    ✅    ✅     ✅ 1429ms  200
  windmill.diegonmarcos.com                   ✅    ✅    ✅     ✅ 857ms  200
  workflows.diegonmarcos.com                  ✅    ✅    ✅     ✅ 931ms  200


2. PRIVATE URLS (WireGuard -> Container)
──────────────────────────────────────────────────────────────
  SERVICE                        UPSTREAM                TCP  HTTP    ms  DETAIL
  ------------------------------------------------------------------------------------------
  authelia (gcp-E2-f_0)          10.0.0.1:9091             ✅     ✅ 394ms  200
  backup-gitea (oci-A1-f_0)      10.0.0.6:3002             ✅     ✅ 732ms  200
  c3-infra-api (oci-A1-f_0)      10.0.0.6:8081             ✅     ✅ 748ms  404
  c3-infra-mcp (oci-A1-f_0)      10.0.0.6:3100             ✅     ✅ 732ms  404
  c3-services-api (oci-A1-f_0)   10.0.0.6:8082             ✅     ✅ 776ms  200
  c3-services-mcp (oci-A1-f_0)   10.0.0.6:3101             ✅     ✅ 732ms  404
  caddy (gcp-E2-f_0)             10.0.0.1:443              ✅     ✅ 391ms  400
  cloud-cgc-mcp (oci-A1-f_0)     10.0.0.6:3105             ❌    ⬚  248ms  TCP: Connection refused (os erro...
  cloud-spec (oci-A1-f_0)        10.0.0.6:3080             ✅     ✅ 1017ms  200
  code-server (oci-A1-f_0)       10.0.0.6:8443             ✅     ✅ 1872ms  200
  crawlee-cloud (oci-A1-f_0)     10.0.0.6:3000             ✅     ✅ 753ms  404
  dagu (oci-E2-f_1)              10.0.0.4:8070             ✅     ✅ 732ms  200
  dbgate (oci-A1-f_0)            10.0.0.6:8086             ✅     ✅ 769ms  200
  dozzle (oci-E2-f_1)            10.0.0.4:9999             ✅     ✅ 737ms  200
  etherpad (oci-A1-f_0)          10.0.0.6:3012             ✅     ✅ 737ms  200
  filebrowser (oci-A1-f_0)       10.0.0.6:3015             ✅     ✅ 1657ms  200
  fluent-bit (oci-E2-f_1)        10.0.0.4:2020             ❌    ⬚  245ms  TCP: Connection refused (os erro...
  gitea (oci-A1-f_0)             10.0.0.6:3002             ✅     ✅ 742ms  200
  google-workspace-mcp (oci-A1-f_0) 10.0.0.6:3104             ✅     ✅ 756ms  200
  grist (oci-A1-f_0)             10.0.0.6:3011             ✅     ✅ 767ms  200
  hedgedoc (oci-A1-f_0)          10.0.0.6:3018             ✅     ✅ 1496ms  200
  hickory-dns (gcp-E2-f_0)       10.0.0.1:53               ✅     ❌ 5429ms  HTTP: error sending request for ...
  introspect-proxy (gcp-E2-f_0)  10.0.0.1:4182             ✅     ✅ 397ms  404
  lgtm (oci-A1-f_0)              10.0.0.6:3200             ✅     ✅ 1791ms  200
  maddy (oci-E2-f_0)             10.0.0.3:443              ❌    ⬚  248ms  TCP: Connection refused (os erro...
  mail-mcp (oci-A1-f_0)          10.0.0.6:3103             ✅     ✅ 743ms  404
  matomo (oci-E2-f_1)            10.0.0.4:8084             ❌    ⬚  245ms  TCP: Connection refused (os erro...
  mattermost-bots (oci-A1-f_0)   10.0.0.6:8065             ✅     ✅ 743ms  200
  mattermost-mcp (oci-A1-f_0)    10.0.0.6:3102             ✅     ✅ 747ms  404
  ntfy (oci-A1-f_0)              10.0.0.6:8090             ✅     ✅ 853ms  200
  ollama (gcp-T4-p_0)            10.0.0.8:11434            ❌    ⬚  5001ms  TCP: timeout 5s
  ollama-hai (oci-A1-f_0)        10.0.0.6:11435            ✅     ✅ 741ms  200
  photoprism (oci-A1-f_0)        10.0.0.6:3013             ✅     ✅ 1323ms  200
  photos-webhook (oci-A1-f_0)    10.0.0.6:5002             ❌    ⬚  248ms  TCP: Connection refused (os erro...
  radicale (oci-A1-f_0)          10.0.0.6:5232             ✅     ✅ 2743ms  200
  redis (gcp-E2-f_0)             10.0.0.1:6379             ✅     ❌ 394ms  HTTP: error sending request for ...
  revealmd (oci-A1-f_0)          10.0.0.6:3014             ❌    ⬚  249ms  TCP: Connection refused (os erro...
  rig-agentic-sonn-14bq8 (oci-A1-f_0) 10.0.0.6:8091             ✅     ✅ 748ms  404
  smtp-proxy (oci-E2-f_0)        10.0.0.3:8080             ✅     ✅ 737ms  405
  snappymail (oci-E2-f_0)        10.0.0.3:8888             ✅     ✅ 741ms  200
  umami (oci-E2-f_1)             10.0.0.4:3006             ✅     ✅ 840ms  200
  vaultwarden (oci-A1-f_0)       10.0.0.6:8880             ✅     ✅ 735ms  200
  windmill (oci-A1-f_0)          10.0.0.6:8000             ✅     ✅ 954ms  200


══════════════════════════════════════════════════════════════
RESULT: 49/59 healthy (10 failed) in 5.4s
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-url-health-report
Run: build.sh all
```
