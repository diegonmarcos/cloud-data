```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-04-17T10:18:18.654320960+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
6 critical, 3 warnings — 9 total

    ❌ A3       oci-apps/docker-cloud-builder-1 — exited
    ⚠️ A3       oci-apps/news-gdelt — unhealthy
    ❌ A3       oci-analytics/umami — exited
    ❌ A1       dns.internal — [err: error sending request for url (https://dns.internal/)]
    ❌ B2       gcp-proxy/authelia (sqlite) — not running
    ❌ B2       gcp-proxy/authelia-redis (redis) — not running
    ❌ B2       gcp-proxy/redis (redis) — not running
    ⚠️ B2       oci-apps/lgtm_grafana (grafana) — unhealthy
    ⚠️ B2       oci-apps/lgtm_loki (loki) — unhealthy


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
✅ git.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3002          [302] 
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
✅ files.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:3015          [200] 
⚠️ sheets.diegonmarcos.com          ✅  ❌  ✅  ❌  10.0.0.6:3011          [302] auth:[401]
✅ doc.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3018          [200] 
❌ dns.internal                     ❌  ❌  ❌  ❌  10.0.0.1:53            [err: error sending request for url (https://dns.internal/)] auth:[err: error sending request for url (https://dns.internal/)]
✅ grafana.diegonmarcos.com         ✅  ❌  ✅  ✅  10.0.0.6:3200          [302] 
⚠️ mail.diegonmarcos.com            ✅  ❌  ✅  ❌  10.0.0.3:443           [302] auth:[502]
⚠️ analytics.diegonmarcos.com       ✅  ❌  ✅  ❌  10.0.0.4:8084          [302] auth:[502]
✅ chat.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.6:8065          [302] 
⚠️ rss.diegonmarcos.com             ✅  ❌  ✅  ❌  10.0.0.6:8090          [302] auth:[401]
✅ photos.diegonmarcos.com          ✅  ❌  ✅  ✅  10.0.0.6:3013          [200] 
✅ cal.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:5232          [302] 
✅ smtp.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.3:8080          [405] 
✅ webmail.diegonmarcos.com         ✅  ❌  ✅  ✅  10.0.0.3:8888          [200] 
✅ mail-stalwart.diegonmarcos.com   ✅  ❌  ✅  ✅  10.0.0.3:2443          [200] 
✅ vault.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:8880          [200] 
✅ windmill.diegonmarcos.com        ✅  ❌  ✅  ✅  10.0.0.6:8000          [302] 
⚠️ app.diegonmarcos.com/etherpad    ❌  ❌  ✅  ✅  10.0.0.6:3012          [404] 
⚠️ app.diegonmarcos.com/filebrowser ❌  ❌  ✅  ✅  10.0.0.6:3015          [404] 
⚠️ app.diegonmarcos.com/hedgedoc    ❌  ❌  ✅  ✅  10.0.0.6:3018          [404] 
⚠️ app.diegonmarcos.com/dozzle      ❌  ❌  ✅  ✅  10.0.0.4:9999          [404] 
⚠️ app.diegonmarcos.com/windmill    ❌  ❌  ✅  ✅  10.0.0.6:8000          [404] 
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

    DNS Name                     📡TCP 🌐HTTP 🐳CTR Port  VM             Container              Code
    ─────────────────────────────────────────────────────────────────────────────────────────────────────────
✅ authelia-redis.app           ✅   ❌   ❌    6380 gcp-proxy      authelia-redis         [err: error sending request for url (https://authelia-redis.app/)]
✅ authelia.app                 ✅   ✅   ❌    9091 gcp-proxy      authelia               [200]
✅ caddy.app                    ✅   ✅   ❌     443 gcp-proxy      caddy                  [400]
✅ hickory-dns.app              ✅   ❌   ❌      53 gcp-proxy      hickory-dns            [502]
✅ introspect-proxy.app         ✅   ✅   ❌    4182 gcp-proxy      introspect-proxy       [404]
✅ redis.app                    ✅   ❌   ❌    6379 gcp-proxy      redis                  [502]
✅ ollama.app                   ✅   ❌   ❌   11434 gcp-t4         ollama                 [502]
✅ dagu.app                     ✅   ✅   ✅    8070 oci-analytics  dagu                   [200]
✅ dozzle.app                   ✅   ✅   ✅    9999 oci-analytics  dozzle                 [200]
✅ matomo.app                   ✅   ❌   ✅    8084 oci-analytics  matomo-hybrid          [502]
✅ umami-db.app                 ✅   ❌   ✅    5442 oci-analytics  umami-db               [err: error sending request for url (https://umami-db.app/)]
✅ umami.app                    ✅   ❌   ❌    3006 oci-analytics  umami                  [502]
✅ backup-gitea.app             ✅   ✅   ✅    3002 oci-apps       gitea                  [200]
✅ c3-infra-api.app             ✅   ❌   ❌    8081 oci-apps       c3-infra-api           [502]
✅ c3-infra-mcp.app             ✅   ✅   ✅    3100 oci-apps       c3-infra-mcp           [404]
✅ c3-services-api.app          ✅   ❌   ❌    8082 oci-apps       c3-services-api        [502]
✅ c3-services-mcp.app          ✅   ✅   ✅    3101 oci-apps       c3-services-mcp        [404]
✅ c3-spec.app                  ✅   ✅   ✅    3080 oci-apps       cloud-spec             [200]
✅ cloud-cgc-mcp.app            ✅   ✅   ✅    3105 oci-apps       cloud-cgc-mcp          [404]
✅ code-server.app              ✅   ✅   ✅    8443 oci-apps       code-server            [302]
✅ crawlee-dashboard.app        ✅   ❌   ✅    3001 oci-apps       crawlee_dashboard      [err: error sending request for url (https://crawlee-dashboard.app/)]
✅ crawlee-db.app               ✅   ❌   ✅    5433 oci-apps       crawlee_db             [err: error sending request for url (https://crawlee-db.app/)]
✅ crawlee-minio.app            ✅   ❌   ✅    9000 oci-apps       crawlee_minio          [err: error sending request for url (https://crawlee-minio.app/)]
✅ crawlee-redis.app            ✅   ❌   ✅    6381 oci-apps       crawlee_redis          [err: error sending request for url (https://crawlee-redis.app/)]
✅ crawlee.app                  ✅   ✅   ✅    3000 oci-apps       crawlee_api            [404]
✅ dbgate.app                   ✅   ✅   ✅    8086 oci-apps       dbgate                 [200]
✅ etherpad-db.app              ✅   ❌   ✅    5436 oci-apps       etherpad_postgres      [err: error sending request for url (https://etherpad-db.app/)]
✅ etherpad.app                 ✅   ✅   ✅    3012 oci-apps       etherpad_app           [200]
✅ filebrowser.app              ✅   ✅   ✅    3015 oci-apps       filebrowser_app        [200]
✅ g-workspace-mcp.app          ✅   ❌   ❌    3104 oci-apps       google-workspace-mcp   [502]
✅ gitea.app                    ✅   ✅   ✅    3002 oci-apps       gitea                  [200]
✅ grafana.app                  ✅   ✅   ✅    3200 oci-apps       lgtm_grafana           [302]
✅ grist.app                    ✅   ✅   ✅    3011 oci-apps       grist_app              [200]
✅ hedgedoc-db.app              ✅   ❌   ✅    5439 oci-apps       hedgedoc_postgres      [err: error sending request for url (https://hedgedoc-db.app/)]
✅ hedgedoc.app                 ✅   ✅   ✅    3018 oci-apps       hedgedoc_app           [200]
✅ lgtm-loki.app                ✅   ❌   ✅    3110 oci-apps       lgtm_loki              [err: error sending request for url (https://lgtm-loki.app/)]
✅ lgtm-mimir.app               ✅   ❌   ✅    9009 oci-apps       lgtm_mimir             [err: error sending request for url (https://lgtm-mimir.app/)]
✅ lgtm-tempo.app               ✅   ❌   ✅    3210 oci-apps       lgtm_tempo             [err: error sending request for url (https://lgtm-tempo.app/)]
✅ mail-mcp.app                 ✅   ✅   ✅    3103 oci-apps       mail-mcp               [404]
✅ mattermost-mcp.app           ✅   ✅   ✅    3102 oci-apps       mattermost-mcp         [404]
✅ mattermost-postgres.app      ✅   ❌   ✅    5435 oci-apps       mattermost-postgres    [err: error sending request for url (https://mattermost-postgres.app/)]
✅ mattermost.app               ✅   ✅   ✅    8065 oci-apps       mattermost             [200]
✅ ntfy.app                     ✅   ✅   ✅    8090 oci-apps       ntfy                   [200]
✅ ollama-hai.app               ✅   ✅   ✅   11435 oci-apps       ollama-hai             [200]
✅ photoprism.app               ✅   ✅   ✅    3013 oci-apps       photoprism_app         [307]
✅ quant-full-db.app            ✅   ❌   ❌    5437 oci-apps       quant_full_db          [err: error sending request for url (https://quant-full-db.app/)]
✅ quant-full-research.app      ✅   ❌   ❌    8890 oci-apps       quant_full_research    [err: error sending request for url (https://quant-full-research.app/)]
✅ quant-light-db.app           ✅   ❌   ✅    5443 oci-apps       quant_light_db         [err: error sending request for url (https://quant-light-db.app/)]
✅ quant-light-engine.app       ✅   ❌   ✅    5001 oci-apps       quant_light_engine     [err: error sending request for url (https://quant-light-engine.app/)]
✅ quant-light-research.app     ✅   ❌   ✅    8889 oci-apps       quant_light_research   [err: error sending request for url (https://quant-light-research.app/)]
✅ radicale.app                 ✅   ✅   ✅    5232 oci-apps       radicale               [302]
✅ revealmd.app                 ✅   ❌   ❌    3014 oci-apps       revealmd_app           [502]
✅ rig-agentic-sonn-14bq8.app   ✅   ✅   ✅    8091 oci-apps       rig-agentic-sonn-14bq8 [404]
✅ vaultwarden.app              ✅   ✅   ✅    8880 oci-apps       vaultwarden            [200]
✅ windmill-app.app             ✅   ✅   ✅    8000 oci-apps       windmill-server        [200]
✅ windmill-db.app              ✅   ❌   ✅    5440 oci-apps       windmill-db            [err: error sending request for url (https://windmill-db.app/)]
✅ maddy.app                    ✅   ❌   ✅     443 oci-mail       maddy                  [502]
✅ smtp-proxy.app               ✅   ✅   ✅    8080 oci-mail       smtp-proxy             [405]
✅ snappymail.app               ✅   ✅   ✅    8888 oci-mail       snappymail             [200]
✅ stalwart.app                 ✅   ❌   ✅    2443 oci-mail       stalwart               [err: error sending request for url (https://stalwart.app/)]

  📡 TCP: 60/60  🌐 HTTP: 31/60  🐳 Container: 46/60

── A3) Containers ────────────────────────────────────────────

gcp-proxy ✅ — 2C/2G — mem 1139M/1952M (58%) — disk 40% — swap 152M/3999M — load 0.04 0.08 0.08 — 0/0 ctrs — 16d 12h
────────────────────────────────────────────────────────────

oci-apps ✅ — 4C/24G — mem 5400M/23975M (22%) — disk 76% — swap 145M/12288M — load 0.19 0.37 0.65 — 46/50 ctrs — 18d 20h
────────────────────────────────────────────────────────────
  ❌ c3-services-api           —      8082   DOWN(?)        Created
  ❌ c3-infra-api              8081   8081   DOWN(?)        Created
  ❌ google-workspace-mcp      —      3104   DOWN(?)        Created
  ❌ docker-cloud-builder-1    —      —      DOWN(0)        Exited (0) 53 minutes ago
  ❌ news-gdelt                —      —      UNHEALTHY      Up 17 hours (unhealthy) cpu=0.
  ⚠️ crawlee_runner            —      —      running        Up 13 hours cpu=0.12% mem=27.3
  ⚠️ crawlee_dashboard         3001   3001   running        Up 13 hours cpu=0.00% mem=26.7
  ⚠️ crawlee_scheduler         —      —      running        Up 13 hours cpu=0.00% mem=248K
  ⚠️ windmill-worker           —      —      running        Up 14 hours cpu=0.81% mem=30.5
  ⚠️ cloud-spec                —      3080   running        Up 14 hours cpu=0.00% mem=332K
  ⚠️ lgtm_tempo                —      3210   running        Up 14 hours cpu=0.02% mem=25.3
  ⚠️ lgtm_mimir                —      9009   running        Up 14 hours cpu=0.42% mem=28.6
  ⚠️ code-server               —      8443   running        Up 14 hours cpu=0.00% mem=93.4
  ⚠️ quant_light_engine        —      5001   running        Up 14 hours cpu=0.00% mem=23.4
  ⚠️ syslog-bridge             —      —      running        Up 14 hours cpu=0.00% mem=15.0
  ⚠️ github-rss                —      —      running        Up 14 hours cpu=0.00% mem=15.3
  ⚠️ ntfy                      —      8090   running        Up 14 hours cpu=0.00% mem=23.6
  ⚠️ mattermost-bots           —      —      running        Up 14 hours cpu=0.01% mem=38.8
  ⚠️ mattermost-mcp            —      3102   running        Up 14 hours cpu=0.20% mem=104.
  ⚠️ mail-mcp                  —      3103   running        Up 14 hours cpu=0.20% mem=142.
  ✅ rig-agentic-sonn-14bq8    —      8091   HEALTHY        Up 21 minutes (healthy) cpu=0.
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 13 hours (healthy) cpu=0.13
  ✅ c3-infra-mcp              —      3100   HEALTHY        Up 13 hours (healthy) cpu=0.12
  ✅ crawlee_api               3000   3000   HEALTHY        Up 13 hours (healthy) cpu=0.00
  ✅ crawlee_redis             —      6381   HEALTHY        Up 13 hours (healthy) cpu=1.69
  ✅ crawlee_minio             —      9000   HEALTHY        Up 13 hours (healthy) cpu=1.36
  ✅ crawlee_db                —      5433   HEALTHY        Up 13 hours (healthy) cpu=1.80
  ✅ windmill-server           —      8000   HEALTHY        Up 14 hours (healthy) cpu=0.03
  ✅ windmill-db               —      5440   HEALTHY        Up 14 hours (healthy) cpu=3.99
  ✅ ollama-hai                —      11435  HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ dbgate                    —      8086   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ grist_app                 —      3011   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 14 hours (healthy) cpu=0.62
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 14 hours (healthy) cpu=0.59
  ✅ etherpad_app              —      3012   HEALTHY        Up 14 hours (healthy) cpu=0.34
  ✅ etherpad_postgres         —      5436   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ filebrowser_app           —      3015   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ quant_light_research      —      8889   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ quant_light_db            —      5443   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ gitea                     —      3002   HEALTHY        Up 14 hours (healthy) cpu=1.88
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 14 hours (healthy) cpu=0.27
  ✅ radicale                  —      5232   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ vaultwarden               —      8880   HEALTHY        Up 14 hours (healthy) cpu=0.01
  ✅ mattermost                —      8065   HEALTHY        Up 14 hours (healthy) cpu=0.02
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 14 hours (healthy) cpu=0.01
  ✅ photoprism_app            —      3013   HEALTHY        Up 14 hours (healthy) cpu=0.00
  ✅ photoprism_rclone         —      —      HEALTHY        Up 14 hours (healthy) cpu=0.01
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 14 hours (healthy) cpu=2.72
  ✅ c3-services-mcp           —      3101   HEALTHY        Up 14 hours (healthy) cpu=1.40

oci-mail ✅ — 1C/1G — mem 648M/954M (67%) — disk 72% — swap 282M/2559M — load 0.23 0.30 0.32 — 5/5 ctrs — 16d 16h
────────────────────────────────────────────────────────────
  ⚠️ stalwart                  2443   2443   running        Up 57 seconds cpu=0.00% mem=48
  ⚠️ smtp-proxy                8080   8080   running        Up 4 minutes cpu=0.00% mem=3.0
  ⚠️ maddy                     —      443    running        Up 23 hours cpu=0.01% mem=12.2
  ⚠️ dagu                      —      —      running        Up 12 days cpu=0.02% mem=11.41
  ✅ snappymail                —      8888   HEALTHY        Up 22 hours (healthy) cpu=0.02

oci-analytics ✅ — 1C/1G — mem 544M/954M (57%) — disk 46% — swap 174M/1535M — load 1.64 0.77 0.83 — 5/6 ctrs — 9d 22h
────────────────────────────────────────────────────────────
  ❌ umami                     —      3006   DOWN(0)        Exited (0) About an hour ago
  ⚠️ matomo-hybrid             —      8084   running        Up 21 hours cpu=0.02% mem=3.79
  ⚠️ dagu                      —      8070   running        Up 22 hours cpu=7.92% mem=21.8
  ⚠️ sauron-forwarder          —      —      running        Up 2 days cpu=0.01% mem=492KiB
  ⚠️ dozzle                    —      9999   running        Up 2 days cpu=0.02% mem=2.191M
  ✅ umami-db                  —      5442   HEALTHY        Up 21 hours (healthy) cpu=7.45


── A3b) Container Drift ──────────────────────────────────

CONTAINER HEALTH — 56/61 running, 32 healthy, 1 unhealthy, 2 exited
──────────────────────────────────────────────────────────────────────────────────────────────────────────────
    VM               Total  Up     Healthy  Unhealthy  Exited   Mem Used   Mem Total  Disk       Load    
    ────────────────────────────────────────────────────────────────────────────────────────────────────
✅ gcp-proxy             0      0        0          0        0 1139M      1952M      40%        0.04    
⚠️ oci-apps             50     46       30          1        1 5400M      23975M     76%        0.19    
✅ oci-mail              5      5        1          0        0 648M       954M       72%        0.23    
⚠️ oci-analytics         6      5        1          0        1 544M       954M       46%        1.64    
    ────────────────────────────────────────────────────────────────────────────────────────────────────

    Container                 Health         VM           Status (docker ps + stats)         
    ───────────────────────────────────────────────────────────────────────────────────────────────
  ❌ c3-services-api           CREATED        oci-apps     Created
  ❌ c3-infra-api              CREATED        oci-apps     Created
  ❌ google-workspace-mcp      CREATED        oci-apps     Created
  ❌ docker-cloud-builder-1    EXITED         oci-apps     Exited (0) 53 minutes ago
  ⚠️ news-gdelt                UNHEALTHY      oci-apps     Up 17 hours (unhealthy) cpu=0.00% mem=48.85MiB / 1
  ✅ crawlee_runner            running        oci-apps     Up 13 hours cpu=0.12% mem=27.32MiB / 23.41GiB
  ✅ crawlee_dashboard         running        oci-apps     Up 13 hours cpu=0.00% mem=26.79MiB / 23.41GiB
  ✅ crawlee_scheduler         running        oci-apps     Up 13 hours cpu=0.00% mem=248KiB / 23.41GiB
  ✅ windmill-worker           running        oci-apps     Up 14 hours cpu=0.81% mem=30.55MiB / 256MiB
  ✅ cloud-spec                running        oci-apps     Up 14 hours cpu=0.00% mem=332KiB / 512MiB
  ✅ lgtm_tempo                running        oci-apps     Up 14 hours cpu=0.02% mem=25.36MiB / 23.41GiB
  ✅ lgtm_mimir                running        oci-apps     Up 14 hours cpu=0.42% mem=28.65MiB / 23.41GiB
  ✅ code-server               running        oci-apps     Up 14 hours cpu=0.00% mem=93.48MiB / 512MiB
  ✅ quant_light_engine        running        oci-apps     Up 14 hours cpu=0.00% mem=23.49MiB / 23.41GiB
  ✅ syslog-bridge             running        oci-apps     Up 14 hours cpu=0.00% mem=15.07MiB / 32MiB
  ✅ github-rss                running        oci-apps     Up 14 hours cpu=0.00% mem=15.32MiB / 32MiB
  ✅ ntfy                      running        oci-apps     Up 14 hours cpu=0.00% mem=23.63MiB / 64MiB
  ✅ mattermost-bots           running        oci-apps     Up 14 hours cpu=0.01% mem=38.84MiB / 512MiB
  ✅ mattermost-mcp            running        oci-apps     Up 14 hours cpu=0.20% mem=104.9MiB / 23.41GiB
  ✅ mail-mcp                  running        oci-apps     Up 14 hours cpu=0.20% mem=142.3MiB / 23.41GiB
  ✅ rig-agentic-sonn-14bq8    healthy        oci-apps     Up 21 minutes (healthy) cpu=0.00% mem=10.31MiB / 2
  ✅ cloud-cgc-mcp             healthy        oci-apps     Up 13 hours (healthy) cpu=0.13% mem=166.6MiB / 23.
  ✅ c3-infra-mcp              healthy        oci-apps     Up 13 hours (healthy) cpu=0.12% mem=146.1MiB / 512
  ✅ crawlee_api               healthy        oci-apps     Up 13 hours (healthy) cpu=0.00% mem=35.36MiB / 23.
  ✅ crawlee_redis             healthy        oci-apps     Up 13 hours (healthy) cpu=1.69% mem=7.176MiB / 23.
  ✅ crawlee_minio             healthy        oci-apps     Up 13 hours (healthy) cpu=1.36% mem=91.44MiB / 23.
  ✅ crawlee_db                healthy        oci-apps     Up 13 hours (healthy) cpu=1.80% mem=20.64MiB / 23.
  ✅ windmill-server           healthy        oci-apps     Up 14 hours (healthy) cpu=0.03% mem=265.2MiB / 512
  ✅ windmill-db               healthy        oci-apps     Up 14 hours (healthy) cpu=3.99% mem=110.9MiB / 128
  ✅ ollama-hai                healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=27MiB / 6GiB
  ✅ dbgate                    healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=53.5MiB / 256M
  ✅ grist_app                 healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=178.4MiB / 512
  ✅ lgtm_grafana              healthy        oci-apps     Up 14 hours (healthy) cpu=0.62% mem=252.5MiB / 23.
  ✅ lgtm_loki                 healthy        oci-apps     Up 14 hours (healthy) cpu=0.59% mem=73.48MiB / 23.
  ✅ etherpad_app              healthy        oci-apps     Up 14 hours (healthy) cpu=0.34% mem=299.9MiB / 512
  ✅ etherpad_postgres         healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=26.66MiB / 512
  ✅ filebrowser_app           healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=25.05MiB / 512
  ✅ quant_light_research      healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=146.3MiB / 23.
  ✅ quant_light_db            healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=17.94MiB / 23.
  ✅ gitea                     healthy        oci-apps     Up 14 hours (healthy) cpu=1.88% mem=116.3MiB / 23.
  ✅ hedgedoc_app              healthy        oci-apps     Up 14 hours (healthy) cpu=0.27% mem=126.9MiB / 512
  ✅ radicale                  healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=24.14MiB / 512
  ✅ hedgedoc_postgres         healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=21.94MiB / 512
  ✅ vaultwarden               healthy        oci-apps     Up 14 hours (healthy) cpu=0.01% mem=39.52MiB / 128
  ✅ mattermost                healthy        oci-apps     Up 14 hours (healthy) cpu=0.02% mem=156.4MiB / 512
  ✅ mattermost-postgres       healthy        oci-apps     Up 14 hours (healthy) cpu=0.01% mem=34.76MiB / 512
  ✅ photoprism_app            healthy        oci-apps     Up 14 hours (healthy) cpu=0.00% mem=129.6MiB / 512
  ✅ photoprism_rclone         healthy        oci-apps     Up 14 hours (healthy) cpu=0.01% mem=27.57MiB / 512
  ✅ photoprism_mariadb        healthy        oci-apps     Up 14 hours (healthy) cpu=2.72% mem=114.8MiB / 512
  ✅ c3-services-mcp           healthy        oci-apps     Up 14 hours (healthy) cpu=1.40% mem=123.2MiB / 512
  ✅ stalwart                  running        oci-mail     Up 57 seconds cpu=0.00% mem=48.35MiB / 256MiB
  ✅ smtp-proxy                running        oci-mail     Up 4 minutes cpu=0.00% mem=3.035MiB / 954.2MiB
  ✅ maddy                     running        oci-mail     Up 23 hours cpu=0.01% mem=12.23MiB / 256MiB
  ✅ dagu                      running        oci-mail     Up 12 days cpu=0.02% mem=11.41MiB / 256MiB
  ✅ snappymail                healthy        oci-mail     Up 22 hours (healthy) cpu=0.02% mem=16.18MiB / 64M
  ❌ umami                     EXITED         oci-analytics Exited (0) About an hour ago
  ✅ matomo-hybrid             running        oci-analytics Up 21 hours cpu=0.02% mem=3.797MiB / 954.2MiB
  ✅ dagu                      running        oci-analytics Up 22 hours cpu=7.92% mem=21.86MiB / 256MiB
  ✅ sauron-forwarder          running        oci-analytics Up 2 days cpu=0.01% mem=492KiB / 954.2MiB
  ✅ dozzle                    running        oci-analytics Up 2 days cpu=0.02% mem=2.191MiB / 64MiB
  ✅ umami-db                  healthy        oci-analytics Up 21 hours (healthy) cpu=7.45% mem=14.73MiB / 128

── A4) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
(TODO)

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  162.159.205.11
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.17
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.24
✅ send.mails.diegonmarcos.com  10    feedback-smtp.us-east-1.amazonses.com.     34.192.233.193
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
     ✅ smtp-proxy           Up 4 minutes cpu=0.00% mem=3.035MiB / 954.2MiB
     ✅ maddy                Up 23 hours cpu=0.01% mem=12.23MiB / 256MiB

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
CPU                58 cores      22 cores      67 cores      57 cores      
RAM                1139M/1952M   5400M/23975M  648M/954M     544M/954M     
RAM %              58%           22%           67%           57%           
Swap               152M/3999M    145M/12288M   282M/2559M    174M/1535M    
Disk               12G/31G       69.2G/95.8G   30G/45G       21G/48G       
Disk %             40%           76%           72%           46%           
Load               0.04 0.08 0.080.19 0.37 0.650.23 0.30 0.321.64 0.77 0.83
Containers         0/0           46/50         5/5           5/6           
Uptime             16d 12h       18d 20h       16d 16h       9d 22h        

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
    authelia             redis      authelia-redis         gcp-proxy
    redis                redis      redis                  gcp-proxy
    matomo               custom     matomo-hybrid          oci-analytics
    umami                postgres   umami-db               oci-analytics
    crawlee-cloud        postgres   crawlee_db             oci-apps
    crawlee-cloud        redis      crawlee_redis          oci-apps
    crawlee-cloud        s3         crawlee_minio          oci-apps
    etherpad             postgres   etherpad_postgres      oci-apps
    gitea                sqlite     gitea                  oci-apps
    grist                sqlite     grist_app              oci-apps
    hedgedoc             postgres   hedgedoc_postgres      oci-apps
    lgtm                 grafana    lgtm_grafana           oci-apps
    lgtm                 loki       lgtm_loki              oci-apps
    lgtm                 tempo      lgtm_tempo             oci-apps
    lgtm                 mimir      lgtm_mimir             oci-apps
    mattermost-bots      postgres   mattermost-postgres    oci-apps
    ntfy                 sqlite     ntfy                   oci-apps
    photoprism           mariadb    photoprism_mariadb     oci-apps
    quant-lab-full       postgres   quant_full_db          oci-apps
    quant-lab-light      postgres   quant_light_db         oci-apps
    vaultwarden          sqlite     vaultwarden            oci-apps
    windmill             postgres   windmill-db            oci-apps
    stalwart             custom     stalwart               oci-mail

── B2) Databases (live) ──────────────────────────────────
DECLARED DATABASES — 23 total (1 mimir, 5 sqlite, 1 loki, 1 s3, 1 tempo, 1 mariadb, 1 grafana, 3 redis, 7 postgres, 2 custom)
    Service              Type       Container              VM               Port   TCP  Health   Size       Backup
    ─────────────────────────────────────────────────────────────────────────────────────────────────────────
❌ authelia             sqlite     authelia               gcp-proxy        9091   ✅   ❌   ?          ✅
❌ authelia             redis      authelia-redis         gcp-proxy        6380   ❌   ❌   ?          ✅
❌ redis                redis      redis                  gcp-proxy        6379   ✅   ❌   ?          ❌
✅ matomo               custom     matomo-hybrid          oci-analytics    8084   ❌   ✅   ?          ✅
✅ umami                postgres   umami-db               oci-analytics    5442   ❌   ✅   9MB        ✅
✅ crawlee-cloud        postgres   crawlee_db             oci-apps         5433   ✅   ✅   ?          ✅
✅ crawlee-cloud        redis      crawlee_redis          oci-apps         6381   ✅   ✅   ?          ✅
✅ crawlee-cloud        s3         crawlee_minio          oci-apps         9000   ✅   ✅   ?          ✅
✅ etherpad             postgres   etherpad_postgres      oci-apps         5436   ✅   ✅   7MB        ✅
✅ gitea                sqlite     gitea                  oci-apps         3002   ✅   ✅   ?          ✅
✅ grist                sqlite     grist_app              oci-apps         3011   ✅   ✅   ?          ✅
✅ hedgedoc             postgres   hedgedoc_postgres      oci-apps         5439   ✅   ✅   8MB        ✅
⚠️ lgtm                 grafana    lgtm_grafana           oci-apps         3200   ✅   ❌   ?          ❌
⚠️ lgtm                 loki       lgtm_loki              oci-apps         3110   ✅   ❌   ?          ❌
✅ lgtm                 tempo      lgtm_tempo             oci-apps         3210   ✅   ✅   ?          ❌
✅ lgtm                 mimir      lgtm_mimir             oci-apps         9009   ✅   ✅   ?          ❌
✅ mattermost-bots      postgres   mattermost-postgres    oci-apps         5435   ✅   ✅   15MB       ✅
✅ ntfy                 sqlite     ntfy                   oci-apps         8090   ✅   ✅   ?          ✅
✅ photoprism           mariadb    photoprism_mariadb     oci-apps         —      —   ✅   ?          ✅
✅ quant-lab-light      postgres   quant_light_db         oci-apps         5443   ✅   ✅   ?          ✅
✅ vaultwarden          sqlite     vaultwarden            oci-apps         8880   ✅   ✅   ?          ✅
✅ windmill             postgres   windmill-db            oci-apps         5440   ✅   ✅   19MB       ✅
✅ stalwart             custom     stalwart               oci-mail         2443   ✅   ✅   ?          ✅

  Healthy: 18/23  Running: 20/23

── B3) Object Storage ──────────────────────────────────
OBJECT STORAGE — 6 buckets (live)
    Bucket                         Provider   Tier           Live   Size       Objects   
    ──────────────────────────────────────────────────────────────────────────────────────────
✅ cloud-backups-binaries-medias  OCI        Standard       ✅   0B         0         
✅ cloud-backups-db               OCI        Standard       ✅   61MB       22        
✅ cloud-backups-media            OCI        Standard       ✅   606B       2         
✅ cloud-backups-non-binaries     OCI        Standard       ✅   0B         0         
✅ my-photos                      OCI        Standard       ✅   2.1GB      1000+     
✅ crawlee_minio                  MinIO      Local          ✅   —          —         


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

NETWORK SECURITY AUDIT
────────────────────────────────────────────────────────────
    Check                         gcp-proxy           oci-mail      oci-analytics           oci-apps
    ────────────────────────────────────────────────────────────────────────────────────────────────
    Declared ports       80,443,465,587,993 25,465,587,993,2025,2443,2465,2587,2993,4190,6190,8080,8443,21027,22000               none 2222,2223,2224,3000,3001,3010,8081,8099
    Scanned (public)     22,443,465,587,993             🔒 none             🔒 none             🔒 none
    Docker host ports                  none          2443,8080               none     3000,3001,8081
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                       ✅ up               ✅ up               ✅ up               ✅ up
    Containers (up/total)                0/0                5/5                5/6              46/50

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
   authelia             redis      authelia-redis         custom         gcp-proxy        authelia-redis.app:6380
   redis                redis      redis                  custom         gcp-proxy        redis.app:6379
   matomo               custom     matomo-hybrid          custom         oci-analytics    matomo.app:8084
   umami                postgres   umami-db               umami          oci-analytics    umami-db.app:5442
   crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps         crawlee-db.app:5433
   crawlee-cloud        redis      crawlee_redis          custom         oci-apps         crawlee-redis.app:6381
   crawlee-cloud        s3         crawlee_minio          custom         oci-apps         crawlee-minio.app:9000
   etherpad             postgres   etherpad_postgres      etherpad       oci-apps         etherpad-db.app:5436
   gitea                sqlite     gitea                  /data/gitea/gitea.db oci-apps         gitea.app:3002
   grist                sqlite     grist_app              /persist/grist-sessions.db oci-apps         grist.app:3011
   hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps         hedgedoc-db.app:5439
   lgtm                 grafana    lgtm_grafana           custom         oci-apps         grafana.app:3200
   lgtm                 loki       lgtm_loki              custom         oci-apps         lgtm-loki.app:3110
   lgtm                 tempo      lgtm_tempo             custom         oci-apps         lgtm-tempo.app:3210
   lgtm                 mimir      lgtm_mimir             custom         oci-apps         lgtm-mimir.app:9009
   mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps         mattermost-postgres.app:5435
   ntfy                 sqlite     ntfy                   /var/cache/ntfy/cache.db oci-apps         ntfy.app:8090
   photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps         embedded
   quant-lab-full       postgres   quant_full_db          custom         oci-apps         quant-full-db.app:5437
   quant-lab-light      postgres   quant_light_db         quantlab       oci-apps         quant-light-db.app:5443
   vaultwarden          sqlite     vaultwarden            /data/db.sqlite3 oci-apps         vaultwarden.app:8880
   windmill             postgres   windmill-db            windmill       oci-apps         windmill-db.app:5440
   stalwart             custom     stalwart               custom         oci-mail         stalwart.app:2443

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
  TOTAL                30.7s
  vm_ssh               22.3s
  mail_dns             19.4s
  databases            16.5s
  public_urls          13.6s
  mesh                 12.1s
  storage              9.0s
  port_scan            6.9s
  private              6.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  30.7s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
