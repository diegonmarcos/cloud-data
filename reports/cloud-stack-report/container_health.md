```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-04-06T04:38:51.059651967+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
46 critical, 2 warnings — 48 total

    ❌ A3       gcp-proxy/authelia — exited
    ❌ A3       gcp-proxy/hickory-dns — exited
    ❌ A3       oci-apps/c3-services-mcp — exited
    ❌ A3       oci-apps/nocodb — exited
    ❌ A3       oci-apps/nocodb-db — exited
    ⚠️ A3       oci-apps/trusting_herschel — unhealthy
    ⚠️ A3       oci-apps/photoprism_app — unhealthy
    ❌ A3       VM oci-analytics — UNREACHABLE
    ❌ A1       auth.diegonmarcos.com — [err: error sending request for url (http://auth.diegonmarcos.com/)]
    ❌ A1       git.diegonmarcos.com — [err: error sending request for url (http://git.diegonmarcos.com/)]
    ❌ A1       api.diegonmarcos.com/c3-api — [err: error sending request for url (http://api.diegonmarcos.com/c3-api)]
    ❌ A1       mcp.diegonmarcos.com/c3-infra-mcp — [err: error sending request for url (http://mcp.diegonmarcos.com/c3-infra-mcp)]
    ❌ A1       api.diegonmarcos.com/services — [err: error sending request for url (http://api.diegonmarcos.com/services)]
    ❌ A1       proxy.diegonmarcos.com — [err: error sending request for url (http://proxy.diegonmarcos.com/)]
    ❌ A1       ide.diegonmarcos.com — [err: error sending request for url (http://ide.diegonmarcos.com/)]
    ❌ A1       api.diegonmarcos.com — [err: error sending request for url (http://api.diegonmarcos.com/)]
    ❌ A1       workflows.diegonmarcos.com — [err: error sending request for url (http://workflows.diegonmarcos.com/)]
    ❌ A1       db.diegonmarcos.com — [err: error sending request for url (http://db.diegonmarcos.com/)]
    ❌ A1       logs.diegonmarcos.com — [err: error sending request for url (http://logs.diegonmarcos.com/)]
    ❌ A1       pad.diegonmarcos.com — [err: error sending request for url (http://pad.diegonmarcos.com/)]
    ❌ A1       files.diegonmarcos.com — [err: error sending request for url (http://files.diegonmarcos.com/)]
    ❌ A1       sheets.diegonmarcos.com — [err: error sending request for url (http://sheets.diegonmarcos.com/)]
    ❌ A1       doc.diegonmarcos.com — [err: error sending request for url (http://doc.diegonmarcos.com/)]
    ❌ A1       dns.internal — [err: error sending request for url (http://dns.internal/)]
    ❌ A1       grafana.diegonmarcos.com — [err: error sending request for url (http://grafana.diegonmarcos.com/)]
    ❌ A1       mail.diegonmarcos.com — [err: error sending request for url (http://mail.diegonmarcos.com/)]
    ❌ A1       analytics.diegonmarcos.com — [err: error sending request for url (http://analytics.diegonmarcos.com/)]
    ❌ A1       chat.diegonmarcos.com — [err: error sending request for url (http://chat.diegonmarcos.com/)]
    ❌ A1       rss.diegonmarcos.com — [err: error sending request for url (http://rss.diegonmarcos.com/)]
    ❌ A1       photos.diegonmarcos.com — [err: error sending request for url (http://photos.diegonmarcos.com/)]
    ❌ A1       cal.diegonmarcos.com — [err: error sending request for url (http://cal.diegonmarcos.com/)]
    ❌ A1       smtp.diegonmarcos.com — [err: error sending request for url (http://smtp.diegonmarcos.com/)]
    ❌ A1       webmail.diegonmarcos.com — [err: error sending request for url (http://webmail.diegonmarcos.com/)]
    ❌ A1       vault.diegonmarcos.com — [err: error sending request for url (http://vault.diegonmarcos.com/)]
    ❌ A1       windmill.diegonmarcos.com — [err: error sending request for url (http://windmill.diegonmarcos.com/)]
    ❌ A1       api.diegonmarcos.com/dash — [err: error sending request for url (http://api.diegonmarcos.com/dash)]
    ❌ A1       api.diegonmarcos.com/crawlee — [err: error sending request for url (http://api.diegonmarcos.com/crawlee)]
    ❌ A1       app.diegonmarcos.com/windmill — [err: error sending request for url (http://app.diegonmarcos.com/windmill)]
    ❌ A1       app.diegonmarcos.com/etherpad — [err: error sending request for url (http://app.diegonmarcos.com/etherpad)]
    ❌ A1       app.diegonmarcos.com/filebrowser — [err: error sending request for url (http://app.diegonmarcos.com/filebrowser)]
    ❌ A1       app.diegonmarcos.com/hedgedoc — [err: error sending request for url (http://app.diegonmarcos.com/hedgedoc)]
    ❌ A1       app.diegonmarcos.com/revealmd — [err: error sending request for url (http://app.diegonmarcos.com/revealmd)]
    ❌ A1       app.diegonmarcos.com/dozzle — [err: error sending request for url (http://app.diegonmarcos.com/dozzle)]
    ❌ A1       app.diegonmarcos.com/grafana — [err: error sending request for url (http://app.diegonmarcos.com/grafana)]
    ❌ A1       app.diegonmarcos.com/gitea — [err: error sending request for url (http://app.diegonmarcos.com/gitea)]
    ❌ A1       app.diegonmarcos.com/crawlee — [err: error sending request for url (http://app.diegonmarcos.com/crawlee)]
    ❌ A1       cloud.diegonmarcos.com — [err: error sending request for url (http://cloud.diegonmarcos.com/)]
    ❌ A1       mcp.diegonmarcos.com — [err: error sending request for url (http://mcp.diegonmarcos.com/)]


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
⚠️ auth.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.1:9091          [err: error sending request for url (http://auth.diegonmarcos.com/)] auth:[err: error sending request for url (https://auth.diegonmarcos.com/)]
⚠️ git.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:3002          [err: error sending request for url (http://git.diegonmarcos.com/)] auth:[err: error sending request for url (https://git.diegonmarcos.com/)]
❌ api.diegonmarcos.com/c3-api      ❌  ❌  ❌  ❌  10.0.0.6:8081          [err: error sending request for url (http://api.diegonmarcos.com/c3-api)] auth:[err: error sending request for url (https://api.diegonmarcos.com/c3-api)]
❌ mcp.diegonmarcos.com/c3-infra-mcp ❌  ❌  ❌  ❌  10.0.0.6:3100          [err: error sending request for url (http://mcp.diegonmarcos.com/c3-infra-mcp)] auth:[err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp)]
❌ api.diegonmarcos.com/services    ❌  ❌  ❌  ❌  10.0.0.6:8082          [err: error sending request for url (http://api.diegonmarcos.com/services)] auth:[err: error sending request for url (https://api.diegonmarcos.com/services)]
⚠️ proxy.diegonmarcos.com           ✅  ❌  ❌  ❌  10.0.0.1:443           [err: error sending request for url (http://proxy.diegonmarcos.com/)] auth:[err: error sending request for url (https://proxy.diegonmarcos.com/)]
⚠️ ide.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:8443          [err: error sending request for url (http://ide.diegonmarcos.com/)] auth:[err: error sending request for url (https://ide.diegonmarcos.com/)]
⚠️ api.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:3000          [err: error sending request for url (http://api.diegonmarcos.com/)] auth:[err: error sending request for url (https://api.diegonmarcos.com/)]
⚠️ workflows.diegonmarcos.com       ✅  ❌  ❌  ❌  10.0.0.3:8070          [err: error sending request for url (http://workflows.diegonmarcos.com/)] auth:[err: error sending request for url (https://workflows.diegonmarcos.com/)]
⚠️ db.diegonmarcos.com              ✅  ❌  ❌  ❌  10.0.0.6:8086          [err: error sending request for url (http://db.diegonmarcos.com/)] auth:[err: error sending request for url (https://db.diegonmarcos.com/)]
⚠️ logs.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.4:9999          [err: error sending request for url (http://logs.diegonmarcos.com/)] auth:[err: error sending request for url (https://logs.diegonmarcos.com/)]
⚠️ pad.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:3012          [err: error sending request for url (http://pad.diegonmarcos.com/)] auth:[err: error sending request for url (https://pad.diegonmarcos.com/)]
⚠️ files.diegonmarcos.com           ✅  ❌  ❌  ❌  10.0.0.6:3015          [err: error sending request for url (http://files.diegonmarcos.com/)] auth:[err: error sending request for url (https://files.diegonmarcos.com/)]
⚠️ sheets.diegonmarcos.com          ✅  ❌  ❌  ❌  10.0.0.6:3011          [err: error sending request for url (http://sheets.diegonmarcos.com/)] auth:[err: error sending request for url (https://sheets.diegonmarcos.com/)]
⚠️ doc.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:3018          [err: error sending request for url (http://doc.diegonmarcos.com/)] auth:[err: error sending request for url (https://doc.diegonmarcos.com/)]
❌ dns.internal                     ❌  ❌  ❌  ❌  10.0.0.1:53            [err: error sending request for url (http://dns.internal/)] auth:[err: error sending request for url (https://dns.internal/)]
⚠️ grafana.diegonmarcos.com         ✅  ❌  ❌  ❌  10.0.0.6:3200          [err: error sending request for url (http://grafana.diegonmarcos.com/)] auth:[err: error sending request for url (https://grafana.diegonmarcos.com/)]
⚠️ mail.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.3:443           [err: error sending request for url (http://mail.diegonmarcos.com/)] auth:[err: error sending request for url (https://mail.diegonmarcos.com/)]
⚠️ analytics.diegonmarcos.com       ✅  ❌  ❌  ❌  10.0.0.4:8084          [err: error sending request for url (http://analytics.diegonmarcos.com/)] auth:[err: error sending request for url (https://analytics.diegonmarcos.com/)]
⚠️ chat.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.6:8065          [err: error sending request for url (http://chat.diegonmarcos.com/)] auth:[err: error sending request for url (https://chat.diegonmarcos.com/)]
⚠️ rss.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.1:8090          [err: error sending request for url (http://rss.diegonmarcos.com/)] auth:[err: error sending request for url (https://rss.diegonmarcos.com/)]
⚠️ photos.diegonmarcos.com          ✅  ❌  ❌  ❌  10.0.0.6:3013          [err: error sending request for url (http://photos.diegonmarcos.com/)] auth:[err: error sending request for url (https://photos.diegonmarcos.com/)]
⚠️ cal.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:5232          [err: error sending request for url (http://cal.diegonmarcos.com/)] auth:[err: error sending request for url (https://cal.diegonmarcos.com/)]
⚠️ smtp.diegonmarcos.com            ✅  ❌  ❌  ❌  10.0.0.3:8080          [err: error sending request for url (http://smtp.diegonmarcos.com/)] auth:[err: error sending request for url (https://smtp.diegonmarcos.com/)]
⚠️ webmail.diegonmarcos.com         ✅  ❌  ❌  ❌  10.0.0.3:8888          [err: error sending request for url (http://webmail.diegonmarcos.com/)] auth:[err: error sending request for url (https://webmail.diegonmarcos.com/)]
⚠️ vault.diegonmarcos.com           ✅  ❌  ❌  ❌  10.0.0.1:8880          [err: error sending request for url (http://vault.diegonmarcos.com/)] auth:[err: error sending request for url (https://vault.diegonmarcos.com/)]
⚠️ windmill.diegonmarcos.com        ✅  ❌  ❌  ❌  10.0.0.6:8000          [err: error sending request for url (http://windmill.diegonmarcos.com/)] auth:[err: error sending request for url (https://windmill.diegonmarcos.com/)]
❌ api.diegonmarcos.com/dash        ❌  ❌  ❌  ❌  diegonmarcos.github.io [err: error sending request for url (http://api.diegonmarcos.com/dash)] auth:[err: error sending request for url (https://api.diegonmarcos.com/dash)]
❌ api.diegonmarcos.com/crawlee     ❌  ❌  ❌  ❌  10.0.0.6:3000          [err: error sending request for url (http://api.diegonmarcos.com/crawlee)] auth:[err: error sending request for url (https://api.diegonmarcos.com/crawlee)]
❌ app.diegonmarcos.com/windmill    ❌  ❌  ❌  ❌  10.0.0.6:8000          [err: error sending request for url (http://app.diegonmarcos.com/windmill)] auth:[err: error sending request for url (https://app.diegonmarcos.com/windmill)]
❌ app.diegonmarcos.com/etherpad    ❌  ❌  ❌  ❌  10.0.0.6:3012          [err: error sending request for url (http://app.diegonmarcos.com/etherpad)] auth:[err: error sending request for url (https://app.diegonmarcos.com/etherpad)]
❌ app.diegonmarcos.com/filebrowser ❌  ❌  ❌  ❌  10.0.0.6:3015          [err: error sending request for url (http://app.diegonmarcos.com/filebrowser)] auth:[err: error sending request for url (https://app.diegonmarcos.com/filebrowser)]
❌ app.diegonmarcos.com/hedgedoc    ❌  ❌  ❌  ❌  10.0.0.6:3018          [err: error sending request for url (http://app.diegonmarcos.com/hedgedoc)] auth:[err: error sending request for url (https://app.diegonmarcos.com/hedgedoc)]
❌ app.diegonmarcos.com/revealmd    ❌  ❌  ❌  ❌  10.0.0.6:3014          [err: error sending request for url (http://app.diegonmarcos.com/revealmd)] auth:[err: error sending request for url (https://app.diegonmarcos.com/revealmd)]
❌ app.diegonmarcos.com/dozzle      ❌  ❌  ❌  ❌  10.0.0.4:9999          [err: error sending request for url (http://app.diegonmarcos.com/dozzle)] auth:[err: error sending request for url (https://app.diegonmarcos.com/dozzle)]
❌ app.diegonmarcos.com/grafana     ❌  ❌  ❌  ❌  10.0.0.6:3016          [err: error sending request for url (http://app.diegonmarcos.com/grafana)] auth:[err: error sending request for url (https://app.diegonmarcos.com/grafana)]
❌ app.diegonmarcos.com/gitea       ❌  ❌  ❌  ❌  10.0.0.6:3017          [err: error sending request for url (http://app.diegonmarcos.com/gitea)] auth:[err: error sending request for url (https://app.diegonmarcos.com/gitea)]
❌ app.diegonmarcos.com/crawlee     ❌  ❌  ❌  ❌  10.0.0.6:3001          [err: error sending request for url (http://app.diegonmarcos.com/crawlee)] auth:[err: error sending request for url (https://app.diegonmarcos.com/crawlee)]
⚠️ cloud.diegonmarcos.com           ✅  ❌  ❌  ❌  10.0.0.6:3080          [err: error sending request for url (http://cloud.diegonmarcos.com/)] auth:[err: error sending request for url (https://cloud.diegonmarcos.com/)]
⚠️ mcp.diegonmarcos.com             ✅  ❌  ❌  ❌  10.0.0.6:3100          [err: error sending request for url (http://mcp.diegonmarcos.com/)] auth:[err: error sending request for url (https://mcp.diegonmarcos.com/)]

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
⏸️ alerts-api.app               ⏸️   ⏸️    5050 oci-analytics    alerts-api             [---]
⏸️ dozzle.app                   ⏸️   ⏸️    9999 oci-analytics    dozzle                 [---]
⏸️ matomo.app                   ⏸️   ⏸️    8084 oci-analytics    matomo-hybrid          [---]
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
⏸️ mattermost-mcp.app           ⏸️   ⏸️    3102 oci-apps         mattermost-mcp         [---]
⏸️ mattermost-postgres.app      ⏸️   ⏸️    5435 oci-apps         mattermost-postgres    [---]
⏸️ mattermost.app               ⏸️   ⏸️    8065 oci-apps         mattermost             [---]
⏸️ ollama-hai.app               ⏸️   ⏸️   11435 oci-apps         ollama-hai             [---]
⏸️ photoprism.app               ⏸️   ⏸️    3013 oci-apps         photoprism_app         [---]
⏸️ quant-full-db.app            ⏸️   ⏸️    5437 oci-apps         quant_full_db          [---]
⏸️ quant-full-research.app      ⏸️   ⏸️    8890 oci-apps         quant_full_research    [---]
⏸️ quant-light-db.app           ⏸️   ⏸️    5443 oci-apps         quant_light_db         [---]
⏸️ quant-light-engine.app       ⏸️   ⏸️    5001 oci-apps         quant_light_engine     [---]
⏸️ quant-light-research.app     ⏸️   ⏸️    8889 oci-apps         quant_light_research   [---]
⏸️ radicale.app                 ⏸️   ⏸️    5232 oci-apps         radicale               [---]
⏸️ revealmd.app                 ⏸️   ⏸️    3014 oci-apps         revealmd_app           [---]
⏸️ rig-agentic-sonn-14bq8.app   ⏸️   ⏸️    8091 oci-apps         rig-agentic-sonn-14bq8 [---]
⏸️ windmill-app.app             ⏸️   ⏸️    8000 oci-apps         windmill-server        [---]
⏸️ windmill-db.app              ⏸️   ⏸️    5440 oci-apps         windmill-db            [---]
⏸️ dagu.app                     ⏸️   ⏸️    8070 oci-mail         dagu                   [---]
⏸️ maddy.app                    ⏸️   ⏸️     443 oci-mail         maddy                  [---]
⏸️ smtp-proxy.app               ⏸️   ⏸️    8080 oci-mail         smtp-proxy             [---]
⏸️ snappymail.app               ⏸️   ⏸️    8888 oci-mail         snappymail             [---]

  📡 TCP: 0/60  🌐 HTTP: 0/60

── A3) Containers ────────────────────────────────────────────

gcp-proxy ✅ — 2C/2G — mem 1075M/1952M (55%) — disk 74% — swap 135M/3999M — load 0.10 0.08 0.02 — 16/18 ctrs — 5d 7h
────────────────────────────────────────────────────────────
  ❌ authelia                  —      9091   DOWN(1)        Exited (1) 9 minutes ago
  ❌ hickory-dns               —      53     DOWN(1)        Exited (1) 17 hours ago
  ⚠️ authelia-redis            —      6380   UP (no hc)     Up 9 minutes
  ⚠️ syslog-bridge             —      —      UP (no hc)     Up 17 hours
  ⚠️ github-rss                —      —      UP (no hc)     Up 17 hours
  ⚠️ ntfy                      —      8090   UP (no hc)     Up 17 hours
  ⚠️ caddy                     443    443    UP (no hc)     Up 19 hours
  ⚠️ sqlite-vaultwarden        —      —      UP (no hc)     Up 4 days
  ⚠️ sqlite-ntfy               —      —      UP (no hc)     Up 4 days
  ⚠️ sqlite-npm                —      —      UP (no hc)     Up 4 days
  ⚠️ sqlite-authelia           —      —      UP (no hc)     Up 4 days
  ⚠️ postlite-vaultwarden      —      —      UP (no hc)     Up 4 days
  ⚠️ postlite-ntfy             —      —      UP (no hc)     Up 4 days
  ⚠️ postlite-npm              —      —      UP (no hc)     Up 4 days
  ⚠️ postlite-authelia         —      —      UP (no hc)     Up 4 days
  ✅ introspect-proxy          —      4182   HEALTHY        Up 19 hours (healthy)
  ✅ redis                     —      6379   HEALTHY        Up 4 days (healthy)
  ✅ vaultwarden               —      8880   HEALTHY        Up 4 days (healthy)

oci-apps ✅ — 4C/24G — mem 5268M/23975M (21%) — disk 60% — swap 150M/12288M — load 0.25 0.42 0.38 — 50/53 ctrs — 7d 14h
────────────────────────────────────────────────────────────
  ❌ c3-services-mcp           —      3101   DOWN(134)      Exited (134) 5 hours ago
  ❌ nocodb                    —      —      DOWN(143)      Exited (143) 17 hours ago
  ❌ nocodb-db                 —      —      DOWN(0)        Exited (0) 17 hours ago
  ❌ trusting_herschel         —      —      UNHEALTHY      Up 2 days (unhealthy)
  ❌ photoprism_app            —      3013   UNHEALTHY      Up 4 days (unhealthy)
  ⚠️ mattermost-bots           —      —      UP (no hc)     Up 43 hours
  ⚠️ mattermost-mcp            —      3102   UP (no hc)     Up 2 days
  ⚠️ mail-mcp                  —      3103   UP (no hc)     Up 43 hours
  ⚠️ crawlee_runner            —      —      UP (no hc)     Up 6 days
  ⚠️ crawlee_dashboard         3001   3001   UP (no hc)     Up 6 days
  ⚠️ crawlee_scheduler         —      —      UP (no hc)     Up 6 days
  ⚠️ quant_light_engine        —      5001   UP (no hc)     Up 6 days
  ⚠️ windmill-worker           —      —      UP (no hc)     Up 7 days
  ⚠️ lgtm_mimir                —      9009   UP (no hc)     Up 7 days
  ⚠️ lgtm_tempo                —      3210   UP (no hc)     Up 7 days
  ⚠️ gitea                     —      3002   UP (no hc)     Up 7 days
  ⚠️ bup-server                —      —      UP (no hc)     Up 7 days
  ⚠️ borg-server               —      —      UP (no hc)     Up 7 days
  ⚠️ cloud-spec                —      3080   UP (no hc)     Up 7 days
  ⚠️ siem-api                  —      —      UP (no hc)     Up 7 days
  ⚠️ code-server               —      8443   UP (no hc)     Up 7 days
  ✅ dbgate                    —      8086   HEALTHY        Up 17 hours (healthy)
  ✅ c3-infra-mcp              —      3100   HEALTHY        Up 43 hours (healthy)
  ✅ c3-infra-api              8081   8081   HEALTHY        Up 3 days (healthy)
  ✅ rig-agentic-hai           —      —      HEALTHY        Up 5 days (healthy)
  ✅ crawlee_api               3000   3000   HEALTHY        Up 6 days (healthy)
  ✅ crawlee_redis             —      6381   HEALTHY        Up 6 days (healthy)
  ✅ crawlee_minio             —      9000   HEALTHY        Up 6 days (healthy)
  ✅ crawlee_db                —      5433   HEALTHY        Up 6 days (healthy)
  ✅ quant_light_research      —      8889   HEALTHY        Up 6 days (healthy)
  ✅ quant_light_db            —      5443   HEALTHY        Up 6 days (healthy)
  ✅ windmill-server           —      8000   HEALTHY        Up 5 days (healthy)
  ✅ windmill-db               —      5440   HEALTHY        Up 7 days (healthy)
  ✅ photoprism_mariadb        —      —      HEALTHY        Up 7 days (healthy)
  ✅ photoprism_rclone         —      —      HEALTHY        Up 7 days (healthy)
  ✅ lgtm_grafana              —      3200   HEALTHY        Up 7 days (healthy)
  ✅ lgtm_loki                 —      3110   HEALTHY        Up 7 days (healthy)
  ✅ ollama-hai                —      11435  HEALTHY        Up 7 days (healthy)
  ✅ mattermost                —      8065   HEALTHY        Up 7 days (healthy)
  ✅ mattermost-postgres       —      5435   HEALTHY        Up 7 days (healthy)
  ✅ hedgedoc_app              —      3018   HEALTHY        Up 7 days (healthy)
  ✅ hedgedoc_postgres         —      5439   HEALTHY        Up 7 days (healthy)
  ✅ etherpad_app              —      3012   HEALTHY        Up 7 days (healthy)
  ✅ etherpad_postgres         —      5436   HEALTHY        Up 7 days (healthy)
  ✅ google-workspace-mcp      —      3104   HEALTHY        Up 7 days (healthy)
  ✅ cloud-cgc-mcp             —      3105   HEALTHY        Up 7 days (healthy)
  ✅ syslog-central            —      —      HEALTHY        Up 7 days (healthy)
  ✅ photos-webhook            —      —      HEALTHY        Up 7 days (healthy)
  ✅ photos-db                 —      —      HEALTHY        Up 7 days (healthy)
  ✅ revealmd_app              —      3014   HEALTHY        Up 7 days (healthy)
  ✅ radicale                  —      5232   HEALTHY        Up 7 days (healthy)
  ✅ grist_app                 —      3011   HEALTHY        Up 7 days (healthy)
  ✅ filebrowser_app           —      3015   HEALTHY        Up 7 days (healthy)

oci-mail ✅ — 1C/1G — mem 624M/954M (65%) — disk 78% — swap 217M/2559M — load 0.00 0.09 0.09 — 4/4 ctrs — 5d 10h
────────────────────────────────────────────────────────────
  ⚠️ dagu                      —      8070   UP (no hc)     Up 40 hours
  ⚠️ maddy                     —      443    UP (no hc)     Up 43 hours
  ⚠️ smtp-proxy                8080   8080   UP (no hc)     Up 4 days
  ✅ snappymail                —      8888   HEALTHY        Up 42 hours (healthy)

oci-analytics ❌ — 1C/1G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────


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
  📨 INBOUND: Gmail → MX → CF Email Routing → CF Worker → Caddy → smtp-proxy → Maddy
     ✅ smtp-proxy           Up 4 days
     ✅ maddy                Up 43 hours

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
CPU                55 cores      21 cores      65 cores      ? cores       
RAM                1075M/1952M   5268M/23975M  624M/954M     ?/?           
RAM %              55%           21%           65%           0%            
Swap               135M/3999M    150M/12288M   217M/2559M    ?             
Disk               23G/31G       54.2G/95.8G   33G/45G       ?/?           
Disk %             74%           60%           78%           ?             
Load               0.10 0.08 0.020.25 0.42 0.380.00 0.09 0.09?             
Containers         16/18         50/53         4/4           0/0           
Uptime             5d 7h         7d 14h        5d 10h        ?             

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
    Scanned (public)     22,443,465,587,993             🔒 none             🔒 none             🔒 none
    Docker host ports                   443               8080               none     3000,3001,8081
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                       ✅ up               ✅ up             ❌ down               ✅ up
    Containers (up/total)              16/18                4/4                0/0              50/53

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
   ntfy                 ?          ntfy                   /var/cache/ntfy/cache.db gcp-proxy        ntfy.app:8090
   vaultwarden          ?          vaultwarden            /data/db.sqlite3 gcp-proxy        vaultwarden.app:8880
   umami                postgres   umami-db               umami          oci-analytics    umami-db.app:5442
   crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps         crawlee-db.app:5433
   etherpad             postgres   etherpad_postgres      etherpad       oci-apps         etherpad-db.app:5436
   gitea                ?          gitea                  /data/gitea/gitea.db oci-apps         backup-gitea.app:3002
   grist                ?          grist_app              /persist/grist-sessions.db oci-apps         grist.app:3011
   hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps         hedgedoc-db.app:5439
   mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps         mattermost-postgres.app:5435
   photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps         embedded
   quant-lab-light      postgres   quant_light_db         quantlab       oci-apps         quant-light-db.app:5443
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
  TOTAL                69.9s
  mail_dns             55.0s
  public_urls          25.4s
  mesh                 16.8s
  vm_ssh               16.8s
  port_scan            5.0s
  private              0.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  69.9s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
