```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗ 
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝ 
         CONTAINER HEALTH — 2026-03-28  06:14:54
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  A) HEALTH
══════════════════════════════════════════════════════════════

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1)
────────────────────────────────────────────────────────────
    Name               Public IP          WG IP          Handshake
────────────────────────────────────────────────────────────
✅ 10.0.0.5           88.30.43.183       10.0.0.5       18 seconds ago
✅ oci-mail           130.110.251.193    10.0.0.3       43 seconds ago
✅ oci-analytics      129.151.228.66     10.0.0.4       59 seconds ago
✅ oci-apps           82.70.229.129      10.0.0.6       1 minute, 24 seconds ago
❌ gcp-t4             34.173.227.250     10.0.0.8       never
❌ 10.0.0.9           none               10.0.0.9       never

PUBLIC URLs
────────────────────────────────────────────────────────────
❌ ide.diegonmarcos.com                → code-server.app:8444   [502]
❌ sheets.diegonmarcos.com             → grist.app:3011         [502]
❌ chat.diegonmarcos.com               → mattermost.app:8065    [502]
✅ photos.diegonmarcos.com             → photoprism.app:3013    [200]
❌ cal.diegonmarcos.com                → radicale.app:5232      [502]
✅ webmail.diegonmarcos.com            → snappymail.app:8888    [200]
✅ mail.diegonmarcos.com               → stalwart.app:8443      [200]
❌ vault.diegonmarcos.com              → vaultwarden.app:8880   [502]
✅ api.diegonmarcos.com                → crawlee.app:3000       [200]
❌ auth.diegonmarcos.com               → authelia.app:9091      [502]
❌ workflows.diegonmarcos.com          → dagu.app:8070          [502]
✅ grafana.diegonmarcos.com            → grafana.app:3200       [200]
❌ analytics.diegonmarcos.com          → matomo.app:8080        [502]
❌ db.diegonmarcos.com                 → nocodb.app:8085        [502]
❌ rss.diegonmarcos.com                → ntfy.app:8090          [502]
✅ windmill.diegonmarcos.com           → windmill-app.app:8000  [200]
✅ git.diegonmarcos.com                → backup-gitea.app:3002  [200]
✅ app.diegonmarcos.com                → path-based             [404]
✅ cloud.diegonmarcos.com              → path-based             [200]
✅ mcp.diegonmarcos.com                → MCP hub                [200]
❌ proxy.diegonmarcos.com              → Infrastructure dashboard (static HTML) [502]

API / MCP ENDPOINTS
────────────────────────────────────────────────────────────
❌ g-workspace            https://mcp.diegonmarcos.com/g-workspace/mcp          [502]
❌ mail-mcp               https://mcp.diegonmarcos.com/mail-mcp/mcp             [502]
❌ mattermost-mcp         https://mcp.diegonmarcos.com/mattermost-mcp/mcp       [502]
❌ c3-infra-mcp           https://mcp.diegonmarcos.com/c3-infra-mcp/mcp         [502]
❌ c3-services-mcp        https://mcp.diegonmarcos.com/c3-services-mcp/mcp      [502]
✅ cloud-cgc-mcp          https://mcp.diegonmarcos.com/cloud-cgc-mcp/mcp        [200]

MAIL PORTS
────────────────────────────────────────────────────────────
⚠️ mail.diegonmarcos.com        :993   IMAPS           tcp open
⚠️ imap.diegonmarcos.com        :993   IMAPS           tcp open
⚠️ mail.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ smtp.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ mail.diegonmarcos.com        :587   Submission      tcp open
⚠️ smtp.diegonmarcos.com        :587   Submission      tcp open

PRIVATE DNS (WireGuard mesh)
────────────────────────────────────────────────────────────
❌ authelia-redis.app           authelia-redis:6380       gcp-E2-f_0
❌ authelia.app                 authelia:9091             gcp-E2-f_0
❌ caddy.app                    caddy:443                 gcp-E2-f_0
❌ hickory-dns.app              hickory-dns:53            gcp-E2-f_0
❌ introspect-proxy.app         introspect-proxy:4182     gcp-E2-f_0
❌ ntfy.app                     ntfy:8090                 gcp-E2-f_0
❌ redis.app                    redis:6379                gcp-E2-f_0
❌ vaultwarden.app              vaultwarden:8880          gcp-E2-f_0
❌ ollama.app                   ollama:11434              gcp-T4-p_0
❌ backup-gitea.app             gitea:3002                oci-A1-f_0
❌ c3-infra-api.app             c3-infra-api:8081         oci-A1-f_0
❌ c3-infra-mcp.app             c3-infra-mcp:3100         oci-A1-f_0
❌ c3-services-api.app          c3-services-api:8082      oci-A1-f_0
❌ c3-services-mcp.app          c3-services-mcp:3101      oci-A1-f_0
❌ c3-spec.app                  cloud-spec:3080           oci-A1-f_0
❌ cloud-cgc-mcp.app            cloud-cgc-mcp:3105        oci-A1-f_0
❌ code-server.app              code-server:8444          oci-A1-f_0
❌ crawlee-dashboard.app        crawlee_dashboard:3001    oci-A1-f_0
❌ crawlee-db.app               crawlee_db:5433           oci-A1-f_0
❌ crawlee-minio.app            crawlee_minio:9000        oci-A1-f_0
❌ crawlee-redis.app            crawlee_redis:6381        oci-A1-f_0
❌ crawlee.app                  crawlee_api:3000          oci-A1-f_0
❌ etherpad.app                 etherpad_app:3012         oci-A1-f_0
❌ filebrowser.app              filebrowser_app:3015      oci-A1-f_0
❌ g-workspace-mcp.app          google-workspace-mcp:3104 oci-A1-f_0
❌ gitea.app                    gitea:3017                oci-A1-f_0
❌ grafana.app                  lgtm_grafana:3200         oci-A1-f_0
❌ grist.app                    grist_app:3011            oci-A1-f_0
❌ hedgedoc.app                 hedgedoc_app:3018         oci-A1-f_0
❌ lgtm-loki.app                lgtm_loki:3110            oci-A1-f_0
❌ lgtm-mimir.app               lgtm_mimir:9009           oci-A1-f_0
❌ lgtm-tempo.app               lgtm_tempo:3210           oci-A1-f_0
❌ mail-mcp.app                 mail-mcp:3103             oci-A1-f_0
❌ mattermost-mcp.app           mattermost-mcp:3102       oci-A1-f_0
❌ mattermost-postgres.app      mattermost-postgres:5435  oci-A1-f_0
❌ mattermost.app               mattermost:8065           oci-A1-f_0
❌ nocodb.app                   nocodb:8085               oci-A1-f_0
❌ ollama-hai.app               ollama-hai:11435          oci-A1-f_0
❌ photoprism.app               photoprism_app:3013       oci-A1-f_0
❌ radicale.app                 radicale:5232             oci-A1-f_0
❌ revealmd.app                 revealmd_app:3014         oci-A1-f_0
❌ windmill-app.app             windmill-server:8000      oci-A1-f_0
❌ windmill-db.app              windmill-db:5440          oci-A1-f_0
❌ dagu.app                     dagu:8070                 oci-E2-f_0
❌ snappymail.app               snappymail:8888           oci-E2-f_0
❌ stalwart.app                 stalwart:8443             oci-E2-f_0
❌ dozzle.app                   dozzle:9999               oci-E2-f_1
❌ matomo.app                   matomo-hybrid:8080        oci-E2-f_1
❌ umami-db.app                 umami-db:5442             oci-E2-f_1
❌ umami.app                    umami:3006                oci-E2-f_1

oci-mail ✅ — oci-mail — 1C/1G — mem 674M/954M (70%) — disk 67% — swap 283M/2559M — load 3.76 3.41 2.99 — 8/8 ctrs — up 2 hours, 47 minutes
────────────────────────────────────────────────────────────
  ✅ stalwart                  UP             Up About an hour
  ✅ caddy                     UP             Up 2 hours
  ✅ smtp-proxy                UP             Up 2 hours
  ✅ dagu                      UP             Up 3 hours
  ✅ fluent-bit                UP             Up 3 hours
  ✅ snappymail                HEALTHY        Up About an hour (healthy)
  ✅ introspect-proxy          HEALTHY        Up 2 hours (healthy)
  ✅ syslog-forwarder          HEALTHY        Up 2 hours (healthy)

oci-analytics ✅ — oci-analytics — 1C/1G — mem 729M/954M (76%) — disk 56% — swap 246M/2559M — load ? — 7/8 ctrs — up 2 hours, 13 minutes
────────────────────────────────────────────────────────────
  ❌ umami-setup               EXITED(1)      Exited (1) 31 minutes ago
  ✅ sauron-forwarder          UP             Up 2 hours
  ✅ matomo-hybrid             UP             Up 2 hours
  ✅ fluent-bit                UP             Up 2 hours
  ✅ dozzle                    UP             Up 2 hours
  ✅ alerts-api                HEALTHY        Up 17 minutes (healthy)
  ✅ umami                     HEALTHY        Up 32 minutes (healthy)
  ✅ umami-db                  HEALTHY        Up 32 minutes (healthy)

oci-apps ✅ — oci-apps — 4C/24G — mem 4684M/23975M (19%) — disk 63% — swap 0M/0M — load 0.18 0.51 0.60 — 50/53 ctrs — up 0d 11h
────────────────────────────────────────────────────────────
  ❌ crawlee_minio_init        EXITED(0)      Exited (0) 2 minutes ago
  ❌ photoprism_rclone         EXITED(1)      Exited (1) 10 minutes ago
  ❌ mattermost-bots           EXITED(1)      Exited (1) 9 minutes ago
  ✅ crawlee_runner            UP             Up 2 minutes
  ✅ crawlee_dashboard         UP             Up 2 minutes
  ✅ crawlee_scheduler         UP             Up 2 minutes
  ✅ windmill-worker           UP             Up 3 minutes
  ✅ siem-api                  UP             Up 7 minutes
  ✅ quant_light_engine        UP             Up 8 minutes
  ✅ mattermost-mcp            UP             Up 10 minutes
  ✅ mail-mcp                  UP             Up 11 minutes
  ✅ code-server               UP             Up 2 hours
  ✅ gitea                     UP             Up 46 minutes
  ✅ bup-server                UP             Up 3 hours
  ✅ borg-server               UP             Up 3 hours
  ✅ lgtm_mimir                UP             Up 3 hours
  ✅ lgtm_tempo                UP             Up 3 hours
  ✅ cloud-spec                UP             Up 3 hours
  ✅ crawlee_api               HEALTHY        Up 2 minutes (healthy)
  ✅ crawlee_minio             HEALTHY        Up 2 minutes (healthy)
  ✅ crawlee_db                HEALTHY        Up 2 minutes (healthy)
  ✅ crawlee_redis             HEALTHY        Up 2 minutes (healthy)
  ✅ windmill-server           HEALTHY        Up 3 minutes (healthy)
  ✅ windmill-db               HEALTHY        Up 3 minutes (healthy)
  ✅ syslog-central            HEALTHY        Up 7 minutes (healthy)
  ✅ ollama-hai                HEALTHY        Up 7 minutes (healthy)
  ✅ quant_light_research      HEALTHY        Up 8 minutes (healthy)
  ✅ quant_light_db            HEALTHY        Up 8 minutes (healthy)
  ✅ revealmd_app              HEALTHY        Up 9 minutes (healthy)
  ✅ photoprism_app            HEALTHY        Up 6 minutes (healthy)
  ✅ photoprism_mariadb        HEALTHY        Up 10 minutes (healthy)
  ✅ radicale                  HEALTHY        Up 10 minutes (healthy)
  ✅ mattermost                HEALTHY        Up 11 minutes (healthy)
  ✅ mattermost-postgres       HEALTHY        Up 11 minutes (healthy)
  ✅ hedgedoc_app              HEALTHY        Up 11 minutes (healthy)
  ✅ hedgedoc_postgres         HEALTHY        Up 11 minutes (healthy)
  ✅ grist_app                 HEALTHY        Up 12 minutes (healthy)
  ✅ google-workspace-mcp      HEALTHY        Up 12 minutes (healthy)
  ✅ etherpad_app              HEALTHY        Up 12 minutes (healthy)
  ✅ etherpad_postgres         HEALTHY        Up 12 minutes (healthy)
  ✅ filebrowser_app           HEALTHY        Up 12 minutes (healthy)
  ✅ rig-agentic-sonn-14bq8    HEALTHY        Up 2 hours (healthy)
  ✅ rig-agentic-hai           HEALTHY        Up 2 hours (healthy)
  ✅ surrealdb                 HEALTHY        Up 3 hours (healthy)
  ✅ nocodb                    HEALTHY        Up 3 hours (healthy)
  ✅ nocodb-db                 HEALTHY        Up 3 hours (healthy)
  ✅ lgtm_grafana              HEALTHY        Up 3 hours (healthy)
  ✅ lgtm_loki                 HEALTHY        Up 3 hours (healthy)
  ✅ c3-services-mcp           HEALTHY        Up 3 hours (healthy)
  ✅ c3-infra-mcp              HEALTHY        Up 3 hours (healthy)
  ✅ c3-infra-api              HEALTHY        Up 3 hours (healthy)
  ✅ photos-webhook            HEALTHY        Up 3 hours (healthy)
  ✅ photos-db                 HEALTHY        Up 3 hours (healthy)

gcp-t4 ❌ — gcp-t4 — 4C/15G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

gcp-proxy ✅ — gcp-proxy — 1C/1G — mem 1035M/1952M (53%) — disk 51% — swap 37M/3999M — load 1.47 2.81 2.38 — 16/19 ctrs — up 3 hours, 14 minutes
────────────────────────────────────────────────────────────
  ❌ vaultwarden               EXITED(1)      Exited (1) 5 minutes ago
  ❌ redis                     EXITED(1)      Exited (1) 6 minutes ago
  ❌ hickory-dns               EXITED(1)      Exited (1) 8 minutes ago
  ✅ fluent-bit                UP             Up 4 minutes
  ✅ sqlite-ntfy               UP             Up 6 minutes
  ✅ postlite-vaultwarden      UP             Up 6 minutes
  ✅ sqlite-vaultwarden        UP             Up 6 minutes
  ✅ postlite-ntfy             UP             Up 6 minutes
  ✅ postlite-npm              UP             Up 6 minutes
  ✅ sqlite-authelia           UP             Up 6 minutes
  ✅ sqlite-npm                UP             Up 6 minutes
  ✅ postlite-authelia         UP             Up 6 minutes
  ✅ authelia-redis            UP             Up 9 minutes
  ✅ caddy                     UP             Up 8 minutes
  ✅ syslog-bridge             UP             Up 7 minutes
  ✅ github-rss                UP             Up 7 minutes
  ✅ ntfy                      UP             Up 8 minutes
  ✅ authelia                  HEALTHY        Up 9 minutes (healthy)
  ✅ introspect-proxy          HEALTHY        Up 9 minutes (healthy)

RESOURCES
────────────────────────────────────────────────────────────
                   gcp-proxy      oci-apps       oci-mail       oci-analytics
────────────────────────────────────────────────────────────
OS                 oci-mail       oci-analytics  oci-apps       gcp-t4         gcp-proxy     
CPU                1 cores        1 cores        4 cores        4 cores        1 cores       
RAM                674M/954M      729M/954M      4684M/23975M   ?/?            1035M/1952M   
RAM %              70%            76%            19%            0%             53%           
Swap               283M/2559M     246M/2559M     0M/0M          ?              37M/3999M     
Disk               28G/45G        25G/48G        57.5G/95.8G    ?/?            16G/31G       
Disk %             67%            56%            63%            ?              51%           
Load               3.76 3.41 2.99 ?              0.18 0.51 0.60 ?              1.47 2.81 2.38
Containers         8/8            7/8            50/53          0/0            16/19         
Uptime             2 hours, 47 minutes 2 hours, 13 minutes 0d 11h         ?              3 hours, 14 minutes

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
⚠️ cloud          main     b0ead8d2 fix: mount runner Docker config dir into HM cl
⚠️ cloud-data     main     e5cf2ac feat: container-health.ts — full stack health r
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
  vm_gcp-proxy        19504ms ███
  public_urls         17092ms ██
  vm_oci-mail         11672ms ██
  vm_oci-analytics    10193ms █
  vm_oci-apps          8963ms █
  vm_gcp-t4            5016ms █
  api_mcp              4553ms █
  mail_ports           1486ms 
  private_dns             1ms 
  TOTAL              208631ms

```
