```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-03-31T10:43:37.502707757+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  177 issues: 12 critical, 98 warnings, 67 info

  CRITICAL:
    ❌ WireGuard interface: TCP 10.0.0.1:22 -> closed
    ❌ Hickory DNS resolver: dig caddy.app @10.0.0.1 -> NXDOMAIN
    ❌ Platform gcp-proxy: gcp-proxy: unreachable (WG down)
    ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: Exited (101) 20 hours ago (exited)
    ❌ Container umami/umami-setup: umami-setup on oci-analytics: Exited (1) About an hour ago (exited)
    ❌ Container windmill/windmill-server: windmill-server on oci-apps: Exited (137) 13 hours ago (exited)
    ❌ TLS cert diegonmarcos.com: TLS connection failed
    ❌ TLS cert api.diegonmarcos.com: TLS connection failed
    ❌ TLS cert auth.diegonmarcos.com: TLS connection failed
    ❌ TLS cert mail.diegonmarcos.com: TLS connection failed
    ❌ TLS cert vault.diegonmarcos.com: TLS connection failed
    ❌ Authelia health: auth.diegonmarcos.com/api/health -> err: error sending request for url (https://auth.diegonmarcos.com/api/health)
  WARNINGS:
    ⚠️  C3 API (public): https://api.diegonmarcos.com/c3-api/health -> err: error sending request for url (https://api.diegonmarcos.com/c3-api/health)
    ⚠️  WG gcp-proxy: gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=fail SSH=fail
    ⚠️  Container authelia/authelia: authelia on gcp-proxy: VM unreachable
    ⚠️  Container authelia/authelia-redis: authelia-redis on gcp-proxy: VM unreachable
    ⚠️  Container caddy/caddy: caddy on gcp-proxy: VM unreachable
    ⚠️  Container caddy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container hickory-dns/hickory-dns: hickory-dns on gcp-proxy: VM unreachable
    ⚠️  Container introspect-proxy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/ntfy: ntfy on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/github-rss: github-rss on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/syslog-bridge: syslog-bridge on gcp-proxy: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container redis/redis: redis on gcp-proxy: VM unreachable
    ⚠️  Container vaultwarden/vaultwarden: vaultwarden on gcp-proxy: VM unreachable
    ⚠️  Private URLs (Hickory): Hickory DNS at 10.0.0.1 is down — falling back to WG IPs
    ⚠️  alerts-api.app:5050: alerts-api.app:5050 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  authelia.app:9091: authelia.app:9091 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  caddy.app:443: caddy.app:443 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  dagu.app:8070: dagu.app:8070 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  dozzle.app:9999: dozzle.app:9999 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  filebrowser.app:3015: filebrowser.app:3015 DNS=ok(15.197.225.128) TCP=FAIL HTTP=skip
    ⚠️  fluent-bit.app:2020: fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  grist.app:3011: grist.app:3011 DNS=ok(35.188.208.237) TCP=FAIL HTTP=skip
    ⚠️  hedgedoc.app:3018: hedgedoc.app:3018 DNS=ok(192.145.46.12) TCP=FAIL HTTP=skip
    ⚠️  hickory-dns.app:53: hickory-dns.app:53 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  introspect-proxy.app:4182: introspect-proxy.app:4182 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  lgtm.app:3200: grafana.app:3200 DNS=ok(216.40.34.41) TCP=FAIL HTTP=skip
    ⚠️  matomo.app:8084: matomo.app:8084 DNS=ok(213.186.33.5) TCP=ok HTTP=err: error sending request for url (http://213.186.33.5:8084/)
    ⚠️  mattermost-bots.app:8065: mattermost.app:8065 DNS=ok(172.234.24.211) TCP=FAIL HTTP=skip
    ⚠️  ntfy.app:8090: ntfy.app:8090 DNS=ok(35.176.213.46) TCP=FAIL HTTP=skip
    ⚠️  ollama.app:11434: ollama.app:11434 DNS=SYS-FAIL→hickory(10.0.0.8) TCP=FAIL HTTP=skip
    ⚠️  photoprism.app:3013: photoprism.app:3013 DNS=ok(176.9.111.126) TCP=FAIL HTTP=skip
    ⚠️  redis.app:6379: redis.app:6379 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  smtp-proxy.app:8080: smtp-proxy.app:8080 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  snappymail.app:8888: snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  umami.app:3006: umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip
    ⚠️  vaultwarden.app:8880: vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip
    ⚠️  windmill.app:8000: windmill-app.app:8000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  Public auth.diegonmarcos.com: auth.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://auth.diegonmarcos.com/), auth=0)
    ⚠️  Public git.diegonmarcos.com: git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0)
    ⚠️  Public api.diegonmarcos.com/c3-api: api.diegonmarcos.com/c3-api: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/c3-api), auth=0)
    ⚠️  Public mcp.diegonmarcos.com/c3-infra-mcp: mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=0)
    ⚠️  Public proxy.diegonmarcos.com: proxy.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=0)
    ⚠️  Public ide.diegonmarcos.com: ide.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://ide.diegonmarcos.com/), auth=0)
    ⚠️  Public api.diegonmarcos.com: api.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=0)
    ⚠️  Public workflows.diegonmarcos.com: workflows.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=0)
    ⚠️  Public logs.diegonmarcos.com: logs.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=0)
    ⚠️  Public pad.diegonmarcos.com: pad.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=0)
    ⚠️  Public files.diegonmarcos.com: files.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=0)
    ⚠️  Public git.diegonmarcos.com: git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0)
    ⚠️  Public sheets.diegonmarcos.com: sheets.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=0)
    ⚠️  Public doc.diegonmarcos.com: doc.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=0)
    ⚠️  Public grafana.diegonmarcos.com: grafana.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=0)
    ⚠️  Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0)
    ⚠️  Public chat.diegonmarcos.com: chat.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=0)
    ⚠️  Public db.diegonmarcos.com: db.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=0)
    ⚠️  Public rss.diegonmarcos.com: rss.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=0)
    ⚠️  Public photos.diegonmarcos.com: photos.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=0)
    ⚠️  Public cal.diegonmarcos.com: cal.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=0)
    ⚠️  Public slides.diegonmarcos.com: slides.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://slides.diegonmarcos.com/), auth=0)
    ⚠️  Public smtp.diegonmarcos.com: smtp.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=0)
    ⚠️  Public webmail.diegonmarcos.com: webmail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=0)
    ⚠️  Public mail.diegonmarcos.com: mail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=0)
    ⚠️  Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0)
    ⚠️  Public vault.diegonmarcos.com: vault.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=0)
    ⚠️  Public windmill.diegonmarcos.com: windmill.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=0)
    ⚠️  Cross backup-gitea: container up, public down: backup-gitea: containers healthy but public URL git.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross c3-infra-api: container up, public down: c3-infra-api: containers healthy but public URL api.diegonmarcos.com/c3-api unreachable — check Caddy/Cloudflare
    ⚠️  Cross c3-infra-mcp: container up, public down: c3-infra-mcp: containers healthy but public URL mcp.diegonmarcos.com/c3-infra-mcp unreachable — check Caddy/Cloudflare
    ⚠️  Cross c3-services-api: container up, public down: c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare
    ⚠️  Cross code-server: container up, public down: code-server: containers healthy but public URL ide.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross crawlee-cloud: container up, public down: crawlee-cloud: containers healthy but public URL api.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross dagu: container up, public down: dagu: containers healthy but public URL workflows.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross dozzle: container up, public down: dozzle: containers healthy but public URL logs.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross etherpad: container up, public down: etherpad: containers healthy but public URL pad.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross filebrowser: container up, public down: filebrowser: containers healthy but public URL files.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross gitea: container up, public down: gitea: containers healthy but public URL git.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross grist: container up, public down: grist: containers healthy but public URL sheets.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross hedgedoc: container up, public down: hedgedoc: containers healthy but public URL doc.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross lgtm: container up, public down: lgtm: containers healthy but public URL grafana.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross matomo: container up, public down: matomo: containers healthy but public URL analytics.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross mattermost-bots: container up, public down: mattermost-bots: containers healthy but public URL chat.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross nocodb: container up, public down: nocodb: containers healthy but public URL db.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross photoprism: container up, public down: photoprism: containers healthy but public URL photos.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross radicale: container up, public down: radicale: containers healthy but public URL cal.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross revealmd: container up, public down: revealmd: containers healthy but public URL slides.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross smtp-proxy: container up, public down: smtp-proxy: containers healthy but public URL smtp.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross snappymail: container up, public down: snappymail: containers healthy but public URL webmail.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross stalwart: container up, public down: stalwart: containers healthy but public URL mail.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  GHA workflows: 5 recent runs, 3 failed
    ⚠️  Drift exited: oci-analytics/umami-setup: umami-setup on oci-analytics is exited: Exited (1) About an hour ago
    ⚠️  Drift exited: oci-mail/introspect-proxy: introspect-proxy on oci-mail is exited: Exited (255) 2 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps is exited: Exited (101) 20 hours ago
    ⚠️  Drift exited: oci-apps/windmill-server: windmill-server on oci-apps is exited: Exited (137) 13 hours ago
    ⚠️  Drift exited: oci-apps/rig: rig on oci-apps is exited: Exited (101) 37 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic: rig-agentic on oci-apps is exited: Exited (101) 37 hours ago
    ⚠️  Drift exited: oci-apps/surrealdb: surrealdb on oci-apps is exited: Exited (2) 37 hours ago
    ⚠️  Caddy TLS: proxy.diegonmarcos.com -> err: error sending request for url (https://proxy.diegonmarcos.com/)
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
    ℹ️  Drift no-port-in-build: alerts-api: alerts-api has port in topology but missing ports.app in build.json
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
    ℹ️  Drift no-port-in-build: mail-mcp: mail-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: matomo: matomo has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mattermost-bots: mattermost-bots has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: mattermost-mcp: mattermost-mcp has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: nocodb: nocodb has port in topology but missing ports.app in build.json
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
    ℹ️  Drift no-port-in-build: stalwart: stalwart has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: umami: umami has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: vaultwarden: vaultwarden has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: windmill: windmill has port in topology but missing ports.app in build.json


0. TIER DASHBOARD
──────────────────────────────────────────────────────────────
    Layer                gcp-proxy        oci-mail         oci-apps         oci-analytics   
                         (front door)     (mail)           (apps)           (analytics)     
    ────────────────────────────────────────────────────────────────────────────────────
    1. Self-check        ⚠️ 4/7                                          
    2. WG Mesh           ❌ 0/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    3. Platform          ❌ 0/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    4. Containers        ❌ 0/11           ✅ 5/5            ⚠️ 45/47         ⚠️ 7/8          
    5. Private URLs      ❌ 0/6            ❌ 0/3            ⚠️ 19/20         ❌ 0/3           
    6. Public URLs       ❌ 0/28                                          
    7. Cross-checks      ❌ 0/23                                          
    8. External          ⚠️ 9/10                                         
    9. Drift             ⚠️ 1/75                                         
    10. Security         ⚠️ 12/19                                        
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✅ C3 API (mesh)                  http://10.0.0.6:8081/health -> 200 (0.5s)
  ⚠️  C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> err: error sending request for url (https://api.diegonmarcos.com/c3-api/health) (8.0s) [WARNING]
  ❌ WireGuard interface            TCP 10.0.0.1:22 -> closed (3.0s) [CRITICAL]
  ✅ Local docker                   Docker 27.5.1 (0.2s)
  ✅ SSH agent                      3 keys loaded (0.0s)
  ✅ cloud-data freshness           generated 2026-03-30T16:11:47.405Z (18h ago)
  ❌ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> NXDOMAIN (3.0s) [CRITICAL]

  Summary: 4/7 passed, 3 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ⚠️  WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=fail SSH=fail (61.0s) [WARNING]
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=TERMINATED Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (48.5s)
  ✅ WG oci-apps                    oci-apps (10.0.0.6): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (37.1s)
  ✅ WG oci-mail                    oci-mail (10.0.0.3): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (25.8s)
  ✅ WG oci-analytics               oci-analytics (10.0.0.4): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (18.0s)

  Summary: 4/5 passed, 1 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ Platform oci-apps              oci-apps: mem 17%, disk 77%, load 0.17 0.36 0.47, 50/56 containers, up 1d 20h (2.7s)
  ✅ Platform oci-mail              oci-mail: mem 65%, disk 68%, load 0.97 0.98 1.00, 6/8 containers, up 0d 1h (2.4s)
  ✅ Platform oci-analytics         oci-analytics: mem 73%, disk 56%, load 2.08 2.12 2.09, 7/8 containers, up 0d 1h (2.4s)
  ❌ Platform gcp-proxy             gcp-proxy: unreachable (WG down) [CRITICAL]
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]

  Summary: 4/5 passed, 1 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ Container alerts-api/alerts-api alerts-api on oci-analytics: Up About an hour (healthy) (healthy)
  ⚠️  Container authelia/authelia    authelia on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container authelia/authelia-redis authelia-redis on gcp-proxy: VM unreachable [WARNING]
  ✅ Container backup-gitea/gitea   gitea on oci-apps: Up 37 hours (none)
  ✅ Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: Up 16 hours (healthy) (healthy)
  ✅ Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: Up 15 hours (healthy) (healthy)
  ⚠️  Container caddy/caddy          caddy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container caddy/introspect-proxy introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ✅ Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container cloud-spec/cloud-spec cloud-spec on oci-apps: Up 37 hours (none)
  ✅ Container code-server/code-server code-server on oci-apps: Up 37 hours (none)
  ✅ Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: Up 20 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: Up 20 hours (none)
  ✅ Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: Up 20 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: Up 20 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: Up 20 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: Up 20 hours (none)
  ✅ Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: Up 20 hours (none)
  ✅ Container dagu/dagu            dagu on oci-mail: Up 2 hours (none)
  ✅ Container dozzle/dozzle        dozzle on oci-analytics: Up About an hour (none)
  ✅ Container etherpad/etherpad_app etherpad_app on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container filebrowser/filebrowser_app filebrowser_app on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container fluent-bit/fluent-bit fluent-bit on oci-analytics: Up About an hour (none)
  ✅ Container gitea/gitea          gitea on oci-apps: Up 37 hours (none)
  ✅ Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container grist/grist_app      grist_app on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: Up 37 hours (healthy) (healthy)
  ⚠️  Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ✅ Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_loki       lgtm_loki on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: Up 37 hours (none)
  ✅ Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: Up 37 hours (none)
  ✅ Container mail-mcp/mail-mcp    mail-mcp on oci-apps: Up 8 hours (none)
  ✅ Container matomo/matomo-hybrid matomo-hybrid on oci-analytics: Up About an hour (none)
  ✅ Container mattermost-bots/mattermost mattermost on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: Up 37 hours (none)
  ✅ Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: Up 37 hours (none)
  ✅ Container nocodb/nocodb        nocodb on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container nocodb/nocodb-db     nocodb-db on oci-apps: Up 37 hours (healthy) (healthy)
  ⚠️  Container ntfy/ntfy            ntfy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container ntfy/github-rss      github-rss on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container ntfy/syslog-bridge   syslog-bridge on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ✅ Container ollama-hai/ollama-hai ollama-hai on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container photoprism/photoprism_app photoprism_app on oci-apps: Up About a minute (health: starting) (starting)
  ✅ Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container photos-webhook/photos-webhook photos-webhook on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_db quant_light_db on oci-apps: Up 21 hours (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: Up 21 hours (none)
  ✅ Container quant-lab-light/quant_light_research quant_light_research on oci-apps: Up 21 hours (healthy) (healthy)
  ✅ Container radicale/radicale    radicale on oci-apps: Up 37 hours (healthy) (healthy)
  ⚠️  Container redis/redis          redis on gcp-proxy: VM unreachable [WARNING]
  ✅ Container revealmd/revealmd_app revealmd_app on oci-apps: Up 37 hours (healthy) (healthy)
  ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: Exited (101) 20 hours ago (exited) [CRITICAL]
  ✅ Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: Up About an hour (none)
  ✅ Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: Up 2 hours (none)
  ✅ Container snappymail/snappymail snappymail on oci-mail: Up 2 hours (healthy) (healthy)
  ✅ Container stalwart/stalwart    stalwart on oci-mail: Up 2 hours (none)
  ✅ Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: Up 2 hours (healthy) (healthy)
  ✅ Container umami/umami          umami on oci-analytics: Up About an hour (healthy) (healthy)
  ✅ Container umami/umami-db       umami-db on oci-analytics: Up About an hour (healthy) (healthy)
  ❌ Container umami/umami-setup    umami-setup on oci-analytics: Exited (1) About an hour ago (exited) [CRITICAL]
  ⚠️  Container vaultwarden/vaultwarden vaultwarden on gcp-proxy: VM unreachable [WARNING]
  ❌ Container windmill/windmill-server windmill-server on oci-apps: Exited (137) 13 hours ago (exited) [CRITICAL]
  ✅ Container windmill/windmill-db windmill-db on oci-apps: Up 37 hours (healthy) (healthy)
  ✅ Container windmill/windmill-worker windmill-worker on oci-apps: Up 37 hours (none)

  Summary: 57/72 passed, 15 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ⚠️  Private URLs (Hickory)         Hickory DNS at 10.0.0.1 is down — falling back to WG IPs [WARNING]
  ⚠️  alerts-api.app:5050            alerts-api.app:5050 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (8.4s) [WARNING]
  ⚠️  authelia.app:9091              authelia.app:9091 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.1s) [WARNING]
  ✅ backup-gitea.app:3002          backup-gitea.app:3002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ✅ c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ✅ c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ✅ c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ⚠️  caddy.app:443                  caddy.app:443 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.1s) [WARNING]
  ✅ cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ✅ cloud-spec.app:3080            c3-spec.app:3080 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ✅ code-server.app:8443           code-server.app:8443 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=302 (8.9s)
  ✅ crawlee-cloud.app:3000         crawlee.app:3000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ⚠️  dagu.app:8070                  dagu.app:8070 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip (8.4s) [WARNING]
  ⚠️  dozzle.app:9999                dozzle.app:9999 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (8.5s) [WARNING]
  ✅ etherpad.app:3012              etherpad.app:3012 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ⚠️  filebrowser.app:3015           filebrowser.app:3015 DNS=ok(15.197.225.128) TCP=FAIL HTTP=skip (8.1s) [WARNING]
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (8.3s) [WARNING]
  ✅ gitea.app:3002                 gitea.app:3002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (23.5s)
  ✅ google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ⚠️  grist.app:3011                 grist.app:3011 DNS=ok(35.188.208.237) TCP=FAIL HTTP=skip (8.1s) [WARNING]
  ⚠️  hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(192.145.46.12) TCP=FAIL HTTP=skip (8.2s) [WARNING]
  ⚠️  hickory-dns.app:53             hickory-dns.app:53 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.1s) [WARNING]
  ⚠️  introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.1s) [WARNING]
  ⚠️  lgtm.app:3200                  grafana.app:3200 DNS=ok(216.40.34.41) TCP=FAIL HTTP=skip (8.2s) [WARNING]
  ✅ mail-mcp.app:3103              mail-mcp.app:3103 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(213.186.33.5) TCP=ok HTTP=err: error sending request for url (http://213.186.33.5:8084/) (13.2s) [WARNING]
  ⚠️  mattermost-bots.app:8065       mattermost.app:8065 DNS=ok(172.234.24.211) TCP=FAIL HTTP=skip (8.1s) [WARNING]
  ✅ mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ✅ nocodb.app:8085                nocodb.app:8085 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ⚠️  ntfy.app:8090                  ntfy.app:8090 DNS=ok(35.176.213.46) TCP=FAIL HTTP=skip (8.1s) [WARNING]
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=SYS-FAIL→hickory(10.0.0.8) TCP=FAIL HTTP=skip (11.1s) [WARNING]
  ✅ ollama-hai.app:11435           ollama-hai.app:11435 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(176.9.111.126) TCP=FAIL HTTP=skip (8.1s) [WARNING]
  ✅ photos-webhook.app:5002        photos-webhook.app:5002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ✅ radicale.app:5232              radicale.app:5232 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=302 (8.9s)
  ⚠️  redis.app:6379                 redis.app:6379 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.1s) [WARNING]
  ✅ revealmd.app:3014              revealmd.app:3014 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (8.9s)
  ✅ rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (8.9s)
  ⚠️  smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip (8.4s) [WARNING]
  ⚠️  snappymail.app:8888            snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip (8.3s) [WARNING]
  ✅ stalwart.app:443               stalwart.app:443 DNS=ok(151.101.1.195) TCP=ok HTTP=n/a (5.1s)
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip (8.1s) [WARNING]
  ⚠️  vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip (5.1s) [WARNING]
  ⚠️  windmill.app:8000              windmill-app.app:8000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (8.3s) [WARNING]

  Summary: 20/44 passed, 24 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ⚠️  Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://auth.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/c3-api), auth=0) (10.3s) [WARNING]
  ⚠️  Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=0) (10.3s) [WARNING]
  ⚠️  Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://ide.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=0) (13.2s) [WARNING]
  ⚠️  Public slides.diegonmarcos.com slides.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://slides.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=0) (10.3s) [WARNING]
  ⚠️  Public windmill.diegonmarcos.com windmill.diegonmarcos.com: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=0) (10.3s) [WARNING]

  Summary: 0/28 passed, 28 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ⚠️  Cross backup-gitea: container up, public down backup-gitea: containers healthy but public URL git.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross c3-infra-api: container up, public down c3-infra-api: containers healthy but public URL api.diegonmarcos.com/c3-api unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross c3-infra-mcp: container up, public down c3-infra-mcp: containers healthy but public URL mcp.diegonmarcos.com/c3-infra-mcp unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross c3-services-api: container up, public down c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross code-server: container up, public down code-server: containers healthy but public URL ide.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross crawlee-cloud: container up, public down crawlee-cloud: containers healthy but public URL api.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross dagu: container up, public down dagu: containers healthy but public URL workflows.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross dozzle: container up, public down dozzle: containers healthy but public URL logs.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross etherpad: container up, public down etherpad: containers healthy but public URL pad.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross filebrowser: container up, public down filebrowser: containers healthy but public URL files.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross gitea: container up, public down gitea: containers healthy but public URL git.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross grist: container up, public down grist: containers healthy but public URL sheets.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross hedgedoc: container up, public down hedgedoc: containers healthy but public URL doc.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross lgtm: container up, public down lgtm: containers healthy but public URL grafana.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross matomo: container up, public down matomo: containers healthy but public URL analytics.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross mattermost-bots: container up, public down mattermost-bots: containers healthy but public URL chat.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross nocodb: container up, public down nocodb: containers healthy but public URL db.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross photoprism: container up, public down photoprism: containers healthy but public URL photos.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross radicale: container up, public down radicale: containers healthy but public URL cal.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross revealmd: container up, public down revealmd: containers healthy but public URL slides.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross smtp-proxy: container up, public down smtp-proxy: containers healthy but public URL smtp.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross snappymail: container up, public down snappymail: containers healthy but public URL webmail.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross stalwart: container up, public down stalwart: containers healthy but public URL mail.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]

  Summary: 0/23 passed, 23 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.1s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (5.3s)
  ⚠️  GHA workflows                  5 recent runs, 3 failed (11.0s) [WARNING]
  ✅ GitHub API                     api.github.com/zen -> 403 (5.2s)
  ✅ MX record                      MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.1s)
  ✅ DKIM dkim._domainkey           DKIM: present (0.0s)
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (5.5s)

  Summary: 9/10 passed, 1 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ⚠️  Drift exited: oci-analytics/umami-setup umami-setup on oci-analytics is exited: Exited (1) About an hour ago [WARNING]
  ⚠️  Drift exited: oci-mail/introspect-proxy introspect-proxy on oci-mail is exited: Exited (255) 2 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps is exited: Exited (101) 20 hours ago [WARNING]
  ✅ Drift exited: oci-apps/crawlee_minio_init crawlee_minio_init on oci-apps exited cleanly [completed init job]
  ⚠️  Drift exited: oci-apps/windmill-server windmill-server on oci-apps is exited: Exited (137) 13 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/rig     rig on oci-apps is exited: Exited (101) 37 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/rig-agentic rig-agentic on oci-apps is exited: Exited (101) 37 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/surrealdb surrealdb on oci-apps is exited: Exited (2) 37 hours ago [WARNING]
  ℹ️  Drift no-domain: c3-diego-personal-data-mcp c3-diego-personal-data-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: c3-services-mcp c3-services-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-cgc-mcp cloud-cgc-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloud-spec    cloud-spec has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloudflare    cloudflare has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: cloudflare-worker cloudflare-worker has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: db-agent      db-agent has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: gcloud        gcloud has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: google-workspace-mcp google-workspace-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: kg-graph      kg-graph has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: mail-mcp      mail-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: mattermost-mcp mattermost-mcp has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama        ollama has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama-arm    ollama-arm has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: ollama-hai    ollama-hai has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: photos-webhook photos-webhook has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: postlite      postlite has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-full quant-lab-full has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-light quant-lab-light has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: redis         redis has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: rig-agentic-hai-1.5bq4 rig-agentic-hai-1.5bq4 has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: sauron-lite   sauron-lite has containers but no domain assigned [INFO]
  ℹ️  Drift no-port-in-build: alerts-api alerts-api has port in topology but missing ports.app in build.json [INFO]
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
  ℹ️  Drift no-port-in-build: mail-mcp mail-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: matomo matomo has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mattermost-bots mattermost-bots has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: mattermost-mcp mattermost-mcp has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: nocodb nocodb has port in topology but missing ports.app in build.json [INFO]
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
  ℹ️  Drift no-port-in-build: stalwart stalwart has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: umami  umami has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: vaultwarden vaultwarden has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: windmill windmill has port in topology but missing ports.app in build.json [INFO]

  Summary: 1/75 passed, 74 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ❌ TLS cert diegonmarcos.com      TLS connection failed (10.2s) [CRITICAL]
  ❌ TLS cert api.diegonmarcos.com  TLS connection failed (10.2s) [CRITICAL]
  ❌ TLS cert auth.diegonmarcos.com TLS connection failed (10.2s) [CRITICAL]
  ❌ TLS cert mail.diegonmarcos.com TLS connection failed (10.2s) [CRITICAL]
  ❌ TLS cert vault.diegonmarcos.com TLS connection failed (10.2s) [CRITICAL]
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.0s)
  ❌ Authelia health                auth.diegonmarcos.com/api/health -> err: error sending request for url (https://auth.diegonmarcos.com/api/health) (5.2s) [CRITICAL]
  ✅ Firewall gcp-proxy             gcp-proxy: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall gcp-t4                gcp-t4: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-apps              oci-apps: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-mail              oci-mail: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-analytics         oci-analytics: no unexpected dangerous ports exposed (3.0s)
  ✅ SSH ports gcp-proxy            gcp-proxy: SSH:22=open Dropbear:2200=closed (3.4s)
  ✅ SSH ports gcp-t4               gcp-t4: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-apps             oci-apps: SSH:22=open Dropbear:2200=closed (3.0s)
  ✅ SSH ports oci-mail             oci-mail: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-analytics        oci-analytics: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ⚠️  Caddy TLS                      proxy.diegonmarcos.com -> err: error sending request for url (https://proxy.diegonmarcos.com/) (5.3s) [WARNING]

  Summary: 12/19 passed, 7 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    130.9s
  L2_wg_mesh               65.1s
  L4-L11_parallel          48.2s
  L11_email_e2e            48.2s
  L5_private_urls          48.2s
  L10_security             48.2s
  L6_public_urls           48.2s
  L8_external              48.2s
  L1_self_check            14.7s
  L3_platform              2.7s
  L9_drift                 0.0s
  L7_cross_checks          0.0s
  L4_containers            0.0s

  Total: 130.9s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 112/289 passed, 12 critical, 98 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
