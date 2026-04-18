```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-04-18T10:48:13.221062936+00:00
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
27 critical, 0 warnings — 27 total

    ❌ A3       VM gcp-proxy — UNREACHABLE
    ❌ A3       VM oci-apps — UNREACHABLE
    ❌ A3       VM oci-mail — UNREACHABLE
    ❌ A3       VM oci-analytics — UNREACHABLE
    ❌ B2       gcp-proxy/authelia (sqlite) — not running
    ❌ B2       gcp-proxy/authelia-redis (redis) — not running
    ❌ B2       gcp-proxy/redis (redis) — not running
    ❌ B2       oci-analytics/matomo-hybrid (custom) — not running
    ❌ B2       oci-analytics/umami-db (postgres) — not running
    ❌ B2       oci-apps/crawlee_db (postgres) — not running
    ❌ B2       oci-apps/crawlee_redis (redis) — not running
    ❌ B2       oci-apps/crawlee_minio (s3) — not running
    ❌ B2       oci-apps/etherpad_postgres (postgres) — not running
    ❌ B2       oci-apps/gitea (sqlite) — not running
    ❌ B2       oci-apps/grist_app (sqlite) — not running
    ❌ B2       oci-apps/hedgedoc_postgres (postgres) — not running
    ❌ B2       oci-apps/lgtm_grafana (grafana) — not running
    ❌ B2       oci-apps/lgtm_loki (loki) — not running
    ❌ B2       oci-apps/lgtm_tempo (tempo) — not running
    ❌ B2       oci-apps/lgtm_mimir (mimir) — not running
    ❌ B2       oci-apps/mattermost-postgres (postgres) — not running
    ❌ B2       oci-apps/ntfy (sqlite) — not running
    ❌ B2       oci-apps/photoprism_mariadb (mariadb) — not running
    ❌ B2       oci-apps/quant_light_db (postgres) — not running
    ❌ B2       oci-apps/vaultwarden (sqlite) — not running
    ❌ B2       oci-apps/windmill-db (postgres) — not running
    ❌ B2       oci-mail/stalwart (custom) — not running


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
✅ sheets.diegonmarcos.com          ✅  ❌  ✅  ✅  10.0.0.6:3011          [302] 
✅ doc.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:3018          [200] 
✅ grafana.diegonmarcos.com         ✅  ❌  ✅  ✅  10.0.0.6:3200          [302] 
✅ mail.diegonmarcos.com            ✅  ❌  ✅  ✅  ?                      [301] 
✅ analytics.diegonmarcos.com       ✅  ❌  ✅  ✅  10.0.0.4:8084          [302] 
✅ chat.diegonmarcos.com            ✅  ❌  ✅  ✅  10.0.0.6:8065          [302] 
✅ rss.diegonmarcos.com             ✅  ❌  ✅  ✅  10.0.0.6:8090          [302] 
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

⚠️  WireGuard/Hickory DOWN — cannot reach .app endpoints
    Run: sudo wg-quick up wg0

    DNS Name                     📡TCP 🌐HTTP 🐳CTR Port  VM             Container              Code
    ─────────────────────────────────────────────────────────────────────────────────────────────────────────
❌ authelia-redis.app           ⏸️   ⏸️   ❌    6380 gcp-proxy      authelia-redis         [---]
❌ authelia.app                 ⏸️   ⏸️   ❌    9091 gcp-proxy      authelia               [---]
❌ caddy.app                    ⏸️   ⏸️   ❌     443 gcp-proxy      caddy                  [---]
❌ hickory-dns.app              ⏸️   ⏸️   ❌      53 gcp-proxy      hickory-dns            [---]
❌ introspect-proxy.app         ⏸️   ⏸️   ❌    4182 gcp-proxy      introspect-proxy       [---]
❌ redis.app                    ⏸️   ⏸️   ❌    6379 gcp-proxy      redis                  [---]
❌ ollama.app                   ⏸️   ⏸️   ❌   11434 gcp-t4         ollama                 [---]
❌ dagu.app                     ⏸️   ⏸️   ❌    8070 oci-analytics  dagu                   [---]
❌ dozzle.app                   ⏸️   ⏸️   ❌    9999 oci-analytics  dozzle                 [---]
❌ matomo.app                   ⏸️   ⏸️   ❌    8084 oci-analytics  matomo-hybrid          [---]
❌ umami-db.app                 ⏸️   ⏸️   ❌    5442 oci-analytics  umami-db               [---]
❌ umami.app                    ⏸️   ⏸️   ❌    3006 oci-analytics  umami                  [---]
❌ backup-gitea.app             ⏸️   ⏸️   ❌    3002 oci-apps       gitea                  [---]
❌ c3-infra-api.app             ⏸️   ⏸️   ❌    8081 oci-apps       c3-infra-api           [---]
❌ c3-infra-mcp.app             ⏸️   ⏸️   ❌    3100 oci-apps       c3-infra-mcp           [---]
❌ c3-services-api.app          ⏸️   ⏸️   ❌    8082 oci-apps       c3-services-api        [---]
❌ c3-services-mcp.app          ⏸️   ⏸️   ❌    3101 oci-apps       c3-services-mcp        [---]
❌ c3-spec.app                  ⏸️   ⏸️   ❌    3080 oci-apps       cloud-spec             [---]
❌ cloud-cgc-mcp.app            ⏸️   ⏸️   ❌    3105 oci-apps       cloud-cgc-mcp          [---]
❌ code-server.app              ⏸️   ⏸️   ❌    8443 oci-apps       code-server            [---]
❌ crawlee-dashboard.app        ⏸️   ⏸️   ❌    3001 oci-apps       crawlee_dashboard      [---]
❌ crawlee-db.app               ⏸️   ⏸️   ❌    5433 oci-apps       crawlee_db             [---]
❌ crawlee-minio.app            ⏸️   ⏸️   ❌    9000 oci-apps       crawlee_minio          [---]
❌ crawlee-redis.app            ⏸️   ⏸️   ❌    6381 oci-apps       crawlee_redis          [---]
❌ crawlee.app                  ⏸️   ⏸️   ❌    3000 oci-apps       crawlee_api            [---]
❌ dbgate.app                   ⏸️   ⏸️   ❌    8086 oci-apps       dbgate                 [---]
❌ etherpad-db.app              ⏸️   ⏸️   ❌    5436 oci-apps       etherpad_postgres      [---]
❌ etherpad.app                 ⏸️   ⏸️   ❌    3012 oci-apps       etherpad_app           [---]
❌ filebrowser.app              ⏸️   ⏸️   ❌    3015 oci-apps       filebrowser_app        [---]
❌ g-workspace-mcp.app          ⏸️   ⏸️   ❌    3104 oci-apps       google-workspace-mcp   [---]
❌ gitea.app                    ⏸️   ⏸️   ❌    3002 oci-apps       gitea                  [---]
❌ grafana.app                  ⏸️   ⏸️   ❌    3200 oci-apps       lgtm_grafana           [---]
❌ grist.app                    ⏸️   ⏸️   ❌    3011 oci-apps       grist_app              [---]
❌ hedgedoc-db.app              ⏸️   ⏸️   ❌    5439 oci-apps       hedgedoc_postgres      [---]
❌ hedgedoc.app                 ⏸️   ⏸️   ❌    3018 oci-apps       hedgedoc_app           [---]
❌ lgtm-loki.app                ⏸️   ⏸️   ❌    3110 oci-apps       lgtm_loki              [---]
❌ lgtm-mimir.app               ⏸️   ⏸️   ❌    9009 oci-apps       lgtm_mimir             [---]
❌ lgtm-tempo.app               ⏸️   ⏸️   ❌    3210 oci-apps       lgtm_tempo             [---]
❌ mail-mcp.app                 ⏸️   ⏸️   ❌    3103 oci-apps       mail-mcp               [---]
❌ mattermost-mcp.app           ⏸️   ⏸️   ❌    3102 oci-apps       mattermost-mcp         [---]
❌ mattermost-postgres.app      ⏸️   ⏸️   ❌    5435 oci-apps       mattermost-postgres    [---]
❌ mattermost.app               ⏸️   ⏸️   ❌    8065 oci-apps       mattermost             [---]
❌ ntfy.app                     ⏸️   ⏸️   ❌    8090 oci-apps       ntfy                   [---]
❌ ollama-hai.app               ⏸️   ⏸️   ❌   11435 oci-apps       ollama-hai             [---]
❌ photoprism.app               ⏸️   ⏸️   ❌    3013 oci-apps       photoprism_app         [---]
❌ quant-full-db.app            ⏸️   ⏸️   ❌    5437 oci-apps       quant_full_db          [---]
❌ quant-full-research.app      ⏸️   ⏸️   ❌    8890 oci-apps       quant_full_research    [---]
❌ quant-light-db.app           ⏸️   ⏸️   ❌    5443 oci-apps       quant_light_db         [---]
❌ quant-light-engine.app       ⏸️   ⏸️   ❌    5001 oci-apps       quant_light_engine     [---]
❌ quant-light-research.app     ⏸️   ⏸️   ❌    8889 oci-apps       quant_light_research   [---]
❌ radicale.app                 ⏸️   ⏸️   ❌    5232 oci-apps       radicale               [---]
❌ revealmd.app                 ⏸️   ⏸️   ❌    3014 oci-apps       revealmd_app           [---]
❌ vaultwarden.app              ⏸️   ⏸️   ❌    8880 oci-apps       vaultwarden            [---]
❌ windmill-app.app             ⏸️   ⏸️   ❌    8000 oci-apps       windmill-server        [---]
❌ windmill-db.app              ⏸️   ⏸️   ❌    5440 oci-apps       windmill-db            [---]
❌ smtp-proxy.app               ⏸️   ⏸️   ❌    8080 oci-mail       smtp-proxy             [---]
❌ snappymail.app               ⏸️   ⏸️   ❌    8888 oci-mail       snappymail             [---]
❌ stalwart.app                 ⏸️   ⏸️   ❌    2443 oci-mail       stalwart               [---]

  📡 TCP: 0/58  🌐 HTTP: 0/58  🐳 Container: 0/58

── A3) Containers ────────────────────────────────────────────

gcp-proxy ❌ — 2C/2G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-apps ❌ — 4C/24G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-mail ❌ — 1C/1G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

oci-analytics ❌ — 1C/1G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────


── A3b) Container Drift ──────────────────────────────────

CONTAINER HEALTH — 0/0 running, 0 healthy, 0 unhealthy, 0 exited
──────────────────────────────────────────────────────────────────────────────────────────────────────────────
    VM               Total            Up     Healthy  Unhealthy  Exited   Mem Used   Mem Total  Disk       Load    
    ────────────────────────────────────────────────────────────────────────────────────────────────────
❌ gcp-proxy        0                     0        0          0        0 ?          ?          ?          ?       
❌ oci-apps         0                     0        0          0        0 ?          ?          ?          ?       
❌ oci-mail         0                     0        0          0        0 ?          ?          ?          ?       
❌ oci-analytics    0                     0        0          0        0 ?          ?          ?          ?       
    ────────────────────────────────────────────────────────────────────────────────────────────────────

    Container                 Health         VM           Status (docker ps + stats)         
    ───────────────────────────────────────────────────────────────────────────────────────────────

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
DECLARED DATABASES — 23 total (5 sqlite, 1 grafana, 2 custom, 1 tempo, 1 mariadb, 1 loki, 3 redis, 1 mimir, 7 postgres, 1 s3)
    Service              Type       Container              VM               Port   TCP  Health   Size       Backup
    ─────────────────────────────────────────────────────────────────────────────────────────────────────────
❌ authelia             sqlite     authelia               gcp-proxy        9091   ❌   ❌   ?          ✅
❌ authelia             redis      authelia-redis         gcp-proxy        6380   ❌   ❌   ?          ✅
❌ redis                redis      redis                  gcp-proxy        6379   ❌   ❌   ?          ❌
❌ matomo               custom     matomo-hybrid          oci-analytics    8084   ❌   ❌   ?          ✅
❌ umami                postgres   umami-db               oci-analytics    5442   ❌   ❌   ?          ✅
❌ crawlee-cloud        postgres   crawlee_db             oci-apps         5433   ❌   ❌   ?          ✅
❌ crawlee-cloud        redis      crawlee_redis          oci-apps         6381   ❌   ❌   ?          ✅
❌ crawlee-cloud        s3         crawlee_minio          oci-apps         9000   ❌   ❌   ?          ✅
❌ etherpad             postgres   etherpad_postgres      oci-apps         5436   ❌   ❌   ?          ✅
❌ gitea                sqlite     gitea                  oci-apps         3002   ❌   ❌   ?          ✅
❌ grist                sqlite     grist_app              oci-apps         3011   ❌   ❌   ?          ✅
❌ hedgedoc             postgres   hedgedoc_postgres      oci-apps         5439   ❌   ❌   ?          ✅
❌ lgtm                 grafana    lgtm_grafana           oci-apps         3200   ❌   ❌   ?          ❌
❌ lgtm                 loki       lgtm_loki              oci-apps         3110   ❌   ❌   ?          ❌
❌ lgtm                 tempo      lgtm_tempo             oci-apps         3210   ❌   ❌   ?          ❌
❌ lgtm                 mimir      lgtm_mimir             oci-apps         9009   ❌   ❌   ?          ❌
❌ mattermost-bots      postgres   mattermost-postgres    oci-apps         5435   ❌   ❌   ?          ✅
❌ ntfy                 sqlite     ntfy                   oci-apps         8090   ❌   ❌   ?          ✅
❌ photoprism           mariadb    photoprism_mariadb     oci-apps         —      —   ❌   ?          ✅
❌ quant-lab-light      postgres   quant_light_db         oci-apps         5443   ❌   ❌   ?          ✅
❌ vaultwarden          sqlite     vaultwarden            oci-apps         8880   ❌   ❌   ?          ✅
❌ windmill             postgres   windmill-db            oci-apps         5440   ❌   ❌   ?          ✅
❌ stalwart             custom     stalwart               oci-mail         2443   ❌   ❌   ?          ✅

  Healthy: 0/23  Running: 0/23

── B3) Object Storage ──────────────────────────────────
OBJECT STORAGE — 1 buckets (live)
    Bucket                         Provider   Tier           Live   Size       Objects   
    ──────────────────────────────────────────────────────────────────────────────────────────
✅ crawlee_minio                  MinIO      Local          ✅   —          —         


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

NETWORK SECURITY AUDIT
────────────────────────────────────────────────────────────
    Check                         gcp-proxy           oci-mail      oci-analytics           oci-apps
    ────────────────────────────────────────────────────────────────────────────────────────────────
    Declared ports       80,443,465,587,993,2443,2465,2587,2993 25,465,587,993,2025,2443,2465,2587,2993,4190,6190,8080,8443,21027,22000               none 2222,2223,2224,3000,3001,3010,8081,8099
    Scanned (public)     22,443,465,587,993             🔒 none             🔒 none             🔒 none
    Docker host ports                  none               none               none               none
    Undeclared leaks                ✅ clean            ✅ clean            ✅ clean            ✅ clean
    WG reachable                     ❌ down             ❌ down             ❌ down             ❌ down
    Containers (up/total)                0/0                0/0                0/0                0/0

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
  TOTAL                58.0s
  mail_dns             55.0s
  databases            3.0s
  port_scan            3.0s
  mesh                 3.0s
  public_urls          1.1s
  vm_ssh               0.6s
  storage              0.0s
  private              0.0s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Engine:    Rust (native async tokio)
  Duration:  58.0s
  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync)

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
