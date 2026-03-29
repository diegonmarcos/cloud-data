```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-03-29T03:30:40.257066363+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
47 critical, 1 warnings — 48 total

    ❌ A3       VM gcp-t4 — UNREACHABLE
    ❌ A3       oci-apps/surrealdb — exited
    ❌ A3       oci-apps/crawlee_runner — exited
    ❌ A3       oci-apps/crawlee_api — exited
    ❌ A3       oci-apps/crawlee_minio_init — exited
    ⚠️ A3       oci-apps/crawlee_db — unhealthy
    ❌ A3       oci-mail/introspect-proxy — exited
    ❌ A3       oci-analytics/umami-setup — exited
    ❌ A1       auth.diegonmarcos.com — [---]
    ❌ A1       git.diegonmarcos.com — [---]
    ❌ A1       api.diegonmarcos.com/c3-api — [---]
    ❌ A1       mcp.diegonmarcos.com/c3-infra-mcp — [---]
    ❌ A1       api.diegonmarcos.com/services — [---]
    ❌ A1       proxy.diegonmarcos.com — [---]
    ❌ A1       ide.diegonmarcos.com — [---]
    ❌ A1       api.diegonmarcos.com — [---]
    ❌ A1       workflows.diegonmarcos.com — [---]
    ❌ A1       logs.diegonmarcos.com — [---]
    ❌ A1       pad.diegonmarcos.com — [---]
    ❌ A1       files.diegonmarcos.com — [---]
    ❌ A1       sheets.diegonmarcos.com — [---]
    ❌ A1       doc.diegonmarcos.com — [---]
    ❌ A1       dns.internal — [---]
    ❌ A1       grafana.diegonmarcos.com — [---]
    ❌ A1       analytics.diegonmarcos.com — [---]
    ❌ A1       chat.diegonmarcos.com — [---]
    ❌ A1       db.diegonmarcos.com — [---]
    ❌ A1       rss.diegonmarcos.com — [---]
    ❌ A1       photos.diegonmarcos.com — [---]
    ❌ A1       cal.diegonmarcos.com — [---]
    ❌ A1       slides.diegonmarcos.com — [---]
    ❌ A1       webmail.diegonmarcos.com — [---]
    ❌ A1       mail.diegonmarcos.com — [---]
    ❌ A1       vault.diegonmarcos.com — [---]
    ❌ A1       windmill.diegonmarcos.com — [---]
    ❌ A1       api.diegonmarcos.com/dash — [---]
    ❌ A1       api.diegonmarcos.com/crawlee — [---]
    ❌ A1       app.diegonmarcos.com/windmill — [---]
    ❌ A1       app.diegonmarcos.com/etherpad — [---]
    ❌ A1       app.diegonmarcos.com/filebrowser — [---]
    ❌ A1       app.diegonmarcos.com/hedgedoc — [---]
    ❌ A1       app.diegonmarcos.com/revealmd — [---]
    ❌ A1       app.diegonmarcos.com/dozzle — [---]
    ❌ A1       app.diegonmarcos.com/grafana — [---]
    ❌ A1       app.diegonmarcos.com/gitea — [---]
    ❌ A1       app.diegonmarcos.com/crawlee — [---]
    ❌ A1       cloud.diegonmarcos.com — [---]
    ❌ A1       mcp.diegonmarcos.com — [---]


══════════════════════════════════════════════════════════════
  A) HEALTH — Live checks
══════════════════════════════════════════════════════════════

── A0) Mesh ──────────────────────────────────────────────────

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1 — front door)
────────────────────────────────────────────────────────────
    Name           Cloud Name         ☁VPS 🌐Pub 🔧DB  🔒WG  Public IP          WG IP          Type     Handshake
────────────────────────────────────────────────────────────
⚠️ oci-mail       oci-E2-f_0         ✅  ✅  ❌  ❌  130.110.251.193    10.0.0.3       VM       no data
⚠️ oci-analytics  oci-E2-f_1         ✅  ✅  ❌  ❌  129.151.228.66     10.0.0.4       VM       no data
⚠️ oci-apps       oci-A1-f_0         ✅  ✅  ❌  ❌  82.70.229.129      10.0.0.6       VM       no data
❌ gcp-t4         ollama-spot-gpu    ❌  ❌  ❌  ❌  34.173.227.250     10.0.0.8       VM       no data
⚠️ gcp-proxy      arch-1             ✅  ✅  ✅  ❌  35.226.147.64      10.0.0.1       HUB      no data
⚠️ surface        —                  ✅  ✅  —  ❌  dynamic            10.0.0.5       CLIENT   no data
⚠️ termux         —                  ✅  ✅  —  ❌  dynamic            10.0.0.9       CLIENT   no data

── A1) Public ────────────────────────────────────────────────

PUBLIC URLs (Caddy routes)
────────────────────────────────────────────────────────────
    URL                              📡TCP 🌐HTTP 🔒HTTPS 🔐AUTH Upstream                  Code
────────────────────────────────────────────────────────────
❌ auth.diegonmarcos.com            ❌  ❌  ❌  —  authelia.app:9091      [---] 
❌ git.diegonmarcos.com             ❌  ❌  ❌  —  backup-gitea.app:3002  [---] 
❌ api.diegonmarcos.com/c3-api      ❌  ❌  ❌  —  c3-infra-api.app:8081  [---] 
❌ mcp.diegonmarcos.com/c3-infra-mcp ❌  ❌  ❌  —  c3-infra-mcp.app:3100  [---] 
❌ api.diegonmarcos.com/services    ❌  ❌  ❌  —  c3-services-api.app:8082 [---] 
❌ proxy.diegonmarcos.com           ❌  ❌  ❌  —  caddy.app:443          [---] 
❌ ide.diegonmarcos.com             ❌  ❌  ❌  —  code-server.app:8443   [---] 
❌ api.diegonmarcos.com             ❌  ❌  ❌  —  crawlee.app:3000       [---] 
❌ workflows.diegonmarcos.com       ❌  ❌  ❌  —  dagu.app:8070          [---] 
❌ logs.diegonmarcos.com            ❌  ❌  ❌  —  dozzle.app:9999        [---] 
❌ pad.diegonmarcos.com             ❌  ❌  ❌  —  etherpad.app:3012      [---] 
❌ files.diegonmarcos.com           ❌  ❌  ❌  —  filebrowser.app:3015   [---] 
❌ sheets.diegonmarcos.com          ❌  ❌  ❌  —  grist.app:3011         [---] 
❌ doc.diegonmarcos.com             ❌  ❌  ❌  —  hedgedoc.app:3018      [---] 
❌ dns.internal                     ❌  ❌  ❌  —  hickory-dns.app:53     [---] 
❌ grafana.diegonmarcos.com         ❌  ❌  ❌  —  grafana.app:3200       [---] 
❌ analytics.diegonmarcos.com       ❌  ❌  ❌  —  matomo.app:8080        [---] 
❌ chat.diegonmarcos.com            ❌  ❌  ❌  —  mattermost.app:8065    [---] 
❌ db.diegonmarcos.com              ❌  ❌  ❌  —  nocodb.app:8085        [---] 
❌ rss.diegonmarcos.com             ❌  ❌  ❌  —  ntfy.app:8090          [---] 
❌ photos.diegonmarcos.com          ❌  ❌  ❌  —  photoprism.app:3013    [---] 
❌ cal.diegonmarcos.com             ❌  ❌  ❌  —  radicale.app:5232      [---] 
❌ slides.diegonmarcos.com          ❌  ❌  ❌  —  revealmd.app:3014      [---] 
❌ webmail.diegonmarcos.com         ❌  ❌  ❌  —  snappymail.app:8888    [---] 
❌ mail.diegonmarcos.com            ❌  ❌  ❌  —  stalwart.app:443       [---] 
❌ vault.diegonmarcos.com           ❌  ❌  ❌  —  vaultwarden.app:8880   [---] 
❌ windmill.diegonmarcos.com        ❌  ❌  ❌  —  windmill-app.app:8000  [---] 
❌ api.diegonmarcos.com/dash        ❌  ❌  ❌  —  diegonmarcos.github.io [---] 
❌ api.diegonmarcos.com/crawlee     ❌  ❌  ❌  —  crawlee.app:3000       [---] 
❌ app.diegonmarcos.com/windmill    ❌  ❌  ❌  —  windmill-app.app:8000  [---] 
❌ app.diegonmarcos.com/etherpad    ❌  ❌  ❌  —  etherpad.app:3012      [---] 
❌ app.diegonmarcos.com/filebrowser ❌  ❌  ❌  —  filebrowser.app:3015   [---] 
❌ app.diegonmarcos.com/hedgedoc    ❌  ❌  ❌  —  hedgedoc.app:3018      [---] 
❌ app.diegonmarcos.com/revealmd    ❌  ❌  ❌  —  revealmd.app:3014      [---] 
❌ app.diegonmarcos.com/dozzle      ❌  ❌  ❌  —  dozzle.app:9999        [---] 
❌ app.diegonmarcos.com/grafana     ❌  ❌  ❌  —  grafana.app:3016       [---] 
❌ app.diegonmarcos.com/gitea       ❌  ❌  ❌  —  gitea.app:3017         [---] 
❌ app.diegonmarcos.com/crawlee     ❌  ❌  ❌  —  crawlee.app:3001       [---] 
❌ cloud.diegonmarcos.com           ❌  ❌  ❌  —  c3-spec.app:3080       [---] 
❌ mcp.diegonmarcos.com             ❌  ❌  ❌  —  c3-infra-mcp.app:3100  [---] 

API / MCP ENDPOINTS
────────────────────────────────────────────────────────────


REPOS & REGISTRIES
────────────────────────────────────────────────────────────
(Rust: TODO)

── A2) Private (WireGuard mesh — .app health) ────────────────

    DNS Name                     📡TCP 🌐HTTP Port    VM               Container              Code
    ───────────────────────────────────────────────────────────────────────────────────────────────
⚠️ authelia-redis.app           ✅   ❌    6380 gcp-proxy        authelia-redis         [---]
⚠️ authelia.app                 ✅   ❌    9091 gcp-proxy        authelia               [---]
❌ caddy.app                    ❌   ❌     443 gcp-proxy        caddy                  [---]
⚠️ hickory-dns.app              ✅   ❌      53 gcp-proxy        hickory-dns            [---]
✅ introspect-proxy.app         ✅   ✅    4182 gcp-proxy        introspect-proxy       [404]
✅ introspect-proxy.app         ✅   ✅    4182 gcp-proxy        introspect-proxy       [404]
✅ ntfy.app                     ✅   ✅    8090 gcp-proxy        ntfy                   [200]
⚠️ redis.app                    ✅   ❌    6379 gcp-proxy        redis                  [---]
✅ vaultwarden.app              ✅   ✅    8880 gcp-proxy        vaultwarden            [200]
❌ ollama.app                   ❌   ❌   11434 gcp-t4           ollama                 [---]
✅ dozzle.app                   ✅   ✅    9999 oci-analytics    dozzle                 [200]
✅ matomo.app                   ✅   ✅    8080 oci-analytics    matomo-hybrid          [503]
⚠️ umami-db.app                 ✅   ❌    5442 oci-analytics    umami-db               [---]
✅ umami.app                    ✅   ✅    3006 oci-analytics    umami                  [200]
❌ backup-gitea.app             ❌   ❌    3002 oci-apps         gitea                  [---]
❌ c3-infra-api.app             ❌   ❌    8081 oci-apps         c3-infra-api           [---]
❌ c3-infra-mcp.app             ❌   ❌    3100 oci-apps         c3-infra-mcp           [---]
❌ c3-services-api.app          ❌   ❌    8082 oci-apps         c3-services-api        [---]
❌ c3-services-mcp.app          ❌   ❌    3101 oci-apps         c3-services-mcp        [---]
❌ c3-spec.app                  ❌   ❌    3080 oci-apps         cloud-spec             [---]
❌ cloud-cgc-mcp.app            ❌   ❌    3105 oci-apps         cloud-cgc-mcp          [---]
❌ code-server.app              ❌   ❌    8443 oci-apps         code-server            [---]
❌ crawlee-dashboard.app        ❌   ❌    3001 oci-apps         crawlee_dashboard      [---]
❌ crawlee-db.app               ❌   ❌    5433 oci-apps         crawlee_db             [---]
❌ crawlee-minio.app            ❌   ❌    9000 oci-apps         crawlee_minio          [---]
❌ crawlee-redis.app            ❌   ❌    6381 oci-apps         crawlee_redis          [---]
❌ crawlee.app                  ❌   ❌    3000 oci-apps         crawlee_api            [---]
⏸️ etherpad-db.app              ⏸️   ⏸️    5436 oci-apps         etherpad_postgres      [---]
❌ etherpad.app                 ❌   ❌    3012 oci-apps         etherpad_app           [---]
❌ filebrowser.app              ❌   ❌    3015 oci-apps         filebrowser_app        [---]
❌ g-workspace-mcp.app          ❌   ❌    3104 oci-apps         google-workspace-mcp   [---]
❌ gitea.app                    ❌   ❌    3017 oci-apps         gitea                  [---]
❌ grafana.app                  ❌   ❌    3200 oci-apps         lgtm_grafana           [---]
❌ grist.app                    ❌   ❌    3011 oci-apps         grist_app              [---]
⏸️ hedgedoc-db.app              ⏸️   ⏸️    5439 oci-apps         hedgedoc_postgres      [---]
❌ hedgedoc.app                 ❌   ❌    3018 oci-apps         hedgedoc_app           [---]
❌ lgtm-loki.app                ❌   ❌    3110 oci-apps         lgtm_loki              [---]
❌ lgtm-mimir.app               ❌   ❌    9009 oci-apps         lgtm_mimir             [---]
❌ lgtm-tempo.app               ❌   ❌    3210 oci-apps         lgtm_tempo             [---]
❌ mail-mcp.app                 ❌   ❌    3103 oci-apps         mail-mcp               [---]
❌ mattermost-mcp.app           ❌   ❌    3102 oci-apps         mattermost-mcp         [---]
❌ mattermost-postgres.app      ❌   ❌    5435 oci-apps         mattermost-postgres    [---]
❌ mattermost.app               ❌   ❌    8065 oci-apps         mattermost             [---]
⏸️ nocodb-db.app                ⏸️   ⏸️    5441 oci-apps         nocodb-db              [---]
❌ nocodb.app                   ❌   ❌    8085 oci-apps         nocodb                 [---]
❌ ollama-hai.app               ❌   ❌   11435 oci-apps         ollama-hai             [---]
❌ photoprism.app               ❌   ❌    3013 oci-apps         photoprism_app         [---]
⏸️ quant-full-db.app            ⏸️   ⏸️    5437 oci-apps         quant_full_db          [---]
⏸️ quant-full-research.app      ⏸️   ⏸️    8890 oci-apps         quant_full_research    [---]
⏸️ quant-light-db.app           ⏸️   ⏸️    5434 oci-apps         quant_light_db         [---]
⏸️ quant-light-engine.app       ⏸️   ⏸️    5001 oci-apps         quant_light_engine     [---]
⏸️ quant-light-research.app     ⏸️   ⏸️    8889 oci-apps         quant_light_research   [---]
❌ radicale.app                 ❌   ❌    5232 oci-apps         radicale               [---]
❌ revealmd.app                 ❌   ❌    3014 oci-apps         revealmd_app           [---]
❌ windmill-app.app             ❌   ❌    8000 oci-apps         windmill-server        [---]
❌ windmill-db.app              ❌   ❌    5440 oci-apps         windmill-db            [---]
✅ dagu.app                     ✅   ✅    8070 oci-mail         dagu                   [200]
✅ snappymail.app               ✅   ✅    8888 oci-mail         snappymail             [200]
⚠️ stalwart.app                 ✅   ❌     443 oci-mail         stalwart               [---]

  📡 TCP: 15/59  🌐 HTTP: 9/59

── A3) Containers ────────────────────────────────────────────

gcp-proxy ✅ — 0C/0G — mem 951M/1952M (48%) — disk 51% — swap 34M/3999M — load 4.99 3.66 1.87 — 19/19 ctrs — up 8 minutes
────────────────────────────────────────────────────────────
  ⚠️ caddy                     443    443    UP (no hc)     Up About a minute
  ⚠️ authelia-redis            —      6380   UP (no hc)     Up 2 minutes
  ⚠️ hickory-dns               —      53     UP (no hc)     Up 5 minutes
  ⚠️ postlite-ntfy             —      —      UP (no hc)     Up 4 minutes
  ⚠️ postlite-authelia         —      —      UP (no hc)     Up 4 minutes
  ⚠️ postlite-npm              —      —      UP (no hc)     Up 4 minutes
  ⚠️ postlite-vaultwarden      —      —      UP (no hc)     Up 4 minutes
  ⚠️ syslog-bridge             —      —      UP (no hc)     Up 4 minutes
  ⚠️ github-rss                —      —      UP (no hc)     Up 4 minutes
  ⚠️ ntfy                      —      8090   UP (no hc)     Up 4 minutes
  ⚠️ sqlite-authelia           —      —      UP (no hc)     Up 4 minutes
  ⚠️ sqlite-npm                —      —      UP (no hc)     Up 4 minutes
  ⚠️ sqlite-vaultwarden        —      —      UP (no hc)     Up 4 minutes
  ⚠️ sqlite-ntfy               —      —      UP (no hc)     Up 4 minutes
  ⚠️ fluent-bit                —      —      UP (no hc)     Up 5 minutes
  ✅ introspect-proxy          —      4182   HEALTHY        Up About a minute (healthy)
  ✅ authelia                  —      9091   HEALTHY        Up 2 minutes (healthy)
  ✅ redis                     —      6379   HEALTHY        Up 3 minutes (healthy)
  ✅ vaultwarden               —      8880   HEALTHY        Up 2 minutes (healthy)

gcp-t4 ❌ — 0C/0G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-apps ✅ — 4C/24G — mem 4465M/23975M (18%) — disk 74% — swap 0M/0M — load 0.42 0.46 0.41 — 50/54 ctrs — up 1d 9h
────────────────────────────────────────────────────────────
  ❌ surrealdb                 —      —      DOWN(2)        Exited (2) 5 hours ago
  ❌ crawlee_runner            —      —      DOWN(1)        Exited (1) 27 minutes ago
  ❌ crawlee_api               3000   3000   DOWN(1)        Exited (1) 27 minutes ago
  ❌ crawlee_minio_init        —      —      DOWN(0)        Exited (0) 5 hours ago
  ❌ crawlee_db                —      5433   UNHEALTHY      Up 50 seconds (unhealthy)
  ⚠️ lgtm_mimir                —      9009   UP (no hc)     Up 50 minutes
  ⚠️ lgtm_tempo                —      3210   UP (no hc)     Up 50 minutes
  ⚠️ windmill-worker           —      —      UP (no hc)     Up 50 minutes
  ⚠️ mattermost-bots           —      —      UP (no hc)     Up 59 minutes
  ⚠️ mattermost-mcp            —      3102   UP (no hc)     Up About an hour
  ⚠️ mail-mcp                  —      3103   UP (no hc)     Up About an hour
  ⚠️ crawlee_scheduler         —      —      UP (no hc)     Up 5 hours
  ⚠️ gitea                     —      3002   UP (no hc)     Up 8 hours
  ⚠️ bup-server                —      —      UP (no hc)     Up 8 hours
  ⚠️ borg-server               —      —      UP (no hc)     Up 8 hours
  ⚠️ cloud-spec                —      3080   UP (no hc)     Up 8 hours
  ⚠️ siem-api                  —      —      UP (no hc)     Up 8 hours
  ⚠️ crawlee_dashboard         3001   3001   UP (no hc)     Up 29 minutes
  ⚠️ quant_light_engine        —      5001   UP (no hc)     Up 8 hours
  ⚠️ code-server               —      8443   UP (no hc)     Up 9 hours
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 50 minutes (healthy)
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 50 minutes (healthy)
  ✅ windmill-server           —      8000   HEALTHY        Up 51 minutes (healthy)
  ✅ windmill-db               —      5440   HEALTHY        Up 51 minutes (healthy)
  ✅ c3-services-mcp           —      3101   HEALTHY        Up 55 minutes (healthy)
  ✅ c3-infra-mcp              —      3100   HEALTHY        Up 56 minutes (healthy)
  ✅ c3-infra-api              8081   8081   HEALTHY        Up 56 minutes (healthy)
  ✅ ollama-hai                —      11435  HEALTHY        Up 58 minutes (healthy)
  ✅ photoprism_app            —      3013   HEALTHY        Up 58 minutes (healthy)
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 58 minutes (healthy)
  ✅ photoprism_rclone         —      —      HEALTHY        Up 58 minutes (healthy)
  ✅ mattermost                —      8065   HEALTHY        Up 59 minutes (healthy)
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 59 minutes (healthy)
  ✅ hedgedoc_app              —      3018   HEALTHY        Up About an hour (healthy)
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up About an hour (healthy)
  ✅ etherpad_app              —      3012   HEALTHY        Up About an hour (healthy)
  ✅ etherpad_postgres         —      5436   HEALTHY        Up About an hour (healthy)
  ✅ google-workspace-mcp      —      3104   HEALTHY        Up About an hour (healthy)
  ✅ quant_light_db            —      5434   HEALTHY        Up 4 hours (healthy)
  ✅ nocodb                    —      8085   HEALTHY        Up 4 hours (healthy)
  ✅ nocodb-db                 —      5441   HEALTHY        Up 4 hours (healthy)
  ✅ crawlee_minio             —      9000   HEALTHY        Up 5 hours (healthy)
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 5 hours (healthy)
  ✅ syslog-central            —      —      HEALTHY        Up 8 hours (healthy)
  ✅ crawlee_redis             —      6381   HEALTHY        Up 8 hours (healthy)
  ✅ rig-agentic-sonn-14bq8    —      —      HEALTHY        Up 8 hours (healthy)
  ✅ rig-agentic-hai           —      —      HEALTHY        Up 8 hours (healthy)
  ✅ photos-webhook            —      —      HEALTHY        Up 8 hours (healthy)
  ✅ photos-db                 —      —      HEALTHY        Up 8 hours (healthy)
  ✅ quant_light_research      —      8889   HEALTHY        Up 8 hours (healthy)
  ✅ revealmd_app              —      3014   HEALTHY        Up 8 hours (healthy)
  ✅ radicale                  —      5232   HEALTHY        Up 8 hours (healthy)
  ✅ grist_app                 —      3011   HEALTHY        Up 8 hours (healthy)
  ✅ filebrowser_app           —      3015   HEALTHY        Up 9 hours (healthy)

oci-mail ✅ — 1C/0G — mem 653M/954M (68%) — disk 68% — swap 108M/2559M — load 0.85 1.00 1.00 — 6/8 ctrs — up 55 minutes
────────────────────────────────────────────────────────────
  ❌ palantir-cron             —      —      DOWN(?)        Created
  ❌ introspect-proxy          —      —      DOWN(255)      Exited (255) 54 minutes ago
  ⚠️ dagu                      —      8070   UP (no hc)     Up 54 minutes
  ⚠️ stalwart                  —      443    UP (no hc)     Up 53 minutes
  ⚠️ smtp-proxy                —      —      UP (no hc)     Up 54 minutes
  ⚠️ fluent-bit                —      —      UP (no hc)     Up 54 minutes
  ✅ snappymail                —      8888   HEALTHY        Up 54 minutes (healthy)
  ✅ syslog-forwarder          —      —      HEALTHY        Up 54 minutes (healthy)

oci-analytics ✅ — 1C/0G — mem 683M/954M (71%) — disk 56% — swap 222M/2559M — load 2.04 2.08 2.00 — 7/8 ctrs — up 31 minutes
────────────────────────────────────────────────────────────
  ❌ umami-setup               —      —      DOWN(1)        Exited (1) 20 minutes ago
  ⚠️ sauron-forwarder          —      —      UP (no hc)     Up 21 minutes
  ⚠️ matomo-hybrid             —      8080   UP (no hc)     Up 21 minutes
  ⚠️ fluent-bit                —      —      UP (no hc)     Up 21 minutes
  ⚠️ dozzle                    —      9999   UP (no hc)     Up 21 minutes
  ✅ alerts-api                —      —      HEALTHY        Up 21 minutes (healthy)
  ✅ umami                     —      3006   HEALTHY        Up 20 minutes (healthy)
  ✅ umami-db                  —      5442   HEALTHY        Up 21 minutes (healthy)


── A4) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
(TODO)

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  162.159.205.13
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.17
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.24
✅ send.mails.diegonmarcos.com  10    feedback-smtp.us-east-1.amazonses.com.     3.218.134.115
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
✅ dkim._domainkey              diegonmarcos.com         Stalwart             RSA 2048
❌ mail._domainkey              diegonmarcos.com         Legacy Mailu         NOT FOUND
✅ google._domainkey            diegonmarcos.com         Google Workspace     RSA 2048
❌ cf2024-1._domainkey          diegonmarcos.com         Cloudflare           NOT FOUND
❌ resend._domainkey.mails      diegonmarcos.com         Resend/SES           NOT FOUND

DMARC — Outbound Policy
─────────────────────
✅ _dmarc.diegonmarcos.com       v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1

(TODO)

(TODO)


══════════════════════════════════════════════════════════════
  B) INFRA — Resources & Stack
══════════════════════════════════════════════════════════════

VPS / VM SPECS (all providers)
────────────────────────────────────────────────────────────
    VM               Provider   Shape                CPU    RAM    Disk     Cost
────────────────────────────────────────────────────────────
   gcp-proxy        GCP        e2-small                  0 0G     0G       Free
   gcp-t4           GCP        n1-standard-4             0 0G     0G       Spot
   oci-apps         OCI        VM.Standard.A1.Flex       4 24G    0G       Free
   oci-mail         OCI        VM.Standard.E2.1.Micro      1 0G     0G       Free
   oci-analytics    OCI        VM.Standard.E2.1.Micro      1 0G     0G       Free

RESOURCES (live)
────────────────────────────────────────────────────────────
gcp-proxy      gcp-t4         oci-apps       oci-mail       oci-analytics 
────────────────────────────────────────────────────────────
CPU                48 cores      ? cores       18 cores      68 cores      71 cores      
RAM                951M/1952M    ?/?           4465M/23975M  653M/954M     683M/954M     
RAM %              48%           0%            18%           68%           71%           
Swap               34M/3999M     ?             0M/0M         108M/2559M    222M/2559M    
Disk               16G/31G       ?/?           67.1G/95.8G   29G/45G       25G/48G       
Disk %             51%           ?             74%           68%           56%           
Load               4.99 3.66 1.87?             0.42 0.46 0.410.85 1.00 1.002.04 2.08 2.00
Containers         19/19         0/0           50/54         6/8           7/8           
Uptime             up 8 minutes  ?             up 1d 9h      up 55 minutes up 31 minutes 

STORAGE
────────────────────────────────────────────────────────────
(TODO)


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔓 gcp-proxy          35.226.147.64      ports: 22, 2200
🔒 gcp-t4             34.173.227.250     ports: none reachable
🔓 oci-apps           82.70.229.129      ports: 22
🔓 oci-mail           130.110.251.193    ports: 22, 25, 465, 587, 993, 8080
🔓 oci-analytics      129.151.228.66     ports: 22

BACKUPS / DATABASES
────────────────────────────────────────────────────────────
   authelia             ?          authelia               /config/db.sqlite3 gcp-proxy        authelia.app:9091
   ntfy                 ?          ntfy                   /var/cache/ntfy/cache.db gcp-proxy        ntfy.app:8090
   vaultwarden          ?          vaultwarden            /data/db.sqlite3 gcp-proxy        vaultwarden.app:8880
   umami                postgres   umami-db               umami          oci-analytics    umami-db.app:5442
   crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps         crawlee-db.app:5433
   etherpad             postgres   etherpad_postgres      etherpad       oci-apps         etherpad-db.app:5436
   gitea                ?          gitea                  /data/gitea/gitea.db oci-apps         backup-gitea.app:3002
   grist                ?          grist_app              /persist/grist-sessions.db oci-apps         grist.app:3011
   hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps         hedgedoc-db.app:5439
   mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps         mattermost-postgres.app:5435
   nocodb               postgres   nocodb-db              nocodb         oci-apps         nocodb-db.app:5441
   photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps         embedded
   quant-lab-light      postgres   quant_light_db         quantlab       oci-apps         quant-light-db.app:5434
   windmill             postgres   windmill-db            windmill       oci-apps         windmill-db.app:5440

DOCKER NETWORKS
────────────────────────────────────────────────────────────
(TODO)

VAULT — Providers
────────────────────────────────────────────────────────────
(TODO)


══════════════════════════════════════════════════════════════
  D) STACK — Framework & Paths
══════════════════════════════════════════════════════════════

FRAMEWORK — Key Paths
────────────────────────────────────────────────────────────
(TODO)


══════════════════════════════════════════════════════════════
  Z) APPENDIX
══════════════════════════════════════════════════════════════

PERFORMANCE
────────────────────────────────────────────────────────────
  TOTAL                85.5s
  vm_ssh               85.3s
  mail_dns             20.6s
  private              15.8s
  public_urls          7.9s
  mesh                 7.8s
  port_scan            7.7s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  85.5s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(async process)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
