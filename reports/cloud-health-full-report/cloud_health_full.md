```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-04-16T11:03:37.007362809+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  218 issues: 15 critical, 137 warnings, 66 info

  CRITICAL:
    ❌ C3 API (mesh): http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health)
    ❌ WG gcp-proxy: gcp-proxy (10.0.0.1): VPS=? Dropbear=fail WG:TCP=ok SSH=fail
    ❌ WG oci-apps: oci-apps (10.0.0.6): VPS=? Dropbear=fail WG:TCP=ok SSH=fail
    ❌ WG oci-mail: oci-mail (10.0.0.3): VPS=? Dropbear=fail WG:TCP=ok SSH=fail
    ❌ WG oci-analytics: oci-analytics (10.0.0.4): VPS=? Dropbear=fail WG:TCP=ok SSH=fail
    ❌ Platform gcp-proxy: gcp-proxy: unreachable (WG down)
    ❌ Platform oci-apps: oci-apps: unreachable (WG down)
    ❌ Platform oci-mail: oci-mail: unreachable (WG down)
    ❌ Platform oci-analytics: oci-analytics: unreachable (WG down)
    ❌ TLS cert diegonmarcos.com: TLS connection failed
    ❌ TLS cert api.diegonmarcos.com: TLS connection failed
    ❌ TLS cert auth.diegonmarcos.com: TLS connection failed
    ❌ TLS cert mail.diegonmarcos.com: TLS connection failed
    ❌ TLS cert vault.diegonmarcos.com: TLS connection failed
    ❌ Authelia health: auth.diegonmarcos.com/api/health -> err: error sending request for url (https://auth.diegonmarcos.com/api/health)
  WARNINGS:
    ⚠️  C3 API (public): https://api.diegonmarcos.com/c3-api/health -> err: error sending request for url (https://api.diegonmarcos.com/c3-api/health)
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  Container authelia/authelia: authelia on gcp-proxy: VM unreachable
    ⚠️  Container authelia/authelia-redis: authelia-redis on gcp-proxy: VM unreachable
    ⚠️  Container backup-gitea/gitea: gitea on oci-apps: VM unreachable
    ⚠️  Container c3-infra-api/c3-infra-api: c3-infra-api on oci-apps: VM unreachable
    ⚠️  Container c3-infra-mcp/c3-infra-mcp: c3-infra-mcp on oci-apps: VM unreachable
    ⚠️  Container c3-services-mcp/c3-services-mcp: c3-services-mcp on oci-apps: VM unreachable
    ⚠️  Container caddy/caddy: caddy on gcp-proxy: VM unreachable
    ⚠️  Container caddy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container cloud-cgc-mcp/cloud-cgc-mcp: cloud-cgc-mcp on oci-apps: VM unreachable
    ⚠️  Container cloud-spec/cloud-spec: cloud-spec on oci-apps: VM unreachable
    ⚠️  Container code-server/code-server: code-server on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_api: crawlee_api on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_dashboard: crawlee_dashboard on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_db: crawlee_db on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_minio: crawlee_minio on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_redis: crawlee_redis on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_runner: crawlee_runner on oci-apps: VM unreachable
    ⚠️  Container crawlee-cloud/crawlee_scheduler: crawlee_scheduler on oci-apps: VM unreachable
    ⚠️  Container dagu/dagu: dagu on oci-analytics: VM unreachable
    ⚠️  Container dbgate/dbgate: dbgate on oci-apps: VM unreachable
    ⚠️  Container dozzle/dozzle: dozzle on oci-analytics: VM unreachable
    ⚠️  Container etherpad/etherpad_app: etherpad_app on oci-apps: VM unreachable
    ⚠️  Container etherpad/etherpad_postgres: etherpad_postgres on oci-apps: VM unreachable
    ⚠️  Container filebrowser/filebrowser_app: filebrowser_app on oci-apps: VM unreachable
    ⚠️  Container fluent-bit/fluent-bit: fluent-bit on oci-analytics: VM unreachable
    ⚠️  Container gitea/gitea: gitea on oci-apps: VM unreachable
    ⚠️  Container google-workspace-mcp/google-workspace-mcp: google-workspace-mcp on oci-apps: VM unreachable
    ⚠️  Container grist/grist_app: grist_app on oci-apps: VM unreachable
    ⚠️  Container hedgedoc/hedgedoc_app: hedgedoc_app on oci-apps: VM unreachable
    ⚠️  Container hedgedoc/hedgedoc_postgres: hedgedoc_postgres on oci-apps: VM unreachable
    ⚠️  Container hickory-dns/hickory-dns: hickory-dns on gcp-proxy: VM unreachable
    ⚠️  Container introspect-proxy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container lgtm/lgtm_grafana: lgtm_grafana on oci-apps: VM unreachable
    ⚠️  Container lgtm/lgtm_loki: lgtm_loki on oci-apps: VM unreachable
    ⚠️  Container lgtm/lgtm_mimir: lgtm_mimir on oci-apps: VM unreachable
    ⚠️  Container lgtm/lgtm_tempo: lgtm_tempo on oci-apps: VM unreachable
    ⚠️  Container maddy/maddy: maddy on oci-mail: VM unreachable
    ⚠️  Container mail-mcp/mail-mcp: mail-mcp on oci-apps: VM unreachable
    ⚠️  Container matomo/matomo-hybrid: matomo-hybrid on oci-analytics: VM unreachable
    ⚠️  Container mattermost-bots/mattermost: mattermost on oci-apps: VM unreachable
    ⚠️  Container mattermost-bots/mattermost-bots: mattermost-bots on oci-apps: VM unreachable
    ⚠️  Container mattermost-bots/mattermost-postgres: mattermost-postgres on oci-apps: VM unreachable
    ⚠️  Container mattermost-mcp/mattermost-mcp: mattermost-mcp on oci-apps: VM unreachable
    ⚠️  Container ntfy/ntfy: ntfy on oci-apps: VM unreachable
    ⚠️  Container ntfy/github-rss: github-rss on oci-apps: VM unreachable
    ⚠️  Container ntfy/syslog-bridge: syslog-bridge on oci-apps: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container ollama-hai/ollama-hai: ollama-hai on oci-apps: VM unreachable
    ⚠️  Container photoprism/photoprism_app: photoprism_app on oci-apps: VM unreachable
    ⚠️  Container photoprism/photoprism_mariadb: photoprism_mariadb on oci-apps: VM unreachable
    ⚠️  Container photoprism/photoprism_rclone: photoprism_rclone on oci-apps: VM unreachable
    ⚠️  Container quant-lab-light/quant_light_db: quant_light_db on oci-apps: VM unreachable
    ⚠️  Container quant-lab-light/quant_light_engine: quant_light_engine on oci-apps: VM unreachable
    ⚠️  Container quant-lab-light/quant_light_research: quant_light_research on oci-apps: VM unreachable
    ⚠️  Container radicale/radicale: radicale on oci-apps: VM unreachable
    ⚠️  Container redis/redis: redis on gcp-proxy: VM unreachable
    ⚠️  Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: VM unreachable
    ⚠️  Container sauron-forwarder/sauron-forwarder: sauron-forwarder on oci-analytics: VM unreachable
    ⚠️  Container smtp-proxy/smtp-proxy: smtp-proxy on oci-mail: VM unreachable
    ⚠️  Container snappymail/snappymail: snappymail on oci-mail: VM unreachable
    ⚠️  Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: VM unreachable
    ⚠️  Container umami/umami: umami on oci-analytics: VM unreachable
    ⚠️  Container umami/umami-db: umami-db on oci-analytics: VM unreachable
    ⚠️  Container umami/umami-setup: umami-setup on oci-analytics: VM unreachable
    ⚠️  Container vaultwarden/vaultwarden: vaultwarden on oci-apps: VM unreachable
    ⚠️  Container windmill/windmill-server: windmill-server on oci-apps: VM unreachable
    ⚠️  Container windmill/windmill-db: windmill-db on oci-apps: VM unreachable
    ⚠️  Container windmill/windmill-worker: windmill-worker on oci-apps: VM unreachable
    ⚠️  backup-gitea.app:3002: backup-gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-api.app:8081: c3-infra-api.app:8081 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-mcp.app:3100: c3-infra-mcp.app:3100 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-services-mcp.app:3101: c3-services-mcp.app:3101 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  caddy.app:443: caddy.app:443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  cloud-cgc-mcp.app:3105: cloud-cgc-mcp.app:3105 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  cloud-spec.app:3080: c3-spec.app:3080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  code-server.app:8443: code-server.app:8443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  crawlee-cloud.app:3000: crawlee.app:3000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  dagu.app:8070: dagu.app:8070 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  dbgate.app:8086: dbgate.app:8086 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  dozzle.app:9999: dozzle.app:9999 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  etherpad.app:3012: etherpad.app:3012 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  filebrowser.app:3015: filebrowser.app:3015 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  fluent-bit.app:2020: fluent-bit.app:2020 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  gitea.app:3002: gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  google-workspace-mcp.app:3104: g-workspace-mcp.app:3104 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  grist.app:3011: grist.app:3011 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  hedgedoc.app:3018: hedgedoc.app:3018 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  introspect-proxy.app:4182: introspect-proxy.app:4182 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  lgtm.app:3200: grafana.app:3200 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  maddy.app:443: maddy.app:443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  mail-mcp.app:3103: mail-mcp.app:3103 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  matomo.app:8084: matomo.app:8084 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  mattermost-bots.app:8065: mattermost.app:8065 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  mattermost-mcp.app:3102: mattermost-mcp.app:3102 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  ollama.app:11434: ollama.app:11434 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  ollama-hai.app:11435: ollama-hai.app:11435 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  photoprism.app:3013: photoprism.app:3013 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  photos-webhook.app:5002: photos-webhook.app:5002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  radicale.app:5232: radicale.app:5232 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  rig-agentic-sonn-14bq8.app:8091: rig-agentic-sonn-14bq8.app:8091 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  smtp-proxy.app:8080: smtp-proxy.app:8080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  snappymail.app:8888: snappymail.app:8888 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  umami.app:3006: umami.app:3006 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  windmill.app:8000: windmill-app.app:8000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  Public auth.diegonmarcos.com: auth.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://auth.diegonmarcos.com/), auth=0)
    ⚠️  Public git.diegonmarcos.com: git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0)
    ⚠️  Public api.diegonmarcos.com/c3-api: api.diegonmarcos.com/c3-api: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/c3-api), auth=0)
    ⚠️  Public mcp.diegonmarcos.com/c3-infra-mcp: mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=0)
    ⚠️  Public proxy.diegonmarcos.com: proxy.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=0)
    ⚠️  Public ide.diegonmarcos.com: ide.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://ide.diegonmarcos.com/), auth=0)
    ⚠️  Public api.diegonmarcos.com: api.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=0)
    ⚠️  Public workflows.diegonmarcos.com: workflows.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=0)
    ⚠️  Public db.diegonmarcos.com: db.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=0)
    ⚠️  Public logs.diegonmarcos.com: logs.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=0)
    ⚠️  Public pad.diegonmarcos.com: pad.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=0)
    ⚠️  Public files.diegonmarcos.com: files.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=0)
    ⚠️  Public git.diegonmarcos.com: git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0)
    ⚠️  Public sheets.diegonmarcos.com: sheets.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=0)
    ⚠️  Public doc.diegonmarcos.com: doc.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=0)
    ⚠️  Public grafana.diegonmarcos.com: grafana.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=0)
    ⚠️  Public mail.diegonmarcos.com: mail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=0)
    ⚠️  Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0)
    ⚠️  Public chat.diegonmarcos.com: chat.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=0)
    ⚠️  Public rss.diegonmarcos.com: rss.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=0)
    ⚠️  Public photos.diegonmarcos.com: photos.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=0)
    ⚠️  Public cal.diegonmarcos.com: cal.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=0)
    ⚠️  Public smtp.diegonmarcos.com: smtp.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=0)
    ⚠️  Public webmail.diegonmarcos.com: webmail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=0)
    ⚠️  Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0)
    ⚠️  Public vault.diegonmarcos.com: vault.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=0)
    ⚠️  Public windmill.diegonmarcos.com: windmill.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=0)
    ⚠️  Cross c3-services-api: container up, public down: c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare
    ⚠️  GHA workflows: gh CLI failed
    ⚠️  DKIM dkim._domainkey: DKIM: NOT FOUND
    ⚠️  Caddy TLS: proxy.diegonmarcos.com -> err: error sending request for url (https://proxy.diegonmarcos.com/)
  INFO:
    ℹ️  Local docker: error: No such file or directory (os error 2)
    ℹ️  Drift no-containers: kg-graph: kg-graph has no containers declared in topology
    ℹ️  Drift no-containers: photos-webhook: photos-webhook has no containers declared in topology
    ℹ️  Drift no-domain: c3-diego-personal-data-mcp: c3-diego-personal-data-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: c3-services-mcp: c3-services-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-cgc-mcp: cloud-cgc-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-spec: cloud-spec has containers but no domain assigned
    ℹ️  Drift no-domain: cloudflare-worker: cloudflare-worker has containers but no domain assigned
    ℹ️  Drift no-domain: db-agent: db-agent has containers but no domain assigned
    ℹ️  Drift no-domain: gcloud: gcloud has containers but no domain assigned
    ℹ️  Drift no-domain: google-workspace-mcp: google-workspace-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: mail-mcp: mail-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: mattermost-mcp: mattermost-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: ollama: ollama has containers but no domain assigned
    ℹ️  Drift no-domain: ollama-arm: ollama-arm has containers but no domain assigned
    ℹ️  Drift no-domain: ollama-hai: ollama-hai has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-full: quant-lab-full has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-light: quant-lab-light has containers but no domain assigned
    ℹ️  Drift no-domain: redis: redis has containers but no domain assigned
    ℹ️  Drift no-domain: revealmd: revealmd has containers but no domain assigned
    ℹ️  Drift no-domain: rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 has containers but no domain assigned
    ℹ️  Drift no-domain: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has containers but no domain assigned
    ℹ️  Drift no-domain: sauron-lite: sauron-lite has containers but no domain assigned
    ℹ️  Drift no-port-in-build: authelia: authelia has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: backup-gitea: backup-gitea has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: c3-infra-api: c3-infra-api has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: c3-infra-mcp: c3-infra-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: c3-services-api: c3-services-api has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: c3-services-mcp: c3-services-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: caddy: caddy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: cloud-cgc-mcp: cloud-cgc-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: cloud-spec: cloud-spec has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: code-server: code-server has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: crawlee-cloud: crawlee-cloud has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: dagu: dagu has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: dbgate: dbgate has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: dozzle: dozzle has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: etherpad: etherpad has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: filebrowser: filebrowser has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: fluent-bit: fluent-bit has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: gitea: gitea has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: google-workspace-mcp: google-workspace-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: grist: grist has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: hedgedoc: hedgedoc has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: hickory-dns: hickory-dns has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: introspect-proxy: introspect-proxy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: lgtm: lgtm has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: maddy: maddy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mail-mcp: mail-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: matomo: matomo has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mattermost-bots: mattermost-bots has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mattermost-mcp: mattermost-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ntfy: ntfy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ollama: ollama has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ollama-hai: ollama-hai has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: photoprism: photoprism has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: photos-webhook: photos-webhook has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: radicale: radicale has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: redis: redis has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: revealmd: revealmd has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: smtp-proxy: smtp-proxy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: snappymail: snappymail has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: umami: umami has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: vaultwarden: vaultwarden has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: windmill: windmill has port in topology but missing ports.app in build.json


0. TIER DASHBOARD
──────────────────────────────────────────────────────────────
    Layer                gcp-proxy        oci-mail         oci-apps         oci-analytics   
                         (front door)     (mail)           (apps)           (analytics)     
    ────────────────────────────────────────────────────────────────────────────────────
    1. Self-check        ⚠️ 3/7                                          
    2. WG Mesh           ❌ 0/1            ❌ 0/1            ❌ 0/1            ❌ 0/1           
    3. Platform          ❌ 0/1            ❌ 0/1            ❌ 0/1            ❌ 0/1           
    4. Containers        ❌ 0/7            ❌ 0/4            ❌ 0/48           ❌ 0/8           
    5. Private URLs      ⚠️ 5/41          —                —                —               
    6. Public URLs       ❌ 0/27                                          
    7. Cross-checks      ❌ 0/1                                           
    8. External          ⚠️ 8/10                                         
    9. Drift             ❌ 0/65                                          
    10. Security         ⚠️ 12/19                                        
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ❌ C3 API (mesh)                  http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health) (0.2s) [CRITICAL]
  ⚠️  C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> err: error sending request for url (https://api.diegonmarcos.com/c3-api/health) (0.4s) [WARNING]
  ✅ WireGuard interface            TCP 10.0.0.1:22 -> open (0.2s)
  ℹ️  Local docker                   error: No such file or directory (os error 2) [INFO]
  ⚠️  SSH agent                      no SSH agent or no keys (0.0s) [WARNING]
  ✅ cloud-data freshness           generated 2026-04-15T17:36:15.684Z (17h ago)
  ✅ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> 10.0.0.1 (0.1s)

  Summary: 3/7 passed, 4 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ❌ WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=? Dropbear=fail WG:TCP=ok SSH=fail (3.3s) [CRITICAL]
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=? Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (6.0s)
  ❌ WG oci-apps                    oci-apps (10.0.0.6): VPS=? Dropbear=fail WG:TCP=ok SSH=fail (3.5s) [CRITICAL]
  ❌ WG oci-mail                    oci-mail (10.0.0.3): VPS=? Dropbear=fail WG:TCP=ok SSH=fail (3.5s) [CRITICAL]
  ❌ WG oci-analytics               oci-analytics (10.0.0.4): VPS=? Dropbear=fail WG:TCP=ok SSH=fail (3.5s) [CRITICAL]

  Summary: 1/5 passed, 4 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ❌ Platform gcp-proxy             gcp-proxy: unreachable (WG down) [CRITICAL]
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]
  ❌ Platform oci-apps              oci-apps: unreachable (WG down) [CRITICAL]
  ❌ Platform oci-mail              oci-mail: unreachable (WG down) [CRITICAL]
  ❌ Platform oci-analytics         oci-analytics: unreachable (WG down) [CRITICAL]

  Summary: 1/5 passed, 4 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ⚠️  Container authelia/authelia    authelia on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container authelia/authelia-redis authelia-redis on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container backup-gitea/gitea   gitea on oci-apps: VM unreachable [WARNING]
  ⚠️  Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: VM unreachable [WARNING]
  ⚠️  Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container caddy/caddy          caddy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container caddy/introspect-proxy introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container cloud-spec/cloud-spec cloud-spec on oci-apps: VM unreachable [WARNING]
  ⚠️  Container code-server/code-server code-server on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: VM unreachable [WARNING]
  ⚠️  Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: VM unreachable [WARNING]
  ⚠️  Container dagu/dagu            dagu on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container dbgate/dbgate        dbgate on oci-apps: VM unreachable [WARNING]
  ⚠️  Container dozzle/dozzle        dozzle on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container etherpad/etherpad_app etherpad_app on oci-apps: VM unreachable [WARNING]
  ⚠️  Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: VM unreachable [WARNING]
  ⚠️  Container filebrowser/filebrowser_app filebrowser_app on oci-apps: VM unreachable [WARNING]
  ⚠️  Container fluent-bit/fluent-bit fluent-bit on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container gitea/gitea          gitea on oci-apps: VM unreachable [WARNING]
  ⚠️  Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container grist/grist_app      grist_app on oci-apps: VM unreachable [WARNING]
  ⚠️  Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: VM unreachable [WARNING]
  ⚠️  Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: VM unreachable [WARNING]
  ⚠️  Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: VM unreachable [WARNING]
  ⚠️  Container lgtm/lgtm_loki       lgtm_loki on oci-apps: VM unreachable [WARNING]
  ⚠️  Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: VM unreachable [WARNING]
  ⚠️  Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: VM unreachable [WARNING]
  ⚠️  Container maddy/maddy          maddy on oci-mail: VM unreachable [WARNING]
  ⚠️  Container mail-mcp/mail-mcp    mail-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container matomo/matomo-hybrid matomo-hybrid on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container mattermost-bots/mattermost mattermost on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ntfy/ntfy            ntfy on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ntfy/github-rss      github-rss on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ntfy/syslog-bridge   syslog-bridge on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ⚠️  Container ollama-hai/ollama-hai ollama-hai on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photoprism/photoprism_app photoprism_app on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: VM unreachable [WARNING]
  ⚠️  Container quant-lab-light/quant_light_db quant_light_db on oci-apps: VM unreachable [WARNING]
  ⚠️  Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: VM unreachable [WARNING]
  ⚠️  Container quant-lab-light/quant_light_research quant_light_research on oci-apps: VM unreachable [WARNING]
  ⚠️  Container radicale/radicale    radicale on oci-apps: VM unreachable [WARNING]
  ⚠️  Container redis/redis          redis on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: VM unreachable [WARNING]
  ⚠️  Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: VM unreachable [WARNING]
  ⚠️  Container snappymail/snappymail snappymail on oci-mail: VM unreachable [WARNING]
  ⚠️  Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: VM unreachable [WARNING]
  ⚠️  Container umami/umami          umami on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container umami/umami-db       umami-db on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container umami/umami-setup    umami-setup on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container vaultwarden/vaultwarden vaultwarden on oci-apps: VM unreachable [WARNING]
  ⚠️  Container windmill/windmill-server windmill-server on oci-apps: VM unreachable [WARNING]
  ⚠️  Container windmill/windmill-db windmill-db on oci-apps: VM unreachable [WARNING]
  ⚠️  Container windmill/windmill-worker windmill-worker on oci-apps: VM unreachable [WARNING]

  Summary: 0/68 passed, 68 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ✅ authelia.app:9091              authelia.app:9091 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (0.5s)
  ⚠️  backup-gitea.app:3002          backup-gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  caddy.app:443                  caddy.app:443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  cloud-spec.app:3080            c3-spec.app:3080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  code-server.app:8443           code-server.app:8443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  crawlee-cloud.app:3000         crawlee.app:3000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  dagu.app:8070                  dagu.app:8070 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  dbgate.app:8086                dbgate.app:8086 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  dozzle.app:9999                dozzle.app:9999 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  etherpad.app:3012              etherpad.app:3012 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  filebrowser.app:3015           filebrowser.app:3015 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  gitea.app:3002                 gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  grist.app:3011                 grist.app:3011 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ✅ hickory-dns.app:53             hickory-dns.app:53 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (0.3s)
  ⚠️  introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  lgtm.app:3200                  grafana.app:3200 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  maddy.app:443                  maddy.app:443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  mail-mcp.app:3103              mail-mcp.app:3103 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  mattermost-bots.app:8065       mattermost.app:8065 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ✅ ntfy.app:8090                  ntfy.app:8090 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (0.5s)
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  ollama-hai.app:11435           ollama-hai.app:11435 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  photos-webhook.app:5002        photos-webhook.app:5002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  radicale.app:5232              radicale.app:5232 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ✅ redis.app:6379                 redis.app:6379 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (0.3s)
  ⚠️  rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  snappymail.app:8888            snappymail.app:8888 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]
  ✅ vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (0.5s)
  ⚠️  windmill.app:8000              windmill-app.app:8000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.3s) [WARNING]

  Summary: 5/41 passed, 36 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ⚠️  Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://auth.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/c3-api), auth=0) (0.5s) [WARNING]
  ⚠️  Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=0) (0.5s) [WARNING]
  ⚠️  Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://ide.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=0) (0.6s) [WARNING]
  ⚠️  Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=0) (0.6s) [WARNING]
  ⚠️  Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=0) (0.6s) [WARNING]
  ⚠️  Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=0) (0.5s) [WARNING]
  ⚠️  Public windmill.diegonmarcos.com windmill.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=0) (0.5s) [WARNING]

  Summary: 0/27 passed, 27 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ⚠️  Cross c3-services-api: container up, public down c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare [WARNING]

  Summary: 0/1 passed, 1 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.0s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (0.5s)
  ⚠️  GHA workflows                  gh CLI failed [WARNING]
  ✅ GitHub API                     api.github.com/zen -> 403 (0.5s)
  ✅ MX record                      MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.0s)
  ⚠️  DKIM dkim._domainkey           DKIM: NOT FOUND (0.0s) [WARNING]
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (0.3s)

  Summary: 8/10 passed, 2 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ℹ️  Drift no-containers: kg-graph  kg-graph has no containers declared in topology [INFO]
  ℹ️  Drift no-containers: photos-webhook photos-webhook has no containers declared in topology [INFO]
  ℹ️  Drift no-domain: c3-diego-personal-data-mcp c3-diego-personal-data-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: c3-services-mcp c3-services-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-cgc-mcp cloud-cgc-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-spec    cloud-spec has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloudflare-worker cloudflare-worker has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: db-agent      db-agent has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: gcloud        gcloud has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: google-workspace-mcp google-workspace-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: mail-mcp      mail-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: mattermost-mcp mattermost-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama        ollama has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama-arm    ollama-arm has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama-hai    ollama-hai has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-full quant-lab-full has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-light quant-lab-light has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: redis         redis has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: revealmd      revealmd has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: rig-agentic-hai-1.5bq4 rig-agentic-hai-1.5bq4 has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: sauron-lite   sauron-lite has containers but no domain assigned [INFO]
  ℹ️  Drift no-port-in-build: authelia authelia has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: backup-gitea backup-gitea has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: c3-infra-api c3-infra-api has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: c3-infra-mcp c3-infra-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: c3-services-api c3-services-api has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: c3-services-mcp c3-services-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: caddy  caddy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: cloud-cgc-mcp cloud-cgc-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: cloud-spec cloud-spec has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: code-server code-server has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: crawlee-cloud crawlee-cloud has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: dagu   dagu has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: dbgate dbgate has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: dozzle dozzle has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: etherpad etherpad has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: filebrowser filebrowser has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: fluent-bit fluent-bit has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: gitea  gitea has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: google-workspace-mcp google-workspace-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: grist  grist has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: hedgedoc hedgedoc has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: hickory-dns hickory-dns has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: introspect-proxy introspect-proxy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: lgtm   lgtm has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: maddy  maddy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mail-mcp mail-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: matomo matomo has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mattermost-bots mattermost-bots has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mattermost-mcp mattermost-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ntfy   ntfy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ollama ollama has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ollama-hai ollama-hai has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: photoprism photoprism has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: photos-webhook photos-webhook has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: radicale radicale has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: redis  redis has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: revealmd revealmd has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: smtp-proxy smtp-proxy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: snappymail snappymail has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: umami  umami has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: vaultwarden vaultwarden has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: windmill windmill has port in topology but missing ports.app in build.json [INFO]

  Summary: 0/65 passed, 65 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ❌ TLS cert diegonmarcos.com      TLS connection failed (0.3s) [CRITICAL]
  ❌ TLS cert api.diegonmarcos.com  TLS connection failed (0.3s) [CRITICAL]
  ❌ TLS cert auth.diegonmarcos.com TLS connection failed (0.3s) [CRITICAL]
  ❌ TLS cert mail.diegonmarcos.com TLS connection failed (0.3s) [CRITICAL]
  ❌ TLS cert vault.diegonmarcos.com TLS connection failed (0.3s) [CRITICAL]
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.0s)
  ❌ Authelia health                auth.diegonmarcos.com/api/health -> err: error sending request for url (https://auth.diegonmarcos.com/api/health) (0.3s) [CRITICAL]
  ✅ Firewall gcp-proxy             gcp-proxy: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall gcp-t4                gcp-t4: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-apps              oci-apps: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-mail              oci-mail: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-analytics         oci-analytics: no unexpected dangerous ports exposed (3.0s)
  ✅ SSH ports gcp-proxy            gcp-proxy: SSH:22=open Dropbear:2200=closed (3.1s)
  ✅ SSH ports gcp-t4               gcp-t4: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-apps             oci-apps: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-mail             oci-mail: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-analytics        oci-analytics: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ⚠️  Caddy TLS                      proxy.diegonmarcos.com -> err: error sending request for url (https://proxy.diegonmarcos.com/) (0.3s) [WARNING]

  Summary: 12/19 passed, 7 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    38.0s
  L5_private_urls          31.1s
  L10_security             31.1s
  L8_external              31.1s
  L6_public_urls           31.1s
  L4-L11_parallel          31.1s
  L11_email_e2e            31.1s
  L2_wg_mesh               6.0s
  L1_self_check            0.9s
  L9_drift                 0.0s
  L4_containers            0.0s
  L3_platform              0.0s
  L7_cross_checks          0.0s

  Total: 38.0s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 31/249 passed, 15 critical, 137 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
