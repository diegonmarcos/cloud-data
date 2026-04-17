```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-04-16T21:15:09.527720733+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  121 issues: 5 critical, 51 warnings, 65 info

  CRITICAL:
    ❌ Container cloud-cgc-mcp/cloud-cgc-mcp: cloud-cgc-mcp on oci-apps: Exited (1) 6 minutes ago (exited)
    ❌ Container fluent-bit/fluent-bit: fluent-bit on oci-analytics: Created (created)
    ❌ Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: NOT FOUND in docker ps
    ❌ Container umami/umami-setup: umami-setup on oci-analytics: Exited (1) 8 hours ago (exited)
    ❌ Drift missing: oci-mail/syslog-forwarder: syslog-forwarder declared in topology but not found in docker on oci-mail
  WARNINGS:
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  cloud-data freshness: generated 2026-04-15T17:36:15.684Z (27h ago)
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  backup-gitea.app:3002: backup-gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-api.app:8081: c3-infra-api.app:8081 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-mcp.app:3100: c3-infra-mcp.app:3100 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-services-mcp.app:3101: c3-services-mcp.app:3101 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
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
    ⚠️  lgtm.app:3200: grafana.app:3200 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
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
    ⚠️  Cross c3-services-api: container up, public down: c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare
    ⚠️  Cross hickory-dns: container up, public down: hickory-dns: containers healthy but public URL dns.internal unreachable — check Caddy/Cloudflare
    ⚠️  Cross umami: public up, container down: umami: public URL responds but container is down — stale cache or wrong routing
    ⚠️  DKIM dkim._domainkey: DKIM: NOT FOUND
    ⚠️  Drift unmanaged: oci-mail/dagu: dagu running on oci-mail but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/ntfy: ntfy running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/github-rss: github-rss running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/syslog-bridge: syslog-bridge running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/vaultwarden: vaultwarden running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/photos-webhook: photos-webhook running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/news-gdelt: news-gdelt running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/c3-services-api: c3-services-api running on oci-apps but not declared in topology
    ⚠️  Drift exited: oci-analytics/umami-setup: umami-setup on oci-analytics is exited: Exited (1) 8 hours ago
    ⚠️  Drift exited: oci-apps/cloud-cgc-mcp: cloud-cgc-mcp on oci-apps is exited: Exited (1) 6 minutes ago
    ⚠️  Drift exited: oci-apps/photos-db: photos-db on oci-apps is exited: Exited (255) 14 minutes ago
  INFO:
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
    1. Self-check        ⚠️ 5/7                                          
    2. WG Mesh           ✅ 1/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    3. Platform          ✅ 1/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    4. Containers        ✅ 7/7            ⚠️ 3/4           ⚠️ 47/48         ⚠️ 6/8          
    5. Private URLs      ⚠️ 8/41          —                —                —               
    6. Public URLs       ✅ 27/27                                         
    7. Cross-checks      ⚠️ 26/29                                        
    8. External          ⚠️ 9/10                                         
    9. Drift             ⚠️ 1/78                                         
    10. Security         ✅ 19/19                                         
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✅ C3 API (mesh)                  http://10.0.0.6:8081/health -> 200 (0.5s)
  ✅ C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> 429 (3.8s)
  ✅ WireGuard interface            TCP 10.0.0.1:22 -> open (0.1s)
  ✅ Local docker                   Docker 27.5.1 (0.1s)
  ⚠️  SSH agent                      no SSH agent or no keys (0.0s) [WARNING]
  ⚠️  cloud-data freshness           generated 2026-04-15T17:36:15.684Z (27h ago) [WARNING]
  ✅ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> 10.0.0.1 (0.1s)

  Summary: 5/7 passed, 2 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ✅ WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (12.2s)
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=TERMINATED Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (10.7s)
  ✅ WG oci-apps                    oci-apps (10.0.0.6): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (8.4s)
  ✅ WG oci-mail                    oci-mail (10.0.0.3): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (7.9s)
  ✅ WG oci-analytics               oci-analytics (10.0.0.4): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (12.7s)

  Summary: 5/5 passed, 0 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ Platform gcp-proxy             gcp-proxy: mem 57%, disk 69%, load 0.12 0.17 0.14, 14/14 containers, up 15d 23h (1.8s)
  ✅ Platform oci-apps              oci-apps: mem 17%, disk 100%, load 7.46 3.31 2.83, 48/52 containers, up 18d 7h (2.2s)
  ✅ Platform oci-mail              oci-mail: mem 68%, disk 78%, load 0.41 0.32 0.26, 4/4 containers, up 16d 3h (2.2s)
  ✅ Platform oci-analytics         oci-analytics: mem 73%, disk 51%, load 0.39 0.86 0.86, 6/8 containers, up 9d 9h (4.8s)
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]

  Summary: 5/5 passed, 0 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ Container authelia/authelia    authelia on gcp-proxy: Up 50 minutes (healthy) (healthy)
  ✅ Container authelia/authelia-redis authelia-redis on gcp-proxy: Up About an hour (none)
  ✅ Container backup-gitea/gitea   gitea on oci-apps: Up 37 minutes (healthy) (healthy)
  ✅ Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: Up About an hour (unhealthy) (unhealthy)
  ✅ Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: Up About a minute (healthy) (healthy)
  ✅ Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: Up 59 minutes (healthy) (healthy)
  ✅ Container caddy/caddy          caddy on gcp-proxy: Up 3 hours (none)
  ✅ Container caddy/introspect-proxy introspect-proxy on gcp-proxy: Up 3 hours (unhealthy) (unhealthy)
  ❌ Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: Exited (1) 6 minutes ago (exited) [CRITICAL]
  ✅ Container cloud-spec/cloud-spec cloud-spec on oci-apps: Up 33 minutes (none)
  ✅ Container code-server/code-server code-server on oci-apps: Up 35 minutes (none)
  ✅ Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: Up 15 minutes (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: Up 15 minutes (none)
  ✅ Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: Up 15 minutes (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: Up 15 minutes (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: Up 15 minutes (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: Up 15 minutes (none)
  ✅ Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: Up 15 minutes (none)
  ✅ Container dagu/dagu            dagu on oci-analytics: Up 9 hours (none)
  ✅ Container dbgate/dbgate        dbgate on oci-apps: Up 32 minutes (healthy) (healthy)
  ✅ Container dozzle/dozzle        dozzle on oci-analytics: Up 40 hours (none)
  ✅ Container etherpad/etherpad_app etherpad_app on oci-apps: Up 35 minutes (healthy) (healthy)
  ✅ Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: Up 36 minutes (healthy) (healthy)
  ✅ Container filebrowser/filebrowser_app filebrowser_app on oci-apps: Up 36 minutes (healthy) (healthy)
  ❌ Container fluent-bit/fluent-bit fluent-bit on oci-analytics: Created (created) [CRITICAL]
  ✅ Container gitea/gitea          gitea on oci-apps: Up 37 minutes (healthy) (healthy)
  ✅ Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: Up 53 minutes (unhealthy) (unhealthy)
  ✅ Container grist/grist_app      grist_app on oci-apps: Up 33 minutes (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: Up 36 minutes (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: Up 37 minutes (healthy) (healthy)
  ✅ Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: Up 3 hours (none)
  ✅ Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: Up 3 hours (unhealthy) (unhealthy)
  ✅ Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: Up 35 minutes (healthy) (healthy)
  ✅ Container lgtm/lgtm_loki       lgtm_loki on oci-apps: Up 35 minutes (healthy) (healthy)
  ✅ Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: Up 35 minutes (none)
  ✅ Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: Up 35 minutes (none)
  ✅ Container maddy/maddy          maddy on oci-mail: Up 10 hours (none)
  ✅ Container mail-mcp/mail-mcp    mail-mcp on oci-apps: Up 57 minutes (none)
  ✅ Container matomo/matomo-hybrid matomo-hybrid on oci-analytics: Up 8 hours (none)
  ✅ Container mattermost-bots/mattermost mattermost on oci-apps: Up 39 minutes (healthy) (healthy)
  ✅ Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: Up 39 minutes (none)
  ✅ Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: Up 39 minutes (healthy) (healthy)
  ✅ Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: Up 54 minutes (none)
  ✅ Container ntfy/ntfy            ntfy on oci-apps: Up 39 minutes (none)
  ✅ Container ntfy/github-rss      github-rss on oci-apps: Up 39 minutes (none)
  ✅ Container ntfy/syslog-bridge   syslog-bridge on oci-apps: Up 39 minutes (none)
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ✅ Container ollama-hai/ollama-hai ollama-hai on oci-apps: Up 29 minutes (healthy) (healthy)
  ✅ Container photoprism/photoprism_app photoprism_app on oci-apps: Up 39 minutes (healthy) (healthy)
  ✅ Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: Up 39 minutes (healthy) (healthy)
  ✅ Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: Up 39 minutes (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_db quant_light_db on oci-apps: Up 37 minutes (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: Up 37 minutes (none)
  ✅ Container quant-lab-light/quant_light_research quant_light_research on oci-apps: Up 37 minutes (healthy) (healthy)
  ✅ Container radicale/radicale    radicale on oci-apps: Up 37 minutes (healthy) (healthy)
  ✅ Container redis/redis          redis on gcp-proxy: Up 3 hours (healthy) (healthy)
  ✅ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: Up 24 minutes (unhealthy) (unhealthy)
  ✅ Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: Up 40 hours (none)
  ✅ Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: Up 7 hours (none)
  ✅ Container snappymail/snappymail snappymail on oci-mail: Up 9 hours (healthy) (healthy)
  ❌ Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: NOT FOUND in docker ps [CRITICAL]
  ✅ Container umami/umami          umami on oci-analytics: Up 8 hours (healthy) (healthy)
  ✅ Container umami/umami-db       umami-db on oci-analytics: Up 8 hours (healthy) (healthy)
  ❌ Container umami/umami-setup    umami-setup on oci-analytics: Exited (1) 8 hours ago (exited) [CRITICAL]
  ✅ Container vaultwarden/vaultwarden vaultwarden on oci-apps: Up 39 minutes (healthy) (healthy)
  ✅ Container windmill/windmill-server windmill-server on oci-apps: Up 29 minutes (healthy) (healthy)
  ✅ Container windmill/windmill-db windmill-db on oci-apps: Up 29 minutes (healthy) (healthy)
  ✅ Container windmill/windmill-worker windmill-worker on oci-apps: Up 29 minutes (none)

  Summary: 63/68 passed, 5 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ✅ authelia.app:9091              authelia.app:9091 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (0.9s)
  ⚠️  backup-gitea.app:3002          backup-gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.7s) [WARNING]
  ⚠️  c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.8s) [WARNING]
  ⚠️  c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.7s) [WARNING]
  ⚠️  c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.7s) [WARNING]
  ✅ caddy.app:443                  caddy.app:443 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (0.7s)
  ⚠️  cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.8s) [WARNING]
  ⚠️  cloud-spec.app:3080            c3-spec.app:3080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.8s) [WARNING]
  ⚠️  code-server.app:8443           code-server.app:8443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.8s) [WARNING]
  ⚠️  crawlee-cloud.app:3000         crawlee.app:3000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.8s) [WARNING]
  ⚠️  dagu.app:8070                  dagu.app:8070 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.8s) [WARNING]
  ⚠️  dbgate.app:8086                dbgate.app:8086 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.1s) [WARNING]
  ⚠️  dozzle.app:9999                dozzle.app:9999 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.9s) [WARNING]
  ⚠️  etherpad.app:3012              etherpad.app:3012 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.1s) [WARNING]
  ⚠️  filebrowser.app:3015           filebrowser.app:3015 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.9s) [WARNING]
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (0.9s) [WARNING]
  ⚠️  gitea.app:3002                 gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.1s) [WARNING]
  ⚠️  google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.0s) [WARNING]
  ⚠️  grist.app:3011                 grist.app:3011 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.0s) [WARNING]
  ⚠️  hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.1s) [WARNING]
  ✅ hickory-dns.app:53             hickory-dns.app:53 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (1.1s)
  ✅ introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=ok(10.0.0.1) TCP=ok HTTP=404 (1.4s)
  ⚠️  lgtm.app:3200                  grafana.app:3200 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.1s) [WARNING]
  ✅ maddy.app:443                  maddy.app:443 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (1.1s)
  ⚠️  mail-mcp.app:3103              mail-mcp.app:3103 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.3s) [WARNING]
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.2s) [WARNING]
  ⚠️  mattermost-bots.app:8065       mattermost.app:8065 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.2s) [WARNING]
  ⚠️  mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.4s) [WARNING]
  ✅ ntfy.app:8090                  ntfy.app:8090 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (1.5s)
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.2s) [WARNING]
  ⚠️  ollama-hai.app:11435           ollama-hai.app:11435 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.2s) [WARNING]
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.4s) [WARNING]
  ⚠️  photos-webhook.app:5002        photos-webhook.app:5002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.4s) [WARNING]
  ⚠️  radicale.app:5232              radicale.app:5232 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.4s) [WARNING]
  ✅ redis.app:6379                 redis.app:6379 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (1.4s)
  ⚠️  rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.4s) [WARNING]
  ⚠️  smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.4s) [WARNING]
  ⚠️  snappymail.app:8888            snappymail.app:8888 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.6s) [WARNING]
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.6s) [WARNING]
  ✅ vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (1.8s)
  ⚠️  windmill.app:8000              windmill-app.app:8000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.6s) [WARNING]

  Summary: 8/41 passed, 33 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ✅ Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.7s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.1s)
  ✅ Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=429 (no-auth=404, auth=429) (2.1s)
  ✅ Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.7s)
  ✅ Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.8s)
  ✅ Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.8s)
  ✅ Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.1s)
  ✅ Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (0.7s)
  ✅ Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=302 AUTH=429 (no-auth=302, auth=429) (0.9s)
  ✅ Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.1s)
  ✅ Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.2s)
  ✅ Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.3s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.2s)
  ✅ Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.0s)
  ✅ Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=200 AUTH=429 (no-auth=200, auth=429) (2.3s)
  ✅ Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.0s)
  ✅ Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.1s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.0s)
  ✅ Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.1s)
  ✅ Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.0s)
  ✅ Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.0s)
  ✅ Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.1s)
  ✅ Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.2s)
  ✅ Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=301 AUTH=301 (no-auth=301, auth=301) (1.1s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.1s)
  ✅ Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.1s)
  ✅ Public windmill.diegonmarcos.com windmill.diegonmarcos.com: HTTPS=429 AUTH=429 (no-auth=429, auth=429) (1.3s)

  Summary: 27/27 passed, 0 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ✅ Cross authelia                 authelia: container=ok public=ok private=n/a
  ✅ Cross backup-gitea             backup-gitea: container=ok public=ok private=n/a
  ✅ Cross c3-infra-api             c3-infra-api: container=ok public=ok private=n/a
  ✅ Cross c3-infra-mcp             c3-infra-mcp: container=ok public=ok private=n/a
  ⚠️  Cross c3-services-api: container up, public down c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare [WARNING]
  ✅ Cross caddy                    caddy: container=ok public=ok private=n/a
  ✅ Cross code-server              code-server: container=ok public=ok private=n/a
  ✅ Cross crawlee-cloud            crawlee-cloud: container=ok public=ok private=n/a
  ✅ Cross dagu                     dagu: container=ok public=ok private=n/a
  ✅ Cross dbgate                   dbgate: container=ok public=ok private=n/a
  ✅ Cross dozzle                   dozzle: container=ok public=ok private=n/a
  ✅ Cross etherpad                 etherpad: container=ok public=ok private=n/a
  ✅ Cross filebrowser              filebrowser: container=ok public=ok private=n/a
  ✅ Cross gitea                    gitea: container=ok public=ok private=n/a
  ✅ Cross grist                    grist: container=ok public=ok private=n/a
  ✅ Cross hedgedoc                 hedgedoc: container=ok public=ok private=n/a
  ⚠️  Cross hickory-dns: container up, public down hickory-dns: containers healthy but public URL dns.internal unreachable — check Caddy/Cloudflare [WARNING]
  ✅ Cross lgtm                     lgtm: container=ok public=ok private=n/a
  ✅ Cross maddy                    maddy: container=ok public=ok private=n/a
  ✅ Cross matomo                   matomo: container=ok public=ok private=n/a
  ✅ Cross mattermost-bots          mattermost-bots: container=ok public=ok private=n/a
  ✅ Cross ntfy                     ntfy: container=ok public=ok private=n/a
  ✅ Cross photoprism               photoprism: container=ok public=ok private=n/a
  ✅ Cross radicale                 radicale: container=ok public=ok private=n/a
  ✅ Cross smtp-proxy               smtp-proxy: container=ok public=ok private=n/a
  ✅ Cross snappymail               snappymail: container=ok public=ok private=n/a
  ⚠️  Cross umami: public up, container down umami: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross vaultwarden              vaultwarden: container=ok public=ok private=n/a
  ✅ Cross windmill                 windmill: container=ok public=ok private=n/a

  Summary: 26/29 passed, 3 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.0s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (1.2s)
  ✅ GHA workflows                  5 recent runs, 0 failed (1.5s)
  ✅ GitHub API                     api.github.com/zen -> 403 (1.3s)
  ✅ MX record                      MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.1s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.1s)
  ⚠️  DKIM dkim._domainkey           DKIM: NOT FOUND (0.1s) [WARNING]
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (0.3s)

  Summary: 9/10 passed, 1 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ❌ Drift missing: oci-mail/syslog-forwarder syslog-forwarder declared in topology but not found in docker on oci-mail [CRITICAL]
  ⚠️  Drift unmanaged: oci-mail/dagu dagu running on oci-mail but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/ntfy ntfy running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/github-rss github-rss running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/syslog-bridge syslog-bridge running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/vaultwarden vaultwarden running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/photos-webhook photos-webhook running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/news-gdelt news-gdelt running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/c3-services-api c3-services-api running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift exited: oci-analytics/umami-setup umami-setup on oci-analytics is exited: Exited (1) 8 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/cloud-cgc-mcp cloud-cgc-mcp on oci-apps is exited: Exited (1) 6 minutes ago [WARNING]
  ⚠️  Drift exited: oci-apps/photos-db photos-db on oci-apps is exited: Exited (255) 14 minutes ago [WARNING]
  ✅ Drift exited: oci-apps/crawlee_minio_init crawlee_minio_init on oci-apps exited cleanly [completed init job]
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

  Summary: 1/78 passed, 77 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✅ TLS cert diegonmarcos.com      expires Jun 29 22:11:40 2026 GMT (74d) (2.0s)
  ✅ TLS cert api.diegonmarcos.com  expires Jun 29 22:10:39 2026 GMT (74d) (1.9s)
  ✅ TLS cert auth.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (74d) (2.1s)
  ✅ TLS cert mail.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (74d) (2.1s)
  ✅ TLS cert vault.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (74d) (2.1s)
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.1s)
  ✅ Authelia health                auth.diegonmarcos.com/api/health -> 429 (0.5s)
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
  ✅ Caddy TLS                      proxy.diegonmarcos.com -> 302 (0.5s)

  Summary: 19/19 passed, 0 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    64.1s
  L5_private_urls          33.4s
  L6_public_urls           33.4s
  L11_email_e2e            33.4s
  L8_external              33.4s
  L4-L11_parallel          33.4s
  L10_security             33.4s
  L2_wg_mesh               21.2s
  L3_platform              4.8s
  L1_self_check            4.6s
  L7_cross_checks          0.0s
  L4_containers            0.0s
  L9_drift                 0.0s

  Total: 64.1s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 169/290 passed, 5 critical, 51 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
