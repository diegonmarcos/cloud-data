```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-04-16T11:02:58.998779864+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
45 critical, 0 warnings — 45 total

    ❌ A3       VM gcp-proxy — UNREACHABLE
    ❌ A3       VM oci-apps — UNREACHABLE
    ❌ A3       VM oci-mail — UNREACHABLE
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
    ❌ A1       mta-sts.diegonmarcos.com — [301]


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
❌ auth.diegonmarcos.com            ❌  ❌  ❌  ❌  10.0.0.1:9091          [err: error sending request for url (http://auth.diegonmarcos.com/)] auth:[err: error sending request for url (https://auth.diegonmarcos.com/)]
❌ git.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:3002          [err: error sending request for url (http://git.diegonmarcos.com/)] auth:[err: error sending request for url (https://git.diegonmarcos.com/)]
❌ api.diegonmarcos.com/c3-api      ❌  ❌  ❌  ❌  10.0.0.6:8081          [err: error sending request for url (http://api.diegonmarcos.com/c3-api)] auth:[err: error sending request for url (https://api.diegonmarcos.com/c3-api)]
❌ mcp.diegonmarcos.com/c3-infra-mcp ❌  ❌  ❌  ❌  10.0.0.6:3100          [err: error sending request for url (http://mcp.diegonmarcos.com/c3-infra-mcp)] auth:[err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp)]
❌ api.diegonmarcos.com/services    ❌  ❌  ❌  ❌  10.0.0.6:8082          [err: error sending request for url (http://api.diegonmarcos.com/services)] auth:[err: error sending request for url (https://api.diegonmarcos.com/services)]
❌ proxy.diegonmarcos.com           ❌  ❌  ❌  ❌  10.0.0.1:443           [err: error sending request for url (http://proxy.diegonmarcos.com/)] auth:[err: error sending request for url (https://proxy.diegonmarcos.com/)]
❌ ide.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:8443          [err: error sending request for url (http://ide.diegonmarcos.com/)] auth:[err: error sending request for url (https://ide.diegonmarcos.com/)]
❌ api.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:3000          [err: error sending request for url (http://api.diegonmarcos.com/)] auth:[err: error sending request for url (https://api.diegonmarcos.com/)]
❌ workflows.diegonmarcos.com       ❌  ❌  ❌  ❌  10.0.0.4:8070          [err: error sending request for url (http://workflows.diegonmarcos.com/)] auth:[err: error sending request for url (https://workflows.diegonmarcos.com/)]
❌ db.diegonmarcos.com              ❌  ❌  ❌  ❌  10.0.0.6:8086          [err: error sending request for url (http://db.diegonmarcos.com/)] auth:[err: error sending request for url (https://db.diegonmarcos.com/)]
❌ logs.diegonmarcos.com            ❌  ❌  ❌  ❌  10.0.0.4:9999          [err: error sending request for url (http://logs.diegonmarcos.com/)] auth:[err: error sending request for url (https://logs.diegonmarcos.com/)]
❌ pad.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:3012          [err: error sending request for url (http://pad.diegonmarcos.com/)] auth:[err: error sending request for url (https://pad.diegonmarcos.com/)]
❌ files.diegonmarcos.com           ❌  ❌  ❌  ❌  10.0.0.6:3015          [err: error sending request for url (http://files.diegonmarcos.com/)] auth:[err: error sending request for url (https://files.diegonmarcos.com/)]
❌ sheets.diegonmarcos.com          ❌  ❌  ❌  ❌  10.0.0.6:3011          [err: error sending request for url (http://sheets.diegonmarcos.com/)] auth:[err: error sending request for url (https://sheets.diegonmarcos.com/)]
❌ doc.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:3018          [err: error sending request for url (http://doc.diegonmarcos.com/)] auth:[err: error sending request for url (https://doc.diegonmarcos.com/)]
❌ dns.internal                     ❌  ❌  ❌  ❌  10.0.0.1:53            [err: error sending request for url (http://dns.internal/)] auth:[err: error sending request for url (https://dns.internal/)]
❌ grafana.diegonmarcos.com         ❌  ❌  ❌  ❌  10.0.0.6:3200          [err: error sending request for url (http://grafana.diegonmarcos.com/)] auth:[err: error sending request for url (https://grafana.diegonmarcos.com/)]
❌ mail.diegonmarcos.com            ❌  ❌  ❌  ❌  10.0.0.3:443           [err: error sending request for url (http://mail.diegonmarcos.com/)] auth:[err: error sending request for url (https://mail.diegonmarcos.com/)]
❌ analytics.diegonmarcos.com       ❌  ❌  ❌  ❌  10.0.0.4:8084          [err: error sending request for url (http://analytics.diegonmarcos.com/)] auth:[err: error sending request for url (https://analytics.diegonmarcos.com/)]
❌ chat.diegonmarcos.com            ❌  ❌  ❌  ❌  10.0.0.6:8065          [err: error sending request for url (http://chat.diegonmarcos.com/)] auth:[err: error sending request for url (https://chat.diegonmarcos.com/)]
❌ rss.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:8090          [err: error sending request for url (http://rss.diegonmarcos.com/)] auth:[err: error sending request for url (https://rss.diegonmarcos.com/)]
❌ photos.diegonmarcos.com          ❌  ❌  ❌  ❌  10.0.0.6:3013          [err: error sending request for url (http://photos.diegonmarcos.com/)] auth:[err: error sending request for url (https://photos.diegonmarcos.com/)]
❌ cal.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:5232          [err: error sending request for url (http://cal.diegonmarcos.com/)] auth:[err: error sending request for url (https://cal.diegonmarcos.com/)]
❌ smtp.diegonmarcos.com            ❌  ❌  ❌  ❌  10.0.0.3:8080          [err: error sending request for url (http://smtp.diegonmarcos.com/)] auth:[err: error sending request for url (https://smtp.diegonmarcos.com/)]
❌ webmail.diegonmarcos.com         ❌  ❌  ❌  ❌  10.0.0.3:8888          [err: error sending request for url (http://webmail.diegonmarcos.com/)] auth:[err: error sending request for url (https://webmail.diegonmarcos.com/)]
❌ vault.diegonmarcos.com           ❌  ❌  ❌  ❌  10.0.0.6:8880          [err: error sending request for url (http://vault.diegonmarcos.com/)] auth:[err: error sending request for url (https://vault.diegonmarcos.com/)]
❌ windmill.diegonmarcos.com        ❌  ❌  ❌  ❌  10.0.0.6:8000          [err: error sending request for url (http://windmill.diegonmarcos.com/)] auth:[err: error sending request for url (https://windmill.diegonmarcos.com/)]
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
❌ cloud.diegonmarcos.com           ❌  ❌  ❌  ❌  10.0.0.6:3080          [err: error sending request for url (http://cloud.diegonmarcos.com/)] auth:[err: error sending request for url (https://cloud.diegonmarcos.com/)]
❌ mcp.diegonmarcos.com             ❌  ❌  ❌  ❌  10.0.0.6:3100          [err: error sending request for url (http://mcp.diegonmarcos.com/)] auth:[err: error sending request for url (https://mcp.diegonmarcos.com/)]
⚠️ mta-sts.diegonmarcos.com         ✅  ✅  ❌  ❌  static                 [301] auth:[521]

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
⚠️ authelia-redis.app           ✅   ❌    6380 gcp-proxy        authelia-redis         [err: error sending request for url (http://10.0.0.1:6380/)]
✅ authelia.app                 ✅   ✅    9091 gcp-proxy        authelia               [200]
❌ caddy.app                    ❌   ❌     443 gcp-proxy        caddy                  [err: error sending request for url (http://10.0.0.1:443/)]
⚠️ hickory-dns.app              ✅   ❌      53 gcp-proxy        hickory-dns            [err: error sending request for url (http://10.0.0.1:53/)]
❌ introspect-proxy.app         ❌   ❌    4182 gcp-proxy        introspect-proxy       [err: error sending request for url (http://10.0.0.1:4182/)]
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
✅ quant-full-research.app      ✅   ✅    8890 oci-apps         quant_full_research    [404]
❌ quant-light-db.app           ❌   ❌    5443 oci-apps         quant_light_db         [err: error sending request for url (http://10.0.0.1:5443/)]
❌ quant-light-engine.app       ❌   ❌    5001 oci-apps         quant_light_engine     [err: error sending request for url (http://10.0.0.1:5001/)]
❌ quant-light-research.app     ❌   ❌    8889 oci-apps         quant_light_research   [err: error sending request for url (http://10.0.0.1:8889/)]
❌ radicale.app                 ❌   ❌    5232 oci-apps         radicale               [err: error sending request for url (http://10.0.0.1:5232/)]
❌ revealmd.app                 ❌   ❌    3014 oci-apps         revealmd_app           [err: error sending request for url (http://10.0.0.1:3014/)]
❌ rig-agentic-sonn-14bq8.app   ❌   ❌    8091 oci-apps         rig-agentic-sonn-14bq8 [err: error sending request for url (http://10.0.0.1:8091/)]
✅ vaultwarden.app              ✅   ✅    8880 oci-apps         vaultwarden            [200]
❌ windmill-app.app             ❌   ❌    8000 oci-apps         windmill-server        [err: error sending request for url (http://10.0.0.1:8000/)]
❌ windmill-db.app              ❌   ❌    5440 oci-apps         windmill-db            [err: error sending request for url (http://10.0.0.1:5440/)]
❌ maddy.app                    ❌   ❌     443 oci-mail         maddy                  [err: error sending request for url (http://10.0.0.1:443/)]
❌ smtp-proxy.app               ❌   ❌    8080 oci-mail         smtp-proxy             [err: error sending request for url (http://10.0.0.1:8080/)]
❌ snappymail.app               ❌   ❌    8888 oci-mail         snappymail             [err: error sending request for url (http://10.0.0.1:8888/)]

  📡 TCP: 10/59  🌐 HTTP: 4/59

── A3) Containers ────────────────────────────────────────────

gcp-proxy ❌ — 2C/2G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-apps ❌ — 4C/24G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-mail ❌ — 1C/1G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

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
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.24
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  162.159.205.11
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.19
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
     ❌ smtp-proxy           not found
     ❌ maddy                not found

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
CPU                ? cores       ? cores       ? cores       ? cores       
RAM                ?/?           ?/?           ?/?           ?/?           
RAM %              0%            0%            0%            0%            
Swap               ?             ?             ?             ?             
Disk               ?/?           ?/?           ?/?           ?/?           
Disk %             ?             ?             ?             ?             
Load               ?             ?             ?             ?             
Containers         0/0           0/0           0/0           0/0           
Uptime             ?             ?             ?             ?             

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
    Scanned (public)                     22             🔒 none             🔒 none             🔒 none
    Docker host ports                  none               none               none               none
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                     ❌ down             ❌ down             ❌ down             ❌ down
    Containers (up/total)                0/0                0/0                0/0                0/0

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔓 gcp-proxy          35.226.147.64      ports: 22
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
  TOTAL                16.1s
  mail_dns             15.9s
  public_urls          6.2s
  private              5.5s
  port_scan            3.0s
  mesh                 3.0s
  vm_ssh               0.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  16.1s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
