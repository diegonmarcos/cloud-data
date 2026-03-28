```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
         CONTAINER HEALTH — 2026-03-28  13:01:46
════════════════════════════════════════════════════════════


══════════════════════════════════════════════════════════════
  A) HEALTH — Live checks
══════════════════════════════════════════════════════════════

── A1) URLs & Endpoints ──────────────────────────────────────

PUBLIC URLs
────────────────────────────────────────────────────────────
❌ ide.diegonmarcos.com                → code-server.app:8443   [502]
❌ sheets.diegonmarcos.com             → grist.app:3011         [502]
❌ chat.diegonmarcos.com               → mattermost.app:8065    [502]
❌ photos.diegonmarcos.com             → photoprism.app:3013    [---]
❌ cal.diegonmarcos.com                → radicale.app:5232      [502]
✅ webmail.diegonmarcos.com            → snappymail.app:8888    [200]
✅ mail.diegonmarcos.com               → stalwart.app:443       [200]
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

── A2) Containers & VMs ──────────────────────────────────────

WIREGUARD MESH (hub: gcp-proxy 10.0.0.1 — front door)
────────────────────────────────────────────────────────────
    Name               Public IP          WG IP          Type     Handshake
────────────────────────────────────────────────────────────
❌ oci-mail           130.110.251.193    10.0.0.3       VM       no data
❌ oci-analytics      129.151.228.66     10.0.0.4       VM       no data
❌ oci-apps           82.70.229.129      10.0.0.6       VM       no data
❌ gcp-t4             34.173.227.250     10.0.0.8       VM       no data
❌ gcp-proxy          35.226.147.64      10.0.0.1       HUB      no data
❌ surface            dynamic            10.0.0.5       CLIENT   no data
❌ termux             dynamic            10.0.0.9       CLIENT   no data

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
❌ code-server.app              code-server:8443          oci-A1-f_0
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
❌ stalwart.app                 stalwart:443              oci-E2-f_0
❌ dozzle.app                   dozzle:9999               oci-E2-f_1
❌ matomo.app                   matomo-hybrid:8080        oci-E2-f_1
❌ umami-db.app                 umami-db:5442             oci-E2-f_1
❌ umami.app                    umami:3006                oci-E2-f_1

oci-mail ✅ — oci-mail — 1C/1G — mem 675M/954M (70%) — disk 67% — swap 170M/2559M — load 0.04 0.14 0.54 — 7/8 ctrs — up 9 hours, 34 minutes
────────────────────────────────────────────────────────────
  ❌ caddy                     EXITED(0)      Exited (0) 6 hours ago
  ✅ stalwart                  UP             Up 6 hours
  ✅ smtp-proxy                UP             Up 9 hours
  ✅ dagu                      UP             Up 9 hours
  ✅ fluent-bit                UP             Up 10 hours
  ✅ snappymail                HEALTHY        Up 8 hours (healthy)
  ✅ introspect-proxy          HEALTHY        Up 9 hours (healthy)
  ✅ syslog-forwarder          HEALTHY        Up 9 hours (healthy)

oci-analytics ✅ — oci-analytics — 1C/1G — mem 724M/954M (75%) — disk 56% — swap 249M/2559M — load 0.35 0.47 0.90 — 7/8 ctrs — up 9 hours, 0 minutes
────────────────────────────────────────────────────────────
  ❌ umami-setup               EXITED(1)      Exited (1) 7 hours ago
  ✅ sauron-forwarder          UP             Up 9 hours
  ✅ matomo-hybrid             UP             Up 9 hours
  ✅ fluent-bit                UP             Up 9 hours
  ✅ dozzle                    UP             Up 9 hours
  ✅ alerts-api                HEALTHY        Up 7 hours (healthy)
  ✅ umami                     HEALTHY        Up 7 hours (healthy)
  ✅ umami-db                  HEALTHY        Up 7 hours (healthy)

oci-apps ✅ — oci-apps — 4C/24G — mem 4683M/23975M (19%) — disk 63% — swap 0M/0M — load 0.72 0.57 0.41 — 48/53 ctrs — up 0d 18h
────────────────────────────────────────────────────────────
  ❌ windmill-server           EXITED(137)    Exited (137) 7 hours ago
  ❌ syslog-central            EXITED(2)      Exited (2) 7 hours ago
  ❌ crawlee_minio_init        EXITED(0)      Exited (0) 7 hours ago
  ❌ photoprism_rclone         EXITED(1)      Exited (1) 7 hours ago
  ❌ mattermost-bots           EXITED(1)      Exited (1) 7 hours ago
  ✅ gitea                     UP             Up 7 hours
  ✅ bup-server                UP             Up 7 hours
  ✅ borg-server               UP             Up 7 hours
  ✅ windmill-worker           UP             Up 7 hours
  ✅ lgtm_tempo                UP             Up 7 hours
  ✅ lgtm_mimir                UP             Up 7 hours
  ✅ cloud-spec                UP             Up 7 hours
  ✅ siem-api                  UP             Up 7 hours
  ✅ crawlee_runner            UP             Up 7 hours
  ✅ crawlee_dashboard         UP             Up 7 hours
  ✅ crawlee_scheduler         UP             Up 7 hours
  ✅ quant_light_engine        UP             Up 7 hours
  ✅ mattermost-mcp            UP             Up 7 hours
  ✅ mail-mcp                  UP             Up 7 hours
  ✅ code-server               UP             Up 9 hours
  ✅ rig-agentic-sonn-14bq8    HEALTHY        Up 6 hours (healthy)
  ✅ rig-agentic-hai           HEALTHY        Up 6 hours (healthy)
  ✅ surrealdb                 HEALTHY        Up 7 hours (healthy)
  ✅ windmill-db               HEALTHY        Up 7 hours (healthy)
  ✅ nocodb                    HEALTHY        Up 7 hours (healthy)
  ✅ nocodb-db                 HEALTHY        Up 7 hours (healthy)
  ✅ lgtm_grafana              HEALTHY        Up 7 hours (healthy)
  ✅ lgtm_loki                 HEALTHY        Up 7 hours (healthy)
  ✅ c3-services-mcp           HEALTHY        Up 7 hours (healthy)
  ✅ c3-infra-mcp              HEALTHY        Up 7 hours (healthy)
  ✅ c3-infra-api              HEALTHY        Up 7 hours (healthy)
  ✅ photos-webhook            HEALTHY        Up 7 hours (healthy)
  ✅ photos-db                 HEALTHY        Up 7 hours (healthy)
  ✅ crawlee_api               HEALTHY        Up 7 hours (healthy)
  ✅ crawlee_minio             HEALTHY        Up 7 hours (healthy)
  ✅ crawlee_db                HEALTHY        Up 7 hours (healthy)
  ✅ crawlee_redis             HEALTHY        Up 7 hours (healthy)
  ✅ ollama-hai                HEALTHY        Up 7 hours (healthy)
  ✅ quant_light_research      HEALTHY        Up 7 hours (healthy)
  ✅ quant_light_db            HEALTHY        Up 7 hours (healthy)
  ✅ revealmd_app              HEALTHY        Up 7 hours (healthy)
  ✅ photoprism_app            HEALTHY        Up 7 hours (healthy)
  ✅ photoprism_mariadb        HEALTHY        Up 7 hours (healthy)
  ✅ radicale                  HEALTHY        Up 7 hours (healthy)
  ✅ mattermost                HEALTHY        Up 7 hours (healthy)
  ✅ mattermost-postgres       HEALTHY        Up 7 hours (healthy)
  ✅ hedgedoc_app              HEALTHY        Up 7 hours (healthy)
  ✅ hedgedoc_postgres         HEALTHY        Up 7 hours (healthy)
  ✅ grist_app                 HEALTHY        Up 7 hours (healthy)
  ✅ google-workspace-mcp      HEALTHY        Up 7 hours (healthy)
  ✅ etherpad_app              HEALTHY        Up 7 hours (healthy)
  ✅ etherpad_postgres         HEALTHY        Up 7 hours (healthy)
  ✅ filebrowser_app           HEALTHY        Up 7 hours (healthy)

gcp-t4 ❌ — gcp-t4 — 4C/15G — mem ?/? (0%) — disk ? — swap ? — load ? — 0/0 ctrs — ?
────────────────────────────────────────────────────────────

gcp-proxy ✅ — gcp-proxy — 1C/1G — mem 896M/1952M (45%) — disk ? — swap 33M/3999M — load ? — 18/19 ctrs — up 11 minutes
────────────────────────────────────────────────────────────
  ❌ redis                     EXITED(1)      Exited (1) 10 seconds ago
  ⚠️ vaultwarden               STARTING       Up 2 seconds (health: starting
  ✅ caddy                     UP             Up 2 minutes
  ✅ hickory-dns               UP             Up About a minute
  ✅ fluent-bit                UP             Up 2 minutes
  ✅ sqlite-ntfy               UP             Up 2 minutes
  ✅ postlite-vaultwarden      UP             Up 2 minutes
  ✅ sqlite-vaultwarden        UP             Up 2 minutes
  ✅ postlite-ntfy             UP             Up 2 minutes
  ✅ postlite-npm              UP             Up 2 minutes
  ✅ sqlite-authelia           UP             Up 2 minutes
  ✅ sqlite-npm                UP             Up 2 minutes
  ✅ postlite-authelia         UP             Up 2 minutes
  ✅ authelia-redis            UP             Up 2 minutes
  ✅ syslog-bridge             UP             Up 57 seconds
  ✅ github-rss                UP             Up 55 seconds
  ✅ ntfy                      UP             Up About a minute
  ✅ introspect-proxy          HEALTHY        Up 2 minutes (healthy)
  ✅ authelia                  HEALTHY        Up 2 minutes (healthy)


── A3) Mail ──────────────────────────────────────────────────

MAIL PORTS
────────────────────────────────────────────────────────────
⚠️ mail.diegonmarcos.com        :993   IMAPS           tcp open
⚠️ imap.diegonmarcos.com        :993   IMAPS           tcp open
⚠️ mail.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ smtp.diegonmarcos.com        :465   SMTPS           tcp open
⚠️ mail.diegonmarcos.com        :587   Submission      tcp open
⚠️ smtp.diegonmarcos.com        :587   Submission      tcp open

MAIL FLOW — Inbound Worker / Outbound Relay
────────────────────────────────────────────────────────────
  INBOUND (Cloudflare Email Routing → Stalwart)
  ─────────────────────────────────────────────
  📨 Cloudflare Worker   Cloudflare Email Worker - routes inbound email (me@diegonmarcos.com) to Mailu via SMTP proxy
     Route:              *@diegonmarcos.com → CF Worker → smtp-proxy:8080 → Stalwart
  ✅ smtp-proxy           Up 9 hours (oci-mail:8080)
  ✅ oci-mail:8080        reachable (CF Worker ingress)
  ✅ oci-mail:25          SMTP open (Stalwart local delivery)

  OUTBOUND PERSONAL (Stalwart → direct SMTP)
  ─────────────────────────────────────────────
  📤 Relay:              Stalwart → 130.110.251.193:465/587 → recipient MX
  ✅ stalwart             Up 6 hours (oci-mail MTA)
  ✅ smtp:465 (SMTPS)     open
  ✅ smtp:587 (Submission) open
  📋 SPF/DKIM/DMARC     via Cloudflare DNS (diegonmarcos.com)

  OUTBOUND TRANSACTIONAL (Resend API → SES)
  ─────────────────────────────────────────────
  📤 Provider:           Resend (Amazon SES us-east-1)
     Domain:             mails.diegonmarcos.com
     Terraform:          ~/git/cloud/b_infra/vps_resend/src/main.tf
  ✅ api.resend.com        [401] (401=needs key, 200=authed)
  ✅ MX send.mails         10 feedback-smtp.us-east-1.amazonses.com.
  ✅ SPF send.mails        "v=spf1 include:amazonses.com ~all"
  ✅ DKIM resend._dk       present


══════════════════════════════════════════════════════════════
  B) INFRA — Resources & Stack
══════════════════════════════════════════════════════════════

VPS / VM SPECS
────────────────────────────────────────────────────────────
    VM               Provider   Shape                CPU    RAM    Disk     Cost
────────────────────────────────────────────────────────────
   oci-E2-f_0       OCI        VM.Standard.E2.1.Micro 1      1G     47G      Free
   oci-E2-f_1       OCI        VM.Standard.E2.1.Micro 1      1G     47G      Free
   oci-A1-f_0       OCI        VM.Standard.A1.Flex  4      24G    100G     Free
   gcp-T4-p_0       GCP        n1-standard-4        4      15G    100G     Spot
   gcp-E2-f_0       GCP        e2-micro             1      1G     30G      Free
   vast-RTX-p_0     Vast.ai    ?                    ?      ?G     ?G       Spot
   github-actions   GitHub     ubuntu-latest        4      16G    14G      2000min/mo
   ghcr.io          GitHub     Container Registry   -      -      ∞        Free (public)

RESOURCES (live)
────────────────────────────────────────────────────────────
                   oci-mail       oci-analytics  oci-apps       gcp-t4         gcp-proxy     
────────────────────────────────────────────────────────────
OS                 oci-mail       oci-analytics  oci-apps       gcp-t4         gcp-proxy     
CPU                1 cores        1 cores        4 cores        4 cores        1 cores       
RAM                675M/954M      724M/954M      4683M/23975M   ?/?            896M/1952M    
RAM %              70%            75%            19%            0%             45%           
Swap               170M/2559M     249M/2559M     0M/0M          ?              33M/3999M     
Disk               28G/45G        25G/48G        57.5G/95.8G    ?/?            ?/?           
Disk %             67%            56%            63%            ?              ?             
Load               0.04 0.14 0.54 0.35 0.47 0.90 0.72 0.57 0.41 ?              ?             
Containers         7/8            7/8            48/53          0/0            18/19         
Uptime             9 hours, 34 minutes 9 hours, 0 minutes 0d 18h         ?              11 minutes    

GIT REPOSITORIES
────────────────────────────────────────────────────────────
⚠️ cloud          main     fb7697c2 fix: HM remote build — create nix-build symlin
⚠️ cloud-data     main     0076661 feat: template-driven health reporter + mail fl
✅ front          main     2df5bc69 add(task): Garmin fenix 8 custom watchface des
⚠️ unix           main     22e6ce5 feat: SSH stale socket cleaner — systemd user t
⚠️ tools          main     fc21dbb fix: L letter foot extended to 7-wide
⚠️ vault          main     ab6f5f0 chore: update vaultwarden setup.ts + tokens

GITHUB / GHCR
────────────────────────────────────────────────────────────
  👤 User:       diegonmarcos
  🔗 Registry:   ghcr.io/diegonmarcos/
  📦 Repos:      github.com/diegonmarcos/

  📦 GHCR Total:  84 (84 public, 0 private)

  Repo                         Public     Private    Total
  ──────────────────────────────────────────────────────────
  cloud                        74         0          74
  cloud-data                   8          0          8
  unix                         2          0          2

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


══════════════════════════════════════════════════════════════
  C) SECURITY
══════════════════════════════════════════════════════════════

OPEN PORTS by Public IP
────────────────────────────────────────────────────────────
🔓 oci-mail           130.110.251.193    ports: 22, 25, 465, 587, 993, 8080
🔓 oci-analytics      129.151.228.66     ports: 22
🔓 oci-apps           82.70.229.129      ports: 22
🔒 gcp-t4             34.173.227.250     ports: none reachable
🔓 gcp-proxy          35.226.147.64      ports: 22, 80, 443, 465, 587, 993, 2200

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

PERFORMANCE
────────────────────────────────────────────────────────────
  vm_gcp-proxy         71.6s ███████
  public_urls          34.7s ███
  vm_gcp-t4             8.0s █
  api_mcp               5.5s █
  vm_oci-apps           5.1s 
  vm_oci-mail           5.1s 
  vm_oci-analytics      4.7s 
  mail_ports            1.1s 
  private_dns           0.0s 
  TOTAL               326.7s

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/container-health.ts
Run: npx tsx container-health.ts
```
