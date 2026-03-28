```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗ 
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝ 
         CONTAINER HEALTH — 2026-03-28  05:47:49
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  A) HEALTH
══════════════════════════════════════════════════════════════

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1)
────────────────────────────────────────────────────────────
    Name               Public IP          WG IP          Handshake
────────────────────────────────────────────────────────────
✅ oci-analytics      129.151.228.66     10.0.0.4       15 seconds ago
✅ 10.0.0.5           88.30.43.183       10.0.0.5       46 seconds ago
✅ oci-mail           130.110.251.193    10.0.0.3       1 minute, 23 seconds ago
❌ 10.0.0.8           34.173.227.250     10.0.0.8       never
❌ oci-apps           82.70.229.129      10.0.0.6       never
❌ 10.0.0.9           none               10.0.0.9       never

PUBLIC URLs
────────────────────────────────────────────────────────────
✅ app.diegonmarcos.com                → path-based routes      [404]
✅ cloud.diegonmarcos.com              → cloud portal           [200]
✅ proxy.diegonmarcos.com              → infra dashboard        [302]
✅ slides.diegonmarcos.com             → revealmd:3014          [200]
✅ pad.diegonmarcos.com                → etherpad:3012          [200]
✅ doc.diegonmarcos.com                → hedgedoc:3018          [200]
✅ files.diegonmarcos.com              → filebrowser:3015       [200]
✅ logs.diegonmarcos.com               → dozzle:9999            [200]
✅ mcp.diegonmarcos.com                → MCP hub                [200]
✅ auth.diegonmarcos.com               → authelia:9091          [200]
✅ vault.diegonmarcos.com              → vaultwarden:8880       [200]
✅ rss.diegonmarcos.com                → ntfy:8090              [302]
✅ ide.diegonmarcos.com                → code-server:8444       [302]
✅ sheets.diegonmarcos.com             → grist:3011             [302]
✅ chat.diegonmarcos.com               → mattermost:8065        [302]
✅ photos.diegonmarcos.com             → photoprism:3013        [200]
✅ cal.diegonmarcos.com                → radicale:5232          [302]
✅ api.diegonmarcos.com                → crawlee:3000           [200]
✅ grafana.diegonmarcos.com            → grafana:3200           [200]
✅ db.diegonmarcos.com                 → nocodb:8085            [302]
✅ windmill.diegonmarcos.com           → windmill:8000          [200]
✅ git.diegonmarcos.com                → gitea:3017             [200]
✅ webmail.diegonmarcos.com            → snappymail:8888        [200]
✅ mail.diegonmarcos.com               → stalwart:8443          [200]
✅ workflows.diegonmarcos.com          → dagu:8070              [302]
✅ analytics.diegonmarcos.com          → matomo:8080            [302]

API / MCP ENDPOINTS
────────────────────────────────────────────────────────────
✅ C3 Infra API           https://api.diegonmarcos.com/c3-api                   [404]
✅ C3 Services API        https://api.diegonmarcos.com/services                 [404]
✅ Crawlee API            https://api.diegonmarcos.com/crawlee                  [404]
❌ Google Workspace MCP   https://mcp.diegonmarcos.com/g-workspace/mcp          [502]
❌ Mail MCP               https://mcp.diegonmarcos.com/mail-mcp/mcp             [502]
❌ Mattermost MCP         https://mcp.diegonmarcos.com/mattermost-mcp/mcp       [502]
❌ C3 Infra MCP           https://mcp.diegonmarcos.com/c3-infra-mcp/mcp         [502]
❌ C3 Services MCP        https://mcp.diegonmarcos.com/c3-services-mcp/mcp      [502]
✅ Cloud CGC MCP          https://mcp.diegonmarcos.com/cloud-cgc-mcp/mcp        [200]

MAIL PORTS
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

PRIVATE DNS (WireGuard mesh)
────────────────────────────────────────────────────────────
✅ authelia.app                 authelia:9091             gcp-proxy
✅ caddy.app                    caddy:443                 gcp-proxy
✅ hickory-dns.app              hickory-dns:53            gcp-proxy
✅ introspect-proxy.app         introspect-proxy:4182     gcp-proxy
✅ ntfy.app                     ntfy:8090                 gcp-proxy
✅ redis.app                    redis:6379                gcp-proxy
✅ vaultwarden.app              vaultwarden:8880          gcp-proxy
❌ c3-infra-api.app             c3-infra-api:8081         oci-apps
❌ c3-infra-mcp.app             c3-infra-mcp:3100         oci-apps
❌ code-server.app              code-server:8444          oci-apps
❌ crawlee.app                  crawlee_api:3000          oci-apps
❌ grafana.app                  lgtm_grafana:3200         oci-apps
❌ grist.app                    grist_app:3011            oci-apps
❌ mattermost.app               mattermost:8065           oci-apps
❌ nocodb.app                   nocodb:8085               oci-apps
✅ dagu.app                     dagu:8070                 oci-mail
❌ stalwart.app                 stalwart:8443             oci-mail
✅ dozzle.app                   dozzle:9999               oci-analytics
✅ matomo.app                   matomo-hybrid:8080        oci-analytics
✅ umami.app                    umami:3006                oci-analytics

gcp-proxy ✅ — Fedora 42 — 2C/2G — mem 847M/1952M (43%) — disk 48% — swap 107M/3999M — load 0.05 0.08 0.36 — 10/10 ctrs — up 2 hours, 47 minutes
────────────────────────────────────────────────────────────
  ✅ authelia-redis            UP             Up 41 minutes
  ✅ caddy                     UP             Up About an hour
  ✅ syslog-bridge             UP             Up 2 hours
  ✅ github-rss                UP             Up 2 hours
  ✅ ntfy                      UP             Up 2 hours
  ✅ hickory-dns               UP             Up 2 hours
  ✅ authelia                  HEALTHY        Up 41 minutes (healthy)
  ✅ introspect-proxy          HEALTHY        Up About an hour (healthy)
  ✅ vaultwarden               HEALTHY        Up About an hour (healthy)
  ✅ redis                     HEALTHY        Up About an hour (healthy)

oci-apps ✅ — Ubuntu ARM — 4C/24G — mem 4325M/23975M (18%) — disk 63% — swap 0M/0M — load 0.29 0.32 0.36 — 47/53 ctrs — up 0d 11h
────────────────────────────────────────────────────────────
  ❌ photoprism_rclone         EXITED(1)      Exited (1) 20 minutes ago
  ❌ mattermost-bots           EXITED(1)      Exited (1) 17 minutes ago
  ❌ windmill-server           EXITED(137)    Exited (137) About a minute ag
  ❌ syslog-central            EXITED(255)    Exited (255) 20 minutes ago
  ❌ siem-api                  EXITED(1)      Exited (1) 9 minutes ago
  ❌ crawlee_minio_init        EXITED(0)      Exited (0) 3 hours ago
  ✅ quant_light_engine        UP             Up About an hour
  ✅ mattermost-mcp            UP             Up About an hour
  ✅ mail-mcp                  UP             Up About an hour
  ✅ code-server               UP             Up About an hour
  ✅ gitea                     UP             Up 20 minutes
  ✅ bup-server                UP             Up 2 hours
  ✅ borg-server               UP             Up 2 hours
  ✅ windmill-worker           UP             Up 2 hours
  ✅ lgtm_mimir                UP             Up 2 hours
  ✅ lgtm_tempo                UP             Up 2 hours
  ✅ cloud-spec                UP             Up 2 hours
  ✅ crawlee_runner            UP             Up 3 hours
  ✅ crawlee_dashboard         UP             Up 3 hours
  ✅ crawlee_scheduler         UP             Up 3 hours
  ✅ ollama-hai                HEALTHY        Up About an hour (healthy)
  ✅ quant_light_research      HEALTHY        Up About an hour (healthy)
  ✅ quant_light_db            HEALTHY        Up About an hour (healthy)
  ✅ revealmd_app              HEALTHY        Up About an hour (healthy)
  ✅ radicale                  HEALTHY        Up About an hour (healthy)
  ✅ photoprism_app            HEALTHY        Up 20 minutes (healthy)
  ✅ photoprism_mariadb        HEALTHY        Up About an hour (healthy)
  ✅ mattermost                HEALTHY        Up About an hour (healthy)
  ✅ mattermost-postgres       HEALTHY        Up About an hour (healthy)
  ✅ google-workspace-mcp      HEALTHY        Up About an hour (healthy)
  ✅ hedgedoc_app              HEALTHY        Up About an hour (healthy)
  ✅ hedgedoc_postgres         HEALTHY        Up About an hour (healthy)
  ✅ grist_app                 HEALTHY        Up About an hour (healthy)
  ✅ etherpad_app              HEALTHY        Up About an hour (healthy)
  ✅ etherpad_postgres         HEALTHY        Up About an hour (healthy)
  ✅ filebrowser_app           HEALTHY        Up About an hour (healthy)
  ✅ rig-agentic-sonn-14bq8    HEALTHY        Up 2 hours (healthy)
  ✅ rig-agentic-hai           HEALTHY        Up 2 hours (healthy)
  ✅ surrealdb                 HEALTHY        Up 2 hours (healthy)
  ✅ windmill-db               HEALTHY        Up 2 hours (healthy)
  ✅ nocodb                    HEALTHY        Up 2 hours (healthy)
  ✅ nocodb-db                 HEALTHY        Up 2 hours (healthy)
  ✅ lgtm_grafana              HEALTHY        Up 2 hours (healthy)
  ✅ lgtm_loki                 HEALTHY        Up 2 hours (healthy)
  ✅ c3-services-mcp           HEALTHY        Up 2 hours (healthy)
  ✅ c3-infra-mcp              HEALTHY        Up 2 hours (healthy)
  ✅ c3-infra-api              HEALTHY        Up 2 hours (healthy)
  ✅ photos-webhook            HEALTHY        Up 3 hours (healthy)
  ✅ photos-db                 HEALTHY        Up 3 hours (healthy)
  ✅ crawlee_api               HEALTHY        Up 3 hours (healthy)
  ✅ crawlee_db                HEALTHY        Up 3 hours (healthy)
  ✅ crawlee_redis             HEALTHY        Up 3 hours (healthy)
  ✅ crawlee_minio             HEALTHY        Up 3 hours (healthy)

oci-mail ✅ — Ubuntu x86 — 2C/1G — mem 788M/954M (82%) — disk 67% — swap 262M/2559M — load 2.13 2.25 2.22 — 8/8 ctrs — up 2 hours, 21 minutes
────────────────────────────────────────────────────────────
  ✅ stalwart                  UP             Up About an hour
  ✅ caddy                     UP             Up 2 hours
  ✅ smtp-proxy                UP             Up 2 hours
  ✅ dagu                      UP             Up 2 hours
  ✅ fluent-bit                UP             Up 2 hours
  ✅ snappymail                HEALTHY        Up 52 minutes (healthy)
  ✅ introspect-proxy          HEALTHY        Up 2 hours (healthy)
  ✅ syslog-forwarder          HEALTHY        Up 2 hours (healthy)

oci-analytics ✅ — Ubuntu x86 — 2C/1G — mem 738M/954M (77%) — disk 56% — swap 189M/2559M — load 1.70 1.87 2.23 — 7/8 ctrs — up 1 hour, 47 minutes
────────────────────────────────────────────────────────────
  ❌ umami-setup               EXITED(1)      Exited (1) 5 minutes ago
  ❌ alerts-api                UNHEALTHY      Up 3 minutes (unhealthy)
  ✅ sauron-forwarder          UP             Up About an hour
  ✅ matomo-hybrid             UP             Up About an hour
  ✅ fluent-bit                UP             Up 2 hours
  ✅ dozzle                    UP             Up 2 hours
  ✅ umami                     HEALTHY        Up 6 minutes (healthy)
  ✅ umami-db                  HEALTHY        Up 6 minutes (healthy)

RESOURCES
────────────────────────────────────────────────────────────
                   gcp-proxy      oci-apps       oci-mail       oci-analytics
────────────────────────────────────────────────────────────
OS                 Fedora 42      Ubuntu ARM     Ubuntu x86     Ubuntu x86    
CPU                2 cores        4 cores        2 cores        2 cores       
RAM                847M/1952M     4325M/23975M   788M/954M      738M/954M     
RAM %              43%            18%            82%            77%           
Swap               107M/3999M     0M/0M          262M/2559M     189M/2559M    
Disk               15G/31G        57.1G/95.8G    28G/45G        25G/48G       
Disk %             48%            63%            67%            56%           
Load               0.05 0.08 0.36 0.29 0.32 0.36 2.13 2.25 2.22 1.70 1.87 2.23
Containers         10/10          47/53          8/8            7/8           
Uptime             2 hours, 47 minutes 0d 11h         2 hours, 21 minutes 1 hour, 47 minutes

SECURITY — Open Ports by Public IP
────────────────────────────────────────────────────────────
🔓 gcp-proxy          35.226.147.64      ports: 22, 80, 443, 465, 587, 993, 2200
🔓 oci-mail           130.110.251.193    ports: 22, 25, 465, 587, 993, 8080
🔓 oci-analytics      129.151.228.66     ports: 22
🔓 oci-apps           82.70.229.129      ports: 22


══════════════════════════════════════════════════════════════
  B) STACK INFO
══════════════════════════════════════════════════════════════

GIT REPOSITORIES
────────────────────────────────────────────────────────────
✅ cloud          main     3e4247d9 feat: docker-buildx-capped wrapper in system-p
⚠️ cloud-data     ?        45fe990 auto: trace ship-gcp-proxy [skip ci]
✅ front          main     2df5bc69 add(task): Garmin fenix 8 custom watchface des
⚠️ unix           main     22e6ce5 feat: SSH stale socket cleaner — systemd user t
⚠️ tools          main     fc21dbb fix: L letter foot extended to 7-wide
⚠️ vault          main     ab6f5f0 chore: update vaultwarden setup.ts + tokens

VPS / VM SPECS
────────────────────────────────────────────────────────────
    VM               Provider   Shape                CPU    RAM    Disk     Cost
────────────────────────────────────────────────────────────
   gcp-proxy        GCP        E2 Micro             2      2G     30G      Free
   gcp-t4           GCP        N1-Std-4 + T4 GPU    4      15G    100G     Spot
   oci-mail         OCI        E2 Micro             2      1G     47G      Free
   oci-analytics    OCI        E2 Micro             2      1G     47G      Free
   oci-apps         OCI        A1 Flex (ARM)        4      24G    100G     Free
   github-actions   GitHub     ubuntu-latest        4      16G    14G      2000min/mo
   ghcr.io          GitHub     Container Registry   -      -      ∞        Free (public)

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
    gcp-proxy            ~/git/cloud/b_infra/home-manager/gcp-proxy/src/
    oci-apps             ~/git/cloud/b_infra/home-manager/oci-apps/src/
    oci-mail             ~/git/cloud/b_infra/home-manager/oci-mail/src/
    oci-analytics        ~/git/cloud/b_infra/home-manager/oci-analytics/src/

  GHA WORKFLOWS
    ship-gcp-proxy       ~/git/cloud/.github/workflows/ship-gcp-proxy.yml
    ship-oci-apps        ~/git/cloud/.github/workflows/ship-oci-apps.yml
    ship-oci-mail        ~/git/cloud/.github/workflows/ship-oci-mail.yml
    ship-oci-analytics   ~/git/cloud/.github/workflows/ship-oci-analytics.yml
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

VAULT — CLI Access Providers
────────────────────────────────────────────────────────────
  🔑 anthropic
  🔑 authelia
  🔑 aws
  🔑 c3-api
  🔑 cloudflare
  🔑 cloudflare-wrangler
  🔑 crawlee
  🔑 gcloud
  🔑 github
  🔑 gpg
  🔑 nocodb
  🔑 oci
  🔑 resend
  🔑 ssh-s21
  🔑 ssh-surface-pro
  🔑 system
  🔑 vaultwarden
  🔑 wireguard

GITHUB / GHCR
────────────────────────────────────────────────────────────
  👤 User:       diegonmarcos
  📦 GHCR images: 85
  🔗 Registry:   ghcr.io/diegonmarcos/

PERFORMANCE
────────────────────────────────────────────────────────────
  private_dns         27204ms ███
  vm_gcp-proxy        20700ms ███
  api_mcp             18066ms ██
  public_urls         17737ms ██
  mail_ports          10464ms █
  vm_oci-mail          6765ms █
  vm_oci-apps          4578ms █
  vm_oci-analytics     4371ms █
  TOTAL              239514ms

```
