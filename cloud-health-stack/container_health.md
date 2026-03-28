```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-03-28  17:39:36
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  ⚠️  ISSUES FOUND
══════════════════════════════════════════════════════════════
11 critical, 2 warnings — 13 total

        Section  Issue
    ──────────────────────────────────────────────────────────────────────
    ❌ A2       VM gcp-t4 — UNREACHABLE
    ❌ A2       oci-analytics/umami-setup — exited(1)
    ❌ A2       oci-apps/crawlee_minio_init — exited(0)
    ❌ A1       MCP g-workspace — [502]
    ❌ A1       MCP mail-mcp — [502]
    ❌ A1       MCP mattermost-mcp — [502]
    ❌ A1       MCP c3-infra-mcp — [502]
    ❌ A1       MCP c3-services-mcp — [502]
    ❌ A3       Stalwart SPF FAIL — VM IP 130.110.251.193 not in SPF (outbound emails rejected)
    ❌ SYS      SSH unreachable: gcp-t4
    ❌ SYS        ❌ gcp-t4: UNREACHABLE
    ⚠️ A3       mails.diegonmarcos.com:25 MX (Resend/SES) — down
    ⚠️ A3       send.mails.diegonmarcos.com:25 SPF (Resend/SES) — down


══════════════════════════════════════════════════════════════
  A) HEALTH — Live checks
══════════════════════════════════════════════════════════════

── A1) URLs & Endpoints ──────────────────────────────────────

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1 — front door)
────────────────────────────────────────────────────────────
    Name               ☁VPS 🌐Pub 🔒WG  Public IP          WG IP          Type     Handshake
────────────────────────────────────────────────────────────
✅ oci-mail           ✅  ✅  ✅  130.110.251.193    10.0.0.3       VM       41 seconds ago
✅ oci-analytics      ✅  ✅  ✅  129.151.228.66     10.0.0.4       VM       22 seconds ago
✅ oci-apps           ✅  ✅  ✅  82.70.229.129      10.0.0.6       VM       43 minutes, 51 seconds ago
❌ gcp-t4             ❌  ❌  ❌  34.173.227.250     10.0.0.8       VM       never
⚠️ gcp-proxy          ✅  ✅  ❌  35.226.147.64      10.0.0.1       HUB      no data
✅ surface            ✅  ✅  ✅  dynamic            10.0.0.5       CLIENT   58 seconds ago
✅ termux             ✅  ✅  ✅  dynamic            10.0.0.9       CLIENT   1 minute, 12 seconds ago

PUBLIC URLs
────────────────────────────────────────────────────────────
✅ ide.diegonmarcos.com                → code-server.app:8443   [302]
✅ sheets.diegonmarcos.com             → grist.app:3011         [302]
✅ chat.diegonmarcos.com               → mattermost.app:8065    [302]
✅ photos.diegonmarcos.com             → photoprism.app:3013    [200]
✅ cal.diegonmarcos.com                → radicale.app:5232      [302]
✅ webmail.diegonmarcos.com            → snappymail.app:8888    [200]
✅ mail.diegonmarcos.com               → stalwart.app:443       [200]
✅ vault.diegonmarcos.com              → vaultwarden.app:8880   [200]
✅ api.diegonmarcos.com                → crawlee.app:3000       [200]
✅ auth.diegonmarcos.com               → authelia.app:9091      [200]
✅ workflows.diegonmarcos.com          → dagu.app:8070          [302]
✅ grafana.diegonmarcos.com            → grafana.app:3200       [200]
✅ analytics.diegonmarcos.com          → matomo.app:8080        [302]
✅ db.diegonmarcos.com                 → nocodb.app:8085        [302]
✅ rss.diegonmarcos.com                → ntfy.app:8090          [302]
✅ windmill.diegonmarcos.com           → windmill-app.app:8000  [200]
✅ git.diegonmarcos.com                → backup-gitea.app:3002  [200]
✅ app.diegonmarcos.com                → path-based             [404]
✅ cloud.diegonmarcos.com              → path-based             [200]
✅ mcp.diegonmarcos.com                → MCP hub                [200]
✅ proxy.diegonmarcos.com              → Infrastructure dashboard (static HTML) [302]
✅ diegonmarcos.com                    → github-pages:landpage  [200]
✅ www.diegonmarcos.com                → github-pages:landpage  [200]
✅ linktree.diegonmarcos.com           → github-pages:linktree  [200]
✅ nexus.diegonmarcos.com              → github-pages:nexus     [200]
✅ suite.diegonmarcos.com              → github-pages:suite     [200]
✅ maps.diegonmarcos.com               → github-pages:mymaps    [200]

API / MCP ENDPOINTS
────────────────────────────────────────────────────────────
❌ g-workspace            https://mcp.diegonmarcos.com/g-workspace/mcp          [502]
❌ mail-mcp               https://mcp.diegonmarcos.com/mail-mcp/mcp             [502]
❌ mattermost-mcp         https://mcp.diegonmarcos.com/mattermost-mcp/mcp       [502]
❌ c3-infra-mcp           https://mcp.diegonmarcos.com/c3-infra-mcp/mcp         [502]
❌ c3-services-mcp        https://mcp.diegonmarcos.com/c3-services-mcp/mcp      [502]
✅ cloud-cgc-mcp          https://mcp.diegonmarcos.com/cloud-cgc-mcp/mcp        [200]

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

── A2) Containers & VMs ──────────────────────────────────────

PRIVATE DNS (WireGuard mesh)
────────────────────────────────────────────────────────────
    DNS Name                     Container:Port                Port    VM
────────────────────────────────────────────────────────────
✅ authelia-redis.app           authelia-redis:6380         6380  gcp-E2-f_0
✅ authelia.app                 authelia:9091               9091  gcp-E2-f_0
✅ caddy.app                    caddy:443                 ⚠️443   gcp-E2-f_0
✅ hickory-dns.app              hickory-dns:53              53    gcp-E2-f_0
✅ introspect-proxy.app         introspect-proxy:4182       4182  gcp-E2-f_0
✅ ntfy.app                     ntfy:8090                   8090  gcp-E2-f_0
✅ redis.app                    redis:6379                  6379  gcp-E2-f_0
✅ vaultwarden.app              vaultwarden:8880            8880  gcp-E2-f_0
✅ ollama.app                   ollama:11434                11434 gcp-T4-p_0
✅ backup-gitea.app             gitea:3002                  3002  oci-A1-f_0
✅ c3-infra-api.app             c3-infra-api:8081           8081  oci-A1-f_0
✅ c3-infra-mcp.app             c3-infra-mcp:3100           3100  oci-A1-f_0
✅ c3-services-api.app          c3-services-api:8082        8082  oci-A1-f_0
✅ c3-services-mcp.app          c3-services-mcp:3101        3101  oci-A1-f_0
✅ c3-spec.app                  cloud-spec:3080             3080  oci-A1-f_0
✅ cloud-cgc-mcp.app            cloud-cgc-mcp:3105          3105  oci-A1-f_0
✅ code-server.app              code-server:8443            8443  oci-A1-f_0
✅ crawlee-dashboard.app        crawlee_dashboard:3001      3001  oci-A1-f_0
✅ crawlee-db.app               crawlee_db:5433             5433  oci-A1-f_0
✅ crawlee-minio.app            crawlee_minio:9000          9000  oci-A1-f_0
✅ crawlee-redis.app            crawlee_redis:6381          6381  oci-A1-f_0
✅ crawlee.app                  crawlee_api:3000            3000  oci-A1-f_0
✅ etherpad.app                 etherpad_app:3012           3012  oci-A1-f_0
✅ filebrowser.app              filebrowser_app:3015        3015  oci-A1-f_0
✅ g-workspace-mcp.app          google-workspace-mcp:3104   3104  oci-A1-f_0
✅ gitea.app                    gitea:3017                  3017  oci-A1-f_0
✅ grafana.app                  lgtm_grafana:3200           3200  oci-A1-f_0
✅ grist.app                    grist_app:3011              3011  oci-A1-f_0
✅ hedgedoc.app                 hedgedoc_app:3018           3018  oci-A1-f_0
✅ lgtm-loki.app                lgtm_loki:3110              3110  oci-A1-f_0
✅ lgtm-mimir.app               lgtm_mimir:9009             9009  oci-A1-f_0
✅ lgtm-tempo.app               lgtm_tempo:3210             3210  oci-A1-f_0
✅ mail-mcp.app                 mail-mcp:3103               3103  oci-A1-f_0
✅ mattermost-mcp.app           mattermost-mcp:3102         3102  oci-A1-f_0
✅ mattermost-postgres.app      mattermost-postgres:5435    5435  oci-A1-f_0
✅ mattermost.app               mattermost:8065             8065  oci-A1-f_0
✅ nocodb.app                   nocodb:8085                 8085  oci-A1-f_0
✅ ollama-hai.app               ollama-hai:11435            11435 oci-A1-f_0
✅ photoprism.app               photoprism_app:3013         3013  oci-A1-f_0
✅ radicale.app                 radicale:5232               5232  oci-A1-f_0
✅ revealmd.app                 revealmd_app:3014           3014  oci-A1-f_0
✅ windmill-app.app             windmill-server:8000        8000  oci-A1-f_0
✅ windmill-db.app              windmill-db:5440            5440  oci-A1-f_0
✅ dagu.app                     dagu:8070                   8070  oci-E2-f_0
✅ snappymail.app               snappymail:8888             8888  oci-E2-f_0
✅ stalwart.app                 stalwart:443              ⚠️443   oci-E2-f_0
✅ dozzle.app                   dozzle:9999                 9999  oci-E2-f_1
✅ matomo.app                   matomo-hybrid:8080          8080  oci-E2-f_1
✅ umami-db.app                 umami-db:5442               5442  oci-E2-f_1
✅ umami.app                    umami:3006                  3006  oci-E2-f_1

  ⚠️  PORT CONFLICTS (1 duplicate ports globally):
     :443    used by: caddy.app, stalwart.app

  ─── DNS CONFIG CHECK ───
  ✅ /etc/resolv.conf     includes 10.0.0.1 (Hickory)
     nameserver 10.0.0.1
     nameserver 8.8.8.8
     nameserver 8.8.4.4
  ✅ dig authelia-redis.app   10.0.0.1 (system DNS)
  ✅ dig @10.0.0.1 authelia-redis.app 10.0.0.1 (Hickory direct)
  ✅ Local DNS properly configured — Hickory resolves .app names

oci-mail ✅ — oci-mail — 1C/1G — mem 588M/954M (61%) — disk 67% — swap 228M/2559M — load 1.17 1.07 1.05 — 7/7 ctrs — up 14 hours, 12 minutes
────────────────────────────────────────────────────────────
  ✅ stalwart                  443     UP             Up 11 hours
  ✅ smtp-proxy                        UP             Up 14 hours
  ✅ dagu                      8070    UP             Up 14 hours
  ✅ fluent-bit                        UP             Up 14 hours
  ✅ snappymail                8888    HEALTHY        Up 13 hours (healthy)
  ✅ introspect-proxy          4182    HEALTHY        Up 14 hours (healthy)
  ✅ syslog-forwarder                  HEALTHY        Up 14 hours (healthy)

oci-analytics ✅ — oci-analytics — 1C/1G — mem 691M/954M (72%) — disk 56% — swap 263M/2559M — load 2.49 2.25 1.93 — 7/8 ctrs — up 13 hours, 38 minutes
────────────────────────────────────────────────────────────
  ❌ umami-setup                       EXITED(1)      Exited (1) 12 hours ago
  ✅ sauron-forwarder                  UP             Up 13 hours
  ✅ matomo-hybrid             8080    UP             Up 13 hours
  ✅ fluent-bit                        UP             Up 13 hours
  ✅ dozzle                    9999    UP             Up 13 hours
  ✅ alerts-api                        HEALTHY        Up 12 hours (healthy)
  ✅ umami                     3006    HEALTHY        Up 12 hours (healthy)
  ✅ umami-db                  5442    HEALTHY        Up 12 hours (healthy)

oci-apps ✅ — oci-apps — 4C/24G — mem 4873M/23975M (20%) — disk 66% — swap 0M/0M — load 0.48 0.47 0.39 — 52/53 ctrs — up 0d 23h
────────────────────────────────────────────────────────────
  ❌ crawlee_minio_init                EXITED(0)      Exited (0) 4 hours ago
  ✅ windmill-worker                   UP             Up 3 hours
  ✅ mattermost-bots                   UP             Up 3 hours
  ✅ gitea                     3002    UP             Up 4 hours
  ✅ bup-server                        UP             Up 4 hours
  ✅ borg-server                       UP             Up 4 hours
  ✅ lgtm_mimir                9009    UP             Up 4 hours
  ✅ lgtm_tempo                3210    UP             Up 4 hours
  ✅ cloud-spec                3080    UP             Up 4 hours
  ✅ crawlee_runner                    UP             Up 4 hours
  ✅ crawlee_dashboard         3001    UP             Up 4 hours
  ✅ crawlee_scheduler                 UP             Up 4 hours
  ✅ siem-api                          UP             Up 4 hours
  ✅ quant_light_engine                UP             Up 4 hours
  ✅ mattermost-mcp            3102    UP             Up 4 hours
  ✅ mail-mcp                  3103    UP             Up 4 hours
  ✅ code-server               8443    UP             Up 4 hours
  ✅ windmill-server           8000    HEALTHY        Up 3 hours (healthy)
  ✅ windmill-db               5440    HEALTHY        Up 3 hours (healthy)
  ✅ photoprism_app            3013    HEALTHY        Up 3 hours (healthy)
  ✅ photoprism_mariadb                HEALTHY        Up 3 hours (healthy)
  ✅ photoprism_rclone                 HEALTHY        Up 3 hours (healthy)
  ✅ mattermost                8065    HEALTHY        Up 3 hours (healthy)
  ✅ mattermost-postgres       5435    HEALTHY        Up 3 hours (healthy)
  ✅ surrealdb                         HEALTHY        Up 4 hours (healthy)
  ✅ photos-webhook                    HEALTHY        Up 4 hours (healthy)
  ✅ photos-db                         HEALTHY        Up 4 hours (healthy)
  ✅ nocodb                    8085    HEALTHY        Up 4 hours (healthy)
  ✅ nocodb-db                         HEALTHY        Up 4 hours (healthy)
  ✅ lgtm_grafana              3200    HEALTHY        Up 4 hours (healthy)
  ✅ lgtm_loki                 3110    HEALTHY        Up 4 hours (healthy)
  ✅ crawlee_api               3000    HEALTHY        Up 4 hours (healthy)
  ✅ crawlee_minio             9000    HEALTHY        Up 4 hours (healthy)
  ✅ crawlee_db                5433    HEALTHY        Up 4 hours (healthy)
  ✅ crawlee_redis             6381    HEALTHY        Up 4 hours (healthy)
  ✅ c3-services-mcp           3101    HEALTHY        Up 4 hours (healthy)
  ✅ c3-infra-mcp              3100    HEALTHY        Up 4 hours (healthy)
  ✅ c3-infra-api              8081    HEALTHY        Up 4 hours (healthy)
  ✅ syslog-central                    HEALTHY        Up 4 hours (healthy)
  ✅ rig-agentic-sonn-14bq8            HEALTHY        Up 4 hours (healthy)
  ✅ rig-agentic-hai                   HEALTHY        Up 4 hours (healthy)
  ✅ ollama-hai                11435   HEALTHY        Up 4 hours (healthy)
  ✅ quant_light_research              HEALTHY        Up 4 hours (healthy)
  ✅ quant_light_db                    HEALTHY        Up 4 hours (healthy)
  ✅ revealmd_app              3014    HEALTHY        Up 4 hours (healthy)
  ✅ radicale                  5232    HEALTHY        Up 4 hours (healthy)
  ✅ hedgedoc_app              3018    HEALTHY        Up 4 hours (healthy)
  ✅ hedgedoc_postgres                 HEALTHY        Up 4 hours (healthy)
  ✅ grist_app                 3011    HEALTHY        Up 4 hours (healthy)
  ✅ google-workspace-mcp      3104    HEALTHY        Up 4 hours (healthy)
  ✅ etherpad_app              3012    HEALTHY        Up 4 hours (healthy)
  ✅ etherpad_postgres                 HEALTHY        Up 4 hours (healthy)
  ✅ filebrowser_app           3015    HEALTHY        Up 4 hours (healthy)

gcp-t4 ❌ — gcp-t4 — 4C/15G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

gcp-proxy ✅ — gcp-proxy — 1C/1G — mem 938M/1952M (48%) — disk 51% — swap 107M/3999M — load 1.64 1.17 0.94 — 19/19 ctrs — up 1 hour, 26 minutes
────────────────────────────────────────────────────────────
  ✅ postlite-vaultwarden              UP             Up 45 minutes
  ✅ postlite-npm                      UP             Up 45 minutes
  ✅ sqlite-authelia                   UP             Up 45 minutes
  ✅ postlite-authelia                 UP             Up 45 minutes
  ✅ postlite-ntfy                     UP             Up 45 minutes
  ✅ sqlite-npm                        UP             Up 45 minutes
  ✅ sqlite-vaultwarden                UP             Up 45 minutes
  ✅ sqlite-ntfy                       UP             Up 45 minutes
  ✅ caddy                     443     UP             Up 45 minutes
  ✅ hickory-dns               53      UP             Up 45 minutes
  ✅ fluent-bit                        UP             Up About an hour
  ✅ authelia-redis            6380    UP             Up 45 minutes
  ✅ syslog-bridge                     UP             Up 45 minutes
  ✅ github-rss                        UP             Up 45 minutes
  ✅ ntfy                      8090    UP             Up 45 minutes
  ✅ vaultwarden               8880    HEALTHY        Up 45 minutes (healthy)
  ✅ redis                     6379    HEALTHY        Up 45 minutes (healthy)
  ✅ introspect-proxy          4182    HEALTHY        Up 45 minutes (healthy)
  ✅ authelia                  9091    HEALTHY        Up 45 minutes (healthy)


── A3) Mail ──────────────────────────────────────────────────

MAIL PORTS (tcp check)
────────────────────────────────────────────────────────────
⚠️ mail.diegonmarcos.com        :993   IMAPS           tcp open
⚠️ imap.diegonmarcos.com        :993   IMAPS           tcp open
⚠️ mail.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ smtp.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ mail.diegonmarcos.com        :587   Submission      tcp open
⚠️ smtp.diegonmarcos.com        :587   Submission      tcp open
❌ mails.diegonmarcos.com       :25    MX (Resend/SES) down
❌ send.mails.diegonmarcos.com  :25    SPF (Resend/SES) down

MX — Inbound Routing (dig MX)
────────────────────────────────────────────────────────────
    Domain                       Pri   Server                                     IP
────────────────────────────────────────────────────────────
✅ diegonmarcos.com             22    route1.mx.cloudflare.net.                  162.159.205.12
✅ diegonmarcos.com             85    route2.mx.cloudflare.net.                  162.159.205.19
✅ diegonmarcos.com             97    route3.mx.cloudflare.net.                  162.159.205.23
✅ send.mails.diegonmarcos.com  10    feedback-smtp.us-east-1.amazonses.com.     3.218.134.115
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
     ✅ smtp-proxy           Up 14 hours (oci-mail:8080)
     ✅ oci-mail:8080        reachable (CF Worker ingress)
     ✅ oci-mail:25          SMTP open (Stalwart local delivery)
     ✅ stalwart             Up 11 hours (oci-mail MTA)

  📱 CLIENT ACCESS: phone/Thunderbird → read/send mail via Caddy L4
     Client → gcp-proxy (35.226.147.64) → Caddy L4 TLS passthrough → oci-mail (130.110.251.193) → Stalwart
     ─────────────────────────────────────────────
     ✅ :993   → 130.110.251.193:993          open (IMAPS -- TLS passthrough to stalwart)
     ✅ :465   → 130.110.251.193:465          open (SMTPS -- TLS passthrough to stalwart)
     ✅ :587   → 130.110.251.193:587          open (SMTP Submission -- TLS passthrough to stalwart)
     ✅ webmail.diegonmarcos.com     [200] (Snappymail)
     ✅ mail.diegonmarcos.com        [200] (Stalwart admin)

  📤 OUTBOUND PERSONAL: me@diegonmarcos.com → someone@gmail.com
     Stalwart → ⚠️ direct from 130.110.251.193 (NOT IN SPF!) → recipient MX
     ─────────────────────────────────────────────
     ✅ stalwart             Up 11 hours (oci-mail MTA)
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
   oci-E2-f_0       OCI        VM.Standard.E2.1.Micro 1      1G     47G      Free
   oci-E2-f_1       OCI        VM.Standard.E2.1.Micro 1      1G     47G      Free
   oci-A1-f_0       OCI        VM.Standard.A1.Flex  4      24G    100G     Free
   gcp-T4-p_0       GCP        n1-standard-4        4      15G    100G     Spot
   gcp-E2-f_0       GCP        e2-micro             1      1G     30G      Free
   vast-RTX-p_0     Vast.ai    ?                    ?      ?G     ?G       Spot
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
CPU                1 cores        1 cores        4 cores        4 cores        1 cores       
RAM                588M/954M      691M/954M      4873M/23975M   ?/?            938M/1952M    
RAM %              61%            72%            20%            0%             48%           
Swap               228M/2559M     263M/2559M     0M/0M          ?              107M/3999M    
Disk               28G/45G        25G/48G        59.7G/95.8G    ?/?            16G/31G       
Disk %             67%            56%            66%            ?              51%           
Load               1.17 1.07 1.05 2.49 2.25 1.93 0.48 0.47 0.39 ?              1.64 1.17 0.94
Containers         7/7            7/8            52/53          0/0            19/19         
Uptime             14 hours, 12 minutes 13 hours, 38 minutes 0d 23h         ?              1 hour, 26 minutes

STORAGE
────────────────────────────────────────────────────────────
  OBJECT STORAGE
    Provider       Type                 Details
    ────────────────────────────────────────────────────────────
    oci            Object Storage       each.value.name (each.value.storage_tier)
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
    gcp-proxy        vaultwarden_data               vaultwarden
    oci-analytics    matomo_data                    matomo
    oci-apps         grist_data                     grist
    oci-apps         mattermost_data                mattermost-bots
    oci-apps         mattermost_postgres            mattermost-bots
    oci-apps         photoprism_originals           photoprism
    oci-apps         photoprism_storage             photoprism
    oci-apps         nocodb_data                    nocodb
    oci-apps         gitea_data                     gitea
    oci-mail         stalwart_data                  stalwart

  DATABASES
    Total: 15 — 5 sqlite, 2 custom, 7 postgres, 1 mariadb
    Service              Type       Container              DB Name        VM
    ───────────────────────────────────────────────────────────────────────────
    authelia             sqlite     authelia               /config/db.sqlite3 gcp-proxy
    ntfy                 sqlite     ntfy                   /var/cache/ntfy/cache.db gcp-proxy
    vaultwarden          sqlite     vaultwarden            /data/db.sqlite3 gcp-proxy
    matomo               custom     matomo-hybrid          custom         oci-analytics
    umami                postgres   umami-db               umami          oci-analytics
    crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps
    etherpad             postgres   etherpad_postgres      etherpad       oci-apps
    gitea                sqlite     gitea                  /data/gitea/gitea.db oci-apps
    grist                sqlite     grist_app              /persist/grist-sessions.db oci-apps
    hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps
    mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps
    nocodb               postgres   nocodb-db              nocodb         oci-apps
    photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps
    quant-lab-light      postgres   quant_light_db         quantlab       oci-apps
    stalwart             custom     stalwart               custom         oci-mail


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
   authelia             sqlite     authelia               /config/db.sqlite3 gcp-proxy        authelia-redis.app:6380
   ntfy                 sqlite     ntfy                   /var/cache/ntfy/cache.db gcp-proxy        ntfy.app:8090
   vaultwarden          sqlite     vaultwarden            /data/db.sqlite3 gcp-proxy        vaultwarden.app:8880
   matomo               custom     matomo-hybrid          custom         oci-analytics    matomo.app:8080
   umami                postgres   umami-db               umami          oci-analytics    umami-db.app:5442
   crawlee-cloud        postgres   crawlee_db             crawlee        oci-apps         crawlee-db.app:5433
   etherpad             postgres   etherpad_postgres      etherpad       oci-apps         embedded
   gitea                sqlite     gitea                  /data/gitea/gitea.db oci-apps         backup-gitea.app:3002
   grist                sqlite     grist_app              /persist/grist-sessions.db oci-apps         grist.app:3011
   hedgedoc             postgres   hedgedoc_postgres      hedgedoc       oci-apps         embedded
   mattermost-bots      postgres   mattermost-postgres    mattermost     oci-apps         mattermost-postgres.app:5435
   nocodb               postgres   nocodb-db              nocodb         oci-apps         embedded
   photoprism           mariadb    photoprism_mariadb     photoprism     oci-apps         embedded
   quant-lab-light      postgres   quant_light_db         quantlab       oci-apps         embedded
   stalwart             custom     stalwart               custom         oci-mail         stalwart.app:443

DOCKER NETWORKS
────────────────────────────────────────────────────────────
    Network                      VM               Services
    ──────────────────────────────────────────────────────────────────────
    auth-net                     gcp-proxy        authelia
    default                      oci-apps         radicale
    etherpad_net                 oci-apps         etherpad

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
  public_urls          23.2s █████
  vm_gcp-proxy         18.1s ████
  api_mcp              16.3s ████
  private_dns          10.4s ██
  vm_gcp-t4             8.0s ██
  mail_ports            5.8s █
  vm_oci-mail           5.2s █
  vm_oci-apps           4.7s █
  vm_oci-analytics      4.4s █
  TOTAL               126.7s

SCRIPT INFO
────────────────────────────────────────────────────────────
  Script:    cloud-data/cloud-health-stack/container-health.ts
  Run:       ./container-health.ts  (or: tsx container-health.ts)
  Node:      v20.19.1
  Platform:  linux x64
  CWD:       /home/diego/Mounts/Git/cloud/cloud-data/cloud-health-stack
  Template:  container_health.md.tpl
  Data src:  /home/diego/Mounts/Git/cloud/cloud-data/

  Dependencies:
    ✅ ssh        /home/diego/.nix-profile/bin/ssh
    ✅ curl       /home/diego/.nix-profile/bin/curl
    ✅ nc         /home/diego/.nix-profile/bin/nc
    ✅ dig        /home/diego/.nix-profile/bin/dig
    ✅ git        /home/diego/.nix-profile/bin/git
    ✅ gh         /home/diego/.nix-profile/bin/gh

  Errors:    2
    [17:40:54] ERROR: SSH unreachable: gcp-t4
    [17:41:12] ERROR:   ❌ gcp-t4: UNREACHABLE

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: ./container-health.ts
```
