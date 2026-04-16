```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-04-16T18:13:02.759183176+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
46 critical, 5 warnings — 51 total

    ❌ A3       gcp-proxy/sqlite-authelia — exited
    ❌ A3       gcp-proxy/sqlite-ntfy — exited
    ❌ A3       gcp-proxy/sqlite-npm — exited
    ❌ A3       gcp-proxy/sqlite-vaultwarden — exited
    ❌ A3       gcp-proxy/authelia — exited
    ⚠️ A3       gcp-proxy/introspect-proxy — unhealthy
    ❌ A3       oci-apps/quant_light_engine — exited
    ❌ A3       oci-apps/quant_light_research — exited
    ❌ A3       oci-apps/crawlee_api — exited
    ❌ A3       oci-apps/crawlee_minio_init — exited
    ❌ A3       oci-apps/crawlee_scheduler — exited
    ❌ A3       oci-apps/rig-agentic-sonn-14bq8 — exited
    ❌ A3       oci-apps/rig-agentic-hai — exited
    ❌ A3       oci-apps/radicale — exited
    ❌ A3       oci-apps/ollama-hai — exited
    ❌ A3       oci-apps/syslog-bridge — exited
    ❌ A3       oci-apps/github-rss — exited
    ❌ A3       oci-apps/ntfy — exited
    ❌ A3       oci-apps/surrealdb — exited
    ❌ A3       oci-apps/code-server — exited
    ❌ A3       oci-apps/c3-infra-mcp — exited
    ❌ A3       oci-apps/gitea — exited
    ❌ A3       oci-apps/bup-server — exited
    ❌ A3       oci-apps/borg-server — exited
    ❌ A3       oci-apps/syslog-central — exited
    ❌ A3       oci-apps/siem-api — exited
    ❌ A3       oci-apps/revealmd_app — exited
    ❌ A3       oci-apps/photos-db — exited
    ❌ A3       oci-apps/grist_app — exited
    ❌ A3       oci-apps/filebrowser_app — exited
    ❌ A3       oci-apps/cloud-spec — exited
    ❌ A3       oci-apps/cloud-cgc-mcp — exited
    ⚠️ A3       oci-apps/news-gdelt — unhealthy
    ⚠️ A3       oci-apps/google-workspace-mcp — unhealthy
    ⚠️ A3       oci-apps/c3-services-api — unhealthy
    ⚠️ A3       oci-apps/c3-infra-api — unhealthy
    ❌ A3       oci-analytics/umami-setup — exited
    ❌ A3       oci-analytics/siem-api — exited
    ❌ A3       oci-analytics/syslog-central — exited
    ❌ A1       auth.diegonmarcos.com — [502]
    ❌ A1       proxy.diegonmarcos.com — [502]
    ❌ A1       ide.diegonmarcos.com — [502]
    ❌ A1       workflows.diegonmarcos.com — [502]
    ❌ A1       db.diegonmarcos.com — [502]
    ❌ A1       sheets.diegonmarcos.com — [502]
    ❌ A1       dns.internal — [err: error sending request for url (https://dns.internal/)]
    ❌ A1       mail.diegonmarcos.com — [err: error sending request for url (https://mail.diegonmarcos.com/)]
    ❌ A1       analytics.diegonmarcos.com — [502]
    ❌ A1       chat.diegonmarcos.com — [502]
    ❌ A1       rss.diegonmarcos.com — [502]
    ❌ A1       cal.diegonmarcos.com — [502]


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
⚠️ auth.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.1:9091          [502] auth:[502]
✅ git.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3002          [200] 
⚠️ api.diegonmarcos.com/c3-api      ❌  ❌  ✅  ✅  10.0.0.6:8081          [404] 
⚠️ mcp.diegonmarcos.com/c3-infra-mcp ❌  ❌  ✅  ✅  10.0.0.6:3100          [200] 
⚠️ api.diegonmarcos.com/services    ❌  ❌  ✅  ✅  10.0.0.6:8082          [404] 
⚠️ proxy.diegonmarcos.com           ✅  ❌  ❌  ❌  10.0.0.1:443           [502] auth:[502]
⚠️ ide.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:8443          [502] auth:[502]
✅ api.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3000          [200] 
⚠️ workflows.diegonmarcos.com       ✅  ❌  ❌  ❌  10.0.0.4:8070          [502] auth:[502]
⚠️ db.diegonmarcos.com              ✅  ❌  ❌  ❌  10.0.0.6:8086          [502] auth:[502]
✅ logs.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.4:9999          [200] 
✅ pad.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3012          [200] 
✅ files.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:3015          [200] 
⚠️ sheets.diegonmarcos.com          ✅  ❌  ❌  ❌  10.0.0.6:3011          [502] auth:[502]
✅ doc.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3018          [200] 
❌ dns.internal                     ❌  ❌  ❌  ❌  10.0.0.1:53            [err: error sending request for url (https://dns.internal/)] auth:[err: error sending request for url (https://dns.internal/)]
⚠️ grafana.diegonmarcos.com         ❌  ❌  ✅  ✅  10.0.0.6:3200          [200] 
⚠️ mail.diegonmarcos.com            ✅  ❌  ❌  ✅  10.0.0.3:443           [err: error sending request for url (https://mail.diegonmarcos.com/)] 
⚠️ analytics.diegonmarcos.com       ✅  ❌  ❌  ❌  10.0.0.4:8084          [502] auth:[502]
⚠️ chat.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.6:8065          [502] auth:[502]
⚠️ rss.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:8090          [502] auth:[502]
✅ photos.diegonmarcos.com          ✅  ❌  ✅  ✅  10.0.0.6:3013          [200] 
⚠️ cal.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:5232          [502] auth:[502]
✅ smtp.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.3:8080          [405] 
✅ webmail.diegonmarcos.com         ✅  ❌  ✅  ✅  10.0.0.3:8888          [301] 
✅ vault.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:8880          [200] 
✅ windmill.diegonmarcos.com        ✅  ❌  ✅  ✅  10.0.0.6:8000          [200] 
⚠️ api.diegonmarcos.com/dash        ❌  ❌  ✅  ✅  diegonmarcos.github.io [301] 
⚠️ api.diegonmarcos.com/crawlee     ❌  ❌  ✅  ✅  10.0.0.6:3000          [404] 
⚠️ app.diegonmarcos.com/windmill    ❌  ❌  ✅  ✅  10.0.0.6:8000          [404] 
⚠️ app.diegonmarcos.com/etherpad    ❌  ❌  ✅  ✅  10.0.0.6:3012          [404] 
⚠️ app.diegonmarcos.com/filebrowser ❌  ❌  ✅  ✅  10.0.0.6:3015          [404] 
⚠️ app.diegonmarcos.com/hedgedoc    ❌  ❌  ✅  ✅  10.0.0.6:3018          [404] 
⚠️ app.diegonmarcos.com/revealmd    ❌  ❌  ✅  ✅  10.0.0.6:3014          [404] 
⚠️ app.diegonmarcos.com/dozzle      ❌  ❌  ✅  ✅  10.0.0.4:9999          [404] 
⚠️ app.diegonmarcos.com/grafana     ❌  ❌  ✅  ✅  10.0.0.6:3016          [404] 
⚠️ app.diegonmarcos.com/gitea       ❌  ❌  ✅  ✅  10.0.0.6:3017          [404] 
⚠️ app.diegonmarcos.com/crawlee     ❌  ❌  ✅  ✅  10.0.0.6:3001          [404] 
✅ cloud.diegonmarcos.com           ✅  ❌  ✅  ✅  10.0.0.6:3080          [200] 
✅ mcp.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3100          [200] 
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
❌ authelia-redis.app           ❌   ❌    6380 gcp-proxy        authelia-redis         [err: error sending request for url (http://10.0.0.1:6380/)]
❌ authelia.app                 ❌   ❌    9091 gcp-proxy        authelia               [err: error sending request for url (http://10.0.0.1:9091/)]
✅ caddy.app                    ✅   ✅     443 gcp-proxy        caddy                  [400]
⚠️ hickory-dns.app              ✅   ❌      53 gcp-proxy        hickory-dns            [err: error sending request for url (http://10.0.0.1:53/)]
✅ introspect-proxy.app         ✅   ✅    4182 gcp-proxy        introspect-proxy       [404]
⚠️ redis.app                    ✅   ❌    6379 gcp-proxy        redis                  [err: error sending request for url (http://10.0.0.1:6379/)]
❌ ollama.app                   ❌   ❌   11434 gcp-t4           ollama                 [err: error sending request for url (http://10.0.0.1:11434/)]
❌ dagu.app                     ❌   ❌    8070 oci-analytics    dagu                   [err: error sending request for url (http://10.0.0.1:8070/)]
❌ dozzle.app                   ❌   ❌    9999 oci-analytics    dozzle                 [err: error sending request for url (http://10.0.0.1:9999/)]
❌ matomo.app                   ❌   ❌    8084 oci-analytics    matomo-hybrid          [err: error sending request for url (http://10.0.0.1:8084/)]
❌ umami-db.app                 ❌   ❌    5442 oci-analytics    umami-db               [err: error sending request for url (http://10.0.0.1:5442/)]
❌ umami.app                    ❌   ❌    3006 oci-analytics    umami                  [err: error sending request for url (http://10.0.0.1:3006/)]
❌ backup-gitea.app             ❌   ❌    3002 oci-apps         gitea                  [err: error sending request for url (http://10.0.0.1:3002/)]
❌ c3-infra-api.app             ❌   ❌    8081 oci-apps         c3-infra-api           [err: error sending request for url (http://10.0.0.1:8081/)]
❌ c3-infra-mcp.app             ❌   ❌    3100 oci-apps         c3-infra-mcp           [err: error sending request for url (http://10.0.0.1:3100/)]
❌ c3-services-api.app          ❌   ❌    8082 oci-apps         c3-services-api        [err: error sending request for url (http://10.0.0.1:8082/)]
❌ c3-services-mcp.app          ❌   ❌    3101 oci-apps         c3-services-mcp        [err: error sending request for url (http://10.0.0.1:3101/)]
❌ c3-spec.app                  ❌   ❌    3080 oci-apps         cloud-spec             [err: error sending request for url (http://10.0.0.1:3080/)]
❌ cloud-cgc-mcp.app            ❌   ❌    3105 oci-apps         cloud-cgc-mcp          [err: error sending request for url (http://10.0.0.1:3105/)]
❌ code-server.app              ❌   ❌    8443 oci-apps         code-server            [err: error sending request for url (http://10.0.0.1:8443/)]
❌ crawlee-dashboard.app        ❌   ❌    3001 oci-apps         crawlee_dashboard      [err: error sending request for url (http://10.0.0.1:3001/)]
⚠️ crawlee-db.app               ✅   ❌    5433 oci-apps         crawlee_db             [err: error sending request for url (http://10.0.0.1:5433/)]
❌ crawlee-minio.app            ❌   ❌    9000 oci-apps         crawlee_minio          [err: error sending request for url (http://10.0.0.1:9000/)]
❌ crawlee-redis.app            ❌   ❌    6381 oci-apps         crawlee_redis          [err: error sending request for url (http://10.0.0.1:6381/)]
❌ crawlee.app                  ❌   ❌    3000 oci-apps         crawlee_api            [err: error sending request for url (http://10.0.0.1:3000/)]
❌ dbgate.app                   ❌   ❌    8086 oci-apps         dbgate                 [err: error sending request for url (http://10.0.0.1:8086/)]
⚠️ etherpad-db.app              ✅   ❌    5436 oci-apps         etherpad_postgres      [err: error sending request for url (http://10.0.0.1:5436/)]
❌ etherpad.app                 ❌   ❌    3012 oci-apps         etherpad_app           [err: error sending request for url (http://10.0.0.1:3012/)]
❌ filebrowser.app              ❌   ❌    3015 oci-apps         filebrowser_app        [err: error sending request for url (http://10.0.0.1:3015/)]
❌ g-workspace-mcp.app          ❌   ❌    3104 oci-apps         google-workspace-mcp   [err: error sending request for url (http://10.0.0.1:3104/)]
❌ gitea.app                    ❌   ❌    3002 oci-apps         gitea                  [err: error sending request for url (http://10.0.0.1:3002/)]
❌ grafana.app                  ❌   ❌    3200 oci-apps         lgtm_grafana           [err: error sending request for url (http://10.0.0.1:3200/)]
❌ grist.app                    ❌   ❌    3011 oci-apps         grist_app              [err: error sending request for url (http://10.0.0.1:3011/)]
❌ hedgedoc-db.app              ❌   ❌    5439 oci-apps         hedgedoc_postgres      [err: error sending request for url (http://10.0.0.1:5439/)]
❌ hedgedoc.app                 ❌   ❌    3018 oci-apps         hedgedoc_app           [err: error sending request for url (http://10.0.0.1:3018/)]
❌ lgtm-loki.app                ❌   ❌    3110 oci-apps         lgtm_loki              [err: error sending request for url (http://10.0.0.1:3110/)]
❌ lgtm-mimir.app               ❌   ❌    9009 oci-apps         lgtm_mimir             [err: error sending request for url (http://10.0.0.1:9009/)]
❌ lgtm-tempo.app               ❌   ❌    3210 oci-apps         lgtm_tempo             [err: error sending request for url (http://10.0.0.1:3210/)]
❌ mail-mcp.app                 ❌   ❌    3103 oci-apps         mail-mcp               [err: error sending request for url (http://10.0.0.1:3103/)]
❌ mattermost-mcp.app           ❌   ❌    3102 oci-apps         mattermost-mcp         [err: error sending request for url (http://10.0.0.1:3102/)]
⚠️ mattermost-postgres.app      ✅   ❌    5435 oci-apps         mattermost-postgres    [err: error sending request for url (http://10.0.0.1:5435/)]
❌ mattermost.app               ❌   ❌    8065 oci-apps         mattermost             [err: error sending request for url (http://10.0.0.1:8065/)]
✅ ntfy.app                     ✅   ✅    8090 oci-apps         ntfy                   [200]
❌ ollama-hai.app               ❌   ❌   11435 oci-apps         ollama-hai             [err: error sending request for url (http://10.0.0.1:11435/)]
❌ photoprism.app               ❌   ❌    3013 oci-apps         photoprism_app         [err: error sending request for url (http://10.0.0.1:3013/)]
❌ quant-full-db.app            ❌   ❌    5437 oci-apps         quant_full_db          [err: error sending request for url (http://10.0.0.1:5437/)]
❌ quant-full-research.app      ❌   ❌    8890 oci-apps         quant_full_research    [err: error sending request for url (http://10.0.0.1:8890/)]
❌ quant-light-db.app           ❌   ❌    5443 oci-apps         quant_light_db         [err: error sending request for url (http://10.0.0.1:5443/)]
❌ quant-light-engine.app       ❌   ❌    5001 oci-apps         quant_light_engine     [err: error sending request for url (http://10.0.0.1:5001/)]
❌ quant-light-research.app     ❌   ❌    8889 oci-apps         quant_light_research   [err: error sending request for url (http://10.0.0.1:8889/)]
❌ radicale.app                 ❌   ❌    5232 oci-apps         radicale               [err: error sending request for url (http://10.0.0.1:5232/)]
❌ revealmd.app                 ❌   ❌    3014 oci-apps         revealmd_app           [err: error sending request for url (http://10.0.0.1:3014/)]
❌ rig-agentic-sonn-14bq8.app   ❌   ❌    8091 oci-apps         rig-agentic-sonn-14bq8 [err: error sending request for url (http://10.0.0.1:8091/)]
✅ vaultwarden.app              ✅   ✅    8880 oci-apps         vaultwarden            [200]
❌ windmill-app.app             ❌   ❌    8000 oci-apps         windmill-server        [err: error sending request for url (http://10.0.0.1:8000/)]
❌ windmill-db.app              ❌   ❌    5440 oci-apps         windmill-db            [err: error sending request for url (http://10.0.0.1:5440/)]
✅ maddy.app                    ✅   ✅     443 oci-mail         maddy                  [400]
❌ smtp-proxy.app               ❌   ❌    8080 oci-mail         smtp-proxy             [err: error sending request for url (http://10.0.0.1:8080/)]
❌ snappymail.app               ❌   ❌    8888 oci-mail         snappymail             [err: error sending request for url (http://10.0.0.1:8888/)]

  📡 TCP: 10/59  🌐 HTTP: 5/59

── A3) Containers ────────────────────────────────────────────

gcp-proxy ✅ — 2C/2G — mem 1020M/1952M (52%) — disk 64% — swap 53M/3999M — load 0.08 0.06 0.08 — 13/18 ctrs — 15d 20h
────────────────────────────────────────────────────────────
  ❌ sqlite-authelia           —      —      DOWN(2)        Exited (2) 27 minutes ago
  ❌ sqlite-ntfy               —      —      DOWN(2)        Exited (2) 27 minutes ago
  ❌ sqlite-npm                —      —      DOWN(2)        Exited (2) 27 minutes ago
  ❌ sqlite-vaultwarden        —      —      DOWN(2)        Exited (2) 27 minutes ago
  ❌ authelia                  —      9091   DOWN(1)        Exited (1) 27 minutes ago
  ❌ introspect-proxy          —      4182   UNHEALTHY      Up 26 minutes (unhealthy) cpu=
  ⚠️ postlite-vaultwarden      —      —      running        Up 27 minutes cpu=0.00% mem=10
  ⚠️ postlite-authelia         —      —      running        Up 27 minutes cpu=0.00% mem=1.
  ⚠️ postlite-ntfy             —      —      running        Up 27 minutes cpu=0.00% mem=5.
  ⚠️ postlite-npm              —      —      running        Up 27 minutes cpu=0.00% mem=3.
  ⚠️ syslog-bridge             —      —      running        Up 27 minutes cpu=0.00% mem=16
  ⚠️ github-rss                —      —      running        Up 27 minutes cpu=0.00% mem=23
  ⚠️ ntfy                      —      —      running        Up 27 minutes cpu=0.00% mem=17
  ⚠️ hickory-dns               —      53     running        Up 27 minutes cpu=0.33% mem=13
  ⚠️ caddy                     443    443    running        Up 27 minutes cpu=0.00% mem=70
  ⚠️ authelia-redis            —      6380   running        Up 27 minutes cpu=0.27% mem=4.
  ✅ vaultwarden               —      —      HEALTHY        Up 27 minutes (healthy) cpu=0.
  ✅ redis                     —      6379   HEALTHY        Up 27 minutes (healthy) cpu=0.

oci-apps ✅ — 4C/24G — mem 3986M/23975M (16%) — disk 70% — swap 133M/12288M — load 0.11 0.15 0.14 — 30/59 ctrs — 18d 4h
────────────────────────────────────────────────────────────
  ❌ crawlee_runner            —      —      DOWN(?)        Created
  ❌ crawlee_dashboard         3001   3001   DOWN(?)        Created
  ❌ photos-webhook            —      —      DOWN(?)        Created
  ❌ quant_light_engine        —      5001   DOWN(255)      Exited (255) 4 hours ago
  ❌ quant_light_research      —      8889   DOWN(255)      Exited (255) 4 hours ago
  ❌ crawlee_api               3000   3000   DOWN(255)      Exited (255) 18 minutes ago
  ❌ crawlee_minio_init        —      —      DOWN(255)      Exited (255) 18 minutes ago
  ❌ crawlee_scheduler         —      —      DOWN(255)      Exited (255) 18 minutes ago
  ❌ rig-agentic-sonn-14bq8    —      8091   DOWN(255)      Exited (255) 4 hours ago
  ❌ rig-agentic-hai           —      —      DOWN(255)      Exited (255) 6 hours ago
  ❌ radicale                  —      5232   DOWN(255)      Exited (255) 18 minutes ago
  ❌ ollama-hai                —      11435  DOWN(255)      Exited (255) 39 minutes ago
  ❌ syslog-bridge             —      —      DOWN(255)      Exited (255) 18 minutes ago
  ❌ github-rss                —      —      DOWN(255)      Exited (255) 18 minutes ago
  ❌ ntfy                      —      8090   DOWN(255)      Exited (255) 18 minutes ago
  ❌ surrealdb                 —      —      DOWN(255)      Exited (255) 6 hours ago
  ❌ code-server               —      8443   DOWN(255)      Exited (255) 18 minutes ago
  ❌ c3-infra-mcp              —      3100   DOWN(1)        Exited (1) 6 hours ago
  ❌ gitea                     —      3002   DOWN(255)      Exited (255) 6 hours ago
  ❌ bup-server                —      —      DOWN(255)      Exited (255) 6 hours ago
  ❌ borg-server               —      —      DOWN(255)      Exited (255) 6 hours ago
  ❌ syslog-central            —      —      DOWN(255)      Exited (255) 39 minutes ago
  ❌ siem-api                  —      —      DOWN(255)      Exited (255) 39 minutes ago
  ❌ revealmd_app              —      3014   DOWN(255)      Exited (255) 39 minutes ago
  ❌ photos-db                 —      —      DOWN(255)      Exited (255) 39 minutes ago
  ❌ grist_app                 —      3011   DOWN(255)      Exited (255) 39 minutes ago
  ❌ filebrowser_app           —      3015   DOWN(255)      Exited (255) 39 minutes ago
  ❌ cloud-spec                —      3080   DOWN(255)      Exited (255) 39 minutes ago
  ❌ cloud-cgc-mcp             —      3105   DOWN(255)      Exited (255) 39 minutes ago
  ❌ news-gdelt                —      —      UNHEALTHY      Up 39 minutes (unhealthy) cpu=
  ❌ google-workspace-mcp      —      3104   UNHEALTHY      Up 4 hours (unhealthy) cpu=1.9
  ❌ c3-services-api           —      8082   UNHEALTHY      Up 4 hours (unhealthy) cpu=0.0
  ❌ c3-infra-api              8081   8081   UNHEALTHY      Up 4 hours (unhealthy) cpu=0.1
  ⚠️ mattermost-bots           —      —      running        Up 4 hours cpu=0.01% mem=54.76
  ⚠️ windmill-worker           —      —      running        Up 4 hours cpu=0.77% mem=26.21
  ⚠️ mail-mcp                  —      3103   running        Up 5 hours cpu=0.14% mem=133.4
  ⚠️ mattermost-mcp            —      3102   running        Up 5 hours cpu=0.04% mem=101Mi
  ⚠️ lgtm_mimir                —      9009   running        Up 31 hours cpu=0.48% mem=32.4
  ⚠️ lgtm_tempo                —      3210   running        Up 31 hours cpu=0.07% mem=21.7
  ✅ quant_light_db            —      5443   HEALTHY        Up 4 hours (healthy) cpu=1.79%
  ✅ vaultwarden               —      8880   HEALTHY        Up 4 hours (healthy) cpu=0.00%
  ✅ photoprism_app            —      3013   HEALTHY        Up 4 hours (healthy) cpu=0.00%
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 4 hours (healthy) cpu=0.01%
  ✅ photoprism_rclone         —      —      HEALTHY        Up 4 hours (healthy) cpu=0.01%
  ✅ crawlee_db                —      5433   HEALTHY        Up 4 hours (healthy) cpu=0.00%
  ✅ crawlee_minio             —      9000   HEALTHY        Up 4 hours (healthy) cpu=1.60%
  ✅ crawlee_redis             —      6381   HEALTHY        Up 4 hours (healthy) cpu=1.45%
  ✅ mattermost                —      8065   HEALTHY        Up 4 hours (healthy) cpu=0.08%
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 4 hours (healthy) cpu=0.01%
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 4 hours (healthy) cpu=0.25%
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up 4 hours (healthy) cpu=0.00%
  ✅ etherpad_app              —      3012   HEALTHY        Up 4 hours (healthy) cpu=0.33%
  ✅ etherpad_postgres         —      5436   HEALTHY        Up 4 hours (healthy) cpu=0.00%
  ✅ windmill-server           —      8000   HEALTHY        Up 4 hours (healthy) cpu=0.08%
  ✅ windmill-db               —      5440   HEALTHY        Up 4 hours (healthy) cpu=1.25%
  ✅ c3-services-mcp           —      3101   HEALTHY        Up 4 hours (healthy) cpu=0.08%
  ✅ dbgate                    —      8086   HEALTHY        Up 6 hours (healthy) cpu=1.56%
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 31 hours (healthy) cpu=0.34
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 31 hours (healthy) cpu=0.64

oci-mail ✅ — 1C/1G — mem 632M/954M (66%) — disk 78% — swap 190M/2559M — load 0.63 0.31 0.22 — 4/4 ctrs — 16d 0h
────────────────────────────────────────────────────────────
  ⚠️ smtp-proxy                8080   8080   running        Up 4 hours cpu=0.00% mem=2.414
  ⚠️ maddy                     —      443    running        Up 7 hours cpu=0.02% mem=9.898
  ⚠️ dagu                      —      —      running        Up 12 days cpu=0.01% mem=13.38
  ✅ snappymail                —      8888   HEALTHY        Up 6 hours (healthy) cpu=0.02%

oci-analytics ✅ — 1C/1G — mem 663M/954M (69%) — disk 51% — swap 264M/1535M — load 0.33 0.72 0.77 — 6/10 ctrs — 9d 6h
────────────────────────────────────────────────────────────
  ❌ fluent-bit                —      —      DOWN(?)        Created
  ❌ umami-setup               —      —      DOWN(1)        Exited (1) 5 hours ago
  ❌ siem-api                  —      —      DOWN(1)        Exited (1) 6 hours ago
  ❌ syslog-central            —      —      DOWN(2)        Exited (2) 6 hours ago
  ⚠️ matomo-hybrid             —      8084   running        Up 5 hours cpu=0.02% mem=3.938
  ⚠️ dagu                      —      8070   running        Up 6 hours cpu=0.07% mem=61.18
  ⚠️ sauron-forwarder          —      —      running        Up 37 hours cpu=0.00% mem=748K
  ⚠️ dozzle                    —      9999   running        Up 37 hours cpu=0.05% mem=5.24
  ✅ umami                     —      3006   HEALTHY        Up 5 hours (healthy) cpu=0.00%
  ✅ umami-db                  —      5442   HEALTHY        Up 5 hours (healthy) cpu=0.00%


── A4) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
(TODO)

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  162.159.205.13
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.18
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.25
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
     ✅ smtp-proxy           Up 4 hours cpu=0.00% mem=2.414MiB / 954.2MiB
     ✅ maddy                Up 7 hours cpu=0.02% mem=9.898MiB / 256MiB

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
CPU                52 cores      16 cores      66 cores      69 cores      
RAM                1020M/1952M   3986M/23975M  632M/954M     663M/954M     
RAM %              52%           16%           66%           69%           
Swap               53M/3999M     133M/12288M   190M/2559M    264M/1535M    
Disk               20G/31G       63.6G/95.8G   33G/45G       23G/48G       
Disk %             64%           70%           78%           51%           
Load               0.08 0.06 0.080.11 0.15 0.140.63 0.31 0.220.33 0.72 0.77
Containers         13/18         30/59         4/4           6/10          
Uptime             15d 20h       18d 4h        16d 0h        9d 6h         

STORAGE
────────────────────────────────────────────────────────────
  OBJECT STORAGE
    oci — archlinux-images (Standard)
    oci — my-photos (Standard)

  DATABASES
    authelia             ?          authelia               gcp-proxy
    umami                postgres   umami-db               oci-analytics
    crawlee-cloud        postgres   crawlee_db             oci-apps
    etherpad             postgres   etherpad_postgres      oci-apps
    gitea                ?          gitea                  oci-apps
    grist                ?          grist_app              oci-apps
    hedgedoc             postgres   hedgedoc_postgres      oci-apps
    mattermost-bots      postgres   mattermost-postgres    oci-apps
    ntfy                 ?          ntfy                   oci-apps
    photoprism           mariadb    photoprism_mariadb     oci-apps
    quant-lab-light      postgres   quant_light_db         oci-apps
    vaultwarden          ?          vaultwarden            oci-apps
    windmill             postgres   windmill-db            oci-apps


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

NETWORK SECURITY AUDIT
────────────────────────────────────────────────────────────
    Check                         gcp-proxy           oci-mail      oci-analytics           oci-apps
    ────────────────────────────────────────────────────────────────────────────────────────────────
    Declared ports       80,443,465,587,993 25,465,587,993,4190,8080,8443,21027,22000               none 2222,2223,2224,3000,3001,3010,8081,8099
    Scanned (public)     22,443,465,587,993             🔒 none             🔒 none             🔒 none
    Docker host ports                   443               8080               none     3000,3001,8081
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                       ✅ up               ✅ up               ✅ up               ✅ up
    Containers (up/total)              13/18                4/4               6/10              30/59

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔓 gcp-proxy          35.226.147.64      ports: 22, 443, 465, 587, 993
🔒 gcp-t4             34.173.227.250     ports: none reachable
🔒 oci-apps           82.70.229.129      ports: none reachable
🔒 oci-mail           130.110.251.193    ports: none reachable
🔒 oci-analytics      129.151.228.66     ports: none reachable

BACKUPS / DATABASES
────────────────────────────────────────────────────────────
   authelia             ?          authelia               /config/db.sqlite3 gcp-proxy        authelia.app:9091
   umami                postgres   umami-db               umami          oci-analytics    umami-db.app:5442
   crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps         crawlee-db.app:5433
   etherpad             postgres   etherpad_postgres      etherpad       oci-apps         etherpad-db.app:5436
   gitea                ?          gitea                  /data/gitea/gitea.db oci-apps         backup-gitea.app:3002
   grist                ?          grist_app              /persist/grist-sessions.db oci-apps         grist.app:3011
   hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps         hedgedoc-db.app:5439
   mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps         mattermost-postgres.app:5435
   ntfy                 ?          ntfy                   /var/cache/ntfy/cache.db oci-apps         ntfy.app:8090
   photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps         embedded
   quant-lab-light      postgres   quant_light_db         quantlab       oci-apps         quant-light-db.app:5443
   vaultwarden          ?          vaultwarden            /data/db.sqlite3 oci-apps         vaultwarden.app:8880
   windmill             postgres   windmill-db            windmill       oci-apps         windmill-db.app:5440

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
  TOTAL                12.7s
  mail_dns             12.5s
  vm_ssh               11.3s
  public_urls          5.7s
  private              5.5s
  port_scan            3.0s
  mesh                 3.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  12.7s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
