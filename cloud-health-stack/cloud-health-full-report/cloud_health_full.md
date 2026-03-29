```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-03-29T21:02:02.324954859+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  170 issues: 20 critical, 125 warnings, 25 info

  CRITICAL:
    [X] C3 API (mesh): http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health)
    [X] Hickory DNS resolver: dig caddy.app @10.0.0.1 -> NXDOMAIN
    [X] WG gcp-proxy: gcp-proxy (10.0.0.1): TCP=ok SSH=fail
    [X] WG gcp-t4: gcp-t4 (10.0.0.8): TCP=fail SSH=fail
    [X] WG oci-apps: oci-apps (10.0.0.6): TCP=fail SSH=fail
    [X] Platform gcp-proxy: gcp-proxy: unreachable (WG down)
    [X] Platform gcp-t4: gcp-t4: unreachable (WG down)
    [X] Platform oci-apps: oci-apps: unreachable (WG down)
    [X] Container umami/umami-setup: umami-setup on oci-analytics: Exited (1) About an hour ago (exited)
    [X] Private URLs (Hickory): Hickory DNS at 10.0.0.1 is down — cannot resolve .app domains
    [X] TLS cert diegonmarcos.com: TLS connection failed
    [X] TLS cert api.diegonmarcos.com: TLS connection failed
    [X] TLS cert auth.diegonmarcos.com: TLS connection failed
    [X] TLS cert mail.diegonmarcos.com: TLS connection failed
    [X] TLS cert vault.diegonmarcos.com: TLS connection failed
    [X] Authelia health: auth.diegonmarcos.com/api/health -> err: error sending request for url (https://auth.diegonmarcos.com/api/health)
    [X] Firewall oci-mail: oci-mail: DANGEROUS ports open: 8080
    [X] SSH ports gcp-t4: gcp-t4: SSH:22=closed Dropbear:2200=closed
    [X] SSH ports oci-mail: oci-mail: SSH:22=closed Dropbear:2200=closed
    [X] SSH ports oci-analytics: oci-analytics: SSH:22=closed Dropbear:2200=closed
  WARNINGS:
    [!] C3 API (public): https://api.diegonmarcos.com/c3-api/health -> err: error sending request for url (https://api.diegonmarcos.com/c3-api/health)
    [!] SSH agent: no SSH agent or no keys
    [!] Container authelia/authelia: authelia on gcp-proxy: VM unreachable
    [!] Container authelia/authelia-redis: authelia-redis on gcp-proxy: VM unreachable
    [!] Container backup-borg/backup-borg: backup-borg on oci-apps: VM unreachable
    [!] Container backup-bup/backup-bup: backup-bup on oci-apps: VM unreachable
    [!] Container backup-gitea/gitea: gitea on oci-apps: VM unreachable
    [!] Container c3-diego-personal-data-mcp/c3-diego-personal-data-mcp: c3-diego-personal-data-mcp on local: VM unreachable
    [!] Container c3-infra-api/c3-infra-api: c3-infra-api on oci-apps: VM unreachable
    [!] Container c3-infra-mcp/c3-infra-mcp: c3-infra-mcp on oci-apps: VM unreachable
    [!] Container c3-services-api/c3-services-api: c3-services-api on oci-apps: VM unreachable
    [!] Container c3-services-mcp/c3-services-mcp: c3-services-mcp on oci-apps: VM unreachable
    [!] Container caddy/caddy: caddy on gcp-proxy: VM unreachable
    [!] Container caddy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    [!] Container caddy-l4-image/caddy-l4-image: caddy-l4-image on local: VM unreachable
    [!] Container cloud-cgc-mcp/cloud-cgc-mcp: cloud-cgc-mcp on oci-apps: VM unreachable
    [!] Container cloud-spec/cloud-spec: cloud-spec on oci-apps: VM unreachable
    [!] Container cloudflare/cloudflare: cloudflare on local: VM unreachable
    [!] Container cloudflare-worker/cloudflare-worker: cloudflare-worker on local: VM unreachable
    [!] Container code-server/code-server: code-server on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_api: crawlee_api on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_dashboard: crawlee_dashboard on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_db: crawlee_db on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_minio: crawlee_minio on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_redis: crawlee_redis on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_runner: crawlee_runner on oci-apps: VM unreachable
    [!] Container crawlee-cloud/crawlee_scheduler: crawlee_scheduler on oci-apps: VM unreachable
    [!] Container db-agent/db-agent: db-agent on all: VM unreachable
    [!] Container etherpad/etherpad_app: etherpad_app on oci-apps: VM unreachable
    [!] Container etherpad/etherpad_postgres: etherpad_postgres on oci-apps: VM unreachable
    [!] Container filebrowser/filebrowser_app: filebrowser_app on oci-apps: VM unreachable
    [!] Container gcloud/gcloud: gcloud on local: VM unreachable
    [!] Container gitea/gitea: gitea on oci-apps: VM unreachable
    [!] Container google-workspace-mcp/google-workspace-mcp: google-workspace-mcp on oci-apps: VM unreachable
    [!] Container grist/grist_app: grist_app on oci-apps: VM unreachable
    [!] Container hedgedoc/hedgedoc_app: hedgedoc_app on oci-apps: VM unreachable
    [!] Container hedgedoc/hedgedoc_postgres: hedgedoc_postgres on oci-apps: VM unreachable
    [!] Container hickory-dns/hickory-dns: hickory-dns on gcp-proxy: VM unreachable
    [!] Container introspect-proxy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    [!] Container kg-graph/kg-graph: kg-graph on oci-apps: VM unreachable
    [!] Container lgtm/lgtm_grafana: lgtm_grafana on oci-apps: VM unreachable
    [!] Container lgtm/lgtm_loki: lgtm_loki on oci-apps: VM unreachable
    [!] Container lgtm/lgtm_mimir: lgtm_mimir on oci-apps: VM unreachable
    [!] Container lgtm/lgtm_tempo: lgtm_tempo on oci-apps: VM unreachable
    [!] Container mail-mcp/mail-mcp: mail-mcp on oci-apps: VM unreachable
    [!] Container mattermost-bots/mattermost: mattermost on oci-apps: VM unreachable
    [!] Container mattermost-bots/mattermost-bots: mattermost-bots on oci-apps: VM unreachable
    [!] Container mattermost-bots/mattermost-postgres: mattermost-postgres on oci-apps: VM unreachable
    [!] Container mattermost-mcp/mattermost-mcp: mattermost-mcp on oci-apps: VM unreachable
    [!] Container nocodb/nocodb: nocodb on oci-apps: VM unreachable
    [!] Container nocodb/nocodb-db: nocodb-db on oci-apps: VM unreachable
    [!] Container ntfy/ntfy: ntfy on gcp-proxy: VM unreachable
    [!] Container ntfy/github-rss: github-rss on gcp-proxy: VM unreachable
    [!] Container ntfy/syslog-bridge: syslog-bridge on gcp-proxy: VM unreachable
    [!] Container ollama/ollama: ollama on gcp-t4: VM unreachable
    [!] Container ollama-arm/ollama-arm: ollama-arm on oci-apps-2: VM unreachable
    [!] Container ollama-hai/ollama-hai: ollama-hai on oci-apps: VM unreachable
    [!] Container photoprism/photoprism_app: photoprism_app on oci-apps: VM unreachable
    [!] Container photoprism/photoprism_mariadb: photoprism_mariadb on oci-apps: VM unreachable
    [!] Container photoprism/photoprism_rclone: photoprism_rclone on oci-apps: VM unreachable
    [!] Container photos-webhook/photos-webhook: photos-webhook on oci-apps: VM unreachable
    [!] Container postlite/postlite: postlite on gcp-proxy: VM unreachable
    [!] Container quant-lab-full/quant_full_analytics: quant_full_analytics on oci-apps: VM unreachable
    [!] Container quant-lab-full/quant_full_db: quant_full_db on oci-apps: VM unreachable
    [!] Container quant-lab-full/quant_full_engine: quant_full_engine on oci-apps: VM unreachable
    [!] Container quant-lab-full/quant_full_ml: quant_full_ml on oci-apps: VM unreachable
    [!] Container quant-lab-full/quant_full_research: quant_full_research on oci-apps: VM unreachable
    [!] Container quant-lab-full/quant_full_risk: quant_full_risk on oci-apps: VM unreachable
    [!] Container quant-lab-light/quant_light_db: quant_light_db on oci-apps: VM unreachable
    [!] Container quant-lab-light/quant_light_engine: quant_light_engine on oci-apps: VM unreachable
    [!] Container quant-lab-light/quant_light_research: quant_light_research on oci-apps: VM unreachable
    [!] Container radicale/radicale: radicale on oci-apps: VM unreachable
    [!] Container redis/redis: redis on gcp-proxy: VM unreachable
    [!] Container revealmd/revealmd_app: revealmd_app on oci-apps: VM unreachable
    [!] Container rig-agentic-hai-1.5bq4/rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 on oci-apps: VM unreachable
    [!] Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: VM unreachable
    [!] Container sauron-central/sauron-central: sauron-central on oci-apps: VM unreachable
    [!] Container sauron-lite/sauron-lite: sauron-lite on all: VM unreachable
    [!] Container vaultwarden/vaultwarden: vaultwarden on gcp-proxy: VM unreachable
    [!] Container windmill/windmill-server: windmill-server on oci-apps: VM unreachable
    [!] Container windmill/windmill-db: windmill-db on oci-apps: VM unreachable
    [!] Container windmill/windmill-worker: windmill-worker on oci-apps: VM unreachable
    [!] Public auth.diegonmarcos.com: auth.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://auth.diegonmarcos.com/), auth=0)
    [!] Public git.diegonmarcos.com: git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0)
    [!] Public api.diegonmarcos.com/c3-api: api.diegonmarcos.com/c3-api: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/c3-api), auth=0)
    [!] Public mcp.diegonmarcos.com/c3-infra-mcp: mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=0)
    [!] Public api.diegonmarcos.com/services: api.diegonmarcos.com/services: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/services), auth=0)
    [!] Public proxy.diegonmarcos.com: proxy.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=0)
    [!] Public ide.diegonmarcos.com: ide.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://ide.diegonmarcos.com/), auth=0)
    [!] Public api.diegonmarcos.com: api.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=0)
    [!] Public workflows.diegonmarcos.com: workflows.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=0)
    [!] Public logs.diegonmarcos.com: logs.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=0)
    [!] Public pad.diegonmarcos.com: pad.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=0)
    [!] Public files.diegonmarcos.com: files.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=0)
    [!] Public git.diegonmarcos.com: git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0)
    [!] Public sheets.diegonmarcos.com: sheets.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=0)
    [!] Public doc.diegonmarcos.com: doc.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=0)
    [!] Public dns.internal: dns.internal: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://dns.internal/), auth=0)
    [!] Public grafana.diegonmarcos.com: grafana.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=0)
    [!] Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0)
    [!] Public chat.diegonmarcos.com: chat.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=0)
    [!] Public db.diegonmarcos.com: db.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=0)
    [!] Public rss.diegonmarcos.com: rss.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=0)
    [!] Public photos.diegonmarcos.com: photos.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=0)
    [!] Public cal.diegonmarcos.com: cal.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=0)
    [!] Public slides.diegonmarcos.com: slides.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://slides.diegonmarcos.com/), auth=0)
    [!] Public smtp.diegonmarcos.com: smtp.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=0)
    [!] Public webmail.diegonmarcos.com: webmail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=0)
    [!] Public mail.diegonmarcos.com: mail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=0)
    [!] Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0)
    [!] Public vault.diegonmarcos.com: vault.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=0)
    [!] Public windmill.diegonmarcos.com: windmill.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=0)
    [!] Cross dagu: container up, public down: dagu: containers healthy but public URL workflows.diegonmarcos.com unreachable — check Caddy/Cloudflare
    [!] Cross dozzle: container up, public down: dozzle: containers healthy but public URL logs.diegonmarcos.com unreachable — check Caddy/Cloudflare
    [!] Cross matomo: container up, public down: matomo: containers healthy but public URL analytics.diegonmarcos.com unreachable — check Caddy/Cloudflare
    [!] Cross smtp-proxy: container up, public down: smtp-proxy: containers healthy but public URL smtp.diegonmarcos.com unreachable — check Caddy/Cloudflare
    [!] Cross snappymail: container up, public down: snappymail: containers healthy but public URL webmail.diegonmarcos.com unreachable — check Caddy/Cloudflare
    [!] Cross stalwart: container up, public down: stalwart: containers healthy but public URL mail.diegonmarcos.com unreachable — check Caddy/Cloudflare
    [!] GHA workflows: 5 recent runs, 2 failed
    [!] Drift unmanaged: oci-mail/palantir-cron: palantir-cron running on oci-mail but not declared in topology
    [!] Drift unmanaged: oci-mail/fluent-bit: fluent-bit running on oci-mail but not declared in topology
    [!] Drift unmanaged: oci-mail/introspect-proxy: introspect-proxy running on oci-mail but not declared in topology
    [!] Drift exited: oci-analytics/umami-setup: umami-setup on oci-analytics is exited: Exited (1) About an hour ago
    [!] Drift exited: oci-mail/introspect-proxy: introspect-proxy on oci-mail is exited: Exited (255) 2 hours ago
    [!] Caddy TLS: proxy.diegonmarcos.com -> err: error sending request for url (https://proxy.diegonmarcos.com/)
  INFO:
    [-] Drift no-domain: c3-diego-personal-data-mcp: c3-diego-personal-data-mcp has containers but no domain assigned
    [-] Drift no-domain: c3-services-mcp: c3-services-mcp has containers but no domain assigned
    [-] Drift no-domain: cloud-cgc-mcp: cloud-cgc-mcp has containers but no domain assigned
    [-] Drift no-domain: cloud-spec: cloud-spec has containers but no domain assigned
    [-] Drift no-domain: cloudflare: cloudflare has containers but no domain assigned
    [-] Drift no-domain: cloudflare-worker: cloudflare-worker has containers but no domain assigned
    [-] Drift no-domain: db-agent: db-agent has containers but no domain assigned
    [-] Drift no-domain: gcloud: gcloud has containers but no domain assigned
    [-] Drift no-domain: google-workspace-mcp: google-workspace-mcp has containers but no domain assigned
    [-] Drift no-domain: kg-graph: kg-graph has containers but no domain assigned
    [-] Drift no-domain: mail-mcp: mail-mcp has containers but no domain assigned
    [-] Drift no-domain: mattermost-mcp: mattermost-mcp has containers but no domain assigned
    [-] Drift no-domain: ollama: ollama has containers but no domain assigned
    [-] Drift no-domain: ollama-arm: ollama-arm has containers but no domain assigned
    [-] Drift no-domain: ollama-hai: ollama-hai has containers but no domain assigned
    [-] Drift no-domain: photos-webhook: photos-webhook has containers but no domain assigned
    [-] Drift no-domain: postlite: postlite has containers but no domain assigned
    [-] Drift no-domain: quant-lab-full: quant-lab-full has containers but no domain assigned
    [-] Drift no-domain: quant-lab-light: quant-lab-light has containers but no domain assigned
    [-] Drift no-domain: redis: redis has containers but no domain assigned
    [-] Drift no-domain: rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 has containers but no domain assigned
    [-] Drift no-domain: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has containers but no domain assigned
    [-] Drift no-domain: sauron-lite: sauron-lite has containers but no domain assigned
    [-] Drift no-port-in-build: hickory-dns: hickory-dns has port in topology but missing ports.app in build.json
    [-] Drift no-port-in-build: lgtm: lgtm has port in topology but missing ports.app in build.json


1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✗ http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health) (8.0s) [CRITICAL]
  ! https://api.diegonmarcos.com/c3-api/health -> err: error sending request for url (https://api.diegonmarcos.com/c3-api/health) (0.2s) [WARNING]
  ✓ TCP 10.0.0.1:22 -> open (0.2s)
  ✓ Docker 27.5.1 (0.2s)
  ! no SSH agent or no keys (0.0s) [WARNING]
  ✓ generated 2026-03-29T11:54:53.169Z (9h ago)
  ✗ dig caddy.app @10.0.0.1 -> NXDOMAIN (3.0s) [CRITICAL]

  Summary: 3/7 passed, 4 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ✗ gcp-proxy (10.0.0.1): TCP=ok SSH=fail (10.2s) [CRITICAL]
  ✗ gcp-t4 (10.0.0.8): TCP=fail SSH=fail (3.0s) [CRITICAL]
  ✗ oci-apps (10.0.0.6): TCP=fail SSH=fail (3.0s) [CRITICAL]
  ✓ oci-mail (10.0.0.3): TCP=ok SSH=ok (3.8s)
  ✓ oci-analytics (10.0.0.4): TCP=ok SSH=ok (7.7s)

  Summary: 2/5 passed, 3 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✓ oci-mail: mem 65%, disk 68%, load 0.97 0.98 1.00, 6/8 containers, up 0d 1h (2.6s)
  ✓ oci-analytics: mem 73%, disk 56%, load 2.08 2.12 2.09, 7/8 containers, up 0d 1h (4.1s)
  ✗ gcp-proxy: unreachable (WG down) [CRITICAL]
  ✗ gcp-t4: unreachable (WG down) [CRITICAL]
  ✗ oci-apps: unreachable (WG down) [CRITICAL]

  Summary: 2/5 passed, 3 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ✓ alerts-api on oci-analytics: Up About an hour (healthy) (healthy)
  ! authelia on gcp-proxy: VM unreachable [WARNING]
  ! authelia-redis on gcp-proxy: VM unreachable [WARNING]
  ! backup-borg on oci-apps: VM unreachable [WARNING]
  ! backup-bup on oci-apps: VM unreachable [WARNING]
  ! gitea on oci-apps: VM unreachable [WARNING]
  ! c3-diego-personal-data-mcp on local: VM unreachable [WARNING]
  ! c3-infra-api on oci-apps: VM unreachable [WARNING]
  ! c3-infra-mcp on oci-apps: VM unreachable [WARNING]
  ! c3-services-api on oci-apps: VM unreachable [WARNING]
  ! c3-services-mcp on oci-apps: VM unreachable [WARNING]
  ! caddy on gcp-proxy: VM unreachable [WARNING]
  ! introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ! caddy-l4-image on local: VM unreachable [WARNING]
  ! cloud-cgc-mcp on oci-apps: VM unreachable [WARNING]
  ! cloud-spec on oci-apps: VM unreachable [WARNING]
  ! cloudflare on local: VM unreachable [WARNING]
  ! cloudflare-worker on local: VM unreachable [WARNING]
  ! code-server on oci-apps: VM unreachable [WARNING]
  ! crawlee_api on oci-apps: VM unreachable [WARNING]
  ! crawlee_dashboard on oci-apps: VM unreachable [WARNING]
  ! crawlee_db on oci-apps: VM unreachable [WARNING]
  ! crawlee_minio on oci-apps: VM unreachable [WARNING]
  ! crawlee_redis on oci-apps: VM unreachable [WARNING]
  ! crawlee_runner on oci-apps: VM unreachable [WARNING]
  ! crawlee_scheduler on oci-apps: VM unreachable [WARNING]
  ✓ dagu on oci-mail: Up 2 hours (none)
  ! db-agent on all: VM unreachable [WARNING]
  ✓ dozzle on oci-analytics: Up About an hour (none)
  ! etherpad_app on oci-apps: VM unreachable [WARNING]
  ! etherpad_postgres on oci-apps: VM unreachable [WARNING]
  ! filebrowser_app on oci-apps: VM unreachable [WARNING]
  ✓ fluent-bit on oci-analytics: Up About an hour (none)
  ! gcloud on local: VM unreachable [WARNING]
  ! gitea on oci-apps: VM unreachable [WARNING]
  ! google-workspace-mcp on oci-apps: VM unreachable [WARNING]
  ! grist_app on oci-apps: VM unreachable [WARNING]
  ! hedgedoc_app on oci-apps: VM unreachable [WARNING]
  ! hedgedoc_postgres on oci-apps: VM unreachable [WARNING]
  ! hickory-dns on gcp-proxy: VM unreachable [WARNING]
  ! introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ! kg-graph on oci-apps: VM unreachable [WARNING]
  ! lgtm_grafana on oci-apps: VM unreachable [WARNING]
  ! lgtm_loki on oci-apps: VM unreachable [WARNING]
  ! lgtm_mimir on oci-apps: VM unreachable [WARNING]
  ! lgtm_tempo on oci-apps: VM unreachable [WARNING]
  ! mail-mcp on oci-apps: VM unreachable [WARNING]
  ✓ matomo-hybrid on oci-analytics: Up About an hour (none)
  ! mattermost on oci-apps: VM unreachable [WARNING]
  ! mattermost-bots on oci-apps: VM unreachable [WARNING]
  ! mattermost-postgres on oci-apps: VM unreachable [WARNING]
  ! mattermost-mcp on oci-apps: VM unreachable [WARNING]
  ! nocodb on oci-apps: VM unreachable [WARNING]
  ! nocodb-db on oci-apps: VM unreachable [WARNING]
  ! ntfy on gcp-proxy: VM unreachable [WARNING]
  ! github-rss on gcp-proxy: VM unreachable [WARNING]
  ! syslog-bridge on gcp-proxy: VM unreachable [WARNING]
  ! ollama on gcp-t4: VM unreachable [WARNING]
  ! ollama-arm on oci-apps-2: VM unreachable [WARNING]
  ! ollama-hai on oci-apps: VM unreachable [WARNING]
  ! photoprism_app on oci-apps: VM unreachable [WARNING]
  ! photoprism_mariadb on oci-apps: VM unreachable [WARNING]
  ! photoprism_rclone on oci-apps: VM unreachable [WARNING]
  ! photos-webhook on oci-apps: VM unreachable [WARNING]
  ! postlite on gcp-proxy: VM unreachable [WARNING]
  ! quant_full_analytics on oci-apps: VM unreachable [WARNING]
  ! quant_full_db on oci-apps: VM unreachable [WARNING]
  ! quant_full_engine on oci-apps: VM unreachable [WARNING]
  ! quant_full_ml on oci-apps: VM unreachable [WARNING]
  ! quant_full_research on oci-apps: VM unreachable [WARNING]
  ! quant_full_risk on oci-apps: VM unreachable [WARNING]
  ! quant_light_db on oci-apps: VM unreachable [WARNING]
  ! quant_light_engine on oci-apps: VM unreachable [WARNING]
  ! quant_light_research on oci-apps: VM unreachable [WARNING]
  ! radicale on oci-apps: VM unreachable [WARNING]
  ! redis on gcp-proxy: VM unreachable [WARNING]
  ! revealmd_app on oci-apps: VM unreachable [WARNING]
  ! rig-agentic-hai-1.5bq4 on oci-apps: VM unreachable [WARNING]
  ! rig-agentic-sonn-14bq8 on oci-apps: VM unreachable [WARNING]
  ! sauron-central on oci-apps: VM unreachable [WARNING]
  ✓ sauron-forwarder on oci-analytics: Up About an hour (none)
  ! sauron-lite on all: VM unreachable [WARNING]
  ✓ smtp-proxy on oci-mail: Up 2 hours (none)
  ✓ snappymail on oci-mail: Up 2 hours (healthy) (healthy)
  ✓ stalwart on oci-mail: Up 2 hours (none)
  ✓ syslog-forwarder on oci-mail: Up 2 hours (healthy) (healthy)
  ✓ umami on oci-analytics: Up About an hour (healthy) (healthy)
  ✓ umami-db on oci-analytics: Up About an hour (healthy) (healthy)
  ✗ umami-setup on oci-analytics: Exited (1) About an hour ago (exited) [CRITICAL]
  ! vaultwarden on gcp-proxy: VM unreachable [WARNING]
  ! windmill-server on oci-apps: VM unreachable [WARNING]
  ! windmill-db on oci-apps: VM unreachable [WARNING]
  ! windmill-worker on oci-apps: VM unreachable [WARNING]

  Summary: 12/93 passed, 81 failed

5. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ! auth.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://auth.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! api.diegonmarcos.com/c3-api: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/c3-api), auth=0) (0.3s) [WARNING]
  ! mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=0) (0.3s) [WARNING]
  ! api.diegonmarcos.com/services: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/services), auth=0) (0.3s) [WARNING]
  ! proxy.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! ide.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://ide.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! api.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! workflows.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! logs.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! pad.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! files.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! sheets.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! doc.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! dns.internal: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://dns.internal/), auth=0) (0.1s) [WARNING]
  ! grafana.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! chat.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! db.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=0) (0.3s) [WARNING]
  ! rss.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! photos.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! cal.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! slides.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://slides.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! smtp.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! webmail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! mail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! vault.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=0) (0.4s) [WARNING]
  ! windmill.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=0) (0.4s) [WARNING]

  Summary: 0/30 passed, 30 failed

6. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ✗ Hickory DNS at 10.0.0.1 is down — cannot resolve .app domains [CRITICAL]

  Summary: 0/1 passed, 1 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ! dagu: containers healthy but public URL workflows.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ! dozzle: containers healthy but public URL logs.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ! matomo: containers healthy but public URL analytics.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ! smtp-proxy: containers healthy but public URL smtp.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ! snappymail: containers healthy but public URL webmail.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ! stalwart: containers healthy but public URL mail.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]

  Summary: 0/6 passed, 6 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✓ dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.1s)
  ✓ ghcr.io/v2/ -> 401 (0.3s)
  ! 5 recent runs, 2 failed (0.7s) [WARNING]
  ✓ api.github.com/zen -> 403 (0.2s)
  ✓ MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.1s)
  ✓ mail.diegonmarcos.com -> 35.226.147.64 (0.1s)
  ✓ DKIM: present (0.0s)
  ✓ SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.1s)
  ✓ DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✓ api.resend.com -> 200 (0.4s)

  Summary: 9/10 passed, 1 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ! palantir-cron running on oci-mail but not declared in topology [WARNING]
  ! fluent-bit running on oci-mail but not declared in topology [WARNING]
  ! introspect-proxy running on oci-mail but not declared in topology [WARNING]
  ! umami-setup on oci-analytics is exited: Exited (1) About an hour ago [WARNING]
  ! introspect-proxy on oci-mail is exited: Exited (255) 2 hours ago [WARNING]
  - c3-diego-personal-data-mcp has containers but no domain assigned [INFO]
  - c3-services-mcp has containers but no domain assigned [INFO]
  - cloud-cgc-mcp has containers but no domain assigned [INFO]
  - cloud-spec has containers but no domain assigned [INFO]
  - cloudflare has containers but no domain assigned [INFO]
  - cloudflare-worker has containers but no domain assigned [INFO]
  - db-agent has containers but no domain assigned [INFO]
  - gcloud has containers but no domain assigned [INFO]
  - google-workspace-mcp has containers but no domain assigned [INFO]
  - kg-graph has containers but no domain assigned [INFO]
  - mail-mcp has containers but no domain assigned [INFO]
  - mattermost-mcp has containers but no domain assigned [INFO]
  - ollama has containers but no domain assigned [INFO]
  - ollama-arm has containers but no domain assigned [INFO]
  - ollama-hai has containers but no domain assigned [INFO]
  - photos-webhook has containers but no domain assigned [INFO]
  - postlite has containers but no domain assigned [INFO]
  - quant-lab-full has containers but no domain assigned [INFO]
  - quant-lab-light has containers but no domain assigned [INFO]
  - redis has containers but no domain assigned [INFO]
  - rig-agentic-hai-1.5bq4 has containers but no domain assigned [INFO]
  - rig-agentic-sonn-14bq8 has containers but no domain assigned [INFO]
  - sauron-lite has containers but no domain assigned [INFO]
  - hickory-dns has port in topology but missing ports.app in build.json [INFO]
  - lgtm has port in topology but missing ports.app in build.json [INFO]

  Summary: 0/30 passed, 30 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✗ TLS connection failed (0.2s) [CRITICAL]
  ✗ TLS connection failed (0.2s) [CRITICAL]
  ✗ TLS connection failed (0.2s) [CRITICAL]
  ✗ TLS connection failed (0.2s) [CRITICAL]
  ✗ TLS connection failed (0.2s) [CRITICAL]
  ✓ DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✓ SPF: present (strict=-all) (0.0s)
  ✗ auth.diegonmarcos.com/api/health -> err: error sending request for url (https://auth.diegonmarcos.com/api/health) (0.1s) [CRITICAL]
  ✓ gcp-proxy: no dangerous ports exposed (3.0s)
  ✓ gcp-t4: no dangerous ports exposed (3.0s)
  ✓ oci-apps: no dangerous ports exposed (3.0s)
  ✗ oci-mail: DANGEROUS ports open: 8080 (3.0s) [CRITICAL]
  ✓ oci-analytics: no dangerous ports exposed (3.0s)
  ✓ gcp-proxy: SSH:22=open Dropbear:2200=closed (3.1s)
  ✗ gcp-t4: SSH:22=closed Dropbear:2200=closed (6.0s) [CRITICAL]
  ✓ oci-apps: SSH:22=open Dropbear:2200=closed (3.0s)
  ✗ oci-mail: SSH:22=closed Dropbear:2200=closed (6.0s) [CRITICAL]
  ✗ oci-analytics: SSH:22=closed Dropbear:2200=closed (6.0s) [CRITICAL]
  ! proxy.diegonmarcos.com -> err: error sending request for url (https://proxy.diegonmarcos.com/) (0.3s) [WARNING]

  Summary: 8/19 passed, 11 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    53.9s
  L5_public_urls           27.9s
  L4-L10_parallel          27.9s
  L10_security             27.9s
  L6_private_urls          27.9s
  L8_external              27.9s
  L1_self_check            11.6s
  L2_wg_mesh               10.2s
  L3_platform              4.1s
  L4_containers            0.0s
  L9_drift                 0.0s
  L7_cross_checks          0.0s

  Total: 53.9s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 36/206 passed, 20 critical, 125 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
