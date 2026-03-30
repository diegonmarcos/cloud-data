```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-03-30T14:26:10.673622143+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
44 critical, 0 warnings — 44 total

    ❌ A3       VM gcp-proxy — UNREACHABLE
    ❌ A3       oci-apps/crawlee_minio — exited
    ❌ A3       oci-apps/rig — exited
    ❌ A3       oci-apps/rig-agentic — exited
    ❌ A3       oci-apps/surrealdb — exited
    ❌ A3       oci-mail/introspect-proxy — exited
    ❌ A3       oci-analytics/umami-setup — exited
    ❌ A1       auth.diegonmarcos.com — [308]
    ❌ A1       git.diegonmarcos.com — [308]
    ❌ A1       api.diegonmarcos.com/c3-api — [308]
    ❌ A1       api.diegonmarcos.com/services — [308]
    ❌ A1       proxy.diegonmarcos.com — [308]
    ❌ A1       ide.diegonmarcos.com — [308]
    ❌ A1       api.diegonmarcos.com — [308]
    ❌ A1       workflows.diegonmarcos.com — [308]
    ❌ A1       pad.diegonmarcos.com — [308]
    ❌ A1       files.diegonmarcos.com — [308]
    ❌ A1       sheets.diegonmarcos.com — [308]
    ❌ A1       dns.internal — [err: error sending request for url (http://dns.internal/)]
    ❌ A1       grafana.diegonmarcos.com — [308]
    ❌ A1       analytics.diegonmarcos.com — [308]
    ❌ A1       chat.diegonmarcos.com — [308]
    ❌ A1       db.diegonmarcos.com — [308]
    ❌ A1       rss.diegonmarcos.com — [308]
    ❌ A1       photos.diegonmarcos.com — [308]
    ❌ A1       cal.diegonmarcos.com — [308]
    ❌ A1       slides.diegonmarcos.com — [308]
    ❌ A1       smtp.diegonmarcos.com — [308]
    ❌ A1       webmail.diegonmarcos.com — [308]
    ❌ A1       mail.diegonmarcos.com — [308]
    ❌ A1       vault.diegonmarcos.com — [308]
    ❌ A1       windmill.diegonmarcos.com — [308]
    ❌ A1       app.diegonmarcos.com/filebrowser — [308]
    ❌ A1       app.diegonmarcos.com/hedgedoc — [308]
    ❌ A1       app.diegonmarcos.com/revealmd — [308]
    ❌ A1       app.diegonmarcos.com/dozzle — [308]
    ❌ A1       app.diegonmarcos.com/windmill — [308]
    ❌ A1       app.diegonmarcos.com/grafana — [308]
    ❌ A1       app.diegonmarcos.com/gitea — [308]
    ❌ A1       app.diegonmarcos.com/crawlee — [308]
    ❌ A1       api.diegonmarcos.com/crawlee — [308]
    ❌ A1       api.diegonmarcos.com/dash — [308]
    ❌ A1       cloud.diegonmarcos.com — [308]
    ❌ A1       mcp.diegonmarcos.com — [308]


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
⚠️ oci-apps       oci-A1-f_0         ✅  ✅  ❌  ❌  82.70.229.129      10.0.0.6       VM       no data
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
⚠️ auth.diegonmarcos.com            ✅  ✅  ❌  ❌  authelia.app:9091      [308] auth:[err: error sending request for url (https://auth.diegonmarcos.com/)]
⚠️ git.diegonmarcos.com             ✅  ✅  ❌  ❌  backup-gitea.app:3002  [308] auth:[err: error sending request for url (https://git.diegonmarcos.com/)]
⚠️ api.diegonmarcos.com/c3-api      ❌  ✅  ❌  ❌  c3-infra-api.app:8081  [308] auth:[err: error sending request for url (https://api.diegonmarcos.com/c3-api)]
⚠️ mcp.diegonmarcos.com/c3-infra-mcp ❌  ✅  ✅  ❌  c3-infra-mcp.app:3100  [200] auth:[err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp)]
⚠️ api.diegonmarcos.com/services    ❌  ✅  ❌  ❌  c3-services-api.app:8082 [308] auth:[502]
⚠️ proxy.diegonmarcos.com           ✅  ✅  ❌  ❌  caddy.app:443          [308] auth:[err: error sending request for url (https://proxy.diegonmarcos.com/)]
⚠️ ide.diegonmarcos.com             ✅  ✅  ❌  ❌  code-server.app:8443   [308] auth:[err: error sending request for url (https://ide.diegonmarcos.com/)]
⚠️ api.diegonmarcos.com             ✅  ✅  ❌  ❌  crawlee.app:3000       [308] auth:[502]
⚠️ workflows.diegonmarcos.com       ✅  ✅  ❌  ❌  dagu.app:8070          [308] auth:[502]
⚠️ logs.diegonmarcos.com            ✅  ✅  ✅  ❌  dozzle.app:9999        [200] auth:[err: error sending request for url (https://logs.diegonmarcos.com/)]
⚠️ pad.diegonmarcos.com             ✅  ✅  ❌  ✅  etherpad.app:3012      [308] 
⚠️ files.diegonmarcos.com           ✅  ✅  ❌  ✅  filebrowser.app:3015   [308] 
⚠️ sheets.diegonmarcos.com          ✅  ✅  ❌  ❌  grist.app:3011         [308] auth:[err: error sending request for url (https://sheets.diegonmarcos.com/)]
⚠️ doc.diegonmarcos.com             ✅  ✅  ✅  ❌  hedgedoc.app:3018      [200] auth:[err: error sending request for url (https://doc.diegonmarcos.com/)]
❌ dns.internal                     ❌  ❌  ❌  ❌  hickory-dns.app:53     [err: error sending request for url (http://dns.internal/)] auth:[err: error sending request for url (https://dns.internal/)]
⚠️ grafana.diegonmarcos.com         ✅  ✅  ❌  ❌  grafana.app:3200       [308] auth:[err: error sending request for url (https://grafana.diegonmarcos.com/)]
⚠️ analytics.diegonmarcos.com       ✅  ✅  ❌  ❌  matomo.app:8080        [308] auth:[err: error sending request for url (https://analytics.diegonmarcos.com/)]
⚠️ chat.diegonmarcos.com            ✅  ✅  ❌  ❌  mattermost.app:8065    [308] auth:[502]
⚠️ db.diegonmarcos.com              ✅  ✅  ❌  ❌  nocodb.app:8085        [308] auth:[err: error sending request for url (https://db.diegonmarcos.com/)]
⚠️ rss.diegonmarcos.com             ✅  ✅  ❌  ❌  ntfy.app:8090          [308] auth:[err: error sending request for url (https://rss.diegonmarcos.com/)]
⚠️ photos.diegonmarcos.com          ✅  ✅  ❌  ❌  photoprism.app:3013    [308] auth:[502]
⚠️ cal.diegonmarcos.com             ✅  ✅  ❌  ❌  radicale.app:5232      [308] auth:[err: error sending request for url (https://cal.diegonmarcos.com/)]
⚠️ slides.diegonmarcos.com          ✅  ✅  ❌  ❌  revealmd.app:3014      [308] auth:[err: error sending request for url (https://slides.diegonmarcos.com/)]
⚠️ smtp.diegonmarcos.com            ✅  ✅  ❌  ❌  smtp-proxy.app:8080    [308] auth:[err: error sending request for url (https://smtp.diegonmarcos.com/)]
⚠️ webmail.diegonmarcos.com         ✅  ✅  ❌  ❌  snappymail.app:8888    [308] auth:[err: error sending request for url (https://webmail.diegonmarcos.com/)]
⚠️ mail.diegonmarcos.com            ✅  ✅  ❌  ❌  stalwart.app:443       [308] auth:[err: error sending request for url (https://mail.diegonmarcos.com/)]
⚠️ vault.diegonmarcos.com           ✅  ✅  ❌  ❌  vaultwarden.app:8880   [308] auth:[err: error sending request for url (https://vault.diegonmarcos.com/)]
⚠️ windmill.diegonmarcos.com        ✅  ✅  ❌  ❌  windmill-app.app:8000  [308] auth:[err: error sending request for url (https://windmill.diegonmarcos.com/)]
⚠️ app.diegonmarcos.com/etherpad    ❌  ✅  ✅  ❌  etherpad.app:3012      [404] auth:[err: error sending request for url (https://app.diegonmarcos.com/etherpad)]
⚠️ app.diegonmarcos.com/filebrowser ❌  ✅  ❌  ❌  filebrowser.app:3015   [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/filebrowser)]
⚠️ app.diegonmarcos.com/hedgedoc    ❌  ✅  ❌  ❌  hedgedoc.app:3018      [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/hedgedoc)]
⚠️ app.diegonmarcos.com/revealmd    ❌  ✅  ❌  ❌  revealmd.app:3014      [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/revealmd)]
⚠️ app.diegonmarcos.com/dozzle      ❌  ✅  ❌  ❌  dozzle.app:9999        [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/dozzle)]
⚠️ app.diegonmarcos.com/windmill    ❌  ✅  ❌  ❌  windmill-app.app:8000  [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/windmill)]
⚠️ app.diegonmarcos.com/grafana     ❌  ✅  ❌  ❌  grafana.app:3016       [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/grafana)]
⚠️ app.diegonmarcos.com/gitea       ❌  ✅  ❌  ❌  gitea.app:3017         [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/gitea)]
⚠️ app.diegonmarcos.com/crawlee     ❌  ✅  ❌  ❌  crawlee.app:3001       [308] auth:[err: error sending request for url (https://app.diegonmarcos.com/crawlee)]
⚠️ api.diegonmarcos.com/crawlee     ❌  ✅  ❌  ❌  crawlee.app:3000       [308] auth:[err: error sending request for url (https://api.diegonmarcos.com/crawlee)]
⚠️ api.diegonmarcos.com/dash        ❌  ✅  ❌  ❌  diegonmarcos.github.io [308] auth:[err: error sending request for url (https://api.diegonmarcos.com/dash)]
⚠️ cloud.diegonmarcos.com           ✅  ✅  ❌  ❌  c3-spec.app:3080       [308] auth:[err: error sending request for url (https://cloud.diegonmarcos.com/)]
⚠️ mcp.diegonmarcos.com             ✅  ✅  ❌  ❌  g-workspace-mcp.app:3104 [308] auth:[err: error sending request for url (https://mcp.diegonmarcos.com/)]

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

⚠️  WireGuard/Hickory DOWN — cannot reach .app endpoints
    Run: sudo wg-quick up wg0

    DNS Name                     📡TCP 🌐HTTP Port    VM               Container              Code
    ───────────────────────────────────────────────────────────────────────────────────────────────
⏸️ authelia-redis.app           ⏸️   ⏸️    6380 gcp-proxy        authelia-redis         [---]
⏸️ authelia.app                 ⏸️   ⏸️    9091 gcp-proxy        authelia               [---]
⏸️ caddy.app                    ⏸️   ⏸️     443 gcp-proxy        caddy                  [---]
⏸️ hickory-dns.app              ⏸️   ⏸️      53 gcp-proxy        hickory-dns            [---]
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

  📡 TCP: 0/59  🌐 HTTP: 0/59

── A3) Containers ────────────────────────────────────────────

gcp-proxy ❌ — 2C/2G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-apps ✅ — 4C/24G — mem 4174M/23975M (17%) — disk 76% — swap 0M/0M — load 0.55 0.71 0.76 — 48/56 ctrs — 1d 0h
────────────────────────────────────────────────────────────
  ❌ crawlee_runner            —      —      DOWN(?)        Created
  ❌ crawlee_dashboard         3001   3001   DOWN(?)        Created
  ❌ crawlee_api               3000   3000   DOWN(?)        Created
  ❌ crawlee_minio_init        —      —      DOWN(?)        Created
  ❌ crawlee_minio             —      9000   DOWN(0)        Exited (0) 5 minutes ago
  ❌ rig                       —      —      DOWN(101)      Exited (101) 17 hours ago
  ❌ rig-agentic               —      —      DOWN(101)      Exited (101) 17 hours ago
  ❌ surrealdb                 —      —      DOWN(2)        Exited (2) 17 hours ago
  ⚠️ photoprism_app            —      3013   STARTING       Up 1 second (health: starting)
  ⚠️ crawlee_scheduler         —      —      UP (no hc)     Up 5 minutes
  ⚠️ quant_light_engine        —      5001   UP (no hc)     Up About an hour
  ⚠️ windmill-worker           —      —      UP (no hc)     Up 17 hours
  ⚠️ mattermost-bots           —      —      UP (no hc)     Up 17 hours
  ⚠️ lgtm_mimir                —      9009   UP (no hc)     Up 17 hours
  ⚠️ lgtm_tempo                —      3210   UP (no hc)     Up 17 hours
  ⚠️ mattermost-mcp            —      3102   UP (no hc)     Up 17 hours
  ⚠️ mail-mcp                  —      3103   UP (no hc)     Up 14 hours
  ⚠️ gitea                     —      3002   UP (no hc)     Up 17 hours
  ⚠️ bup-server                —      —      UP (no hc)     Up 17 hours
  ⚠️ borg-server               —      —      UP (no hc)     Up 17 hours
  ⚠️ cloud-spec                —      3080   UP (no hc)     Up 17 hours
  ⚠️ siem-api                  —      —      UP (no hc)     Up 17 hours
  ⚠️ code-server               —      8443   UP (no hc)     Up 17 hours
  ✅ crawlee_db                —      5433   HEALTHY        Up 5 minutes (healthy)
  ✅ crawlee_redis             —      6381   HEALTHY        Up 5 minutes (healthy)
  ✅ quant_light_research      —      8889   HEALTHY        Up About an hour (healthy)
  ✅ quant_light_db            —      5434   HEALTHY        Up About an hour (healthy)
  ✅ c3-infra-mcp              —      3100   HEALTHY        Up About an hour (healthy)
  ✅ c3-infra-api              8081   8081   HEALTHY        Up 17 hours (healthy)
  ✅ windmill-server           —      8000   HEALTHY        Up 17 hours (healthy)
  ✅ windmill-db               —      5440   HEALTHY        Up 17 hours (healthy)
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 17 hours (healthy)
  ✅ photoprism_rclone         —      —      HEALTHY        Up 17 hours (healthy)
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 17 hours (healthy)
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 17 hours (healthy)
  ✅ c3-services-mcp           —      3101   HEALTHY        Up 17 hours (healthy)
  ✅ ollama-hai                —      11435  HEALTHY        Up 17 hours (healthy)
  ✅ mattermost                —      8065   HEALTHY        Up 17 hours (healthy)
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 17 hours (healthy)
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 17 hours (healthy)
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up 17 hours (healthy)
  ✅ etherpad_app              —      3012   HEALTHY        Up 17 hours (healthy)
  ✅ etherpad_postgres         —      5436   HEALTHY        Up 17 hours (healthy)
  ✅ google-workspace-mcp      —      3104   HEALTHY        Up 17 hours (healthy)
  ✅ nocodb                    —      8085   HEALTHY        Up 17 hours (healthy)
  ✅ nocodb-db                 —      5441   HEALTHY        Up 17 hours (healthy)
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 17 hours (healthy)
  ✅ syslog-central            —      —      HEALTHY        Up 17 hours (healthy)
  ✅ rig-agentic-sonn-14bq8    —      —      HEALTHY        Up 17 hours (healthy)
  ✅ rig-agentic-hai           —      —      HEALTHY        Up 17 hours (healthy)
  ✅ photos-webhook            —      —      HEALTHY        Up 17 hours (healthy)
  ✅ photos-db                 —      —      HEALTHY        Up 17 hours (healthy)
  ✅ revealmd_app              —      3014   HEALTHY        Up 17 hours (healthy)
  ✅ radicale                  —      5232   HEALTHY        Up 17 hours (healthy)
  ✅ grist_app                 —      3011   HEALTHY        Up 17 hours (healthy)
  ✅ filebrowser_app           —      3015   HEALTHY        Up 17 hours (healthy)

oci-mail ✅ — 1C/1G — mem 629M/954M (65%) — disk 68% — swap 115M/2559M — load 0.97 0.98 1.00 — 6/8 ctrs — 0d 1h
────────────────────────────────────────────────────────────
  ❌ palantir-cron             —      —      DOWN(?)        Created
  ❌ introspect-proxy          —      —      DOWN(255)      Exited (255) 2 hours ago
  ⚠️ dagu                      —      8070   UP (no hc)     Up 2 hours
  ⚠️ stalwart                  —      443    UP (no hc)     Up 2 hours
  ⚠️ smtp-proxy                8080   8080   UP (no hc)     Up 2 hours
  ⚠️ fluent-bit                —      —      UP (no hc)     Up 2 hours
  ✅ snappymail                —      8888   HEALTHY        Up 2 hours (healthy)
  ✅ syslog-forwarder          —      —      HEALTHY        Up 2 hours (healthy)

oci-analytics ✅ — 1C/1G — mem 703M/954M (73%) — disk 56% — swap 232M/2559M — load 2.08 2.12 2.09 — 7/8 ctrs — 0d 1h
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
  📨 INBOUND: Gmail → MX → CF Email Routing → CF Worker → Caddy → smtp-proxy → Stalwart
     ✅ smtp-proxy           Up 2 hours
     ✅ stalwart             Up 2 hours

  📱 CLIENT: Phone/Thunderbird → gcp-proxy (35.226.147.64) → Caddy L4 → oci-mail → Stalwart

  📤 OUTBOUND PERSONAL: Stalwart → ⚠️ direct from 130.110.251.193 (NOT IN SPF!)
     ❌ SPF FAIL  ✅ DKIM OK  ❌ DMARC=reject

  📤 OUTBOUND TRANSACTIONAL: App → Resend API → SES → recipient
     ✅ SPF OK  ✅ DKIM OK  ✅ DMARC OK


══════════════════════════════════════════════════════════════
  B) INFRA — Resources & Stack
══════════════════════════════════════════════════════════════

VPS / VM SPECS (all providers)
────────────────────────────────────────────────────────────
    VM               Provider   Shape                CPU    RAM    Disk     Cost
────────────────────────────────────────────────────────────
   gcp-proxy        GCP        e2-small                  2 2G     0G       Free
   gcp-t4           GCP        n1-standard-4             4 15G    0G       Spot
   oci-apps         OCI        VM.Standard.A1.Flex       4 24G    0G       Free
   oci-mail         OCI        VM.Standard.E2.1.Micro      1 1G     0G       Free
   oci-analytics    OCI        VM.Standard.E2.1.Micro      1 1G     0G       Free

RESOURCES (live)
────────────────────────────────────────────────────────────
gcp-proxy      oci-apps       oci-mail       oci-analytics 
────────────────────────────────────────────────────────────
CPU                ? cores       17 cores      65 cores      73 cores      
RAM                ?/?           4174M/23975M  629M/954M     703M/954M     
RAM %              0%            17%           65%           73%           
Swap               ?             0M/0M         115M/2559M    232M/2559M    
Disk               ?/?           69.1G/95.8G   29G/45G       25G/48G       
Disk %             ?             76%           68%           56%           
Load               ?             0.55 0.71 0.760.97 0.98 1.002.08 2.12 2.09
Containers         0/0           48/56         6/8           7/8           
Uptime             ?             1d 0h         0d 1h         0d 1h         

STORAGE
────────────────────────────────────────────────────────────
  OBJECT STORAGE
    oci — archlinux-images (Standard)
    oci — my-photos (Standard)

  DATABASES
    authelia             ?          authelia               gcp-proxy
    ntfy                 ?          ntfy                   gcp-proxy
    vaultwarden          ?          vaultwarden            gcp-proxy
    umami                postgres   umami-db               oci-analytics
    crawlee-cloud        postgres   crawlee_db             oci-apps
    etherpad             postgres   etherpad_postgres      oci-apps
    gitea                ?          gitea                  oci-apps
    grist                ?          grist_app              oci-apps
    hedgedoc             postgres   hedgedoc_postgres      oci-apps
    mattermost-bots      postgres   mattermost-postgres    oci-apps
    nocodb               postgres   nocodb-db              oci-apps
    photoprism           mariadb    photoprism_mariadb     oci-apps
    quant-lab-light      postgres   quant_light_db         oci-apps
    windmill             postgres   windmill-db            oci-apps


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

NETWORK SECURITY AUDIT
────────────────────────────────────────────────────────────
    Check                         gcp-proxy           oci-mail      oci-analytics           oci-apps
    ────────────────────────────────────────────────────────────────────────────────────────────────
    Declared ports       80,443,465,587,993 25,465,587,993,4190,8080,8443,21027,22000               none 2222,2223,2224,3000,3001,3010,8081,8099
    Scanned (public)     22,80,443,465,587,993 25,465,587,993,8080             🔒 none                 22
    Docker host ports                  none               8080               none     3000,3001,8081
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                     ❌ down               ✅ up               ✅ up               ✅ up
    Containers (up/total)                0/0                6/8                7/8              48/56

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔓 gcp-proxy          35.226.147.64      ports: 22, 80, 443, 465, 587, 993
🔒 gcp-t4             34.173.227.250     ports: none reachable
🔓 oci-apps           82.70.229.129      ports: 22
🔓 oci-mail           130.110.251.193    ports: 25, 465, 587, 993, 8080
🔒 oci-analytics      129.151.228.66     ports: none reachable

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
    Network                      Services
    ────────────────────────────────────────────────────────────
    auth-net                     authelia
    default                      radicale
    etherpad_net                 etherpad

VAULT — Providers
────────────────────────────────────────────────────────────
  🔑 authelia             🔑 aws                  🔑 c3-api              
  🔑 cloudflare-wrangler  🔑 cloudflare           🔑 crawlee             
  🔑 gcloud               🔑 github               🔑 gpg                 
  🔑 nocodb               🔑 oci                  🔑 resend              
  🔑 ssh-s21              🔑 ssh-surface-pro      🔑 system              
  🔑 wireguard            🔑 anthropic            🔑 vaultwarden         


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
    OCI                  ~/git/cloud/b_infra/vps_oci/src/main.tf
    GCP                  ~/git/cloud/b_infra/vps_gcloud/src/main.tf
    Cloudflare           ~/git/cloud/a_solutions/ba-clo_cloudflare/src/main.tf


══════════════════════════════════════════════════════════════
  Z) APPENDIX
══════════════════════════════════════════════════════════════

PERFORMANCE
────────────────────────────────────────────────────────────
  TOTAL                60.7s
  mail_dns             55.0s
  vm_ssh               22.0s
  public_urls          8.0s
  mesh                 7.7s
  port_scan            5.0s
  private              0.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  60.7s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
