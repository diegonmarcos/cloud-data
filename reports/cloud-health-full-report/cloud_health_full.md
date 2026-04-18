```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-04-18T18:56:17.094945746+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  220 issues: 11 critical, 138 warnings, 71 info

  CRITICAL:
    ❌ C3 API (mesh): http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health)
    ❌ WireGuard interface: TCP 10.0.0.1:22 -> closed
    ❌ Hickory DNS resolver: dig caddy.app @10.0.0.1 -> NXDOMAIN
    ❌ WG gcp-proxy: gcp-proxy (10.0.0.1): VPS=? Dropbear=fail WG:TCP=fail SSH=fail
    ❌ WG oci-apps: oci-apps (10.0.0.6): VPS=? Dropbear=fail WG:TCP=fail SSH=fail
    ❌ WG oci-mail: oci-mail (10.0.0.3): VPS=? Dropbear=fail WG:TCP=fail SSH=fail
    ❌ WG oci-analytics: oci-analytics (10.0.0.4): VPS=? Dropbear=fail WG:TCP=fail SSH=fail
    ❌ Platform gcp-proxy: gcp-proxy: unreachable (WG down)
    ❌ Platform oci-apps: oci-apps: unreachable (WG down)
    ❌ Platform oci-mail: oci-mail: unreachable (WG down)
    ❌ Platform oci-analytics: oci-analytics: unreachable (WG down)
  WARNINGS:
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  Container authelia/authelia: authelia on gcp-proxy: VM unreachable
    ⚠️  Container authelia/authelia-redis: authelia-redis on gcp-proxy: VM unreachable
    ⚠️  Container c3-infra-api/c3-infra-api: c3-infra-api on oci-apps: VM unreachable
    ⚠️  Container c3-infra-mcp/c3-infra-mcp: c3-infra-mcp on oci-apps: VM unreachable
    ⚠️  Container c3-services-api/c3-services-api: c3-services-api on oci-apps: VM unreachable
    ⚠️  Container c3-services-mcp/c3-services-mcp: c3-services-mcp on oci-apps: VM unreachable
    ⚠️  Container caddy/caddy: caddy on gcp-proxy: VM unreachable
    ⚠️  Container cloud-builder-x/cloud-builder-x: cloud-builder-x on oci-apps: VM unreachable
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
    ⚠️  Container matomo/matomo-hybrid: matomo-hybrid on oci-apps: VM unreachable
    ⚠️  Container mattermost-bots/mattermost: mattermost on oci-apps: VM unreachable
    ⚠️  Container mattermost-bots/mattermost-bots: mattermost-bots on oci-apps: VM unreachable
    ⚠️  Container mattermost-bots/mattermost-postgres: mattermost-postgres on oci-apps: VM unreachable
    ⚠️  Container mattermost-mcp/mattermost-mcp: mattermost-mcp on oci-apps: VM unreachable
    ⚠️  Container news-gdelt/news-gdelt: news-gdelt on oci-apps: VM unreachable
    ⚠️  Container ntfy/ntfy: ntfy on oci-apps: VM unreachable
    ⚠️  Container ntfy/github-rss: github-rss on oci-apps: VM unreachable
    ⚠️  Container ntfy/syslog-bridge: syslog-bridge on oci-apps: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container ollama-hai/ollama-hai: ollama-hai on oci-apps: VM unreachable
    ⚠️  Container photoprism/photoprism_app: photoprism_app on oci-apps: VM unreachable
    ⚠️  Container photoprism/photoprism_mariadb: photoprism_mariadb on oci-apps: VM unreachable
    ⚠️  Container photoprism/photoprism_rclone: photoprism_rclone on oci-apps: VM unreachable
    ⚠️  Container photos-webhook/photos-webhook: photos-webhook on oci-apps: VM unreachable
    ⚠️  Container quant-lab-light/quant_light_db: quant_light_db on oci-apps: VM unreachable
    ⚠️  Container quant-lab-light/quant_light_engine: quant_light_engine on oci-apps: VM unreachable
    ⚠️  Container quant-lab-light/quant_light_research: quant_light_research on oci-apps: VM unreachable
    ⚠️  Container radicale/radicale: radicale on oci-apps: VM unreachable
    ⚠️  Container redis/redis: redis on gcp-proxy: VM unreachable
    ⚠️  Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: VM unreachable
    ⚠️  Container sauron-forwarder/sauron-forwarder: sauron-forwarder on oci-analytics: VM unreachable
    ⚠️  Container smtp-proxy/smtp-proxy: smtp-proxy on oci-mail: VM unreachable
    ⚠️  Container snappymail/snappymail: snappymail on oci-mail: VM unreachable
    ⚠️  Container stalwart/stalwart: stalwart on oci-mail: VM unreachable
    ⚠️  Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: VM unreachable
    ⚠️  Container umami/umami: umami on oci-apps: VM unreachable
    ⚠️  Container umami/umami-db: umami-db on oci-apps: VM unreachable
    ⚠️  Container umami/umami-setup: umami-setup on oci-apps: VM unreachable
    ⚠️  Container vaultwarden/vaultwarden: vaultwarden on oci-apps: VM unreachable
    ⚠️  Private URLs (Hickory): Hickory DNS at 10.0.0.1 is down — falling back to WG IPs
    ⚠️  authelia.app:9091: authelia.app:9091 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-api.app:8081: c3-infra-api.app:8081 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-mcp.app:3100: c3-infra-mcp.app:3100 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  c3-services-api.app:8082: c3-services-api.app:8082 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  c3-services-mcp.app:3101: c3-services-mcp.app:3101 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  cloud-cgc-mcp.app:3105: cloud-cgc-mcp.app:3105 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  cloud-spec.app:3080: c3-spec.app:3080 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  code-server.app:8443: code-server.app:8443 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  crawlee-cloud.app:3000: crawlee.app:3000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  dagu.app:8070: dagu.app:8070 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  dbgate.app:8086: dbgate.app:8086 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  dozzle.app:9999: dozzle.app:9999 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  etherpad.app:3012: etherpad.app:3012 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  filebrowser.app:3015: filebrowser.app:3015 DNS=ok(15.197.225.128) TCP=FAIL HTTP=skip
    ⚠️  fluent-bit.app:2020: fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  gitea.app:3002: gitea.app:3002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  google-workspace-mcp.app:3104: g-workspace-mcp.app:3104 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  grist.app:3011: grist.app:3011 DNS=ok(35.188.208.237) TCP=FAIL HTTP=skip
    ⚠️  hedgedoc.app:3018: hedgedoc.app:3018 DNS=ok(192.145.46.12) TCP=FAIL HTTP=skip
    ⚠️  hickory-dns.app:53: hickory-dns.app:53 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  introspect-proxy.app:4182: introspect-proxy.app:4182 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  lgtm.app:3200: grafana.app:3200 DNS=ok(216.40.34.41) TCP=FAIL HTTP=skip
    ⚠️  mail-mcp.app:3103: mail-mcp.app:3103 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  matomo.app:8084: matomo.app:8084 DNS=ok(213.186.33.5) TCP=ok HTTP=err: error sending request for url (http://213.186.33.5:8084/)
    ⚠️  mattermost-bots.app:8065: mattermost.app:8065 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  mattermost-mcp.app:3102: mattermost-mcp.app:3102 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  ntfy.app:8090: ntfy.app:8090 DNS=ok(35.176.213.46) TCP=FAIL HTTP=skip
    ⚠️  ollama.app:11434: ollama.app:11434 DNS=SYS-FAIL→hickory(10.0.0.8) TCP=FAIL HTTP=skip
    ⚠️  ollama-hai.app:11435: ollama-hai.app:11435 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  photoprism.app:3013: photoprism.app:3013 DNS=ok(176.9.111.126) TCP=FAIL HTTP=skip
    ⚠️  photos-webhook.app:5002: photos-webhook.app:5002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  radicale.app:5232: radicale.app:5232 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  redis.app:6379: redis.app:6379 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  rig-agentic-sonn-14bq8.app:8091: rig-agentic-sonn-14bq8.app:8091 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  smtp-proxy.app:8080: smtp-proxy.app:8080 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  snappymail.app:8888: snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  stalwart.app:2443: stalwart.app:2443 DNS=ok(151.101.1.195) TCP=FAIL HTTP=skip
    ⚠️  umami.app:3006: umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip
    ⚠️  vaultwarden.app:8880: vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip
    ⚠️  Public smtp.diegonmarcos.com: smtp.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502)
    ⚠️  Public mail-stalwart.diegonmarcos.com: mail-stalwart.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502)
    ⚠️  Cross authelia: public up, container down: authelia: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross c3-infra-api: public up, container down: c3-infra-api: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross c3-infra-mcp: public up, container down: c3-infra-mcp: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross c3-services-api: public up, container down: c3-services-api: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross caddy: public up, container down: caddy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross code-server: public up, container down: code-server: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross crawlee-cloud: public up, container down: crawlee-cloud: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross dagu: public up, container down: dagu: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross dbgate: public up, container down: dbgate: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross dozzle: public up, container down: dozzle: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross etherpad: public up, container down: etherpad: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross filebrowser: public up, container down: filebrowser: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross gitea: public up, container down: gitea: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross grist: public up, container down: grist: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross hedgedoc: public up, container down: hedgedoc: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross lgtm: public up, container down: lgtm: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross maddy: public up, container down: maddy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross matomo: public up, container down: matomo: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross mattermost-bots: public up, container down: mattermost-bots: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross ntfy: public up, container down: ntfy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross photoprism: public up, container down: photoprism: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross radicale: public up, container down: radicale: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross snappymail: public up, container down: snappymail: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross umami: public up, container down: umami: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross vaultwarden: public up, container down: vaultwarden: public URL responds but container is down — stale cache or wrong routing
    ⚠️  GHA workflows: gh CLI failed
    ⚠️  DKIM dkim._domainkey: DKIM: NOT FOUND
  INFO:
    ℹ️  Local docker: error: No such file or directory (os error 2)
    ℹ️  Drift no-domain: c3-diego-personal-data-mcp: c3-diego-personal-data-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: c3-services-mcp: c3-services-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-builder-x: cloud-builder-x has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-cgc-mcp: cloud-cgc-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: cloud-spec: cloud-spec has containers but no domain assigned
    ℹ️  Drift no-domain: cloudflare-worker: cloudflare-worker has containers but no domain assigned
    ℹ️  Drift no-domain: db-agent: db-agent has containers but no domain assigned
    ℹ️  Drift no-domain: gcloud: gcloud has containers but no domain assigned
    ℹ️  Drift no-domain: google-workspace-mcp: google-workspace-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: kg-graph: kg-graph has containers but no domain assigned
    ℹ️  Drift no-domain: mail-mcp: mail-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: mattermost-mcp: mattermost-mcp has containers but no domain assigned
    ℹ️  Drift no-domain: news-gdelt: news-gdelt has containers but no domain assigned
    ℹ️  Drift no-domain: ollama: ollama has containers but no domain assigned
    ℹ️  Drift no-domain: ollama-arm: ollama-arm has containers but no domain assigned
    ℹ️  Drift no-domain: ollama-hai: ollama-hai has containers but no domain assigned
    ℹ️  Drift no-domain: photos-webhook: photos-webhook has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-full: quant-lab-full has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-light: quant-lab-light has containers but no domain assigned
    ℹ️  Drift no-domain: redis: redis has containers but no domain assigned
    ℹ️  Drift no-domain: revealmd: revealmd has containers but no domain assigned
    ℹ️  Drift no-domain: rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 has containers but no domain assigned
    ℹ️  Drift no-domain: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has containers but no domain assigned
    ℹ️  Drift no-domain: sauron-lite: sauron-lite has containers but no domain assigned
    ℹ️  Drift no-port-in-build: authelia: authelia has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: backup-borg: backup-borg has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: backup-bup: backup-bup has port in topology but missing ports.app in build.json
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
    ℹ️  Drift no-port-in-build: kg-graph: kg-graph has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: lgtm: lgtm has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mail-mcp: mail-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: matomo: matomo has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mattermost-bots: mattermost-bots has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mattermost-mcp: mattermost-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ntfy: ntfy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ollama: ollama has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ollama-arm: ollama-arm has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: ollama-hai: ollama-hai has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: photoprism: photoprism has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: photos-webhook: photos-webhook has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: radicale: radicale has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: redis: redis has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: revealmd: revealmd has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: rig-agentic-hai-1.5bq4: rig-agentic-hai-1.5bq4 has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: smtp-proxy: smtp-proxy has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: snappymail: snappymail has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: stalwart: stalwart has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: umami: umami has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: vaultwarden: vaultwarden has port in topology but missing ports.app in build.json


0. TIER DASHBOARD
──────────────────────────────────────────────────────────────
    Layer                gcp-proxy        oci-mail         oci-apps         oci-analytics   
                         (front door)     (mail)           (apps)           (analytics)     
    ────────────────────────────────────────────────────────────────────────────────────
    1. Self-check        ⚠️ 2/7                                          
    2. WG Mesh           ❌ 0/1            ❌ 0/1            ❌ 0/1            ❌ 0/1           
    3. Platform          ❌ 0/1            ❌ 0/1            ❌ 0/1            ❌ 0/1           
    4. Containers        ❌ 0/6            ❌ 0/5            ❌ 0/52           ❌ 0/4           
    5. Private URLs      ❌ 0/5            ❌ 0/2            ❌ 0/19           ❌ 0/3           
    6. Public URLs       ⚠️ 25/27                                        
    7. Cross-checks      ❌ 0/25                                          
    8. External          ⚠️ 8/10                                         
    9. Drift             ❌ 0/70                                          
    10. Security         ✅ 19/19                                         
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ❌ C3 API (mesh)                  http://10.0.0.6:8081/health -> err: error sending request for url (http://10.0.0.6:8081/health) (8.0s) [CRITICAL]
  ✅ C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> 302 (0.2s)
  ❌ WireGuard interface            TCP 10.0.0.1:22 -> closed (3.0s) [CRITICAL]
  ℹ️  Local docker                   error: No such file or directory (os error 2) [INFO]
  ⚠️  SSH agent                      no SSH agent or no keys (0.0s) [WARNING]
  ✅ cloud-data freshness           generated 2026-04-18T15:04:46.311Z (3h ago)
  ❌ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> NXDOMAIN (3.0s) [CRITICAL]

  Summary: 2/7 passed, 5 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ❌ WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=? Dropbear=fail WG:TCP=fail SSH=fail (3.1s) [CRITICAL]
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=? Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (6.0s)
  ❌ WG oci-apps                    oci-apps (10.0.0.6): VPS=? Dropbear=fail WG:TCP=fail SSH=fail (6.0s) [CRITICAL]
  ❌ WG oci-mail                    oci-mail (10.0.0.3): VPS=? Dropbear=fail WG:TCP=fail SSH=fail (6.0s) [CRITICAL]
  ❌ WG oci-analytics               oci-analytics (10.0.0.4): VPS=? Dropbear=fail WG:TCP=fail SSH=fail (6.0s) [CRITICAL]

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
  ⚠️  Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: VM unreachable [WARNING]
  ⚠️  Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container c3-services-api/c3-services-api c3-services-api on oci-apps: VM unreachable [WARNING]
  ⚠️  Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container caddy/caddy          caddy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container cloud-builder-x/cloud-builder-x cloud-builder-x on oci-apps: VM unreachable [WARNING]
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
  ⚠️  Container matomo/matomo-hybrid matomo-hybrid on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-bots/mattermost mattermost on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: VM unreachable [WARNING]
  ⚠️  Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: VM unreachable [WARNING]
  ⚠️  Container news-gdelt/news-gdelt news-gdelt on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ntfy/ntfy            ntfy on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ntfy/github-rss      github-rss on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ntfy/syslog-bridge   syslog-bridge on oci-apps: VM unreachable [WARNING]
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ⚠️  Container ollama-hai/ollama-hai ollama-hai on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photoprism/photoprism_app photoprism_app on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: VM unreachable [WARNING]
  ⚠️  Container photos-webhook/photos-webhook photos-webhook on oci-apps: VM unreachable [WARNING]
  ⚠️  Container quant-lab-light/quant_light_db quant_light_db on oci-apps: VM unreachable [WARNING]
  ⚠️  Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: VM unreachable [WARNING]
  ⚠️  Container quant-lab-light/quant_light_research quant_light_research on oci-apps: VM unreachable [WARNING]
  ⚠️  Container radicale/radicale    radicale on oci-apps: VM unreachable [WARNING]
  ⚠️  Container redis/redis          redis on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: VM unreachable [WARNING]
  ⚠️  Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: VM unreachable [WARNING]
  ⚠️  Container snappymail/snappymail snappymail on oci-mail: VM unreachable [WARNING]
  ⚠️  Container stalwart/stalwart    stalwart on oci-mail: VM unreachable [WARNING]
  ⚠️  Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: VM unreachable [WARNING]
  ⚠️  Container umami/umami          umami on oci-apps: VM unreachable [WARNING]
  ⚠️  Container umami/umami-db       umami-db on oci-apps: VM unreachable [WARNING]
  ⚠️  Container umami/umami-setup    umami-setup on oci-apps: VM unreachable [WARNING]
  ⚠️  Container vaultwarden/vaultwarden vaultwarden on oci-apps: VM unreachable [WARNING]

  Summary: 0/68 passed, 68 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ⚠️  Private URLs (Hickory)         Hickory DNS at 10.0.0.1 is down — falling back to WG IPs [WARNING]
  ⚠️  authelia.app:9091              authelia.app:9091 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  c3-services-api.app:8082       c3-services-api.app:8082 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ✅ caddy.app:443                  caddy.app:443 DNS=ok(204.69.207.1) TCP=ok HTTP=n/a (0.1s)
  ⚠️  cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  cloud-spec.app:3080            c3-spec.app:3080 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  code-server.app:8443           code-server.app:8443 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  crawlee-cloud.app:3000         crawlee.app:3000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.0s) [WARNING]
  ⚠️  dagu.app:8070                  dagu.app:8070 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  dbgate.app:8086                dbgate.app:8086 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  dozzle.app:9999                dozzle.app:9999 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  etherpad.app:3012              etherpad.app:3012 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  filebrowser.app:3015           filebrowser.app:3015 DNS=ok(15.197.225.128) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  gitea.app:3002                 gitea.app:3002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (16.0s) [WARNING]
  ⚠️  google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.0s) [WARNING]
  ⚠️  grist.app:3011                 grist.app:3011 DNS=ok(35.188.208.237) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(192.145.46.12) TCP=FAIL HTTP=skip (3.2s) [WARNING]
  ⚠️  hickory-dns.app:53             hickory-dns.app:53 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  lgtm.app:3200                  grafana.app:3200 DNS=ok(216.40.34.41) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  mail-mcp.app:3103              mail-mcp.app:3103 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(213.186.33.5) TCP=ok HTTP=err: error sending request for url (http://213.186.33.5:8084/) (8.3s) [WARNING]
  ⚠️  mattermost-bots.app:8065       mattermost.app:8065 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  ntfy.app:8090                  ntfy.app:8090 DNS=ok(35.176.213.46) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=SYS-FAIL→hickory(10.0.0.8) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  ollama-hai.app:11435           ollama-hai.app:11435 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(176.9.111.126) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  photos-webhook.app:5002        photos-webhook.app:5002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  radicale.app:5232              radicale.app:5232 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.0s) [WARNING]
  ⚠️  redis.app:6379                 redis.app:6379 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  snappymail.app:8888            snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  stalwart.app:2443              stalwart.app:2443 DNS=ok(151.101.1.195) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip (0.2s) [WARNING]

  Summary: 1/41 passed, 40 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ✅ Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.3s)
  ✅ Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (0.3s)
  ✅ Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.3s)
  ✅ Public api.diegonmarcos.com/services api.diegonmarcos.com/services: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (0.3s)
  ✅ Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.3s)
  ✅ Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.3s)
  ✅ Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.2s)
  ✅ Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.3s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.2s)
  ✅ Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=301 AUTH=301 (no-auth=301, auth=301) (0.3s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.4s)
  ✅ Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ⚠️  Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502) (0.5s) [WARNING]
  ✅ Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.3s)
  ⚠️  Public mail-stalwart.diegonmarcos.com mail-stalwart.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502) (0.5s) [WARNING]
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.3s)
  ✅ Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.7s)

  Summary: 25/27 passed, 2 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ⚠️  Cross authelia: public up, container down authelia: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross c3-infra-api: public up, container down c3-infra-api: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross c3-infra-mcp: public up, container down c3-infra-mcp: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross c3-services-api: public up, container down c3-services-api: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross caddy: public up, container down caddy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross code-server: public up, container down code-server: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross crawlee-cloud: public up, container down crawlee-cloud: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross dagu: public up, container down dagu: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross dbgate: public up, container down dbgate: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross dozzle: public up, container down dozzle: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross etherpad: public up, container down etherpad: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross filebrowser: public up, container down filebrowser: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross gitea: public up, container down gitea: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross grist: public up, container down grist: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross hedgedoc: public up, container down hedgedoc: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross lgtm: public up, container down lgtm: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross maddy: public up, container down maddy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross matomo: public up, container down matomo: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross mattermost-bots: public up, container down mattermost-bots: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross ntfy: public up, container down ntfy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross photoprism: public up, container down photoprism: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross radicale: public up, container down radicale: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross snappymail: public up, container down snappymail: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross umami: public up, container down umami: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross vaultwarden: public up, container down vaultwarden: public URL responds but container is down — stale cache or wrong routing [WARNING]

  Summary: 0/25 passed, 25 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.0s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (0.2s)
  ⚠️  GHA workflows                  gh CLI failed [WARNING]
  ✅ GitHub API                     api.github.com/zen -> 403 (0.2s)
  ✅ MX record                      MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.0s)
  ⚠️  DKIM dkim._domainkey           DKIM: NOT FOUND (0.0s) [WARNING]
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (0.2s)

  Summary: 8/10 passed, 2 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ℹ️  Drift no-domain: c3-diego-personal-data-mcp c3-diego-personal-data-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: c3-services-mcp c3-services-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-builder-x cloud-builder-x has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-cgc-mcp cloud-cgc-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-spec    cloud-spec has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloudflare-worker cloudflare-worker has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: db-agent      db-agent has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: gcloud        gcloud has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: google-workspace-mcp google-workspace-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: kg-graph      kg-graph has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: mail-mcp      mail-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: mattermost-mcp mattermost-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: news-gdelt    news-gdelt has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama        ollama has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama-arm    ollama-arm has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama-hai    ollama-hai has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: photos-webhook photos-webhook has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-full quant-lab-full has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-light quant-lab-light has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: redis         redis has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: revealmd      revealmd has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: rig-agentic-hai-1.5bq4 rig-agentic-hai-1.5bq4 has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: sauron-lite   sauron-lite has containers but no domain assigned [INFO]
  ℹ️  Drift no-port-in-build: authelia authelia has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: backup-borg backup-borg has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: backup-bup backup-bup has port in topology but missing ports.app in build.json [INFO]
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
  ℹ️  Drift no-port-in-build: kg-graph kg-graph has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: lgtm   lgtm has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mail-mcp mail-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: matomo matomo has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mattermost-bots mattermost-bots has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mattermost-mcp mattermost-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ntfy   ntfy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ollama ollama has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ollama-arm ollama-arm has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: ollama-hai ollama-hai has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: photoprism photoprism has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: photos-webhook photos-webhook has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: radicale radicale has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: redis  redis has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: revealmd revealmd has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: rig-agentic-hai-1.5bq4 rig-agentic-hai-1.5bq4 has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: smtp-proxy smtp-proxy has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: snappymail snappymail has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: stalwart stalwart has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: umami  umami has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: vaultwarden vaultwarden has port in topology but missing ports.app in build.json [INFO]

  Summary: 0/70 passed, 70 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✅ TLS cert diegonmarcos.com      expires Jun 29 22:11:40 2026 GMT (72d) (0.2s)
  ✅ TLS cert api.diegonmarcos.com  expires Jun 29 22:10:39 2026 GMT (72d) (0.3s)
  ✅ TLS cert auth.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (72d) (0.3s)
  ✅ TLS cert mail.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (72d) (0.3s)
  ✅ TLS cert vault.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (72d) (0.3s)
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.0s)
  ✅ Authelia health                auth.diegonmarcos.com/api/health -> 200 (0.2s)
  ✅ Firewall gcp-proxy             gcp-proxy: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall gcp-t4                gcp-t4: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-apps              oci-apps: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-mail              oci-mail: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-analytics         oci-analytics: no unexpected dangerous ports exposed (3.0s)
  ✅ SSH ports gcp-proxy            gcp-proxy: SSH:22=open Dropbear:2200=closed (0.1s)
  ✅ SSH ports gcp-t4               gcp-t4: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-apps             oci-apps: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-mail             oci-mail: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-analytics        oci-analytics: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ Caddy TLS                      proxy.diegonmarcos.com -> 302 (0.2s)

  Summary: 19/19 passed, 0 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    48.1s
  L6_public_urls           27.9s
  L5_private_urls          27.9s
  L10_security             27.9s
  L4-L11_parallel          27.9s
  L8_external              27.9s
  L11_email_e2e            27.9s
  L1_self_check            14.2s
  L2_wg_mesh               6.0s
  L7_cross_checks          0.0s
  L9_drift                 0.0s
  L3_platform              0.0s
  L4_containers            0.0s

  Total: 48.1s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 58/278 passed, 11 critical, 138 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
