```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-03-29T21:16:14.711171775+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  250 issues: 80 critical, 145 warnings, 25 info

  CRITICAL:
    ❌ C3 API (mesh): http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health)
    ❌ WG gcp-proxy: gcp-proxy (10.0.0.1): TCP=ok SSH=fail
    ❌ WG gcp-t4: gcp-t4 (10.0.0.8): TCP=fail SSH=fail
    ❌ Platform gcp-proxy: gcp-proxy: unreachable (WG down)
    ❌ Platform gcp-t4: gcp-t4: unreachable (WG down)
    ❌ Container backup-borg/backup-borg: backup-borg on oci-apps: NOT FOUND in docker ps
    ❌ Container backup-bup/backup-bup: backup-bup on oci-apps: NOT FOUND in docker ps
    ❌ Container backup-gitea/gitea: gitea on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container c3-infra-api/c3-infra-api: c3-infra-api on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container c3-services-api/c3-services-api: c3-services-api on oci-apps: NOT FOUND in docker ps
    ❌ Container c3-services-mcp/c3-services-mcp: c3-services-mcp on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container cloud-cgc-mcp/cloud-cgc-mcp: cloud-cgc-mcp on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container cloud-spec/cloud-spec: cloud-spec on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container code-server/code-server: code-server on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_api: crawlee_api on oci-apps: Exited (1) 18 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_dashboard: crawlee_dashboard on oci-apps: Exited (255) 16 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_db: crawlee_db on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_minio: crawlee_minio on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_redis: crawlee_redis on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_runner: crawlee_runner on oci-apps: Exited (1) 18 hours ago (exited)
    ❌ Container crawlee-cloud/crawlee_scheduler: crawlee_scheduler on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container etherpad/etherpad_app: etherpad_app on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container etherpad/etherpad_postgres: etherpad_postgres on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container filebrowser/filebrowser_app: filebrowser_app on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container gitea/gitea: gitea on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container google-workspace-mcp/google-workspace-mcp: google-workspace-mcp on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container grist/grist_app: grist_app on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container hedgedoc/hedgedoc_app: hedgedoc_app on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container hedgedoc/hedgedoc_postgres: hedgedoc_postgres on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container kg-graph/kg-graph: kg-graph on oci-apps: NOT FOUND in docker ps
    ❌ Container lgtm/lgtm_grafana: lgtm_grafana on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container lgtm/lgtm_loki: lgtm_loki on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container lgtm/lgtm_mimir: lgtm_mimir on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container lgtm/lgtm_tempo: lgtm_tempo on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container mail-mcp/mail-mcp: mail-mcp on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container mattermost-bots/mattermost: mattermost on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container mattermost-bots/mattermost-bots: mattermost-bots on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container mattermost-bots/mattermost-postgres: mattermost-postgres on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container mattermost-mcp/mattermost-mcp: mattermost-mcp on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container nocodb/nocodb: nocodb on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container nocodb/nocodb-db: nocodb-db on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container ollama-hai/ollama-hai: ollama-hai on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container photoprism/photoprism_app: photoprism_app on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container photoprism/photoprism_mariadb: photoprism_mariadb on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container photoprism/photoprism_rclone: photoprism_rclone on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container photos-webhook/photos-webhook: photos-webhook on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container quant-lab-full/quant_full_analytics: quant_full_analytics on oci-apps: NOT FOUND in docker ps
    ❌ Container quant-lab-full/quant_full_db: quant_full_db on oci-apps: NOT FOUND in docker ps
    ❌ Container quant-lab-full/quant_full_engine: quant_full_engine on oci-apps: NOT FOUND in docker ps
    ❌ Container quant-lab-full/quant_full_ml: quant_full_ml on oci-apps: NOT FOUND in docker ps
    ❌ Container quant-lab-full/quant_full_research: quant_full_research on oci-apps: NOT FOUND in docker ps
    ❌ Container quant-lab-full/quant_full_risk: quant_full_risk on oci-apps: NOT FOUND in docker ps
    ❌ Container quant-lab-light/quant_light_db: quant_light_db on oci-apps: Exited (1) 10 hours ago (exited)
    ❌ Container quant-lab-light/quant_light_engine: quant_light_engine on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container quant-lab-light/quant_light_research: quant_light_research on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container radicale/radicale: radicale on oci-apps: Exited (255) 16 hours ago (exited)
    ❌ Container revealmd/revealmd_app: revealmd_app on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container rig-agentic-hai-1.5bq4/rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 on oci-apps: NOT FOUND in docker ps
    ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container sauron-central/sauron-central: sauron-central on oci-apps: NOT FOUND in docker ps
    ❌ Container umami/umami-setup: umami-setup on oci-analytics: Exited (1) About an hour ago (exited)
    ❌ Container windmill/windmill-server: windmill-server on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container windmill/windmill-db: windmill-db on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Container windmill/windmill-worker: windmill-worker on oci-apps: Exited (255) 7 hours ago (exited)
    ❌ Drift missing: oci-apps/quant_full_db: quant_full_db declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/backup-bup: backup-bup declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/sauron-central: sauron-central declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/kg-graph: kg-graph declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/quant_full_engine: quant_full_engine declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/quant_full_ml: quant_full_ml declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/quant_full_analytics: quant_full_analytics declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/backup-borg: backup-borg declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/c3-services-api: c3-services-api declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/quant_full_risk: quant_full_risk declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/quant_full_research: quant_full_research declared in topology but not found in docker on oci-apps
    ❌ Firewall oci-mail: oci-mail: DANGEROUS ports open: 8080
    ❌ SSH ports gcp-t4: gcp-t4: SSH:22=closed Dropbear:2200=closed
    ❌ SSH ports oci-mail: oci-mail: SSH:22=closed Dropbear:2200=closed
    ❌ SSH ports oci-analytics: oci-analytics: SSH:22=closed Dropbear:2200=closed
  WARNINGS:
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  Container authelia/authelia: authelia on gcp-proxy: VM unreachable
    ⚠️  Container authelia/authelia-redis: authelia-redis on gcp-proxy: VM unreachable
    ⚠️  Container c3-diego-personal-data-mcp/c3-diego-personal-data-mcp: c3-diego-personal-data-mcp on local: VM unreachable
    ⚠️  Container caddy/caddy: caddy on gcp-proxy: VM unreachable
    ⚠️  Container caddy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container caddy-l4-image/caddy-l4-image: caddy-l4-image on local: VM unreachable
    ⚠️  Container cloudflare/cloudflare: cloudflare on local: VM unreachable
    ⚠️  Container cloudflare-worker/cloudflare-worker: cloudflare-worker on local: VM unreachable
    ⚠️  Container db-agent/db-agent: db-agent on all: VM unreachable
    ⚠️  Container gcloud/gcloud: gcloud on local: VM unreachable
    ⚠️  Container hickory-dns/hickory-dns: hickory-dns on gcp-proxy: VM unreachable
    ⚠️  Container introspect-proxy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/ntfy: ntfy on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/github-rss: github-rss on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/syslog-bridge: syslog-bridge on gcp-proxy: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container ollama-arm/ollama-arm: ollama-arm on oci-apps-2: VM unreachable
    ⚠️  Container postlite/postlite: postlite on gcp-proxy: VM unreachable
    ⚠️  Container redis/redis: redis on gcp-proxy: VM unreachable
    ⚠️  Container sauron-lite/sauron-lite: sauron-lite on all: VM unreachable
    ⚠️  Container vaultwarden/vaultwarden: vaultwarden on gcp-proxy: VM unreachable
    ⚠️  Public dns.internal: dns.internal: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://dns.internal/), auth=0)
    ⚠️  Private c3-infra-api: c3-infra-api.app (10.0.0.6:8081) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private c3-services-api: c3-services-api.app (10.0.0.6:8082) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private c3-services-mcp: c3-services-mcp.app (10.0.0.6:3101) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private cloud-cgc-mcp: cloud-cgc-mcp.app (10.0.0.6:3105) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private cloud-spec: c3-spec.app (10.0.0.6:3080) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private code-server: code-server.app (10.0.0.6:8443) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private crawlee-cloud: crawlee.app (10.0.0.6:3000) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private etherpad: etherpad.app (10.0.0.6:3012) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private filebrowser: filebrowser.app (10.0.0.6:3015) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private gitea: gitea.app (10.0.0.6:3017) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private google-workspace-mcp: g-workspace-mcp.app (10.0.0.6:3104) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private grist: grist.app (10.0.0.6:3011) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private hedgedoc: hedgedoc.app (10.0.0.6:3018) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private hickory-dns: hickory-dns.app (10.0.0.1:53) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.1:53/)]
    ⚠️  Private lgtm: grafana.app (10.0.0.6:3200) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private mail-mcp: mail-mcp.app (10.0.0.6:3103) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private matomo: matomo.app (10.0.0.4:8080) TCP=ok HTTP=fail [503]
    ⚠️  Private mattermost-bots: mattermost.app (10.0.0.6:8065) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private mattermost-mcp: mattermost-mcp.app (10.0.0.6:3102) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private nocodb: nocodb.app (10.0.0.6:8085) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private ollama: ollama.app (10.0.0.8:11434) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private ollama-hai: ollama-hai.app (10.0.0.6:11435) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private photoprism: photoprism.app (10.0.0.6:3013) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private radicale: radicale.app (10.0.0.6:5232) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private redis: redis.app (10.0.0.1:6379) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.1:6379/)]
    ⚠️  Private revealmd: revealmd.app (10.0.0.6:3014) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private stalwart: stalwart.app (10.0.0.3:443) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.3:443/)]
    ⚠️  Private vaultwarden: vaultwarden.app (10.0.0.1:8880) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.1:8880/)]
    ⚠️  Private windmill: windmill-app.app (10.0.0.6:8000) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Cross authelia: public up, container down: authelia: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross backup-gitea: public up, container down: backup-gitea: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross c3-infra-api: public up, container down: c3-infra-api: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross c3-services-api: public up, container down: c3-services-api: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross caddy: public up, container down: caddy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross code-server: public up, container down: code-server: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross crawlee-cloud: public up, container down: crawlee-cloud: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross etherpad: public up, container down: etherpad: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross filebrowser: public up, container down: filebrowser: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross gitea: public up, container down: gitea: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross grist: public up, container down: grist: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross hedgedoc: public up, container down: hedgedoc: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross lgtm: public up, container down: lgtm: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross mattermost-bots: public up, container down: mattermost-bots: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross nocodb: public up, container down: nocodb: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross ntfy: public up, container down: ntfy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross photoprism: public up, container down: photoprism: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross radicale: public up, container down: radicale: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross revealmd: public up, container down: revealmd: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross umami: public up, container down: umami: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross vaultwarden: public up, container down: vaultwarden: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross windmill: public up, container down: windmill: public URL responds but container is down — stale cache or wrong routing
    ⚠️  GHA workflows: 5 recent runs, 2 failed
    ⚠️  Drift unmanaged: oci-mail/introspect-proxy: introspect-proxy running on oci-mail but not declared in topology
    ⚠️  Drift unmanaged: oci-mail/fluent-bit: fluent-bit running on oci-mail but not declared in topology
    ⚠️  Drift unmanaged: oci-mail/palantir-cron: palantir-cron running on oci-mail but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/crawlee_minio_init: crawlee_minio_init running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/rig-agentic: rig-agentic running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/rig-agentic-hai: rig-agentic-hai running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/photos-db: photos-db running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/bup-server: bup-server running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/siem-api: siem-api running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/rig: rig running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/syslog-central: syslog-central running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/borg-server: borg-server running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/surrealdb: surrealdb running on oci-apps but not declared in topology
    ⚠️  Drift exited: oci-apps/windmill-worker: windmill-worker on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/windmill-server: windmill-server on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/windmill-db: windmill-db on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/rig: rig on oci-apps is exited: Exited (101) 9 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic: rig-agentic on oci-apps is exited: Exited (101) 9 hours ago
    ⚠️  Drift exited: oci-apps/photoprism_app: photoprism_app on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/photoprism_mariadb: photoprism_mariadb on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/photoprism_rclone: photoprism_rclone on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/mattermost-bots: mattermost-bots on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/c3-infra-api: c3-infra-api on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/lgtm_grafana: lgtm_grafana on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/lgtm_loki: lgtm_loki on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/lgtm_mimir: lgtm_mimir on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/lgtm_tempo: lgtm_tempo on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/c3-services-mcp: c3-services-mcp on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/ollama-hai: ollama-hai on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/mattermost: mattermost on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/mattermost-postgres: mattermost-postgres on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/mattermost-mcp: mattermost-mcp on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/mail-mcp: mail-mcp on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/hedgedoc_app: hedgedoc_app on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/hedgedoc_postgres: hedgedoc_postgres on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/etherpad_app: etherpad_app on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/etherpad_postgres: etherpad_postgres on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/google-workspace-mcp: google-workspace-mcp on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/quant_light_db: quant_light_db on oci-apps is exited: Exited (1) 10 hours ago
    ⚠️  Drift exited: oci-apps/nocodb: nocodb on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/nocodb-db: nocodb-db on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/surrealdb: surrealdb on oci-apps is exited: Exited (2) 10 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_runner: crawlee_runner on oci-apps is exited: Exited (1) 18 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_api: crawlee_api on oci-apps is exited: Exited (1) 18 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_minio_init: crawlee_minio_init on oci-apps is exited: Exited (0) 10 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_minio: crawlee_minio on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_db: crawlee_db on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_scheduler: crawlee_scheduler on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/cloud-cgc-mcp: cloud-cgc-mcp on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/gitea: gitea on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/bup-server: bup-server on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/borg-server: borg-server on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/cloud-spec: cloud-spec on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/syslog-central: syslog-central on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/siem-api: siem-api on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_dashboard: crawlee_dashboard on oci-apps is exited: Exited (255) 16 hours ago
    ⚠️  Drift exited: oci-apps/crawlee_redis: crawlee_redis on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic-hai: rig-agentic-hai on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/photos-webhook: photos-webhook on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/photos-db: photos-db on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/quant_light_engine: quant_light_engine on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/quant_light_research: quant_light_research on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/revealmd_app: revealmd_app on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/radicale: radicale on oci-apps is exited: Exited (255) 16 hours ago
    ⚠️  Drift exited: oci-apps/grist_app: grist_app on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/code-server: code-server on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-apps/filebrowser_app: filebrowser_app on oci-apps is exited: Exited (255) 7 hours ago
    ⚠️  Drift exited: oci-analytics/umami-setup: umami-setup on oci-analytics is exited: Exited (1) About an hour ago
    ⚠️  Drift exited: oci-mail/introspect-proxy: introspect-proxy on oci-mail is exited: Exited (255) 2 hours ago
  INFO:
    ℹ️  Drift no-domain: c3-diego-personal-data-mcp: c3-diego-personal-data-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: c3-services-mcp: c3-services-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-cgc-mcp: cloud-cgc-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-spec: cloud-spec has containers but no domain assigned
    ℹ️  Drift no-domain: cloudflare: cloudflare has containers but no domain assigned
    ℹ️  Drift no-domain: cloudflare-worker: cloudflare-worker has containers but no domain assigned
    ℹ️  Drift no-domain: db-agent: db-agent has containers but no domain assigned
    ℹ️  Drift no-domain: gcloud: gcloud has containers but no domain assigned
    ℹ️  Drift no-domain: google-workspace-mcp: google-workspace-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: kg-graph: kg-graph has containers but no domain assigned
    ℹ️  Drift no-domain: mail-mcp: mail-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: mattermost-mcp: mattermost-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: ollama: ollama has containers but no domain assigned
    ℹ️  Drift no-domain: ollama-arm: ollama-arm has containers but no domain assigned
    ℹ️  Drift no-domain: ollama-hai: ollama-hai has containers but no domain assigned
    ℹ️  Drift no-domain: photos-webhook: photos-webhook has containers but no domain assigned
    ℹ️  Drift no-domain: postlite: postlite has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-full: quant-lab-full has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-light: quant-lab-light has containers but no domain assigned
    ℹ️  Drift no-domain: redis: redis has containers but no domain assigned
    ℹ️  Drift no-domain: rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 has containers but no domain assigned
    ℹ️  Drift no-domain: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has containers but no domain assigned
    ℹ️  Drift no-domain: sauron-lite: sauron-lite has containers but no domain assigned
    ℹ️  Drift no-port-in-build: hickory-dns: hickory-dns has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: lgtm: lgtm has port in topology but missing ports.app in build.json


1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ❌ http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health) (0.3s) [CRITICAL]
  ✅ https://api.diegonmarcos.com/c3-api/health -> 302 (0.5s)
  ✅ TCP 10.0.0.1:22 -> open (0.1s)
  ✅ Docker 27.5.1 (0.2s)
  ⚠️  no SSH agent or no keys (0.0s) [WARNING]
  ✅ generated 2026-03-29T16:34:30.987Z (4h ago)
  ✅ dig caddy.app @10.0.0.1 -> 10.0.0.1 (0.1s)

  Summary: 5/7 passed, 2 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ❌ gcp-proxy (10.0.0.1): TCP=ok SSH=fail (10.1s) [CRITICAL]
  ❌ gcp-t4 (10.0.0.8): TCP=fail SSH=fail (3.0s) [CRITICAL]
  ✅ oci-apps (10.0.0.6): TCP=ok SSH=ok (3.5s)
  ✅ oci-mail (10.0.0.3): TCP=ok SSH=ok (4.1s)
  ✅ oci-analytics (10.0.0.4): TCP=ok SSH=ok (5.6s)

  Summary: 3/5 passed, 2 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ oci-apps: mem 3%, disk 76%, load 0.00 0.02 0.00, 1/56 containers, up 0d 7h (2.7s)
  ✅ oci-mail: mem 65%, disk 68%, load 0.97 0.98 1.00, 6/8 containers, up 0d 1h (2.4s)
  ✅ oci-analytics: mem 73%, disk 56%, load 2.08 2.12 2.09, 7/8 containers, up 0d 1h (3.3s)
  ❌ gcp-proxy: unreachable (WG down) [CRITICAL]
  ❌ gcp-t4: unreachable (WG down) [CRITICAL]

  Summary: 3/5 passed, 2 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ alerts-api on oci-analytics: Up About an hour (healthy) (healthy)
  ⚠️  authelia on gcp-proxy: VM unreachable [WARNING]
  ⚠️  authelia-redis on gcp-proxy: VM unreachable [WARNING]
  ❌ backup-borg on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ backup-bup on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ gitea on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ⚠️  c3-diego-personal-data-mcp on local: VM unreachable [WARNING]
  ❌ c3-infra-api on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ✅ c3-infra-mcp on oci-apps: Up 5 hours (healthy) (healthy)
  ❌ c3-services-api on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ c3-services-mcp on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ⚠️  caddy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  caddy-l4-image on local: VM unreachable [WARNING]
  ❌ cloud-cgc-mcp on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ cloud-spec on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ⚠️  cloudflare on local: VM unreachable [WARNING]
  ⚠️  cloudflare-worker on local: VM unreachable [WARNING]
  ❌ code-server on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ crawlee_api on oci-apps: Exited (1) 18 hours ago (exited) [CRITICAL]
  ❌ crawlee_dashboard on oci-apps: Exited (255) 16 hours ago (exited) [CRITICAL]
  ❌ crawlee_db on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ crawlee_minio on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ crawlee_redis on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ crawlee_runner on oci-apps: Exited (1) 18 hours ago (exited) [CRITICAL]
  ❌ crawlee_scheduler on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ✅ dagu on oci-mail: Up 2 hours (none)
  ⚠️  db-agent on all: VM unreachable [WARNING]
  ✅ dozzle on oci-analytics: Up About an hour (none)
  ❌ etherpad_app on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ etherpad_postgres on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ filebrowser_app on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ✅ fluent-bit on oci-analytics: Up About an hour (none)
  ⚠️  gcloud on local: VM unreachable [WARNING]
  ❌ gitea on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ google-workspace-mcp on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ grist_app on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ hedgedoc_app on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ hedgedoc_postgres on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ⚠️  hickory-dns on gcp-proxy: VM unreachable [WARNING]
  ⚠️  introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ❌ kg-graph on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ lgtm_grafana on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ lgtm_loki on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ lgtm_mimir on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ lgtm_tempo on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ mail-mcp on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ✅ matomo-hybrid on oci-analytics: Up About an hour (none)
  ❌ mattermost on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ mattermost-bots on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ mattermost-postgres on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ mattermost-mcp on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ nocodb on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ nocodb-db on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ⚠️  ntfy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  github-rss on gcp-proxy: VM unreachable [WARNING]
  ⚠️  syslog-bridge on gcp-proxy: VM unreachable [WARNING]
  ⚠️  ollama on gcp-t4: VM unreachable [WARNING]
  ⚠️  ollama-arm on oci-apps-2: VM unreachable [WARNING]
  ❌ ollama-hai on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ photoprism_app on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ photoprism_mariadb on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ photoprism_rclone on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ photos-webhook on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ⚠️  postlite on gcp-proxy: VM unreachable [WARNING]
  ❌ quant_full_analytics on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ quant_full_db on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ quant_full_engine on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ quant_full_ml on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ quant_full_research on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ quant_full_risk on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ quant_light_db on oci-apps: Exited (1) 10 hours ago (exited) [CRITICAL]
  ❌ quant_light_engine on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ quant_light_research on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ radicale on oci-apps: Exited (255) 16 hours ago (exited) [CRITICAL]
  ⚠️  redis on gcp-proxy: VM unreachable [WARNING]
  ❌ revealmd_app on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ rig-agentic-hai-1.5bq4 on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ❌ rig-agentic-sonn-14bq8 on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ sauron-central on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ✅ sauron-forwarder on oci-analytics: Up About an hour (none)
  ⚠️  sauron-lite on all: VM unreachable [WARNING]
  ✅ smtp-proxy on oci-mail: Up 2 hours (none)
  ✅ snappymail on oci-mail: Up 2 hours (healthy) (healthy)
  ✅ stalwart on oci-mail: Up 2 hours (none)
  ✅ syslog-forwarder on oci-mail: Up 2 hours (healthy) (healthy)
  ✅ umami on oci-analytics: Up About an hour (healthy) (healthy)
  ✅ umami-db on oci-analytics: Up About an hour (healthy) (healthy)
  ❌ umami-setup on oci-analytics: Exited (1) About an hour ago (exited) [CRITICAL]
  ⚠️  vaultwarden on gcp-proxy: VM unreachable [WARNING]
  ❌ windmill-server on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ windmill-db on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]
  ❌ windmill-worker on oci-apps: Exited (255) 7 hours ago (exited) [CRITICAL]

  Summary: 13/93 passed, 80 failed

5. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ✅ auth.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (9.5s)
  ✅ git.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (9.7s)
  ✅ api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (3.9s)
  ✅ mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.5s)
  ✅ api.diegonmarcos.com/services: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (3.8s)
  ✅ proxy.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (10.6s)
  ✅ ide.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)
  ✅ api.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.9s)
  ✅ workflows.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (9.7s)
  ✅ logs.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.9s)
  ✅ pad.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.5s)
  ✅ files.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.5s)
  ✅ git.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (9.7s)
  ✅ sheets.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)
  ✅ doc.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.5s)
  ⚠️  dns.internal: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://dns.internal/), auth=0) (0.1s) [WARNING]
  ✅ grafana.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)
  ✅ analytics.diegonmarcos.com: HTTPS=302 AUTH=503 (no-auth=302, auth=503) (9.7s)
  ✅ chat.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)
  ✅ db.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)
  ✅ rss.diegonmarcos.com: HTTPS=302 AUTH=401 (no-auth=302, auth=401) (9.5s)
  ✅ photos.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.9s)
  ✅ cal.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)
  ✅ slides.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.8s)
  ✅ smtp.diegonmarcos.com: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (4.2s)
  ✅ webmail.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (3.9s)
  ✅ mail.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (9.8s)
  ✅ analytics.diegonmarcos.com: HTTPS=302 AUTH=503 (no-auth=302, auth=503) (9.7s)
  ✅ vault.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=200) (12.4s)
  ✅ windmill.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (9.6s)

  Summary: 29/30 passed, 1 failed

6. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ✅ authelia.app (10.0.0.1:9091) TCP=ok HTTP=ok [200] (0.6s)
  ✅ backup-gitea.app (10.0.0.6:3002) TCP=ok HTTP=ok [200] (0.9s)
  ⚠️  c3-infra-api.app (10.0.0.6:8081) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ c3-infra-mcp.app (10.0.0.6:3100) TCP=ok HTTP=ok [404] (0.9s)
  ⚠️  c3-services-api.app (10.0.0.6:8082) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  c3-services-mcp.app (10.0.0.6:3101) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ caddy.app (10.0.0.1:443) TCP=ok HTTP=ok [400] (3.2s)
  ⚠️  cloud-cgc-mcp.app (10.0.0.6:3105) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  c3-spec.app (10.0.0.6:3080) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  code-server.app (10.0.0.6:8443) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  crawlee.app (10.0.0.6:3000) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ dagu.app (10.0.0.3:8070) TCP=ok HTTP=ok [200] (1.1s)
  ✅ dozzle.app (10.0.0.4:9999) TCP=ok HTTP=ok [200] (3.3s)
  ⚠️  etherpad.app (10.0.0.6:3012) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  filebrowser.app (10.0.0.6:3015) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  gitea.app (10.0.0.6:3017) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  g-workspace-mcp.app (10.0.0.6:3104) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  grist.app (10.0.0.6:3011) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  hedgedoc.app (10.0.0.6:3018) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  hickory-dns.app (10.0.0.1:53) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.1:53/)] (5.7s) [WARNING]
  ✅ introspect-proxy.app (10.0.0.1:4182) TCP=ok HTTP=ok [404] (0.6s)
  ⚠️  grafana.app (10.0.0.6:3200) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  mail-mcp.app (10.0.0.6:3103) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  matomo.app (10.0.0.4:8080) TCP=ok HTTP=fail [503] (1.8s) [WARNING]
  ⚠️  mattermost.app (10.0.0.6:8065) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  mattermost-mcp.app (10.0.0.6:3102) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  nocodb.app (10.0.0.6:8085) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ ntfy.app (10.0.0.1:8090) TCP=ok HTTP=ok [200] (0.6s)
  ⚠️  ollama.app (10.0.0.8:11434) TCP=fail HTTP=fail [tcp-fail] (3.1s) [WARNING]
  ⚠️  ollama-hai.app (10.0.0.6:11435) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  photoprism.app (10.0.0.6:3013) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  radicale.app (10.0.0.6:5232) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ⚠️  redis.app (10.0.0.1:6379) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.1:6379/)] (0.6s) [WARNING]
  ⚠️  revealmd.app (10.0.0.6:3014) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ smtp-proxy.app (10.0.0.3:8080) TCP=ok HTTP=ok [404] (1.1s)
  ✅ snappymail.app (10.0.0.3:8888) TCP=ok HTTP=ok [200] (1.1s)
  ⚠️  stalwart.app (10.0.0.3:443) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.3:443/)] (1.0s) [WARNING]
  ✅ umami.app (10.0.0.4:3006) TCP=ok HTTP=ok [200] (2.1s)
  ⚠️  vaultwarden.app (10.0.0.1:8880) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.1:8880/)] (8.3s) [WARNING]
  ⚠️  windmill-app.app (10.0.0.6:8000) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]

  Summary: 11/40 passed, 29 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ⚠️  authelia: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  backup-gitea: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  c3-infra-api: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ c3-infra-mcp: container=ok public=ok private=ok
  ⚠️  c3-services-api: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  caddy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  code-server: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  crawlee-cloud: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ dagu: container=ok public=ok private=ok
  ✅ dozzle: container=ok public=ok private=ok
  ⚠️  etherpad: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  filebrowser: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  gitea: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  grist: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  hedgedoc: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  lgtm: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  mattermost-bots: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  nocodb: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  ntfy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  photoprism: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  radicale: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  revealmd: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ smtp-proxy: container=ok public=ok private=ok
  ✅ snappymail: container=ok public=ok private=ok
  ⚠️  umami: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  vaultwarden: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  windmill: public URL responds but container is down — stale cache or wrong routing [WARNING]

  Summary: 5/27 passed, 22 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.0s)
  ✅ ghcr.io/v2/ -> 401 (0.3s)
  ⚠️  5 recent runs, 2 failed (0.7s) [WARNING]
  ✅ api.github.com/zen -> 403 (0.2s)
  ✅ MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ mail.diegonmarcos.com -> 35.226.147.64 (0.0s)
  ✅ DKIM: present (0.0s)
  ✅ SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ api.resend.com -> 200 (0.4s)

  Summary: 9/10 passed, 1 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ❌ quant_full_db declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ backup-bup declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ sauron-central declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ kg-graph declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ rig-agentic-hai-1.5bq4 declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ quant_full_engine declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ quant_full_ml declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ quant_full_analytics declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ backup-borg declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ c3-services-api declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ quant_full_risk declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ quant_full_research declared in topology but not found in docker on oci-apps [CRITICAL]
  ⚠️  introspect-proxy running on oci-mail but not declared in topology [WARNING]
  ⚠️  fluent-bit running on oci-mail but not declared in topology [WARNING]
  ⚠️  palantir-cron running on oci-mail but not declared in topology [WARNING]
  ⚠️  crawlee_minio_init running on oci-apps but not declared in topology [WARNING]
  ⚠️  rig-agentic running on oci-apps but not declared in topology [WARNING]
  ⚠️  rig-agentic-hai running on oci-apps but not declared in topology [WARNING]
  ⚠️  photos-db running on oci-apps but not declared in topology [WARNING]
  ⚠️  bup-server running on oci-apps but not declared in topology [WARNING]
  ⚠️  siem-api running on oci-apps but not declared in topology [WARNING]
  ⚠️  rig running on oci-apps but not declared in topology [WARNING]
  ⚠️  syslog-central running on oci-apps but not declared in topology [WARNING]
  ⚠️  borg-server running on oci-apps but not declared in topology [WARNING]
  ⚠️  surrealdb running on oci-apps but not declared in topology [WARNING]
  ⚠️  windmill-worker on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  windmill-server on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  windmill-db on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  rig on oci-apps is exited: Exited (101) 9 hours ago [WARNING]
  ⚠️  rig-agentic on oci-apps is exited: Exited (101) 9 hours ago [WARNING]
  ⚠️  photoprism_app on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  photoprism_mariadb on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  photoprism_rclone on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  mattermost-bots on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  c3-infra-api on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  lgtm_grafana on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  lgtm_loki on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  lgtm_mimir on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  lgtm_tempo on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  c3-services-mcp on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  ollama-hai on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  mattermost on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  mattermost-postgres on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  mattermost-mcp on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  mail-mcp on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  hedgedoc_app on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  hedgedoc_postgres on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  etherpad_app on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  etherpad_postgres on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  google-workspace-mcp on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  quant_light_db on oci-apps is exited: Exited (1) 10 hours ago [WARNING]
  ⚠️  nocodb on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  nocodb-db on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  surrealdb on oci-apps is exited: Exited (2) 10 hours ago [WARNING]
  ⚠️  crawlee_runner on oci-apps is exited: Exited (1) 18 hours ago [WARNING]
  ⚠️  crawlee_api on oci-apps is exited: Exited (1) 18 hours ago [WARNING]
  ⚠️  crawlee_minio_init on oci-apps is exited: Exited (0) 10 hours ago [WARNING]
  ⚠️  crawlee_minio on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  crawlee_db on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  crawlee_scheduler on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  cloud-cgc-mcp on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  gitea on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  bup-server on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  borg-server on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  cloud-spec on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  syslog-central on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  siem-api on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  crawlee_dashboard on oci-apps is exited: Exited (255) 16 hours ago [WARNING]
  ⚠️  crawlee_redis on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  rig-agentic-sonn-14bq8 on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  rig-agentic-hai on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  photos-webhook on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  photos-db on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  quant_light_engine on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  quant_light_research on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  revealmd_app on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  radicale on oci-apps is exited: Exited (255) 16 hours ago [WARNING]
  ⚠️  grist_app on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  code-server on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  filebrowser_app on oci-apps is exited: Exited (255) 7 hours ago [WARNING]
  ⚠️  umami-setup on oci-analytics is exited: Exited (1) About an hour ago [WARNING]
  ⚠️  introspect-proxy on oci-mail is exited: Exited (255) 2 hours ago [WARNING]
  ℹ️  c3-diego-personal-data-mcp has containers but no domain assigned [INFO]
  ℹ️  c3-services-mcp has containers but no domain assigned [INFO]
  ℹ️  cloud-cgc-mcp has containers but no domain assigned [INFO]
  ℹ️  cloud-spec has containers but no domain assigned [INFO]
  ℹ️  cloudflare has containers but no domain assigned [INFO]
  ℹ️  cloudflare-worker has containers but no domain assigned [INFO]
  ℹ️  db-agent has containers but no domain assigned [INFO]
  ℹ️  gcloud has containers but no domain assigned [INFO]
  ℹ️  google-workspace-mcp has containers but no domain assigned [INFO]
  ℹ️  kg-graph has containers but no domain assigned [INFO]
  ℹ️  mail-mcp has containers but no domain assigned [INFO]
  ℹ️  mattermost-mcp has containers but no domain assigned [INFO]
  ℹ️  ollama has containers but no domain assigned [INFO]
  ℹ️  ollama-arm has containers but no domain assigned [INFO]
  ℹ️  ollama-hai has containers but no domain assigned [INFO]
  ℹ️  photos-webhook has containers but no domain assigned [INFO]
  ℹ️  postlite has containers but no domain assigned [INFO]
  ℹ️  quant-lab-full has containers but no domain assigned [INFO]
  ℹ️  quant-lab-light has containers but no domain assigned [INFO]
  ℹ️  redis has containers but no domain assigned [INFO]
  ℹ️  rig-agentic-hai-1.5bq4 has containers but no domain assigned [INFO]
  ℹ️  rig-agentic-sonn-14bq8 has containers but no domain assigned [INFO]
  ℹ️  sauron-lite has containers but no domain assigned [INFO]
  ℹ️  hickory-dns has port in topology but missing ports.app in build.json [INFO]
  ℹ️  lgtm has port in topology but missing ports.app in build.json [INFO]

  Summary: 0/107 passed, 107 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✅ expires Jun 23 11:05:14 2026 GMT (85d) (0.9s)
  ✅ expires Jun 23 11:05:21 2026 GMT (85d) (0.9s)
  ✅ expires Jun 23 11:05:21 2026 GMT (85d) (0.9s)
  ✅ expires Jun 23 11:05:21 2026 GMT (85d) (0.9s)
  ✅ expires Jun 23 11:05:21 2026 GMT (85d) (0.9s)
  ✅ DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✅ SPF: present (strict=-all) (0.0s)
  ✅ auth.diegonmarcos.com/api/health -> 200 (2.5s)
  ✅ gcp-proxy: no dangerous ports exposed (3.0s)
  ✅ gcp-t4: no dangerous ports exposed (3.0s)
  ✅ oci-apps: no dangerous ports exposed (3.0s)
  ❌ oci-mail: DANGEROUS ports open: 8080 (3.0s) [CRITICAL]
  ✅ oci-analytics: no dangerous ports exposed (3.0s)
  ✅ gcp-proxy: SSH:22=open Dropbear:2200=closed (3.2s)
  ❌ gcp-t4: SSH:22=closed Dropbear:2200=closed (6.0s) [CRITICAL]
  ✅ oci-apps: SSH:22=open Dropbear:2200=closed (3.0s)
  ❌ oci-mail: SSH:22=closed Dropbear:2200=closed (6.0s) [CRITICAL]
  ❌ oci-analytics: SSH:22=closed Dropbear:2200=closed (6.0s) [CRITICAL]
  ✅ proxy.diegonmarcos.com -> 302 (0.6s)

  Summary: 15/19 passed, 4 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    46.1s
  L10_security             31.4s
  L8_external              31.4s
  L6_private_urls          31.4s
  L4-L10_parallel          31.4s
  L5_public_urls           31.4s
  L2_wg_mesh               10.1s
  L3_platform              3.3s
  L1_self_check            1.2s
  L7_cross_checks          0.0s
  L9_drift                 0.0s
  L4_containers            0.0s

  Total: 46.1s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 93/343 passed, 80 critical, 145 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
