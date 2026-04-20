```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-04-19T15:02:54.052380542+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  133 issues: 10 critical, 53 warnings, 70 info

  CRITICAL:
    ❌ Container c3-services-mcp/c3-services-mcp: c3-services-mcp on oci-apps: Exited (134) 24 hours ago (exited)
    ❌ Container cloud-builder-x/cloud-builder-x: cloud-builder-x on oci-apps: NOT FOUND in docker ps
    ❌ Container fluent-bit/fluent-bit: fluent-bit on oci-analytics: NOT FOUND in docker ps
    ❌ Container photos-webhook/photos-webhook: photos-webhook on oci-apps: NOT FOUND in docker ps
    ❌ Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: NOT FOUND in docker ps
    ❌ Container umami/umami-setup: umami-setup on oci-apps: Exited (1) About an hour ago (exited)
    ❌ Drift missing: oci-apps/photos-webhook: photos-webhook declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-apps/cloud-builder-x: cloud-builder-x declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-mail/syslog-forwarder: syslog-forwarder declared in topology but not found in docker on oci-mail
    ❌ Drift missing: oci-analytics/fluent-bit: fluent-bit declared in topology but not found in docker on oci-analytics
  WARNINGS:
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  c3-infra-api.app:8081: c3-infra-api.app:8081 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-infra-mcp.app:3100: c3-infra-mcp.app:3100 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-services-api.app:8082: c3-services-api.app:8082 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
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
    ⚠️  hickory-dns.app:53: hickory-dns.app:53 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  introspect-proxy.app:4182: introspect-proxy.app:4182 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
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
    ⚠️  stalwart.app:2443: stalwart.app:2443 DNS=ok(10.0.0.1) TCP=ok HTTP=err: error sending request for url (http://10.0.0.1:2443/)
    ⚠️  umami.app:3006: umami.app:3006 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  Cross hickory-dns: container up, public down: hickory-dns: containers healthy but public URL dns.internal unreachable — check Caddy/Cloudflare
    ⚠️  Cross umami: public up, container down: umami: public URL responds but container is down — stale cache or wrong routing
    ⚠️  GHA workflows: 5 recent runs, 1 failed
    ⚠️  DKIM dkim._domainkey: DKIM: NOT FOUND
    ⚠️  Drift unmanaged: oci-mail/dagu: dagu running on oci-mail but not declared in topology
    ⚠️  Drift unmanaged: oci-mail/maddy-sorter: maddy-sorter running on oci-mail but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/vaultwarden: vaultwarden running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/github-rss: github-rss running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/ntfy: ntfy running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: gcp-proxy/syslog-bridge: syslog-bridge running on gcp-proxy but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/cloud-builder-x-cloud-builder-1: cloud-builder-x-cloud-builder-1 running on oci-apps but not declared in topology
    ⚠️  Drift exited: oci-apps/umami-setup: umami-setup on oci-apps is exited: Exited (1) About an hour ago
    ⚠️  Drift exited: oci-apps/cloud-builder-x-cloud-builder-1: cloud-builder-x-cloud-builder-1 on oci-apps is exited: Exited (0) 47 hours ago
    ⚠️  Drift exited: oci-apps/c3-services-mcp: c3-services-mcp on oci-apps is exited: Exited (134) 24 hours ago
    ⚠️  Drift exited: oci-mail/dagu: dagu on oci-mail is exited: Exited (255) 22 hours ago
    ⚠️  DMARC policy: DMARC: NOT FOUND
  INFO:
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
    1. Self-check        ⚠️ 6/7                                          
    2. WG Mesh           ✅ 1/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    3. Platform          ✅ 1/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    4. Containers        ✅ 7/7            ⚠️ 4/5           ⚠️ 48/52         ⚠️ 3/4          
    5. Private URLs      ⚠️ 5/40          —                —                —               
    6. Public URLs       ✅ 27/27                                         
    7. Cross-checks      ⚠️ 26/28                                        
    8. External          ⚠️ 8/10                                         
    9. Drift             ❌ 0/85                                          
    10. Security         ⚠️ 18/19                                        
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✅ C3 API (mesh)                  http://10.0.0.6:8081/health -> 200 (1.6s)
  ✅ C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> 302 (2.5s)
  ✅ WireGuard interface            TCP 10.0.0.1:22 -> open (1.5s)
  ✅ Local docker                   Docker 27.5.1 (0.3s)
  ⚠️  SSH agent                      no SSH agent or no keys (0.0s) [WARNING]
  ✅ cloud-data freshness           generated 2026-04-19T14:48:27.404Z (0h ago)
  ✅ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> 10.0.0.1 (0.2s)

  Summary: 6/7 passed, 1 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ✅ WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (14.4s)
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=TERMINATED Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (10.2s)
  ✅ WG oci-apps                    oci-apps (10.0.0.6): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (10.6s)
  ✅ WG oci-mail                    oci-mail (10.0.0.3): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (10.2s)
  ✅ WG oci-analytics               oci-analytics (10.0.0.4): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (9.6s)

  Summary: 5/5 passed, 0 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ Platform gcp-proxy             gcp-proxy: mem 58%, disk 69%, load 0.00 0.02 0.04, 14/14 containers, up 18d 17h (3.2s)
  ✅ Platform oci-apps              oci-apps: mem 21%, disk 84%, load 0.23 0.18 0.21, 48/51 containers, up 21d 1h (4.4s)
  ✅ Platform oci-mail              oci-mail: mem 69%, disk 77%, load 0.34 0.33 0.32, 5/6 containers, up 18d 21h (4.4s)
  ✅ Platform oci-analytics         oci-analytics: mem 63%, disk 78%, load 0.03 0.29 0.59, 3/3 containers, up 12d 3h (4.4s)
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]

  Summary: 5/5 passed, 0 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ Container authelia/authelia    authelia on gcp-proxy: Up 2 days (healthy) (healthy)
  ✅ Container authelia/postlite-authelia postlite-authelia on gcp-proxy: Up 2 days (none)
  ✅ Container authelia/authelia-redis authelia-redis on gcp-proxy: Up 2 days (none)
  ✅ Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: Up 2 days (unhealthy) (unhealthy)
  ✅ Container c3-services-api/c3-services-api c3-services-api on oci-apps: Up 2 days (healthy) (healthy)
  ❌ Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: Exited (134) 24 hours ago (exited) [CRITICAL]
  ✅ Container caddy/caddy          caddy on gcp-proxy: Up About an hour (none)
  ❌ Container cloud-builder-x/cloud-builder-x cloud-builder-x on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ✅ Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container cloud-spec/cloud-spec cloud-spec on oci-apps: Up 2 days (none)
  ✅ Container code-server/code-server code-server on oci-apps: Up 2 days (none)
  ✅ Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: Up 2 days (none)
  ✅ Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: Up 2 days (none)
  ✅ Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: Up 2 days (none)
  ✅ Container dagu/dagu            dagu on oci-analytics: Up 46 hours (none)
  ✅ Container dbgate/dbgate        dbgate on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container dozzle/dozzle        dozzle on oci-analytics: Up 4 days (none)
  ✅ Container etherpad/etherpad_app etherpad_app on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container filebrowser/filebrowser_app filebrowser_app on oci-apps: Up 2 days (healthy) (healthy)
  ❌ Container fluent-bit/fluent-bit fluent-bit on oci-analytics: NOT FOUND in docker ps [CRITICAL]
  ✅ Container gitea/gitea          gitea on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container grist/grist_app      grist_app on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: Up 2 days (none)
  ✅ Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: Up About an hour (healthy) (healthy)
  ✅ Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container lgtm/lgtm_loki       lgtm_loki on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: Up 2 days (none)
  ✅ Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: Up 2 days (none)
  ✅ Container maddy/maddy          maddy on oci-mail: Up 6 hours (none)
  ✅ Container mail-mcp/mail-mcp    mail-mcp on oci-apps: Up 28 hours (none)
  ✅ Container matomo/matomo-hybrid matomo-hybrid on oci-apps: Up 26 hours (none)
  ✅ Container mattermost-bots/mattermost mattermost on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: Up 2 days (none)
  ✅ Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: Up 2 days (none)
  ✅ Container news-gdelt/news-gdelt news-gdelt on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container ntfy/ntfy            ntfy on oci-apps: Up 2 days (none)
  ✅ Container ntfy/github-rss      github-rss on oci-apps: Up 2 days (none)
  ✅ Container ntfy/syslog-bridge   syslog-bridge on oci-apps: Up 2 days (none)
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ✅ Container ollama-hai/ollama-hai ollama-hai on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container photoprism/photoprism_app photoprism_app on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: Up 2 days (healthy) (healthy)
  ❌ Container photos-webhook/photos-webhook photos-webhook on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ✅ Container quant-lab-light/quant_light_db quant_light_db on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: Up 2 days (none)
  ✅ Container quant-lab-light/quant_light_research quant_light_research on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container radicale/radicale    radicale on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container redis/redis          redis on gcp-proxy: Up 2 days (healthy) (healthy)
  ✅ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: Up 2 days (healthy) (healthy)
  ✅ Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: Up 4 days (none)
  ✅ Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: Up 6 hours (none)
  ✅ Container snappymail/snappymail snappymail on oci-mail: Up 6 hours (healthy) (healthy)
  ✅ Container stalwart/stalwart    stalwart on oci-mail: Up 6 hours (none)
  ❌ Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: NOT FOUND in docker ps [CRITICAL]
  ✅ Container umami/umami          umami on oci-apps: Up About an hour (healthy) (healthy)
  ✅ Container umami/umami-db       umami-db on oci-apps: Up About an hour (healthy) (healthy)
  ❌ Container umami/umami-setup    umami-setup on oci-apps: Exited (1) About an hour ago (exited) [CRITICAL]
  ✅ Container vaultwarden/vaultwarden vaultwarden on oci-apps: Up 2 days (healthy) (healthy)

  Summary: 62/69 passed, 7 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ✅ authelia.app:9091              authelia.app:9091 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (3.1s)
  ⚠️  c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (1.5s) [WARNING]
  ⚠️  c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.4s) [WARNING]
  ⚠️  c3-services-api.app:8082       c3-services-api.app:8082 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.7s) [WARNING]
  ⚠️  c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.4s) [WARNING]
  ✅ caddy.app:443                  caddy.app:443 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (2.7s)
  ⚠️  cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.7s) [WARNING]
  ⚠️  cloud-spec.app:3080            c3-spec.app:3080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.7s) [WARNING]
  ⚠️  code-server.app:8443           code-server.app:8443 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.7s) [WARNING]
  ⚠️  crawlee-cloud.app:3000         crawlee.app:3000 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (2.7s) [WARNING]
  ⚠️  dagu.app:8070                  dagu.app:8070 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  dbgate.app:8086                dbgate.app:8086 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  dozzle.app:9999                dozzle.app:9999 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  etherpad.app:3012              etherpad.app:3012 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  filebrowser.app:3015           filebrowser.app:3015 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  gitea.app:3002                 gitea.app:3002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.1s) [WARNING]
  ⚠️  google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  grist.app:3011                 grist.app:3011 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (3.0s) [WARNING]
  ⚠️  hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  hickory-dns.app:53             hickory-dns.app:53 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  lgtm.app:3200                  grafana.app:3200 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  mail-mcp.app:3103              mail-mcp.app:3103 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ⚠️  mattermost-bots.app:8065       mattermost.app:8065 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.1s) [WARNING]
  ✅ ntfy.app:8090                  ntfy.app:8090 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (8.8s)
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  ollama-hai.app:11435           ollama-hai.app:11435 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  photos-webhook.app:5002        photos-webhook.app:5002 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  radicale.app:5232              radicale.app:5232 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ✅ redis.app:6379                 redis.app:6379 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (6.7s)
  ⚠️  rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  snappymail.app:8888            snappymail.app:8888 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ⚠️  stalwart.app:2443              stalwart.app:2443 DNS=ok(10.0.0.1) TCP=ok HTTP=err: error sending request for url (http://10.0.0.1:2443/) (8.8s) [WARNING]
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(10.0.0.1) TCP=FAIL HTTP=skip (6.7s) [WARNING]
  ✅ vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (8.8s)

  Summary: 5/40 passed, 35 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ✅ Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (6.4s)
  ✅ Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=0 (no-auth=404, auth=0) (9.5s)
  ✅ Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.6s)
  ✅ Public api.diegonmarcos.com/services api.diegonmarcos.com/services: HTTPS=404 AUTH=0 (no-auth=404, auth=0) (9.4s)
  ✅ Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (3.7s)
  ✅ Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (2.9s)
  ✅ Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (8.9s)
  ✅ Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (3.1s)
  ✅ Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (3.2s)
  ✅ Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (8.9s)
  ✅ Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (12.0s)
  ✅ Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=200) (12.0s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=0 AUTH=302 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=302) (12.0s)
  ✅ Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (3.2s)
  ✅ Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (9.6s)
  ✅ Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (3.7s)
  ✅ Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=301 AUTH=301 (no-auth=301, auth=301) (6.5s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (14.5s)
  ✅ Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (6.8s)
  ✅ Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (14.5s)
  ✅ Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (8.9s)
  ✅ Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=0 AUTH=302 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=302) (11.8s)
  ✅ Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=405 AUTH=405 (no-auth=405, auth=405) (8.9s)
  ✅ Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (6.9s)
  ✅ Public mail-stalwart.diegonmarcos.com mail-stalwart.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (9.6s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=302 (no-auth=302, auth=302) (6.4s)
  ✅ Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (12.4s)

  Summary: 27/27 passed, 0 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ✅ Cross authelia                 authelia: container=ok public=ok private=n/a
  ✅ Cross c3-infra-api             c3-infra-api: container=ok public=ok private=n/a
  ✅ Cross c3-infra-mcp             c3-infra-mcp: container=ok public=ok private=n/a
  ✅ Cross c3-services-api          c3-services-api: container=ok public=ok private=n/a
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
  ✅ Cross stalwart                 stalwart: container=ok public=ok private=n/a
  ⚠️  Cross umami: public up, container down umami: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross vaultwarden              vaultwarden: container=ok public=ok private=n/a

  Summary: 26/28 passed, 2 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.1s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (3.1s)
  ⚠️  GHA workflows                  5 recent runs, 1 failed (7.4s) [WARNING]
  ✅ GitHub API                     api.github.com/zen -> 403 (1.8s)
  ✅ MX record                      MX diegonmarcos.com -> 97 route3.mx.cloudflare.net., 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net. (0.1s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.0s)
  ⚠️  DKIM dkim._domainkey           DKIM: NOT FOUND (0.0s) [WARNING]
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (2.8s)

  Summary: 8/10 passed, 2 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ❌ Drift missing: oci-apps/photos-webhook photos-webhook declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ Drift missing: oci-apps/cloud-builder-x cloud-builder-x declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ Drift missing: oci-mail/syslog-forwarder syslog-forwarder declared in topology but not found in docker on oci-mail [CRITICAL]
  ❌ Drift missing: oci-analytics/fluent-bit fluent-bit declared in topology but not found in docker on oci-analytics [CRITICAL]
  ⚠️  Drift unmanaged: oci-mail/dagu dagu running on oci-mail but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-mail/maddy-sorter maddy-sorter running on oci-mail but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/vaultwarden vaultwarden running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/github-rss github-rss running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/ntfy ntfy running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: gcp-proxy/syslog-bridge syslog-bridge running on gcp-proxy but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/cloud-builder-x-cloud-builder-1 cloud-builder-x-cloud-builder-1 running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift exited: oci-apps/umami-setup umami-setup on oci-apps is exited: Exited (1) About an hour ago [WARNING]
  ⚠️  Drift exited: oci-apps/cloud-builder-x-cloud-builder-1 cloud-builder-x-cloud-builder-1 on oci-apps is exited: Exited (0) 47 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/c3-services-mcp c3-services-mcp on oci-apps is exited: Exited (134) 24 hours ago [WARNING]
  ⚠️  Drift exited: oci-mail/dagu    dagu on oci-mail is exited: Exited (255) 22 hours ago [WARNING]
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

  Summary: 0/85 passed, 85 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✅ TLS cert diegonmarcos.com      expires Jun 29 22:11:40 2026 GMT (71d) (9.4s)
  ✅ TLS cert api.diegonmarcos.com  expires Jun 29 22:10:39 2026 GMT (71d) (8.3s)
  ✅ TLS cert auth.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (71d) (8.3s)
  ✅ TLS cert mail.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (71d) (12.0s)
  ✅ TLS cert vault.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (71d) (12.0s)
  ⚠️  DMARC policy                   DMARC: NOT FOUND (5.0s) [WARNING]
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.2s)
  ✅ Authelia health                auth.diegonmarcos.com/api/health -> 200 (1.7s)
  ✅ Firewall gcp-proxy             gcp-proxy: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall gcp-t4                gcp-t4: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-apps              oci-apps: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-mail              oci-mail: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-analytics         oci-analytics: no unexpected dangerous ports exposed (3.0s)
  ✅ SSH ports gcp-proxy            gcp-proxy: SSH:22=open Dropbear:2200=closed (0.4s)
  ✅ SSH ports gcp-t4               gcp-t4: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-apps             oci-apps: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-mail             oci-mail: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-analytics        oci-analytics: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ Caddy TLS                      proxy.diegonmarcos.com -> 302 (0.8s)

  Summary: 18/19 passed, 1 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    76.7s
  L11_email_e2e            47.0s
  L4-L11_parallel          47.0s
  L5_private_urls          47.0s
  L6_public_urls           47.0s
  L8_external              47.0s
  L10_security             47.0s
  L2_wg_mesh               19.1s
  L1_self_check            6.1s
  L3_platform              4.4s
  L9_drift                 0.0s
  L7_cross_checks          0.0s
  L4_containers            0.0s

  Total: 76.7s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 163/296 passed, 10 critical, 53 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
