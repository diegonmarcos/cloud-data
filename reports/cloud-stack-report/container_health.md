```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-04-20T10:54:20.522112348+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
5 critical, 9 warnings — 14 total

    ❌ A3       oci-apps/umami-setup — exited
    ❌ A3       oci-apps/cloud-builder-x-cloud-builder-1 — exited
    ❌ A3       oci-apps/c3-services-mcp — exited
    ⚠️ A3       oci-apps/c3-infra-mcp — unhealthy
    ❌ A3       oci-mail/dagu — exited
    ❌ A1       dns.internal — [err: reqwest::Error { kind: Request, url: "https://dns.internal/", source: hyper_util::client::legacy::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: Name or service not known" })) }]
    ⚠️ B2       gcp-proxy/authelia (sqlite) — unhealthy
    ⚠️ B2       oci-apps/crawlee_db (postgres) — unhealthy
    ⚠️ B2       oci-apps/gitea (sqlite) — unhealthy
    ⚠️ B2       oci-apps/grist_app (sqlite) — unhealthy
    ⚠️ B2       oci-apps/ntfy (sqlite) — unhealthy
    ⚠️ B2       oci-apps/photoprism_mariadb (mariadb) — unhealthy
    ⚠️ B2       oci-apps/quant_light_db (postgres) — unhealthy
    ⚠️ B2       oci-apps/vaultwarden (sqlite) — unhealthy


══════════════════════════════════════════════════════════════
  A) HEALTH — Live checks
══════════════════════════════════════════════════════════════

── A0) Mesh ──────────────────────────────────────────────────

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1 — front door)
────────────────────────────────────────────────────────────
    Name           Cloud Name         ☁VPS 🌐Pub 🔧DB  🔒WG  Public IP          WG IP          Type     Handshake
────────────────────────────────────────────────────────────
❌ oci-mail       oci-E2-f_0         ❌  ❌  ❌  ❌  130.110.251.193    10.0.0.3       VM       no data
❌ oci-analytics  oci-E2-f_1         ❌  ❌  ❌  ❌  129.151.228.66     10.0.0.4       VM       no data
❌ oci-apps       oci-A1-f_0         ❌  ❌  ❌  ❌  82.70.229.129      10.0.0.6       VM       no data
❌ gcp-t4         ollama-spot-gpu    ❌  ❌  ❌  ❌  34.173.227.250     10.0.0.8       VM       no data
⚠️ gcp-proxy      arch-1             ✅  ✅  ❌  ❌  35.226.147.64      10.0.0.1       HUB      no data
⚠️ gha-runner     —                  ✅  ✅  —  ❌  dynamic            10.0.0.200     CLIENT   no data
⚠️ surface        —                  ✅  ✅  —  ❌  dynamic            10.0.0.5       CLIENT   no data
⚠️ termux         —                  ✅  ✅  —  ❌  dynamic            10.0.0.9       CLIENT   no data

── A1) Public ────────────────────────────────────────────────

PUBLIC URLs (Caddy routes)
────────────────────────────────────────────────────────────
    URL                              📡TCP 🌐HTTP 🔒HTTPS 🔐AUTH Upstream                  Code
────────────────────────────────────────────────────────────
✅ auth.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.1:9091          [200] 
⚠️ api.diegonmarcos.com/c3-api      ❌  ❌  ✅  ✅  10.0.0.6:8081          [404] 
⚠️ mcp.diegonmarcos.com/c3-infra-mcp ❌  ❌  ✅  ✅  10.0.0.6:3100          [200] 
⚠️ api.diegonmarcos.com/services    ❌  ❌  ✅  ✅  10.0.0.6:8082          [404] 
✅ proxy.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.1:443           [302] 
✅ ide.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:8443          [302] 
✅ api.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3000          [200] 
✅ workflows.diegonmarcos.com       ✅  ❌  ✅  ✅  10.0.0.4:8070          [302] 
✅ db.diegonmarcos.com              ✅  ❌  ✅  ✅  10.0.0.6:8086          [302] 
✅ logs.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.4:9999          [200] 
✅ pad.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3012          [200] 
✅ files.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:3015          [429] 
✅ git.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3002          [302] 
⚠️ sheets.diegonmarcos.com          ✅  ❌  ✅  ❌  10.0.0.6:3011          [302] auth:[401]
✅ doc.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3018          [200] 
❌ dns.internal                     ❌  ❌  ❌  ❌  10.0.0.1:53            [err: reqwest::Error { kind: Request, url: "https://dns.internal/", source: hyper_util::client::legacy::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: Name or service not known" })) }] auth:[err: reqwest::Error { kind: Request, url: "https://dns.internal/", source: hyper_util::client::legacy::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: Name or service not known" })) }]
✅ grafana.diegonmarcos.com         ✅  ❌  ✅  ✅  10.0.0.6:3200          [302] 
✅ mail.diegonmarcos.com            ✅  ❌  ✅  ✅  ?                      [301] 
✅ analytics.diegonmarcos.com       ✅  ❌  ✅  ✅  10.0.0.6:8084          [302] 
✅ chat.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.6:8065          [302] 
⚠️ rss.diegonmarcos.com             ✅  ❌  ✅  ❌  10.0.0.6:8090          [302] auth:[401]
✅ photos.diegonmarcos.com          ✅  ❌  ✅  ✅  10.0.0.6:3013          [200] 
✅ cal.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:5232          [302] 
✅ smtp.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.3:8080          [405] 
✅ webmail.diegonmarcos.com         ✅  ❌  ✅  ✅  10.0.0.3:8888          [302] 
✅ mail-stalwart.diegonmarcos.com   ✅  ❌  ✅  ✅  10.0.0.3:2443          [200] 
✅ vault.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:8880          [200] 
⚠️ app.diegonmarcos.com/etherpad    ❌  ❌  ✅  ✅  10.0.0.6:3012          [404] 
⚠️ app.diegonmarcos.com/filebrowser ❌  ❌  ✅  ✅  10.0.0.6:3015          [404] 
⚠️ app.diegonmarcos.com/hedgedoc    ❌  ❌  ✅  ✅  10.0.0.6:3018          [404] 
⚠️ app.diegonmarcos.com/dozzle      ❌  ❌  ✅  ✅  10.0.0.4:9999          [404] 
⚠️ app.diegonmarcos.com/revealmd    ❌  ❌  ✅  ✅  10.0.0.6:3014          [404] 
⚠️ app.diegonmarcos.com/grafana     ❌  ❌  ✅  ✅  10.0.0.6:3016          [404] 
⚠️ app.diegonmarcos.com/gitea       ❌  ❌  ✅  ✅  10.0.0.6:3017          [404] 
⚠️ app.diegonmarcos.com/crawlee     ❌  ❌  ✅  ✅  10.0.0.6:3001          [404] 
⚠️ api.diegonmarcos.com/crawlee     ❌  ❌  ✅  ✅  10.0.0.6:3000          [404] 
⚠️ api.diegonmarcos.com/dash        ❌  ❌  ✅  ✅  diegonmarcos.github.io [301] 
✅ cloud.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:3080          [200] 
✅ mcp.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3104          [200] 
✅ mta-sts.diegonmarcos.com         ✅  ✅  ✅  ✅  static                 [404] 

API / MCP ENDPOINTS
────────────────────────────────────────────────────────────
  mcp.diegonmarcos.com/c3-infra-mcp/mcp
  mcp.diegonmarcos.com/c3-services-mcp/mcp
  mcp.diegonmarcos.com/cloud-cgc-mcp/mcp
  mcp.diegonmarcos.com/g-workspace/mcp
  mcp.diegonmarcos.com/mail-mcp/mcp
  mcp.diegonmarcos.com/mattermost-mcp/mcp

REPOS & REGISTRIES
────────────────────────────────────────────────────────────
  GIT REPOS
    cloud
    cloud-data
    front
    unix
    tools
    vault

  CONTAINER REGISTRY: ghcr.io/diegonmarcos/

── A2) Private (WireGuard mesh — .app health) ────────────────

    DNS Name                     📡TCP 🌐HTTP Port    VM               Container              Code
    ───────────────────────────────────────────────────────────────────────────────────────────────
⏸️ authelia-redis.app           ⏸️   ⏸️    6380 gcp-proxy        authelia-redis         [---]
⚠️ authelia.app                 ✅   ❌    9091 gcp-proxy        authelia               [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:9091/", source: hyper_util::client::legacy::Error(SendRequest, hyper::Error(UnexpectedMessage)) }]
⏸️ caddy.app                    ⏸️   ⏸️     443 gcp-proxy        caddy                  [---]
⏸️ hickory-dns.app              ⏸️   ⏸️      53 gcp-proxy        hickory-dns            [---]
⏸️ introspect-proxy.app         ⏸️   ⏸️    4182 gcp-proxy        introspect-proxy       [---]
⏸️ redis.app                    ⏸️   ⏸️    6379 gcp-proxy        redis                  [---]
⏸️ ollama.app                   ⏸️   ⏸️   11434 gcp-t4           ollama                 [---]
❌ dagu.app                     ❌   ❌    8070 oci-analytics    dagu                   [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:8070/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:8070, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ dozzle.app                   ❌   ❌    9999 oci-analytics    dozzle                 [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:9999/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:9999, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ c3-infra-api.app             ❌   ❌    8081 oci-apps         c3-infra-api           [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:8081/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:8081, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ c3-infra-mcp.app             ❌   ❌    3100 oci-apps         c3-infra-mcp           [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3100/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3100, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ c3-services-api.app          ❌   ❌    8082 oci-apps         c3-services-api        [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:8082/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:8082, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ c3-services-mcp.app          ❌   ❌    3101 oci-apps         c3-services-mcp        [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3101/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3101, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ c3-spec.app                  ❌   ❌    3080 oci-apps         cloud-spec             [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3080/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3080, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ cloud-cgc-mcp.app            ❌   ❌    3105 oci-apps         cloud-cgc-mcp          [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3105/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3105, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ code-server.app              ❌   ❌    8443 oci-apps         code-server            [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:8443/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:8443, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ crawlee-dashboard.app        ❌   ❌    3001 oci-apps         crawlee_dashboard      [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3001/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3001, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
⚠️ crawlee-db.app               ✅   ❌    5433 oci-apps         crawlee_db             [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:5433/", source: hyper_util::client::legacy::Error(SendRequest, hyper::Error(IncompleteMessage)) }]
❌ crawlee-minio.app            ❌   ❌    9000 oci-apps         crawlee_minio          [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:9000/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:9000, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ crawlee-redis.app            ❌   ❌    6381 oci-apps         crawlee_redis          [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:6381/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:6381, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
❌ crawlee.app                  ❌   ❌    3000 oci-apps         crawlee_api            [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3000/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3000, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
⏸️ dbgate.app                   ⏸️   ⏸️    8086 oci-apps         dbgate                 [---]
⏸️ etherpad-db.app              ⏸️   ⏸️    5436 oci-apps         etherpad_postgres      [---]
⏸️ etherpad.app                 ⏸️   ⏸️    3012 oci-apps         etherpad_app           [---]
⏸️ filebrowser.app              ⏸️   ⏸️    3015 oci-apps         filebrowser_app        [---]
⏸️ g-workspace-mcp.app          ⏸️   ⏸️    3104 oci-apps         google-workspace-mcp   [---]
⏸️ gitea.app                    ⏸️   ⏸️    3002 oci-apps         gitea                  [---]
⏸️ grafana.app                  ⏸️   ⏸️    3200 oci-apps         lgtm_grafana           [---]
⏸️ grist.app                    ⏸️   ⏸️    3011 oci-apps         grist_app              [---]
⏸️ hedgedoc-db.app              ⏸️   ⏸️    5439 oci-apps         hedgedoc_postgres      [---]
⏸️ hedgedoc.app                 ⏸️   ⏸️    3018 oci-apps         hedgedoc_app           [---]
⏸️ lgtm-loki.app                ⏸️   ⏸️    3110 oci-apps         lgtm_loki              [---]
⏸️ lgtm-mimir.app               ⏸️   ⏸️    9009 oci-apps         lgtm_mimir             [---]
⏸️ lgtm-tempo.app               ⏸️   ⏸️    3210 oci-apps         lgtm_tempo             [---]
⏸️ mail-mcp.app                 ⏸️   ⏸️    3103 oci-apps         mail-mcp               [---]
⏸️ matomo.app                   ⏸️   ⏸️    8084 oci-apps         matomo-hybrid          [---]
⏸️ mattermost-mcp.app           ⏸️   ⏸️    3102 oci-apps         mattermost-mcp         [---]
⏸️ mattermost-postgres.app      ⏸️   ⏸️    5435 oci-apps         mattermost-postgres    [---]
⏸️ mattermost.app               ⏸️   ⏸️    8065 oci-apps         mattermost             [---]
⏸️ ntfy.app                     ⏸️   ⏸️    8090 oci-apps         ntfy                   [---]
⏸️ ollama-hai.app               ⏸️   ⏸️   11435 oci-apps         ollama-hai             [---]
⏸️ photoprism.app               ⏸️   ⏸️    3013 oci-apps         photoprism_app         [---]
⏸️ quant-full-db.app            ⏸️   ⏸️    5437 oci-apps         quant_full_db          [---]
⏸️ quant-full-research.app      ⏸️   ⏸️    8890 oci-apps         quant_full_research    [---]
⏸️ quant-light-db.app           ⏸️   ⏸️    5443 oci-apps         quant_light_db         [---]
⏸️ quant-light-engine.app       ⏸️   ⏸️    5001 oci-apps         quant_light_engine     [---]
⏸️ quant-light-research.app     ⏸️   ⏸️    8889 oci-apps         quant_light_research   [---]
⏸️ radicale.app                 ⏸️   ⏸️    5232 oci-apps         radicale               [---]
⏸️ revealmd.app                 ⏸️   ⏸️    3014 oci-apps         revealmd_app           [---]
⏸️ umami-db.app                 ⏸️   ⏸️    5442 oci-apps         umami-db               [---]
❌ umami.app                    ❌   ❌    3006 oci-apps         umami                  [err: reqwest::Error { kind: Request, url: "http://10.0.0.1:3006/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", 10.0.0.1:3006, Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }]
⏸️ vaultwarden.app              ⏸️   ⏸️    8880 oci-apps         vaultwarden            [---]
⏸️ smtp-proxy.app               ⏸️   ⏸️    8080 oci-mail         smtp-proxy             [---]
⏸️ snappymail.app               ⏸️   ⏸️    8888 oci-mail         snappymail             [---]
⏸️ stalwart.app                 ⏸️   ⏸️    2443 oci-mail         stalwart               [---]

  📡 TCP: 2/55  🌐 HTTP: 0/55

── A3) Containers ────────────────────────────────────────────

gcp-proxy ✅ — 2C/2G — mem 1126M/1952M (57%) — disk 72% — swap 146M/3999M — load 0.11 0.12 0.09 — 14/14 ctrs — 19d 13h
────────────────────────────────────────────────────────────
  ⚠️ caddy                     443    443    running        Up 17 hours cpu=0.10% mem=52.6
  ⚠️ hickory-dns               —      53     running        Up 19 hours cpu=0.00% mem=9.26
  ⚠️ authelia-redis            —      6380   running        Up 3 days cpu=0.39% mem=6.492M
  ⚠️ postlite-vaultwarden      —      —      running        Up 3 days cpu=0.00% mem=500KiB
  ⚠️ postlite-authelia         —      —      running        Up 3 days cpu=0.00% mem=588KiB
  ⚠️ postlite-ntfy             —      —      running        Up 3 days cpu=0.00% mem=592KiB
  ⚠️ postlite-npm              —      —      running        Up 3 days cpu=0.00% mem=10.22M
  ⚠️ syslog-bridge             —      —      running        Up 3 days cpu=0.07% mem=4.875M
  ⚠️ github-rss                —      —      running        Up 3 days cpu=0.00% mem=16.63M
  ⚠️ ntfy                      —      —      running        Up 3 days cpu=0.00% mem=16.16M
  ✅ introspect-proxy          —      4182   HEALTHY        Up 17 hours (healthy) cpu=0.02
  ✅ authelia                  —      9091   HEALTHY        Up 3 days (healthy) cpu=0.02% 
  ✅ vaultwarden               —      —      HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ redis                     —      6379   HEALTHY        Up 3 days (healthy) cpu=0.38% 

oci-apps ✅ — 4C/24G — mem 5225M/23975M (21%) — disk 84% — swap 126M/12288M — load 0.41 0.28 0.22 — 48/51 ctrs — 21d 21h
────────────────────────────────────────────────────────────
  ❌ umami-setup               —      —      DOWN(1)        Exited (1) 17 hours ago
  ❌ cloud-builder-x-cloud-builder-1 —      —      DOWN(0)        Exited (0) 2 days ago
  ❌ c3-services-mcp           —      3101   DOWN(134)      Exited (134) 44 hours ago
  ❌ c3-infra-mcp              —      3100   UNHEALTHY      Up 3 days (unhealthy) cpu=0.08
  ⚠️ matomo-hybrid             —      8084   running        Up 46 hours cpu=0.04% mem=308.
  ⚠️ mail-mcp                  —      3103   running        Up 2 days cpu=0.22% mem=150.4M
  ⚠️ crawlee_runner            —      —      running        Up 3 days cpu=0.04% mem=28.25M
  ⚠️ crawlee_dashboard         3001   3001   running        Up 3 days cpu=0.00% mem=26.51M
  ⚠️ crawlee_scheduler         —      —      running        Up 3 days cpu=0.00% mem=248KiB
  ⚠️ cloud-spec                —      3080   running        Up 3 days cpu=0.00% mem=332KiB
  ⚠️ lgtm_tempo                —      3210   running        Up 3 days cpu=0.04% mem=25.61M
  ⚠️ lgtm_mimir                —      9009   running        Up 3 days cpu=0.43% mem=32.36M
  ⚠️ code-server               —      8443   running        Up 3 days cpu=0.00% mem=93.89M
  ⚠️ quant_light_engine        —      5001   running        Up 3 days cpu=0.00% mem=23.49M
  ⚠️ syslog-bridge             —      —      running        Up 3 days cpu=0.00% mem=15.08M
  ⚠️ github-rss                —      —      running        Up 3 days cpu=0.00% mem=15.82M
  ⚠️ ntfy                      —      8090   running        Up 3 days cpu=0.04% mem=22.93M
  ⚠️ mattermost-bots           —      —      running        Up 3 days cpu=0.01% mem=38.85M
  ⚠️ mattermost-mcp            —      3102   running        Up 3 days cpu=0.08% mem=105.3M
  ✅ umami                     —      3006   HEALTHY        Up 17 hours (healthy) cpu=0.00
  ✅ umami-db                  —      5442   HEALTHY        Up 17 hours (healthy) cpu=0.01
  ✅ news-gdelt                —      —      HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ google-workspace-mcp      —      3104   HEALTHY        Up 3 days (healthy) cpu=2.51% 
  ✅ c3-services-api           —      8082   HEALTHY        Up 3 days (healthy) cpu=0.08% 
  ✅ c3-infra-api              8081   8081   HEALTHY        Up 3 days (healthy) cpu=0.07% 
  ✅ rig-agentic-sonn-14bq8    —      —      HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 3 days (healthy) cpu=0.06% 
  ✅ crawlee_api               3000   3000   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ crawlee_redis             —      6381   HEALTHY        Up 3 days (healthy) cpu=0.50% 
  ✅ crawlee_minio             —      9000   HEALTHY        Up 3 days (healthy) cpu=0.10% 
  ✅ crawlee_db                —      5433   HEALTHY        Up 3 days (healthy) cpu=0.06% 
  ✅ ollama-hai                —      11435  HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ dbgate                    —      8086   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ grist_app                 —      3011   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 3 days (healthy) cpu=0.49% 
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 3 days (healthy) cpu=0.65% 
  ✅ etherpad_app              —      3012   HEALTHY        Up 3 days (healthy) cpu=2.07% 
  ✅ etherpad_postgres         —      5436   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ filebrowser_app           —      3015   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ quant_light_research      —      8889   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ quant_light_db            —      5443   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ gitea                     —      3002   HEALTHY        Up 3 days (healthy) cpu=2.05% 
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 3 days (healthy) cpu=0.24% 
  ✅ radicale                  —      5232   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ vaultwarden               —      8880   HEALTHY        Up 3 days (healthy) cpu=0.05% 
  ✅ mattermost                —      8065   HEALTHY        Up 3 days (healthy) cpu=0.03% 
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 3 days (healthy) cpu=1.87% 
  ✅ photoprism_app            —      3013   HEALTHY        Up 3 days (healthy) cpu=0.00% 
  ✅ photoprism_rclone         —      —      HEALTHY        Up 3 days (healthy) cpu=0.01% 
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 3 days (healthy) cpu=0.02% 

oci-mail ✅ — 1C/1G — mem 650M/954M (68%) — disk 78% — swap 336M/2559M — load 3.29 2.06 1.85 — 6/7 ctrs — 19d 17h
────────────────────────────────────────────────────────────
  ❌ dagu                      —      —      DOWN(255)      Exited (255) 42 hours ago
  ⚠️ stalwart-sorter           —      —      running        Up 16 hours cpu=0.00% mem=7.63
  ⚠️ stalwart                  2443   2443   running        Up 16 hours cpu=0.00% mem=8.79
  ⚠️ maddy                     —      —      running        Up 18 hours cpu=49.06% mem=24.
  ⚠️ maddy-sorter              —      —      running        Up 18 hours cpu=0.00% mem=7.03
  ⚠️ smtp-proxy                8080   8080   running        Up 26 hours cpu=0.00% mem=8.56
  ✅ snappymail                —      8888   HEALTHY        Up 26 hours (healthy) cpu=41.9

oci-analytics ✅ — 1C/1G — mem 600M/954M (62%) — disk 80% — swap 86M/2559M — load 0.15 0.37 0.40 — 3/3 ctrs — 12d 23h
────────────────────────────────────────────────────────────
  ⚠️ dagu                      —      8070   running        Up 2 days cpu=0.15% mem=53.18M
  ⚠️ sauron-forwarder          —      —      running        Up 5 days cpu=0.00% mem=716KiB
  ⚠️ dozzle                    —      9999   running        Up 5 days cpu=0.00% mem=31.35M


── A3b) Container Drift ──────────────────────────────────

$CONTAINER_DRIFT

── A4) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
(TODO)

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.17
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.24
✅ send.mails.diegonmarcos.com  10    feedback-smtp.us-east-1.amazonses.com.     18.235.76.96
❌ mails.diegonmarcos.com       —     no MX record                               

SPF — Outbound Policy: IP Allowlist
────────────────────────────────────────────────────────────
✅ diegonmarcos.com                 include:_spf.mx.cloudflare.net                ip4:104.30.0.0/19
✅ diegonmarcos.com                 include:amazonses.com                         
✅ diegonmarcos.com                 include:eu.rp.oracleemaildelivery.com         ip4:192.29.200.0/25, ip4:138.1.108.0/25, ip4:130.35.116.0/25
✅ send.mails.diegonmarcos.com      include:amazonses.com                         (same)
⚠️ diegonmarcos.com                 oci-mail VM IP 130.110.251.193 NOT IN SPF!

DKIM — Outbound Policy: Cryptographic Signatures
────────────────────────────────────────────────────────────
❌ dkim._domainkey              diegonmarcos.com         Stalwart             NOT FOUND
❌ mail._domainkey              diegonmarcos.com         Legacy Mailu         NOT FOUND
✅ google._domainkey            diegonmarcos.com         Google Workspace     RSA 2048
❌ cf2024-1._domainkey          diegonmarcos.com         Cloudflare           NOT FOUND
❌ resend._domainkey.mails      diegonmarcos.com         Resend/SES           NOT FOUND

DMARC — Outbound Policy
─────────────────────
✅ _dmarc.diegonmarcos.com       v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1

MAIL AUTH — Authorized Senders
────────────────────────────────────────────────────────────
    Sender               Domain                     Auth Method      SPF IP Range                   DKIM Selector
────────────────────────────────────────────────────────────
✅ Cloudflare           diegonmarcos.com           Email Routing    104.30.0.0/19                  cf2024-1._domainkey
⚠️ Stalwart             diegonmarcos.com           Direct SMTP      130.110.251.193 NOT IN SPF!    dkim._domainkey
✅ Google               diegonmarcos.com           Google SMTP      (via include)                  google._domainkey
✅ Resend/SES           mails.diegonmarcos.com     API + SES        54.240.0.0/18                  resend._dk.mails
✅ OCI Email Dlv        diegonmarcos.com           SMTP Relay       192.29.200.0/25                (via Stalwart)

MAIL FLOW — Pipeline Status
─────────────────────────────────
  📨 INBOUND: Gmail → MX → CF Email Routing → CF Worker → Caddy → smtp-proxy → Maddy
     ✅ smtp-proxy           Up 26 hours cpu=0.00% mem=8.562MiB / 954.2MiB
     ✅ maddy                Up 18 hours cpu=49.06% mem=24.25MiB / 256MiB

  📱 CLIENT: Phone/Thunderbird → gcp-proxy (35.226.147.64) → Caddy L4 → oci-mail → Maddy

  📤 OUTBOUND: Maddy → OCI SMTP relay from 130.110.251.193
     ✅ SPF OK  ✅ DKIM OK

  📤 OUTBOUND TRANSACTIONAL: App → Resend API → SES → recipient
     ✅ SPF OK  ✅ DKIM OK  ✅ DMARC OK


══════════════════════════════════════════════════════════════
  B) INFRA — Resources & Stack
══════════════════════════════════════════════════════════════

VPS / VM SPECS (all providers)
────────────────────────────────────────────────────────────
    VM               Provider   Shape                CPU    RAM    Disk     Cost
────────────────────────────────────────────────────────────
   gcp-proxy        GCP        e2-small                  2 2G     32G      Free
   gcp-t4           GCP        n1-standard-4             4 15G    50G      Spot
   oci-apps         OCI        VM.Standard.A1.Flex       4 24G    0G       Free
   oci-mail         OCI        VM.Standard.E2.1.Micro      1 1G     0G       Free
   oci-analytics    OCI        VM.Standard.E2.1.Micro      1 1G     0G       Free

RESOURCES (live)
────────────────────────────────────────────────────────────
gcp-proxy      oci-apps       oci-mail       oci-analytics 
────────────────────────────────────────────────────────────
CPU                57 cores      21 cores      68 cores      62 cores      
RAM                1126M/1952M   5225M/23975M  650M/954M     600M/954M     
RAM %              57%           21%           68%           62%           
Swap               146M/3999M    126M/12288M   336M/2559M    86M/2559M     
Disk               22G/31G       76.5G/95.8G   33G/45G       36G/48G       
Disk %             72%           84%           78%           80%           
Load               0.11 0.12 0.090.41 0.28 0.223.29 2.06 1.850.15 0.37 0.40
Containers         14/14         48/51         6/7           3/3           
Uptime             19d 13h       21d 21h       19d 17h       12d 23h       

STORAGE
────────────────────────────────────────────────────────────
  OBJECT STORAGE
    oci — cloud-backups-binaries-medias (Standard)
    oci — cloud-backups-db (Standard)
    oci — cloud-backups-media (Archive)
    oci — cloud-backups-non-binaries (Standard)
    oci — my-photos (Standard)

  DATABASES
    authelia             sqlite     authelia               gcp-proxy
    crawlee-cloud        postgres   crawlee_db             oci-apps
    etherpad             postgres   etherpad_postgres      oci-apps
    gitea                sqlite     gitea                  oci-apps
    grist                sqlite     grist_app              oci-apps
    hedgedoc             postgres   hedgedoc_postgres      oci-apps
    mattermost-bots      postgres   mattermost-postgres    oci-apps
    ntfy                 sqlite     ntfy                   oci-apps
    photoprism           mariadb    photoprism_mariadb     oci-apps
    quant-lab-light      postgres   quant_light_db         oci-apps
    umami                postgres   umami-db               oci-apps
    vaultwarden          sqlite     vaultwarden            oci-apps

── B2) Databases (live) ──────────────────────────────────
    Service              Type       Container              VM               Port   TCP  Health   Size       Backup
    ─────────────────────────────────────────────────────────────────────────────────────────────────────────
⚠️ authelia             sqlite     authelia               gcp-proxy        9091   ✅   ❌   ?          ✅
⚠️ crawlee-cloud        postgres   crawlee_db             oci-apps         5433   ✅   ❌   ?          ✅
✅ etherpad             postgres   etherpad_postgres      oci-apps         5436   ✅   ✅   7MB        ✅
⚠️ gitea                sqlite     gitea                  oci-apps         3002   ✅   ❌   ?          ✅
⚠️ grist                sqlite     grist_app              oci-apps         3011   ✅   ❌   ?          ✅
✅ hedgedoc             postgres   hedgedoc_postgres      oci-apps         5439   ✅   ✅   8MB        ✅
✅ mattermost-bots      postgres   mattermost-postgres    oci-apps         5435   ✅   ✅   15MB       ✅
⚠️ ntfy                 sqlite     ntfy                   oci-apps         8090   ✅   ❌   ?          ✅
⚠️ photoprism           mariadb    photoprism_mariadb     oci-apps         —      —   ❌   ?          ✅
⚠️ quant-lab-light      postgres   quant_light_db         oci-apps         5443   ✅   ❌   ?          ✅
✅ umami                postgres   umami-db               oci-apps         5442   ❌   ✅   11MB       ✅
⚠️ vaultwarden          sqlite     vaultwarden            oci-apps         8880   ✅   ❌   ?          ✅

  Healthy: 4/12  Running: 12/12

── B3) Object Storage ──────────────────────────────────
    Bucket                         Provider   Tier           Size       Objects   
    ────────────────────────────────────────────────────────────────────────────────
    cloud-backups-binaries-medias  oci        Standard       —          —         
    cloud-backups-db               oci        Standard       —          —         
    cloud-backups-media            oci        Archive        —          —         
    cloud-backups-non-binaries     oci        Standard       —          —         
    my-photos                      oci        Standard       —          —         


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

NETWORK SECURITY AUDIT
────────────────────────────────────────────────────────────
    Check                         gcp-proxy           oci-mail      oci-analytics           oci-apps
    ────────────────────────────────────────────────────────────────────────────────────────────────
    Declared ports       80,443,465,587,993,2443,2465,2587,2993 25,465,587,993,2025,2443,2465,2587,2993,4190,6190,8080,8443,21027,22000               none 2222,2223,2224,3000,3001,3010,8081,8099
    Scanned (public)     22,443,465,587,993             🔒 none             🔒 none             🔒 none
    Docker host ports                   443          2443,8080               none     3000,3001,8081
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                       ✅ up               ✅ up               ✅ up               ✅ up
    Containers (up/total)              14/14                6/7                3/3              48/51

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔓 gcp-proxy          35.226.147.64      ports: 22, 443, 465, 587, 993
🔒 gcp-t4             34.173.227.250     ports: none reachable
🔒 oci-apps           82.70.229.129      ports: none reachable
🔒 oci-mail           130.110.251.193    ports: none reachable
🔒 oci-analytics      129.151.228.66     ports: none reachable

BACKUPS / DATABASES
────────────────────────────────────────────────────────────
   authelia             sqlite     authelia               /config/db.sqlite3 gcp-proxy        authelia.app:9091
   crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps         crawlee-db.app:5433
   etherpad             postgres   etherpad_postgres      etherpad       oci-apps         etherpad-db.app:5436
   gitea                sqlite     gitea                  /data/gitea/gitea.db oci-apps         gitea.app:3002
   grist                sqlite     grist_app              /persist/grist-sessions.db oci-apps         grist.app:3011
   hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps         hedgedoc-db.app:5439
   mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps         mattermost-postgres.app:5435
   ntfy                 sqlite     ntfy                   /var/cache/ntfy/cache.db oci-apps         ntfy.app:8090
   photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps         embedded
   quant-lab-light      postgres   quant_light_db         quantlab       oci-apps         quant-light-db.app:5443
   umami                postgres   umami-db               umami          oci-apps         umami-db.app:5442
   vaultwarden          sqlite     vaultwarden            /data/db.sqlite3 oci-apps         vaultwarden.app:8880

DOCKER NETWORKS
────────────────────────────────────────────────────────────
    Network                      Services
    ────────────────────────────────────────────────────────────
    auth-net                     authelia
    default                      radicale
    etherpad_net                 etherpad

VAULT — Providers
────────────────────────────────────────────────────────────
  (vault not available)


══════════════════════════════════════════════════════════════
  D) STACK — Framework & Paths
══════════════════════════════════════════════════════════════

FRAMEWORK — Key Paths
────────────────────────────────────────────────────────────
  BUILD ENGINES
    Service engine       ~/git/cloud/a_solutions/_engine.sh
    HM engine            ~/git/cloud/b_infra/home-manager/_engine.sh
    Front engine         ~/git/front/1.ops/build_main.sh
    NixOS host           ~/git/unix/aa_nixos-surface_host/build.sh

  HOME-MANAGER
    gcp-proxy            ~/git/cloud/b_infra/home-manager/gcp-proxy/src/
    gcp-t4               ~/git/cloud/b_infra/home-manager/gcp-t4/src/
    oci-apps             ~/git/cloud/b_infra/home-manager/oci-apps/src/
    oci-mail             ~/git/cloud/b_infra/home-manager/oci-mail/src/
    oci-analytics        ~/git/cloud/b_infra/home-manager/oci-analytics/src/

  DATA
    cloud-data           ~/git/cloud-data/
    Topology             ~/git/cloud-data/cloud-data-topology.json
    Consolidated         ~/git/cloud-data/_cloud-data-consolidated.json

  TERRAFORM
    OCI                  ~/git/cloud/c_vps/vps_oci/src/main.tf
    GCP                  ~/git/cloud/c_vps/vps_gcloud/src/main.tf
    Cloudflare           ~/git/cloud/c_vps/ba-clo_cloudflare/src/main.tf


══════════════════════════════════════════════════════════════
  Z) APPENDIX
══════════════════════════════════════════════════════════════

PERFORMANCE
────────────────────────────────────────────────────────────
  TOTAL                41.6s
  vm_ssh               41.3s
  mail_dns             23.5s
  public_urls          20.9s
  databases            19.8s
  private              16.8s
  mesh                 13.8s
  port_scan            7.4s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  41.6s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
