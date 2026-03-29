```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-03-29T04:52:32.978297393+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
48 critical, 0 warnings — 48 total

    ❌ A3       VM gcp-proxy — UNREACHABLE
    ❌ A3       oci-apps/surrealdb — exited
    ❌ A3       oci-apps/crawlee_runner — exited
    ❌ A3       oci-apps/crawlee_api — exited
    ❌ A3       oci-apps/crawlee_minio_init — exited
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
    ❌ A1       smtp.diegonmarcos.com — [---]
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
❌ smtp.diegonmarcos.com            ❌  ❌  ❌  —  smtp-proxy.app:8080    [---] 
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

⚠️  WireGuard/Hickory DOWN — cannot reach .app endpoints
    Run: sudo wg-quick up wg0

    DNS Name                     📡TCP 🌐HTTP Port    VM               Container              Code
    ───────────────────────────────────────────────────────────────────────────────────────────────
⏸️ authelia-redis.app           ⏸️   ⏸️    6380 gcp-proxy        authelia-redis         [---]
⏸️ authelia.app                 ⏸️   ⏸️    9091 gcp-proxy        authelia               [---]
⏸️ caddy.app                    ⏸️   ⏸️     443 gcp-proxy        caddy                  [---]
⏸️ hickory-dns.app              ⏸️   ⏸️      53 gcp-proxy        hickory-dns            [---]
⏸️ introspect-proxy.app         ⏸️   ⏸️    4182 gcp-proxy        introspect-proxy       [---]
⏸️ introspect-proxy.app         ⏸️   ⏸️    4182 gcp-proxy        introspect-proxy       [---]
⏸️ ntfy.app                     ⏸️   ⏸️    8090 gcp-proxy        ntfy                   [---]
⏸️ redis.app                    ⏸️   ⏸️    6379 gcp-proxy        redis                  [---]
⏸️ vaultwarden.app              ⏸️   ⏸️    8880 gcp-proxy        vaultwarden            [---]
⏸️ ollama.app                   ⏸️   ⏸️   11434 gcp-t4           ollama                 [---]
⏸️ dozzle.app                   ⏸️   ⏸️    9999 oci-analytics    dozzle                 [---]
⏸️ matomo.app                   ⏸️   ⏸️    8080 oci-analytics    matomo-hybrid          [---]
⏸️ umami-db.app                 ⏸️   ⏸️    5442 oci-analytics    umami-db               [---]
⏸️ umami.app                    ⏸️   ⏸️    3006 oci-analytics    umami                  [---]
⏸️ backup-gitea.app             ⏸️   ⏸️    3002 oci-apps         gitea                  [---]
⏸️ c3-infra-api.app             ⏸️   ⏸️    8081 oci-apps         c3-infra-api           [---]
⏸️ c3-infra-mcp.app             ⏸️   ⏸️    3100 oci-apps         c3-infra-mcp           [---]
⏸️ c3-services-api.app          ⏸️   ⏸️    8082 oci-apps         c3-services-api        [---]
⏸️ c3-services-mcp.app          ⏸️   ⏸️    3101 oci-apps         c3-services-mcp        [---]
⏸️ c3-spec.app                  ⏸️   ⏸️    3080 oci-apps         cloud-spec             [---]
⏸️ cloud-cgc-mcp.app            ⏸️   ⏸️    3105 oci-apps         cloud-cgc-mcp          [---]
⏸️ code-server.app              ⏸️   ⏸️    8443 oci-apps         code-server            [---]
⏸️ crawlee-dashboard.app        ⏸️   ⏸️    3001 oci-apps         crawlee_dashboard      [---]
⏸️ crawlee-db.app               ⏸️   ⏸️    5433 oci-apps         crawlee_db             [---]
⏸️ crawlee-minio.app            ⏸️   ⏸️    9000 oci-apps         crawlee_minio          [---]
⏸️ crawlee-redis.app            ⏸️   ⏸️    6381 oci-apps         crawlee_redis          [---]
⏸️ crawlee.app                  ⏸️   ⏸️    3000 oci-apps         crawlee_api            [---]
⏸️ etherpad-db.app              ⏸️   ⏸️    5436 oci-apps         etherpad_postgres      [---]
⏸️ etherpad.app                 ⏸️   ⏸️    3012 oci-apps         etherpad_app           [---]
⏸️ filebrowser.app              ⏸️   ⏸️    3015 oci-apps         filebrowser_app        [---]
⏸️ g-workspace-mcp.app          ⏸️   ⏸️    3104 oci-apps         google-workspace-mcp   [---]
⏸️ gitea.app                    ⏸️   ⏸️    3017 oci-apps         gitea                  [---]
⏸️ grafana.app                  ⏸️   ⏸️    3200 oci-apps         lgtm_grafana           [---]
⏸️ grist.app                    ⏸️   ⏸️    3011 oci-apps         grist_app              [---]
⏸️ hedgedoc-db.app              ⏸️   ⏸️    5439 oci-apps         hedgedoc_postgres      [---]
⏸️ hedgedoc.app                 ⏸️   ⏸️    3018 oci-apps         hedgedoc_app           [---]
⏸️ lgtm-loki.app                ⏸️   ⏸️    3110 oci-apps         lgtm_loki              [---]
⏸️ lgtm-mimir.app               ⏸️   ⏸️    9009 oci-apps         lgtm_mimir             [---]
⏸️ lgtm-tempo.app               ⏸️   ⏸️    3210 oci-apps         lgtm_tempo             [---]
⏸️ mail-mcp.app                 ⏸️   ⏸️    3103 oci-apps         mail-mcp               [---]
⏸️ mattermost-mcp.app           ⏸️   ⏸️    3102 oci-apps         mattermost-mcp         [---]
⏸️ mattermost-postgres.app      ⏸️   ⏸️    5435 oci-apps         mattermost-postgres    [---]
⏸️ mattermost.app               ⏸️   ⏸️    8065 oci-apps         mattermost             [---]
⏸️ nocodb-db.app                ⏸️   ⏸️    5441 oci-apps         nocodb-db              [---]
⏸️ nocodb.app                   ⏸️   ⏸️    8085 oci-apps         nocodb                 [---]
⏸️ ollama-hai.app               ⏸️   ⏸️   11435 oci-apps         ollama-hai             [---]
⏸️ photoprism.app               ⏸️   ⏸️    3013 oci-apps         photoprism_app         [---]
⏸️ quant-full-db.app            ⏸️   ⏸️    5437 oci-apps         quant_full_db          [---]
⏸️ quant-full-research.app      ⏸️   ⏸️    8890 oci-apps         quant_full_research    [---]
⏸️ quant-light-db.app           ⏸️   ⏸️    5434 oci-apps         quant_light_db         [---]
⏸️ quant-light-engine.app       ⏸️   ⏸️    5001 oci-apps         quant_light_engine     [---]
⏸️ quant-light-research.app     ⏸️   ⏸️    8889 oci-apps         quant_light_research   [---]
⏸️ radicale.app                 ⏸️   ⏸️    5232 oci-apps         radicale               [---]
⏸️ revealmd.app                 ⏸️   ⏸️    3014 oci-apps         revealmd_app           [---]
⏸️ windmill-app.app             ⏸️   ⏸️    8000 oci-apps         windmill-server        [---]
⏸️ windmill-db.app              ⏸️   ⏸️    5440 oci-apps         windmill-db            [---]
⏸️ dagu.app                     ⏸️   ⏸️    8070 oci-mail         dagu                   [---]
⏸️ smtp-proxy.app               ⏸️   ⏸️    8080 oci-mail         smtp-proxy             [---]
⏸️ snappymail.app               ⏸️   ⏸️    8888 oci-mail         snappymail             [---]
⏸️ stalwart.app                 ⏸️   ⏸️     443 oci-mail         stalwart               [---]

  📡 TCP: 0/60  🌐 HTTP: 0/60

── A3) Containers ────────────────────────────────────────────

gcp-proxy ❌ — 0C/0G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-apps ✅ — 4C/24G — mem 4813M/23975M (20%) — disk 74% — swap 0M/0M — load 0.53 0.39 0.37 — 50/54 ctrs — 1d 10h
────────────────────────────────────────────────────────────
  ❌ surrealdb                 —      —      DOWN(2)        Exited (2) 6 hours ago
  ❌ crawlee_runner            —      —      DOWN(1)        Exited (1) 2 hours ago
  ❌ crawlee_api               3000   3000   DOWN(1)        Exited (1) 2 hours ago
  ❌ crawlee_minio_init        —      —      DOWN(0)        Exited (0) 6 hours ago
  ⚠️ crawlee_db                —      5433   STARTING       Up 12 seconds (health: startin
  ⚠️ lgtm_mimir                —      9009   UP (no hc)     Up 2 hours
  ⚠️ lgtm_tempo                —      3210   UP (no hc)     Up 2 hours
  ⚠️ windmill-worker           —      —      UP (no hc)     Up 2 hours
  ⚠️ mattermost-bots           —      —      UP (no hc)     Up 2 hours
  ⚠️ mattermost-mcp            —      3102   UP (no hc)     Up 2 hours
  ⚠️ mail-mcp                  —      3103   UP (no hc)     Up 2 hours
  ⚠️ crawlee_scheduler         —      —      UP (no hc)     Up 6 hours
  ⚠️ gitea                     —      3002   UP (no hc)     Up 9 hours
  ⚠️ bup-server                —      —      UP (no hc)     Up 9 hours
  ⚠️ borg-server               —      —      UP (no hc)     Up 9 hours
  ⚠️ cloud-spec                —      3080   UP (no hc)     Up 9 hours
  ⚠️ siem-api                  —      —      UP (no hc)     Up 10 hours
  ⚠️ crawlee_dashboard         3001   3001   UP (no hc)     Up 2 hours
  ⚠️ quant_light_engine        —      5001   UP (no hc)     Up 10 hours
  ⚠️ code-server               —      8443   UP (no hc)     Up 10 hours
  ✅ c3-infra-mcp              —      3100   HEALTHY        Up 46 minutes (healthy)
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 2 hours (healthy)
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 2 hours (healthy)
  ✅ windmill-server           —      8000   HEALTHY        Up 2 hours (healthy)
  ✅ windmill-db               —      5440   HEALTHY        Up 2 hours (healthy)
  ✅ c3-services-mcp           —      3101   HEALTHY        Up 2 hours (healthy)
  ✅ c3-infra-api              8081   8081   HEALTHY        Up 2 hours (healthy)
  ✅ ollama-hai                —      11435  HEALTHY        Up 2 hours (healthy)
  ✅ photoprism_app            —      3013   HEALTHY        Up 2 hours (healthy)
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 2 hours (healthy)
  ✅ photoprism_rclone         —      —      HEALTHY        Up 2 hours (healthy)
  ✅ mattermost                —      8065   HEALTHY        Up 2 hours (healthy)
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 2 hours (healthy)
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 2 hours (healthy)
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up 2 hours (healthy)
  ✅ etherpad_app              —      3012   HEALTHY        Up 2 hours (healthy)
  ✅ etherpad_postgres         —      5436   HEALTHY        Up 2 hours (healthy)
  ✅ google-workspace-mcp      —      3104   HEALTHY        Up 2 hours (healthy)
  ✅ quant_light_db            —      5434   HEALTHY        Up 6 hours (healthy)
  ✅ nocodb                    —      8085   HEALTHY        Up 6 hours (healthy)
  ✅ nocodb-db                 —      5441   HEALTHY        Up 6 hours (healthy)
  ✅ crawlee_minio             —      9000   HEALTHY        Up 6 hours (healthy)
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 6 hours (healthy)
  ✅ syslog-central            —      —      HEALTHY        Up 10 hours (healthy)
  ✅ crawlee_redis             —      6381   HEALTHY        Up 10 hours (healthy)
  ✅ rig-agentic-sonn-14bq8    —      —      HEALTHY        Up 10 hours (healthy)
  ✅ rig-agentic-hai           —      —      HEALTHY        Up 10 hours (healthy)
  ✅ photos-webhook            —      —      HEALTHY        Up 10 hours (healthy)
  ✅ photos-db                 —      —      HEALTHY        Up 10 hours (healthy)
  ✅ quant_light_research      —      8889   HEALTHY        Up 10 hours (healthy)
  ✅ revealmd_app              —      3014   HEALTHY        Up 10 hours (healthy)
  ✅ radicale                  —      5232   HEALTHY        Up 10 hours (healthy)
  ✅ grist_app                 —      3011   HEALTHY        Up 10 hours (healthy)
  ✅ filebrowser_app           —      3015   HEALTHY        Up 10 hours (healthy)

oci-mail ✅ — 1C/0G — mem 629M/954M (65%) — disk 68% — swap 115M/2559M — load 0.97 0.98 1.00 — 6/8 ctrs — 0d 1h
────────────────────────────────────────────────────────────
  ❌ palantir-cron             —      —      DOWN(?)        Created
  ❌ introspect-proxy          —      —      DOWN(255)      Exited (255) 2 hours ago
  ⚠️ dagu                      —      8070   UP (no hc)     Up 2 hours
  ⚠️ stalwart                  —      443    UP (no hc)     Up 2 hours
  ⚠️ smtp-proxy                8080   8080   UP (no hc)     Up 2 hours
  ⚠️ fluent-bit                —      —      UP (no hc)     Up 2 hours
  ✅ snappymail                —      8888   HEALTHY        Up 2 hours (healthy)
  ✅ syslog-forwarder          —      —      HEALTHY        Up 2 hours (healthy)

oci-analytics ✅ — 1C/0G — mem 703M/954M (73%) — disk 56% — swap 232M/2559M — load 2.08 2.12 2.09 — 7/8 ctrs — 0d 1h
────────────────────────────────────────────────────────────
  ❌ umami-setup               —      —      DOWN(1)        Exited (1) About an hour ago
  ⚠️ sauron-forwarder          —      —      UP (no hc)     Up About an hour
  ⚠️ matomo-hybrid             —      8080   UP (no hc)     Up About an hour
  ⚠️ fluent-bit                —      —      UP (no hc)     Up About an hour
  ⚠️ dozzle                    —      9999   UP (no hc)     Up About an hour
  ✅ alerts-api                —      —      HEALTHY        Up About an hour (healthy)
  ✅ umami                     —      3006   HEALTHY        Up About an hour (healthy)
  ✅ umami-db                  —      5442   HEALTHY        Up About an hour (healthy)


── A4) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
(TODO)

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
❌ diegonmarcos.com             —     no MX record                               
❌ send.mails.diegonmarcos.com  —     no MX record                               
❌ mails.diegonmarcos.com       —     no MX record                               

SPF — Outbound Policy: IP Allowlist
────────────────────────────────────────────────────────────
⚠️ diegonmarcos.com                 oci-mail VM IP 130.110.251.193 NOT IN SPF!

DKIM — Outbound Policy: Cryptographic Signatures
────────────────────────────────────────────────────────────
❌ dkim._domainkey              diegonmarcos.com         Stalwart             NOT FOUND
❌ mail._domainkey              diegonmarcos.com         Legacy Mailu         NOT FOUND
❌ google._domainkey            diegonmarcos.com         Google Workspace     NOT FOUND
❌ cf2024-1._domainkey          diegonmarcos.com         Cloudflare           NOT FOUND
❌ resend._domainkey.mails      diegonmarcos.com         Resend/SES           NOT FOUND

DMARC — Outbound Policy
─────────────────────
✅ _dmarc.diegonmarcos.com       NO DMARC

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
gcp-proxy      oci-apps       oci-mail       oci-analytics 
────────────────────────────────────────────────────────────
CPU                ? cores       20 cores      65 cores      73 cores      
RAM                ?/?           4813M/23975M  629M/954M     703M/954M     
RAM %              0%            20%           65%           73%           
Swap               ?             0M/0M         115M/2559M    232M/2559M    
Disk               ?/?           67.0G/95.8G   29G/45G       25G/48G       
Disk %             ?             74%           68%           56%           
Load               ?             0.53 0.39 0.370.97 0.98 1.002.08 2.12 2.09
Containers         0/0           50/54         6/8           7/8           
Uptime             ?             1d 10h        0d 1h         0d 1h         

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
  TOTAL                62.6s
  mail_dns             56.3s
  public_urls          9.8s
  mesh                 9.6s
  vm_ssh               9.6s
  port_scan            6.3s
  private              0.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  62.6s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(async process)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
