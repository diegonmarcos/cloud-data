```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-03-29  02:26:26
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
17 critical, 6 warnings — 23 total

        Section  Issue
    ──────────────────────────────────────────────────────────────────────
    ❌ A2       VM oci-mail — UNREACHABLE
    ❌ A2       VM oci-analytics — UNREACHABLE
    ❌ A2       VM gcp-t4 — UNREACHABLE
    ❌ A2       VM gcp-proxy — UNREACHABLE
    ❌ A2       oci-apps/surrealdb — exited(2)
    ❌ A2       oci-apps/crawlee_minio_init — exited(0)
    ❌ A2       oci-apps/crawlee_dashboard — exited(0)
    ❌ A1       dns.internal — [---]
    ❌ A3       Stalwart SPF FAIL — VM IP 130.110.251.193 not in SPF (outbound emails rejected)
    ❌ SYS      SSH unreachable: oci-mail
    ❌ SYS      SSH unreachable: oci-analytics
    ❌ SYS      SSH unreachable: gcp-t4
    ❌ SYS      SSH unreachable: gcp-proxy
    ❌ SYS        ❌ oci-mail: UNREACHABLE
    ❌ SYS        ❌ oci-analytics: UNREACHABLE
    ❌ SYS        ❌ gcp-t4: UNREACHABLE
    ❌ SYS        ❌ gcp-proxy: UNREACHABLE
    ⚠️ A2       oci-apps/crawlee_db — unhealthy
    ⚠️ A3       mail.diegonmarcos.com:25 SMTP — down
    ⚠️ A3       mail.diegonmarcos.com:4190 ManageSieve — down
    ⚠️ A3       smtp.diegonmarcos.com:25 SMTP — down
    ⚠️ A3       mails.diegonmarcos.com:25 MX (Resend/SES) — down
    ⚠️ A3       send.mails.diegonmarcos.com:25 SPF (Resend/SES) — down


══════════════════════════════════════════════════════════════
  A) HEALTH — Live checks
══════════════════════════════════════════════════════════════

── A0) Mesh ──────────────────────────────────────────────────

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1 — front door)
────────────────────────────────────────────────────────────
    Name           Cloud Name         ☁VPS 🌐Pub 🔒WG  Public IP          WG IP          Type     Handshake
────────────────────────────────────────────────────────────
⚠️ oci-mail       oci-E2-f_0         ❌  ✅  ❌  130.110.251.193    10.0.0.3       VM       no data
⚠️ oci-analytics  oci-E2-f_1         ❌  ✅  ❌  129.151.228.66     10.0.0.4       VM       no data
⚠️ oci-apps       oci-A1-f_0         ✅  ✅  ❌  82.70.229.129      10.0.0.6       VM       no data
❌ gcp-t4         ollama-spot-gpu    ❌  ❌  ❌  34.173.227.250     10.0.0.8       VM       no data
⚠️ gcp-proxy      arch-1             ✅  ✅  ❌  35.226.147.64      10.0.0.1       HUB      no data
⚠️ surface        —                  ✅  ✅  ❌  dynamic            10.0.0.5       CLIENT   no data
⚠️ termux         —                  ✅  ✅  ❌  dynamic            10.0.0.9       CLIENT   no data

── A1) Public ────────────────────────────────────────────────

PUBLIC URLs (Caddy routes)
────────────────────────────────────────────────────────────
    URL                              📡TCP 🌐HTTP 🔒HTTPS 🔐AUTH Upstream                  Code
────────────────────────────────────────────────────────────
❌ ide.diegonmarcos.com             ❌  ❌  ❌  —  code-server.app:8443   [---] 
❌ pad.diegonmarcos.com             ❌  ❌  ❌  —  etherpad.app:3012      [---] 
❌ files.diegonmarcos.com           ❌  ❌  ❌  —  filebrowser.app:3015   [---] 
❌ sheets.diegonmarcos.com          ❌  ❌  ❌  —  grist.app:3011         [---] 
❌ doc.diegonmarcos.com             ❌  ❌  ❌  —  hedgedoc.app:3018      [---] 
❌ chat.diegonmarcos.com            ❌  ❌  ❌  —  mattermost.app:8065    [---] 
❌ photos.diegonmarcos.com          ❌  ❌  ❌  —  photoprism.app:3013    [---] 
❌ cal.diegonmarcos.com             ❌  ❌  ❌  —  radicale.app:5232      [---] 
❌ slides.diegonmarcos.com          ❌  ❌  ❌  —  revealmd.app:3014      [---] 
❌ webmail.diegonmarcos.com         ❌  ❌  ❌  —  snappymail.app:8888    [---] 
✅ mail.diegonmarcos.com            ✅  ✅  ✅  ✅  stalwart.app:443       [200] 
✅ vault.diegonmarcos.com           ✅  ✅  ✅  ✅  vaultwarden.app:8880   [200] 
✅ api.diegonmarcos.com             ✅  ✅  ✅  ✅  crawlee.app:3000       [200] 
❌ dns.internal                     ❌  ❌  ❌  —  hickory-dns.app:53     [000] 
✅ auth.diegonmarcos.com            ✅  ✅  ✅  ✅  authelia.app:9091      [200] 
✅ proxy.diegonmarcos.com           ✅  ✅  ✅  ✅  caddy.app:443          [302] 
⚠️ api.diegonmarcos.com/c3-api      ❌  ✅  ✅  ✅  c3-infra-api.app:8081  [404] 
⚠️ mcp.diegonmarcos.com/c3-infra-mcp ❌  ✅  ✅  ✅  c3-infra-mcp.app:3100  [200] 
⚠️ api.diegonmarcos.com/services    ❌  ✅  ✅  ✅  c3-services-api.app:8082 [404] 
✅ workflows.diegonmarcos.com       ✅  ✅  ✅  ✅  dagu.app:8070          [302] 
✅ logs.diegonmarcos.com            ✅  ✅  ✅  ✅  dozzle.app:9999        [200] 
✅ grafana.diegonmarcos.com         ✅  ✅  ✅  ✅  grafana.app:3200       [200] 
✅ analytics.diegonmarcos.com       ✅  ✅  ✅  ✅  matomo.app:8080        [302] 
⚠️ db.diegonmarcos.com              ✅  ✅  ✅  ❌  nocodb.app:8085        [302] auth:[502]
⚠️ rss.diegonmarcos.com             ✅  ✅  ✅  ❌  ntfy.app:8090          [302] auth:[401]
✅ windmill.diegonmarcos.com        ✅  ✅  ✅  ✅  windmill-app.app:8000  [200] 
✅ git.diegonmarcos.com             ✅  ✅  ✅  ✅  backup-gitea.app:3002  [200] 
⚠️ api.diegonmarcos.com/dash        ❌  ✅  ✅  ✅  diegonmarcos.github.io [301] 
⚠️ api.diegonmarcos.com/crawlee     ❌  ✅  ✅  ✅  crawlee.app:3000       [404] 
⚠️ app.diegonmarcos.com/windmill    ❌  ✅  ✅  ✅  windmill-app.app:8000  [404] 
⚠️ app.diegonmarcos.com/etherpad    ❌  ✅  ✅  ✅  etherpad.app:3012      [404] 
⚠️ app.diegonmarcos.com/filebrowser ❌  ✅  ✅  ✅  filebrowser.app:3015   [404] 
⚠️ app.diegonmarcos.com/hedgedoc    ❌  ✅  ✅  ✅  hedgedoc.app:3018      [404] 
⚠️ app.diegonmarcos.com/revealmd    ❌  ✅  ✅  ✅  revealmd.app:3014      [404] 
⚠️ app.diegonmarcos.com/dozzle      ❌  ✅  ✅  ✅  dozzle.app:9999        [404] 
⚠️ app.diegonmarcos.com/grafana     ❌  ✅  ✅  ✅  grafana.app:3016       [404] 
⚠️ app.diegonmarcos.com/gitea       ❌  ✅  ✅  ✅  gitea.app:3017         [404] 
⚠️ app.diegonmarcos.com/crawlee     ❌  ✅  ✅  ✅  crawlee.app:3001       [404] 
✅ cloud.diegonmarcos.com           ✅  ✅  ✅  ✅  c3-spec.app:3080       [200] 
✅ mcp.diegonmarcos.com             ✅  ✅  ✅  ✅  c3-infra-mcp.app:3100  [200] 
✅ app.diegonmarcos.com             ✅  ✅  ✅  ✅  path-based             [404] 

API / MCP ENDPOINTS
────────────────────────────────────────────────────────────


REPOS & REGISTRIES
────────────────────────────────────────────────────────────
  GIT REPOS (github.com)
    Repo           URL                                              Status
    ──────────────────────────────────────────────────────────────────────
    ✅ cloud        github.com/diegonmarcos/cloud                  [200] main ⚠️dirty
    ✅ cloud-data   github.com/diegonmarcos/cloud-data             [200] main ⚠️dirty
    ❌ front        github.com/diegonmarcos/front                  [404] main
    ✅ unix         github.com/diegonmarcos/unix                   [200] main ⚠️dirty
    ❌ tools        github.com/diegonmarcos/tools                  [404] main ⚠️dirty
    ❌ vault        github.com/diegonmarcos/vault                  [404] main ⚠️dirty

  CONTAINER REGISTRY (ghcr.io)
    Image                                    Status
    ──────────────────────────────────────────────────
    ✅ ghcr.io/diegonmarcos/caddy-custom      74 images (cloud)
    ✅ ghcr.io/diegonmarcos/dozzle            8 images (cloud-data)
    ✅ ghcr.io/diegonmarcos/diego-user-env    2 images (unix)
    📦 Total: 84 container images

── A2) Private (WireGuard mesh — .app health) ────────────────

    DNS Name                     📡TCP 🌐HTTP Port    VM               Container              Code
    ───────────────────────────────────────────────────────────────────────────────────────────────
❌ authelia-redis.app           ❌   ❌     6380 gcp-proxy        authelia-redis         [---]
❌ authelia.app                 ❌   ❌     9091 gcp-proxy        authelia               [---]
❌ caddy.app                    ❌   ❌   ⚠️443  gcp-proxy        caddy                  [---]
❌ hickory-dns.app              ❌   ❌     53   gcp-proxy        hickory-dns            [---]
❌ introspect-proxy.app         ❌   ❌   ⚠️4182 gcp-proxy        introspect-proxy       [---]
❌ introspect-proxy.app         ❌   ❌   ⚠️4182 gcp-proxy        introspect-proxy       [---]
❌ ntfy.app                     ❌   ❌     8090 gcp-proxy        ntfy                   [---]
❌ redis.app                    ❌   ❌     6379 gcp-proxy        redis                  [---]
❌ vaultwarden.app              ❌   ❌     8880 gcp-proxy        vaultwarden            [---]
❌ ollama.app                   ❌   ❌     11434 gcp-t4           ollama                 [---]
✅ dozzle.app                   ✅   ✅     9999 oci-analytics    dozzle                 [200]
✅ matomo.app                   ✅   ✅     8080 oci-analytics    matomo-hybrid          [503]
⚠️ umami-db.app                 ✅   ❌     5442 oci-analytics    umami-db               [000]
✅ umami.app                    ✅   ✅     3006 oci-analytics    umami                  [200]
❌ backup-gitea.app             ❌   ❌     3002 oci-apps         gitea                  [---]
❌ c3-infra-api.app             ❌   ❌     8081 oci-apps         c3-infra-api           [---]
❌ c3-infra-mcp.app             ❌   ❌     3100 oci-apps         c3-infra-mcp           [---]
❌ c3-services-api.app          ❌   ❌     8082 oci-apps         c3-services-api        [---]
❌ c3-services-mcp.app          ❌   ❌     3101 oci-apps         c3-services-mcp        [---]
❌ c3-spec.app                  ❌   ❌     3080 oci-apps         cloud-spec             [---]
❌ cloud-cgc-mcp.app            ❌   ❌     3105 oci-apps         cloud-cgc-mcp          [---]
❌ code-server.app              ❌   ❌     8443 oci-apps         code-server            [---]
❌ crawlee-dashboard.app        ❌   ❌     3001 oci-apps         crawlee_dashboard      [---]
❌ crawlee-db.app               ❌   ❌     5433 oci-apps         crawlee_db             [---]
❌ crawlee-minio.app            ❌   ❌     9000 oci-apps         crawlee_minio          [---]
❌ crawlee-redis.app            ❌   ❌     6381 oci-apps         crawlee_redis          [---]
❌ crawlee.app                  ❌   ❌     3000 oci-apps         crawlee_api            [---]
❌ etherpad-db.app              ❌   ❌     5436 oci-apps         etherpad_postgres      [---]
❌ etherpad.app                 ❌   ❌     3012 oci-apps         etherpad_app           [---]
❌ filebrowser.app              ❌   ❌     3015 oci-apps         filebrowser_app        [---]
❌ g-workspace-mcp.app          ❌   ❌     3104 oci-apps         google-workspace-mcp   [---]
❌ gitea.app                    ❌   ❌     3017 oci-apps         gitea                  [---]
❌ grafana.app                  ❌   ❌     3200 oci-apps         lgtm_grafana           [---]
❌ grist.app                    ❌   ❌     3011 oci-apps         grist_app              [---]
❌ hedgedoc-db.app              ❌   ❌     5439 oci-apps         hedgedoc_postgres      [---]
❌ hedgedoc.app                 ❌   ❌     3018 oci-apps         hedgedoc_app           [---]
❌ lgtm-loki.app                ❌   ❌     3110 oci-apps         lgtm_loki              [---]
❌ lgtm-mimir.app               ❌   ❌     9009 oci-apps         lgtm_mimir             [---]
❌ lgtm-tempo.app               ❌   ❌     3210 oci-apps         lgtm_tempo             [---]
❌ mail-mcp.app                 ❌   ❌     3103 oci-apps         mail-mcp               [---]
❌ mattermost-mcp.app           ❌   ❌     3102 oci-apps         mattermost-mcp         [---]
❌ mattermost-postgres.app      ❌   ❌     5435 oci-apps         mattermost-postgres    [---]
❌ mattermost.app               ❌   ❌     8065 oci-apps         mattermost             [---]
❌ nocodb-db.app                ❌   ❌     5441 oci-apps         nocodb-db              [---]
❌ nocodb.app                   ❌   ❌     8085 oci-apps         nocodb                 [---]
❌ ollama-hai.app               ❌   ❌     11435 oci-apps         ollama-hai             [---]
❌ photoprism.app               ❌   ❌     3013 oci-apps         photoprism_app         [---]
❌ quant-full-db.app            ❌   ❌     5437 oci-apps         quant_full_db          [---]
❌ quant-full-research.app      ❌   ❌     8890 oci-apps         quant_full_research    [---]
❌ quant-light-db.app           ❌   ❌     5434 oci-apps         quant_light_db         [---]
❌ quant-light-engine.app       ❌   ❌     5001 oci-apps         quant_light_engine     [---]
❌ quant-light-research.app     ❌   ❌     8889 oci-apps         quant_light_research   [---]
❌ radicale.app                 ❌   ❌     5232 oci-apps         radicale               [---]
❌ revealmd.app                 ❌   ❌     3014 oci-apps         revealmd_app           [---]
❌ windmill-app.app             ❌   ❌     8000 oci-apps         windmill-server        [---]
❌ windmill-db.app              ❌   ❌     5440 oci-apps         windmill-db            [---]
✅ dagu.app                     ✅   ✅     8070 oci-mail         dagu                   [200]
⚠️ snappymail.app               ✅   ❌     8888 oci-mail         snappymail             [---]
⚠️ stalwart.app                 ✅   ❌   ⚠️443  oci-mail         stalwart               [000]

  ⚠️  PORT CONFLICTS (2 duplicate ports globally):
     :443    used by: caddy.app, stalwart.app
     :4182   used by: introspect-proxy.app, introspect-proxy.app

  ─── DNS CONFIG CHECK ───
  ❌ /etc/resolv.conf     MISSING 10.0.0.1 — .app names won't resolve!
     nameserver 8.8.8.8
     nameserver 8.8.4.4
  ❌ dig authelia-redis.app   NXDOMAIN (system DNS)
  ✅ dig @10.0.0.1 authelia-redis.app 10.0.0.1 (Hickory direct)
  ⚠️  System DNS can't resolve .app — add 10.0.0.1 to resolv.conf
     All checks above test via system DNS — if Hickory not configured, all fail

  📡 TCP: 7/59  🌐 HTTP: 4/59

── A3) Containers ────────────────────────────────────────────

oci-mail ❌ — oci-mail — 1C/?G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-analytics ❌ — oci-analytics — 1C/?G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-apps ✅ — oci-apps — 4C/24G — mem 4918M/23975M (20%) — disk 71% — swap 0M/0M — load 0.47 0.41 0.43 — 49/54 ctrs — up 1d 8h
────────────────────────────────────────────────────────────
  ❌ crawlee_runner            —      —      DOWN(?)        Created
  ❌ crawlee_api               —      3000   DOWN(?)        Created
  ❌ surrealdb                 —      —      DOWN(2)        Exited (2) 4 hours ago
  ❌ crawlee_minio_init        —      —      DOWN(0)        Exited (0) 4 hours ago
  ❌ crawlee_dashboard         —      3001   DOWN(0)        Exited (0) 4 hours ago
  ❌ crawlee_db                —      5433   UNHEALTHY      Up 45 seconds (unhealthy)
  ⚠️ photoprism_app            —      3013   STARTING       Up About a minute (health: sta
  ⚠️ mail-mcp                  —      3103   UP (no hc)     Up 52 seconds
  ⚠️ windmill-worker           —      —      UP (no hc)     Up 3 hours
  ⚠️ mattermost-bots           —      —      UP (no hc)     Up 3 hours
  ⚠️ crawlee_scheduler         —      —      UP (no hc)     Up 4 hours
  ⚠️ gitea                     —      3002   UP (no hc)     Up 7 hours
  ⚠️ bup-server                —      —      UP (no hc)     Up 7 hours
  ⚠️ lgtm_tempo                —      3210   UP (no hc)     Up 7 hours
  ⚠️ lgtm_mimir                —      9009   UP (no hc)     Up 7 hours
  ⚠️ borg-server               —      —      UP (no hc)     Up 7 hours
  ⚠️ cloud-spec                —      3080   UP (no hc)     Up 7 hours
  ⚠️ siem-api                  —      —      UP (no hc)     Up 7 hours
  ⚠️ quant_light_engine        —      5001   UP (no hc)     Up 7 hours
  ⚠️ mattermost-mcp            —      3102   UP (no hc)     Up 7 hours
  ⚠️ code-server               —      8443   UP (no hc)     Up 7 hours
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 51 seconds (healthy)
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up About a minute (healthy)
  ✅ etherpad_app              —      3012   HEALTHY        Up About a minute (healthy)
  ✅ etherpad_postgres         —      5436   HEALTHY        Up About a minute (healthy)
  ✅ google-workspace-mcp      —      3104   HEALTHY        Up 2 minutes (healthy)
  ✅ c3-infra-mcp              —      3100   HEALTHY        Up 2 hours (healthy)
  ✅ windmill-server           —      8000   HEALTHY        Up 3 hours (healthy)
  ✅ windmill-db               —      5440   HEALTHY        Up 3 hours (healthy)
  ✅ quant_light_db            —      5434   HEALTHY        Up 3 hours (healthy)
  ✅ photoprism_rclone         —      —      HEALTHY        Up 3 hours (healthy)
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 3 hours (healthy)
  ✅ nocodb                    —      8085   HEALTHY        Up 3 hours (healthy)
  ✅ nocodb-db                 —      5441   HEALTHY        Up 3 hours (healthy)
  ✅ crawlee_minio             —      9000   HEALTHY        Up 4 hours (healthy)
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 4 hours (healthy)
  ✅ c3-infra-api              —      8081   HEALTHY        Up 4 hours (healthy)
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 7 hours (healthy)
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 7 hours (healthy)
  ✅ c3-services-mcp           —      3101   HEALTHY        Up 7 hours (healthy)
  ✅ syslog-central            —      —      HEALTHY        Up 7 hours (healthy)
  ✅ crawlee_redis             —      6381   HEALTHY        Up 7 hours (healthy)
  ✅ rig-agentic-sonn-14bq8    —      —      HEALTHY        Up 7 hours (healthy)
  ✅ rig-agentic-hai           —      —      HEALTHY        Up 7 hours (healthy)
  ✅ photos-webhook            —      —      HEALTHY        Up 7 hours (healthy)
  ✅ photos-db                 —      —      HEALTHY        Up 7 hours (healthy)
  ✅ ollama-hai                —      11435  HEALTHY        Up 7 hours (healthy)
  ✅ quant_light_research      —      8889   HEALTHY        Up 7 hours (healthy)
  ✅ revealmd_app              —      3014   HEALTHY        Up 7 hours (healthy)
  ✅ radicale                  —      5232   HEALTHY        Up 7 hours (healthy)
  ✅ mattermost                —      8065   HEALTHY        Up 7 hours (healthy)
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 7 hours (healthy)
  ✅ grist_app                 —      3011   HEALTHY        Up 7 hours (healthy)
  ✅ filebrowser_app           —      3015   HEALTHY        Up 7 hours (healthy)

gcp-t4 ❌ — gcp-t4 — 0C/?G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

gcp-proxy ❌ — gcp-proxy — 0C/?G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────


── A4) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
❌ mail.diegonmarcos.com        :25    SMTP            down
⚠️ mail.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ mail.diegonmarcos.com        :587   Submission      tcp open
⚠️ mail.diegonmarcos.com        :993   IMAPS           tcp open
❌ mail.diegonmarcos.com        :4190  ManageSieve     down
❌ smtp.diegonmarcos.com        :25    SMTP            down
⚠️ smtp.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ smtp.diegonmarcos.com        :587   Submission      tcp open
⚠️ imap.diegonmarcos.com        :993   IMAPS           tcp open
❌ mails.diegonmarcos.com       :25    MX (Resend/SES) down
❌ send.mails.diegonmarcos.com  :25    SPF (Resend/SES) down

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  162.159.205.12
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.25
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.19
✅ send.mails.diegonmarcos.com  10    feedback-smtp.us-east-1.amazonses.com.     34.192.233.193
❌ mails.diegonmarcos.com       —     no MX record
  ─── checks ───
  ✅ Cloudflare Email Routing active (3 MX routes for diegonmarcos.com)
  ✅ Resend bounce handler (send.mails MX → SES feedback)
  ❌ No MX for mails.diegonmarcos.com (normal — Resend is API-only, no inbound)

SPF — Outbound Policy: IP Allowlist (dig TXT)
────────────────────────────────────────────────────────────
    Domain                           Include                                       Resolved IPs
────────────────────────────────────────────────────────────
✅ diegonmarcos.com                 include:_spf.mx.cloudflare.net                ip4:104.30.0.0/19
✅ diegonmarcos.com                 include:amazonses.com                         ip4:199.255.192.0/22, ip4:199.127.232.0/22, ip4:54.240.0.0/18
✅ diegonmarcos.com                 include:eu.rp.oracleemaildelivery.com         ip4:192.29.200.0/25, ip4:138.1.108.0/25, ip4:130.35.116.0/25
✅ send.mails.diegonmarcos.com      include:amazonses.com                         (same as above)
⚠️ diegonmarcos.com                 oci-mail VM IP 130.110.251.193 NOT IN SPF!
  ─── checks ───
  ✅ SPF record exists for diegonmarcos.com
  ✅ SPF record exists for send.mails.diegonmarcos.com
  ✅ All includes resolve successfully
  ❌ oci-mail VM IP 130.110.251.193 NOT in any SPF range!
     → Stalwart sends directly from this IP — receivers will SPF FAIL
     → FIX: add ip4:130.110.251.193 to SPF or relay via OCI Email Delivery

DKIM — Outbound Policy: Cryptographic Signatures (dig TXT)
────────────────────────────────────────────────────────────
    Selector                     Domain                   Signer               Key Size
────────────────────────────────────────────────────────────
✅ dkim._domainkey              diegonmarcos.com         Stalwart             RSA 1024
❌ mail._domainkey              diegonmarcos.com         Legacy Mailu         NOT FOUND
✅ google._domainkey            diegonmarcos.com         Google Workspace     RSA 1024
✅ cf2024-1._domainkey          diegonmarcos.com         Cloudflare           RSA 1024
❌ resend._domainkey.mails      diegonmarcos.com         Resend/SES           NOT FOUND
  ─── checks ───
  ⚠️ All 5 DKIM selectors — some missing!
  ⚠️ dkim._domainkey uses RSA 1024 — weaker than 2048 (provider limitation)
  ⚠️ google._domainkey uses RSA 1024 — weaker than 2048 (provider limitation)
  ⚠️ cf2024-1._domainkey uses RSA 1024 — weaker than 2048 (provider limitation)

DMARC — Outbound Policy: Receiver Instructions (dig TXT)
────────────────────────────────────────────────────────────
✅ _dmarc.diegonmarcos.com       "v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1"
  ─── checks ───
  ✅ Policy: p=reject (strictest — good)
  ✅ Aggregate reports: mailto:postmaster@diegonmarcos.com
  ✅ Forensic reports: mailto:postmaster@diegonmarcos.com
  ✅ Subdomain policy: sp=reject

MAIL AUTH — Authorized Senders
────────────────────────────────────────────────────────────
    Sender               Domain                     Auth Method      SPF IP Range                   DKIM Selector
────────────────────────────────────────────────────────────
✅ Cloudflare           diegonmarcos.com           Email Routing    104.30.0.0/19                  cf2024-1._domainkey
⚠️ Stalwart             diegonmarcos.com           Direct SMTP      130.110.251.193 NOT IN SPF!    dkim._domainkey
✅ Google               diegonmarcos.com           Google SMTP      (via google include)           google._domainkey
⚠️ Legacy Mailu         diegonmarcos.com           DECOMMISSIONED   —                              mail._domainkey
✅ Resend/SES           mails.diegonmarcos.com     API + SES        54.240.0.0/18                  resend._dk.mails
✅ OCI Email Dlv        diegonmarcos.com           SMTP Relay       192.29.200.0/25                (via Stalwart)
  ─── checks ───
  ❌ Stalwart: SPF will FAIL — IP 130.110.251.193 not in any SPF include
  ⚠️ Stalwart: not configured to relay via OCI Email Delivery
  ⚠️ Legacy Mailu: stale DKIM key in DNS
  ✅ Resend/Cloudflare/Google: fully authorized
  ✅ OCI Email Delivery: in SPF range, but Stalwart not using as relay

MAIL FLOW — Pipeline Status
────────────────────────────────────────────────────────────

  📨 INBOUND EMAIL: someone@gmail.com → me@diegonmarcos.com
     Gmail → MX → CF Email Routing → CF Worker → oci-mail:8080 → smtp-proxy → Stalwart
     ─────────────────────────────────────────────
     ❌ smtp-proxy           not found (oci-mail:8080)
     ✅ oci-mail:8080        reachable (CF Worker ingress)
     ✅ oci-mail:25          SMTP open (Stalwart local delivery)
     ❌ stalwart             not found (oci-mail MTA)

  📱 CLIENT ACCESS: phone/Thunderbird → read/send mail via Caddy L4
     Client → gcp-proxy (35.226.147.64) → Caddy L4 TLS passthrough → oci-mail (130.110.251.193) → Stalwart
     ─────────────────────────────────────────────
     ✅ webmail.diegonmarcos.com     [200] (Snappymail)
     ✅ mail.diegonmarcos.com        [200] (Stalwart admin)

  📤 OUTBOUND PERSONAL: me@diegonmarcos.com → someone@gmail.com
     Stalwart → ⚠️ direct from 130.110.251.193 (NOT IN SPF!) → recipient MX
     ─────────────────────────────────────────────
     ❌ stalwart             not found (oci-mail MTA)
     ✅ smtp:465 (SMTPS)     open (client → gcp-proxy L4 → stalwart)
     ✅ smtp:587 (Submission) open (client → gcp-proxy L4 → stalwart)
     ❌ SPF WILL FAIL        VM IP 130.110.251.193 not in SPF
     ✅ DKIM OK              dkim._domainkey present
     ❌ DMARC RESULT         p=reject + SPF fail = REJECTED

  📤 OUTBOUND TRANSACTIONAL: noreply@mails.diegonmarcos.com → someone@gmail.com
     App → Resend API → Amazon SES (us-east-1) → recipient MX
     ─────────────────────────────────────────────
     ✅ api.resend.com       [401]
     ✅ SPF OK  ✅ DKIM OK  ✅ DMARC OK
     ✅ Terraform            ~/git/cloud/b_infra/vps_resend/src/main.tf


══════════════════════════════════════════════════════════════
  B) INFRA — Resources & Stack
══════════════════════════════════════════════════════════════

VPS / VM SPECS (all providers)
────────────────────────────────────────────────────────────
    VM               Provider   Shape                CPU    RAM    Disk     Cost
────────────────────────────────────────────────────────────
   oci-mail         ?          VM.Standard.E2.1.Micro 1      ?G     ?G       ?
   oci-analytics    ?          VM.Standard.E2.1.Micro 1      ?G     ?G       ?
   oci-apps         ?          VM.Standard.A1.Flex  4      24G    ?G       ?
   gcp-t4           ?          n1-standard-4        ?      ?G     ?G       ?
   gcp-proxy        ?          e2-small             ?      ?G     ?G       ?
   vast-ollama      ?          ?                    ?      ?G     ?G       ?
   gha-cloud        GitHub     ubuntu-latest (x86)  4      16G    14G      2000min/mo
   gha-cloud-data   GitHub     ubuntu-latest (x86)  4      16G    14G      2000min/mo
   gha-front        GitHub     ubuntu-latest (x86)  4      16G    14G      2000min/mo
   gha-unix         GitHub     ubuntu-latest (x86)  4      16G    14G      2000min/mo
   gha-tools        GitHub     ubuntu-latest (x86)  4      16G    14G      2000min/mo

RESOURCES (live)
────────────────────────────────────────────────────────────
                   oci-mail       oci-analytics  oci-apps       gcp-t4         gcp-proxy     
────────────────────────────────────────────────────────────
OS                 oci-mail       oci-analytics  oci-apps       gcp-t4         gcp-proxy     
CPU                1 cores        1 cores        4 cores        0 cores        0 cores       
RAM                ?/?            ?/?            4918M/23975M   ?/?            ?/?           
RAM %              0%             0%             20%            0%             0%            
Swap               ?              ?              0M/0M          ?              ?             
Disk               ?/?            ?/?            64.8G/95.8G    ?/?            ?/?           
Disk %             ?              ?              71%            ?              ?             
Load               ?              ?              0.47 0.41 0.43 ?              ?             
Containers         0/0            0/0            49/54          0/0            0/0           
Uptime             ?              ?              1d 8h          ?              ?             

STORAGE
────────────────────────────────────────────────────────────
  OBJECT STORAGE
    Provider       Type                 Details
    ────────────────────────────────────────────────────────────
    oci            Object Storage       archlinux-images (Standard)
    oci            Object Storage       my-photos (Standard)
    GitHub         Container Registry   ghcr.io/diegonmarcos/ (84 images)

  DATA / FILES (git repositories)
    Repo           Path                                     Purpose
    ─────────────────────────────────────────────────────────────────
    cloud          ~/git/cloud                              Services, infra, HM, workflows
    cloud-data     ~/git/cloud/cloud-data                   Generated config, topology, manifests
    front          ~/git/front                              32 front-end projects
    unix           ~/git/unix                               NixOS host, HM desktop/termux
    tools          ~/git/tools                              CLI tools, scripts
    vault          ~/git/vault                              Credentials, keys, 2FA, IDs

  DOCKER VOLUMES (persistent, named)
    VM               Volume                         Service
    ────────────────────────────────────────────────────────────
    gcp-proxy        vaultwarden_data               ?
    oci-analytics    matomo_data                    ?
    oci-apps         grist_data                     ?
    oci-apps         mattermost_data                ?
    oci-apps         mattermost_postgres            ?
    oci-apps         photoprism_originals           ?
    oci-apps         photoprism_storage             ?
    oci-apps         nocodb_data                    ?
    oci-apps         gitea_data                     ?
    oci-mail         stalwart_data                  ?

  DATABASES
    Total: 14 — 2 sqlite, 3 ?, 8 postgres, 1 mariadb
    Service              Type       Container              DB Name        VM
    ───────────────────────────────────────────────────────────────────────────
    authelia             sqlite     authelia               /config/db.sqlite3 gcp-proxy
    ntfy                 ?          ntfy                   /var/cache/ntfy/cache.db gcp-proxy
    vaultwarden          sqlite     vaultwarden            /data/db.sqlite3 gcp-proxy
    umami                postgres   umami-db               umami          oci-analytics
    crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps
    etherpad             postgres   etherpad_postgres      etherpad       oci-apps
    gitea                ?          gitea                  /data/gitea/gitea.db oci-apps
    grist                ?          grist_app              /persist/grist-sessions.db oci-apps
    hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps
    mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps
    nocodb               postgres   nocodb-db              nocodb         oci-apps
    photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps
    quant-lab-light      postgres   quant_light_db         quantlab       oci-apps
    windmill             postgres   windmill-db            windmill       oci-apps


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔒 oci-mail           130.110.251.193    ports: none reachable
🔒 oci-analytics      129.151.228.66     ports: none reachable
🔒 oci-apps           82.70.229.129      ports: none reachable
🔒 gcp-t4             34.173.227.250     ports: none reachable
🔒 gcp-proxy          35.226.147.64      ports: none reachable

BACKUPS / DATABASES
────────────────────────────────────────────────────────────
    Service              DB Type    Container              DB Name        VM               DNS / Access
    ──────────────────────────────────────────────────────────────────────────────────────────
   authelia             sqlite     authelia               /config/db.sqlite3 gcp-proxy        authelia.app:9091
   ntfy                 ?          ntfy                   /var/cache/ntfy/cache.db gcp-proxy        ntfy.app:8090
   vaultwarden          sqlite     vaultwarden            /data/db.sqlite3 gcp-proxy        vaultwarden.app:8880
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
    Network                      VM               Services
    ──────────────────────────────────────────────────────────────────────
    auth-net                     gcp-E2-f_0       authelia
    default                      oci-A1-f_0       radicale
    etherpad_net                 oci-A1-f_0       etherpad

VAULT — Providers
────────────────────────────────────────────────────────────
  🔑 anthropic           🔑 authelia            🔑 aws                
  🔑 c3-api              🔑 cloudflare          🔑 cloudflare-wrangler
  🔑 crawlee             🔑 gcloud              🔑 github             
  🔑 gpg                 🔑 nocodb              🔑 oci                
  🔑 resend              🔑 ssh-s21             🔑 ssh-surface-pro    
  🔑 system              🔑 vaultwarden         🔑 wireguard          


══════════════════════════════════════════════════════════════
  D) STACK — Framework & Paths
══════════════════════════════════════════════════════════════

FRAMEWORK — Key Paths
────────────────────────────────────────────────────────────
  BUILD ENGINES
    Service engine       ~/git/cloud/a_solutions/_engine.sh
    HM engine            ~/git/cloud/b_infra/home-manager/_engine.sh
    Workflow engine      ~/git/cloud/workflows/build.sh
    Front engine         ~/git/front/1.ops/build_main.sh
    NixOS host           ~/git/unix/aa_nixos-surface_host/build.sh
    HM desktop           ~/git/unix/ba_flakes_desktop/build.sh

  HOME-MANAGER FLAKES
    Shared modules       ~/git/cloud/b_infra/home-manager/_shared/modules/
    oci-mail             ~/git/cloud/b_infra/home-manager/oci-mail/src/
    oci-analytics        ~/git/cloud/b_infra/home-manager/oci-analytics/src/
    oci-apps             ~/git/cloud/b_infra/home-manager/oci-apps/src/
    gcp-t4               ~/git/cloud/b_infra/home-manager/gcp-t4/src/
    gcp-proxy            ~/git/cloud/b_infra/home-manager/gcp-proxy/src/

  GHA WORKFLOWS
    ship-oci-mail        ~/git/cloud/.github/workflows/ship-oci-mail.yml
    ship-oci-analytics   ~/git/cloud/.github/workflows/ship-oci-analytics.yml
    ship-oci-apps        ~/git/cloud/.github/workflows/ship-oci-apps.yml
    ship-gcp-t4          ~/git/cloud/.github/workflows/ship-gcp-t4.yml
    ship-gcp-proxy       ~/git/cloud/.github/workflows/ship-gcp-proxy.yml
    ship-home-manager    ~/git/cloud/.github/workflows/ship-home-manager.yml
    ship-ghcr            ~/git/cloud/.github/workflows/ship-ghcr.yml
    Templates            ~/git/cloud/workflows/src/

  DAGU WORKFLOWS
    DAGs source          ~/git/cloud/a_solutions/bc-obs_dagu/src/dags/
    deploy-pull-up       ~/git/cloud/a_solutions/bc-obs_dagu/src/dags/ops_deploy-pull-up.yaml
    cloud-data sync      ~/git/cloud/a_solutions/bc-obs_dagu/src/dags/sync_cloud-data.yaml

  DATA
    cloud-data           ~/git/cloud/cloud-data/
    Container manifests  ~/git/cloud/cloud-data/cloud-data-containers-{vm}.json
    Topology             ~/git/cloud/cloud-data/cloud-data-topology.json
    GHA config           ~/git/cloud/cloud-data/cloud-data-gha-config.json
    Consolidated         ~/git/cloud/cloud-data/_cloud-data-consolidated.json

  TERRAFORM
    OCI                  ~/git/cloud/b_infra/vps_oci/src/main.tf
    GCP                  ~/git/cloud/b_infra/vps_gcloud/src/main.tf
    Cloudflare           ~/git/cloud/a_solutions/ba-clo_cloudflare/src/main.tf

  SECURITY
    Vault                ~/git/vault/
    SOPS secrets         ~/git/cloud/a_solutions/*/src/secrets.yaml
    SSH keys             ~/git/vault/A0_keys/ssh/


══════════════════════════════════════════════════════════════
  Z) APPENDIX
══════════════════════════════════════════════════════════════

PERFORMANCE
────────────────────────────────────────────────────────────
  private_health       40.7s ████████
  public_urls          38.2s ███████
  public_urls_multi    28.0s █████
  mail_ports           13.5s ███
  open_ports           12.5s ██
  vm_gcp-proxy          8.4s ██
  vm_oci-mail           8.1s ██
  vm_oci-analytics      8.1s ██
  vm_gcp-t4             8.0s ██
  vm_oci-apps           4.3s █
  api_mcp               0.0s 
  TOTAL               156.8s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Script:    cloud-data/cloud-health-stack/container-health.ts
  Run:       ./container-health.ts  (or: tsx container-health.ts)
  Node:      v20.19.1
  Platform:  linux x64
  CWD:       /home/diego/Mounts/Git/cloud-data/cloud-health-stack
  Template:  container_health.md.tpl
  Data src:  /home/diego/Mounts/Git/cloud-data/

  Dependencies:
    ✅ ssh        /home/diego/.nix-profile/bin/ssh
    ✅ curl       /home/diego/.nix-profile/bin/curl
    ✅ nc         /home/diego/.nix-profile/bin/nc
    ✅ dig        /home/diego/.nix-profile/bin/dig
    ✅ git        /home/diego/.nix-profile/bin/git
    ✅ gh         /home/diego/.nix-profile/bin/gh

  Errors:    8
    [02:27:26] ERROR: SSH unreachable: oci-mail
    [02:27:34] ERROR: SSH unreachable: oci-analytics
    [02:27:47] ERROR: SSH unreachable: gcp-t4
    [02:27:55] ERROR: SSH unreachable: gcp-proxy
    [02:27:55] ERROR:   ❌ oci-mail: UNREACHABLE
    [02:27:55] ERROR:   ❌ oci-analytics: UNREACHABLE
    [02:27:55] ERROR:   ❌ gcp-t4: UNREACHABLE
    [02:27:55] ERROR:   ❌ gcp-proxy: UNREACHABLE

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
