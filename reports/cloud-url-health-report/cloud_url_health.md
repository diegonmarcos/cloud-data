```

  ██╗   ██╗██████╗ ██╗          ██╗  ██╗███████╗ █████╗ ██╗  ████████╗██╗  ██╗
  ██║   ██║██╔══██╗██║          ██║  ██║██╔════╝██╔══██╗██║  ╚══██╔══╝██║  ██║
  ██║   ██║██████╔╝██║          ███████║█████╗  ███████║██║     ██║   ███████║
  ██║   ██║██╔══██╗██║          ██╔══██║██╔══╝  ██╔══██║██║     ██║   ██╔══██║
  ╚██████╔╝██║  ██║███████╗     ██║  ██║███████╗██║  ██║███████╗██║   ██║  ██║
   ╚═════╝ ╚═╝  ╚═╝╚══════╝     ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝╚═╝   ╚═╝  ╚═╝
  CLOUD URL HEALTH — 2026-04-05T17:00:40.373376927+00:00
══════════════════════════════════════════════════════════════

  SUMMARY
══════════════════════════════════════════════════════════════
  Public:  9/16 healthy
  Private: 33/44 healthy
  Duration: 6.5s

  ISSUES
══════════════════════════════════════════════════════════════
  FAIL  auth.diegonmarcos.com  HTTP 502
  FAIL  cal.diegonmarcos.com  HTTP 502
  FAIL  chat.diegonmarcos.com  HTTP 502
  FAIL  db.diegonmarcos.com  HTTP 502
  FAIL  ide.diegonmarcos.com  HTTP 502
  FAIL  sheets.diegonmarcos.com  HTTP 502
  FAIL  workflows.diegonmarcos.com  HTTP 502
  FAIL  alerts-api (oci-E2-f_1)  TCP: Connection refused (os error 111)
  FAIL  authelia (gcp-E2-f_0)  TCP: Connection refused (os error 111)
  FAIL  c3-services-api (oci-A1-f_0)  TCP: Connection refused (os error 111)
  FAIL  fluent-bit (oci-E2-f_1)  TCP: Connection refused (os error 111)
  FAIL  hickory-dns (gcp-E2-f_0)  TCP: Connection refused (os error 111)
  FAIL  maddy (oci-E2-f_0)  TCP: Connection refused (os error 111)
  FAIL  matomo (oci-E2-f_1)  TCP: Connection refused (os error 111)
  FAIL  ollama (gcp-T4-p_0)  TCP: timeout 5s
  FAIL  photoprism (oci-A1-f_0)  TCP: Connection refused (os error 111)
  FAIL  redis (gcp-E2-f_0)  HTTP: error sending request for url (http://10.0.0.1:6379/)
  FAIL  umami (oci-E2-f_1)  TCP: Connection refused (os error 111)


1. PUBLIC URLS (Cloudflare -> Caddy -> Backend)
──────────────────────────────────────────────────────────────
  DOMAIN                                    DNS  TCP  TLS  HTTP    ms  DETAIL
  ------------------------------------------------------------------------------------------
  auth.diegonmarcos.com                      OK   OK   OK  FAIL 5961ms  HTTP 502
  cal.diegonmarcos.com                       OK   OK   OK  FAIL 6198ms  HTTP 502
  chat.diegonmarcos.com                      OK   OK   OK  FAIL 6151ms  HTTP 502
  db.diegonmarcos.com                        OK   OK   OK  FAIL 6104ms  HTTP 502
  diegonmarcos.com                           OK   OK   OK    OK 6058ms  200
  git.diegonmarcos.com                       OK   OK   OK    OK 5937ms  200
  grafana.diegonmarcos.com                   OK   OK   OK    OK 5939ms  200
  ide.diegonmarcos.com                       OK   OK   OK  FAIL 6235ms  HTTP 502
  mail.diegonmarcos.com                      OK   OK   OK    OK 6050ms  200
  photos.diegonmarcos.com                    OK   OK   OK    OK 6029ms  200
  sheets.diegonmarcos.com                    OK   OK   OK  FAIL 6006ms  HTTP 502
  smtp.diegonmarcos.com                      OK   OK   OK    OK 6129ms  404
  vault.diegonmarcos.com                     OK   OK   OK    OK 5928ms  200
  webmail.diegonmarcos.com                   OK   OK   OK    OK 6467ms  200
  windmill.diegonmarcos.com                  OK   OK   OK    OK 5921ms  200
  workflows.diegonmarcos.com                 OK   OK   OK  FAIL 6038ms  HTTP 502


2. PRIVATE URLS (WireGuard -> Container)
──────────────────────────────────────────────────────────────
  SERVICE                        UPSTREAM                TCP  HTTP    ms  DETAIL
  ------------------------------------------------------------------------------------------
  alerts-api (oci-E2-f_1)        10.0.0.4:5050          FAIL    -- 275ms  TCP: Connection refused (os erro...
  authelia (gcp-E2-f_0)          10.0.0.1:9091          FAIL    -- 191ms  TCP: Connection refused (os erro...
  backup-gitea (oci-A1-f_0)      10.0.0.6:3002            OK    OK 840ms  200
  c3-infra-api (oci-A1-f_0)      10.0.0.6:8081            OK    OK 843ms  404
  c3-infra-mcp (oci-A1-f_0)      10.0.0.6:3100            OK    OK 856ms  404
  c3-services-api (oci-A1-f_0)   10.0.0.6:8082          FAIL    -- 302ms  TCP: Connection refused (os erro...
  c3-services-mcp (oci-A1-f_0)   10.0.0.6:3101            OK    OK 865ms  404
  caddy (gcp-E2-f_0)             10.0.0.1:443             OK    OK 511ms  400
  cloud-cgc-mcp (oci-A1-f_0)     10.0.0.6:3105            OK    OK 864ms  404
  cloud-spec (oci-A1-f_0)        10.0.0.6:3080            OK    OK 864ms  200
  code-server (oci-A1-f_0)       10.0.0.6:8443            OK    OK 1161ms  200
  crawlee-cloud (oci-A1-f_0)     10.0.0.6:3000            OK    OK 875ms  404
  dagu (oci-E2-f_0)              10.0.0.3:8070            OK    OK 862ms  200
  dbgate (oci-A1-f_0)            10.0.0.6:8086            OK    OK 875ms  200
  dozzle (oci-E2-f_1)            10.0.0.4:9999            OK    OK 2885ms  200
  etherpad (oci-A1-f_0)          10.0.0.6:3012            OK    OK 899ms  200
  filebrowser (oci-A1-f_0)       10.0.0.6:3015            OK    OK 947ms  200
  fluent-bit (oci-E2-f_1)        10.0.0.4:2020          FAIL    -- 312ms  TCP: Connection refused (os erro...
  gitea (oci-A1-f_0)             10.0.0.6:3002            OK    OK 917ms  200
  google-workspace-mcp (oci-A1-f_0) 10.0.0.6:3104            OK    OK 947ms  200
  grist (oci-A1-f_0)             10.0.0.6:3011            OK    OK 948ms  200
  hedgedoc (oci-A1-f_0)          10.0.0.6:3018            OK    OK 976ms  200
  hickory-dns (gcp-E2-f_0)       10.0.0.1:53            FAIL    -- 184ms  TCP: Connection refused (os erro...
  introspect-proxy (gcp-E2-f_0)  10.0.0.1:4182            OK    OK 613ms  404
  lgtm (oci-A1-f_0)              10.0.0.6:3200            OK    OK 1219ms  200
  maddy (oci-E2-f_0)             10.0.0.3:443           FAIL    -- 317ms  TCP: Connection refused (os erro...
  mail-mcp (oci-A1-f_0)          10.0.0.6:3103            OK    OK 972ms  404
  matomo (oci-E2-f_1)            10.0.0.4:8084          FAIL    -- 314ms  TCP: Connection refused (os erro...
  mattermost-bots (oci-A1-f_0)   10.0.0.6:8065            OK    OK 923ms  200
  mattermost-mcp (oci-A1-f_0)    10.0.0.6:3102            OK    OK 938ms  404
  ntfy (gcp-E2-f_0)              10.0.0.1:8090            OK    OK 810ms  200
  ollama (gcp-T4-p_0)            10.0.0.8:11434         FAIL    -- 5002ms  TCP: timeout 5s
  ollama-hai (oci-A1-f_0)        10.0.0.6:11435           OK    OK 907ms  200
  photoprism (oci-A1-f_0)        10.0.0.6:3013          FAIL    -- 309ms  TCP: Connection refused (os erro...
  photos-webhook (oci-A1-f_0)    10.0.0.6:5002            OK    OK 918ms  404
  radicale (oci-A1-f_0)          10.0.0.6:5232            OK    OK 1964ms  200
  redis (gcp-E2-f_0)             10.0.0.1:6379            OK  FAIL 606ms  HTTP: error sending request for ...
  revealmd (oci-A1-f_0)          10.0.0.6:3014            OK    OK 916ms  200
  rig-agentic-sonn-14bq8 (oci-A1-f_0) 10.0.0.6:8091            OK    OK 904ms  404
  smtp-proxy (oci-E2-f_0)        10.0.0.3:8080            OK    OK 917ms  404
  snappymail (oci-E2-f_0)        10.0.0.3:8888            OK    OK 911ms  200
  umami (oci-E2-f_1)             10.0.0.4:3006          FAIL    -- 318ms  TCP: Connection refused (os erro...
  vaultwarden (gcp-E2-f_0)       10.0.0.1:8880            OK    OK 498ms  200
  windmill (oci-A1-f_0)          10.0.0.6:8000            OK    OK 962ms  200


══════════════════════════════════════════════════════════════
RESULT: 42/60 healthy (18 failed) in 6.5s
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-url-health-report
Run: build.sh all
```
