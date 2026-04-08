```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-04-06T04:40:53.663191845+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  139 issues: 9 critical, 63 warnings, 67 info

  CRITICAL:
    ❌ Hickory DNS resolver: dig caddy.app @10.0.0.1 -> NXDOMAIN
    ❌ Container authelia/authelia: authelia on gcp-proxy: Exited (1) 9 minutes ago (exited)
    ❌ Container c3-services-mcp/c3-services-mcp: c3-services-mcp on oci-apps: Exited (134) 6 hours ago (exited)
    ❌ Container hickory-dns/hickory-dns: hickory-dns on gcp-proxy: Exited (1) 17 hours ago (exited)
    ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: NOT FOUND in docker ps
    ❌ Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: NOT FOUND in docker ps
    ❌ Drift missing: oci-apps/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 declared in topology but not found in docker on oci-apps
    ❌ Drift missing: oci-mail/syslog-forwarder: syslog-forwarder declared in topology but not found in docker on oci-mail
    ❌ Authelia health: auth.diegonmarcos.com/api/health -> 502
  WARNINGS:
    ⚠️  C3 API (public): https://api.diegonmarcos.com/c3-api/health -> 502
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  Platform oci-analytics: oci-analytics: rsync failed
    ⚠️  Container alerts-api/alerts-api: alerts-api on oci-analytics: VM unreachable
    ⚠️  Container dozzle/dozzle: dozzle on oci-analytics: VM unreachable
    ⚠️  Container fluent-bit/fluent-bit: fluent-bit on oci-analytics: VM unreachable
    ⚠️  Container matomo/matomo-hybrid: matomo-hybrid on oci-analytics: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container sauron-forwarder/sauron-forwarder: sauron-forwarder on oci-analytics: VM unreachable
    ⚠️  Container umami/umami: umami on oci-analytics: VM unreachable
    ⚠️  Container umami/umami-db: umami-db on oci-analytics: VM unreachable
    ⚠️  Container umami/umami-setup: umami-setup on oci-analytics: VM unreachable
    ⚠️  Private URLs (Hickory): Hickory DNS at 10.0.0.1 is down — falling back to WG IPs
    ⚠️  alerts-api.app:5050: alerts-api.app:5050 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  authelia.app:9091: authelia.app:9091 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  c3-services-mcp.app:3101: c3-services-mcp.app:3101 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  filebrowser.app:3015: filebrowser.app:3015 DNS=ok(3.33.251.168) TCP=FAIL HTTP=skip
    ⚠️  fluent-bit.app:2020: fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  grist.app:3011: grist.app:3011 DNS=ok(35.188.208.237) TCP=FAIL HTTP=skip
    ⚠️  hedgedoc.app:3018: hedgedoc.app:3018 DNS=ok(192.145.46.12) TCP=FAIL HTTP=skip
    ⚠️  hickory-dns.app:53: hickory-dns.app:53 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip
    ⚠️  lgtm.app:3200: grafana.app:3200 DNS=ok(216.40.34.41) TCP=FAIL HTTP=skip
    ⚠️  matomo.app:8084: matomo.app:8084 DNS=ok(213.186.33.5) TCP=ok HTTP=err: error sending request for url (http://213.186.33.5:8084/)
    ⚠️  mattermost-bots.app:8065: mattermost.app:8065 DNS=ok(172.234.24.211) TCP=FAIL HTTP=skip
    ⚠️  ntfy.app:8090: ntfy.app:8090 DNS=ok(35.176.213.46) TCP=FAIL HTTP=skip
    ⚠️  ollama.app:11434: ollama.app:11434 DNS=SYS-FAIL→hickory(10.0.0.8) TCP=FAIL HTTP=skip
    ⚠️  photoprism.app:3013: photoprism.app:3013 DNS=ok(176.9.111.126) TCP=FAIL HTTP=skip
    ⚠️  umami.app:3006: umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip
    ⚠️  vaultwarden.app:8880: vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip
    ⚠️  Public auth.diegonmarcos.com: auth.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502)
    ⚠️  Public proxy.diegonmarcos.com: proxy.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=502)
    ⚠️  Public ide.diegonmarcos.com: ide.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502)
    ⚠️  Public workflows.diegonmarcos.com: workflows.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=502)
    ⚠️  Public db.diegonmarcos.com: db.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=502)
    ⚠️  Public sheets.diegonmarcos.com: sheets.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=502)
    ⚠️  Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=502)
    ⚠️  Public chat.diegonmarcos.com: chat.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=502)
    ⚠️  Public rss.diegonmarcos.com: rss.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=502)
    ⚠️  Public cal.diegonmarcos.com: cal.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=502)
    ⚠️  Public analytics.diegonmarcos.com: analytics.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=502)
    ⚠️  Cross c3-services-api: container up, public down: c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare
    ⚠️  Cross caddy: container up, public down: caddy: containers healthy but public URL proxy.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross code-server: container up, public down: code-server: containers healthy but public URL ide.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross dagu: container up, public down: dagu: containers healthy but public URL workflows.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross dbgate: container up, public down: dbgate: containers healthy but public URL db.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross dozzle: public up, container down: dozzle: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross grist: container up, public down: grist: containers healthy but public URL sheets.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross mattermost-bots: container up, public down: mattermost-bots: containers healthy but public URL chat.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross ntfy: container up, public down: ntfy: containers healthy but public URL rss.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  Cross radicale: container up, public down: radicale: containers healthy but public URL cal.diegonmarcos.com unreachable — check Caddy/Cloudflare
    ⚠️  GHCR registry: ghcr.io/v2/ -> err: error sending request for url (https://ghcr.io/v2/)
    ⚠️  GHA workflows: 5 recent runs, 2 failed
    ⚠️  Drift unmanaged: oci-apps/trusting_herschel: trusting_herschel running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/revealmd_app: revealmd_app running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/nocodb: nocodb running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/photos-webhook: photos-webhook running on oci-apps but not declared in topology
    ⚠️  Drift unmanaged: oci-apps/nocodb-db: nocodb-db running on oci-apps but not declared in topology
    ⚠️  Drift exited: gcp-proxy/authelia: authelia on gcp-proxy is exited: Exited (1) 9 minutes ago
    ⚠️  Drift exited: gcp-proxy/hickory-dns: hickory-dns on gcp-proxy is exited: Exited (1) 17 hours ago
    ⚠️  Drift exited: oci-apps/c3-services-mcp: c3-services-mcp on oci-apps is exited: Exited (134) 6 hours ago
    ⚠️  Drift exited: oci-apps/nocodb: nocodb on oci-apps is exited: Exited (143) 17 hours ago
    ⚠️  Drift exited: oci-apps/nocodb-db: nocodb-db on oci-apps is exited: Exited (0) 17 hours ago
    ⚠️  Caddy TLS: proxy.diegonmarcos.com -> 502
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
    ℹ️  Drift no-domain: postlite: postlite has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-full: quant-lab-full has containers but no domain assigned
    ℹ️  Drift no-domain: quant-lab-light: quant-lab-light has containers but no domain assigned
    ℹ️  Drift no-domain: redis: redis has containers but no domain assigned
    ℹ️  Drift no-domain: revealmd: revealmd has containers but no domain assigned
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
    1. Self-check        ⚠️ 4/7                                          
    2. WG Mesh           ✅ 1/1            ✅ 1/1            ✅ 1/1            ✅ 1/1           
    3. Platform          ✅ 1/1            ✅ 1/1            ✅ 1/1            ❌ 0/1           
    4. Containers        ⚠️ 9/11          ⚠️ 4/5           ⚠️ 42/44         ❌ 0/8           
    5. Private URLs      ⚠️ 2/5           ✅ 3/3            ⚠️ 18/19         ⚠️ 1/3          
    6. Public URLs       ⚠️ 16/27                                        
    7. Cross-checks      ⚠️ 15/25                                        
    8. External          ⚠️ 8/10                                         
    9. Drift             ❌ 0/79                                          
    10. Security         ⚠️ 17/19                                        
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✅ C3 API (mesh)                  http://10.0.0.6:8081/health -> 200 (0.5s)
  ⚠️  C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> 502 (0.5s) [WARNING]
  ✅ WireGuard interface            TCP 10.0.0.1:22 -> open (0.1s)
  ✅ Local docker                   Docker 27.5.1 (0.3s)
  ⚠️  SSH agent                      no SSH agent or no keys (0.0s) [WARNING]
  ✅ cloud-data freshness           generated 2026-04-05T10:55:04.745Z (17h ago)
  ❌ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> NXDOMAIN (3.0s) [CRITICAL]

  Summary: 4/7 passed, 3 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ✅ WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (10.7s)
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=TERMINATED Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (9.0s)
  ✅ WG oci-apps                    oci-apps (10.0.0.6): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (8.2s)
  ✅ WG oci-mail                    oci-mail (10.0.0.3): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (7.6s)
  ✅ WG oci-analytics               oci-analytics (10.0.0.4): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (7.6s)

  Summary: 5/5 passed, 0 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ Platform gcp-proxy             gcp-proxy: mem 55%, disk 74%, load 0.10 0.08 0.02, 16/18 containers, up 5d 7h (1.2s)
  ✅ Platform oci-apps              oci-apps: mem 21%, disk 60%, load 0.15 0.29 0.34, 50/53 containers, up 7d 14h (2.3s)
  ✅ Platform oci-mail              oci-mail: mem 65%, disk 78%, load 0.00 0.09 0.09, 4/4 containers, up 5d 10h (2.3s)
  ⚠️  Platform oci-analytics         oci-analytics: rsync failed (0.5s) [WARNING]
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]

  Summary: 4/5 passed, 1 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ⚠️  Container alerts-api/alerts-api alerts-api on oci-analytics: VM unreachable [WARNING]
  ❌ Container authelia/authelia    authelia on gcp-proxy: Exited (1) 9 minutes ago (exited) [CRITICAL]
  ✅ Container authelia/authelia-redis authelia-redis on gcp-proxy: Up 9 minutes (none)
  ✅ Container backup-gitea/gitea   gitea on oci-apps: Up 7 days (none)
  ✅ Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: Up 3 days (healthy) (healthy)
  ✅ Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: Up 43 hours (healthy) (healthy)
  ❌ Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: Exited (134) 6 hours ago (exited) [CRITICAL]
  ✅ Container caddy/caddy          caddy on gcp-proxy: Up 19 hours (none)
  ✅ Container caddy/introspect-proxy introspect-proxy on gcp-proxy: Up 19 hours (healthy) (healthy)
  ✅ Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container cloud-spec/cloud-spec cloud-spec on oci-apps: Up 7 days (none)
  ✅ Container code-server/code-server code-server on oci-apps: Up 7 days (none)
  ✅ Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: Up 6 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: Up 6 days (none)
  ✅ Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: Up 6 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: Up 6 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: Up 6 days (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: Up 6 days (none)
  ✅ Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: Up 6 days (none)
  ✅ Container dagu/dagu            dagu on oci-mail: Up 40 hours (none)
  ✅ Container dbgate/dbgate        dbgate on oci-apps: Up 17 hours (healthy) (healthy)
  ⚠️  Container dozzle/dozzle        dozzle on oci-analytics: VM unreachable [WARNING]
  ✅ Container etherpad/etherpad_app etherpad_app on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container filebrowser/filebrowser_app filebrowser_app on oci-apps: Up 7 days (healthy) (healthy)
  ⚠️  Container fluent-bit/fluent-bit fluent-bit on oci-analytics: VM unreachable [WARNING]
  ✅ Container gitea/gitea          gitea on oci-apps: Up 7 days (none)
  ✅ Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container grist/grist_app      grist_app on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: Up 7 days (healthy) (healthy)
  ❌ Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: Exited (1) 17 hours ago (exited) [CRITICAL]
  ✅ Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: Up 19 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container lgtm/lgtm_loki       lgtm_loki on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: Up 7 days (none)
  ✅ Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: Up 7 days (none)
  ✅ Container maddy/maddy          maddy on oci-mail: Up 43 hours (none)
  ✅ Container mail-mcp/mail-mcp    mail-mcp on oci-apps: Up 43 hours (none)
  ⚠️  Container matomo/matomo-hybrid matomo-hybrid on oci-analytics: VM unreachable [WARNING]
  ✅ Container mattermost-bots/mattermost mattermost on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: Up 43 hours (none)
  ✅ Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: Up 2 days (none)
  ✅ Container ntfy/ntfy            ntfy on gcp-proxy: Up 17 hours (none)
  ✅ Container ntfy/github-rss      github-rss on gcp-proxy: Up 17 hours (none)
  ✅ Container ntfy/syslog-bridge   syslog-bridge on gcp-proxy: Up 17 hours (none)
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ✅ Container ollama-hai/ollama-hai ollama-hai on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container photoprism/photoprism_app photoprism_app on oci-apps: Up 4 days (unhealthy) (unhealthy)
  ✅ Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_db quant_light_db on oci-apps: Up 6 days (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: Up 6 days (none)
  ✅ Container quant-lab-light/quant_light_research quant_light_research on oci-apps: Up 6 days (healthy) (healthy)
  ✅ Container radicale/radicale    radicale on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container redis/redis          redis on gcp-proxy: Up 4 days (healthy) (healthy)
  ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: NOT FOUND in docker ps [CRITICAL]
  ⚠️  Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: VM unreachable [WARNING]
  ✅ Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: Up 4 days (none)
  ✅ Container snappymail/snappymail snappymail on oci-mail: Up 42 hours (healthy) (healthy)
  ❌ Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: NOT FOUND in docker ps [CRITICAL]
  ⚠️  Container umami/umami          umami on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container umami/umami-db       umami-db on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container umami/umami-setup    umami-setup on oci-analytics: VM unreachable [WARNING]
  ✅ Container vaultwarden/vaultwarden vaultwarden on gcp-proxy: Up 4 days (healthy) (healthy)
  ✅ Container windmill/windmill-server windmill-server on oci-apps: Up 5 days (healthy) (healthy)
  ✅ Container windmill/windmill-db windmill-db on oci-apps: Up 7 days (healthy) (healthy)
  ✅ Container windmill/windmill-worker windmill-worker on oci-apps: Up 7 days (none)

  Summary: 55/69 passed, 14 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ⚠️  Private URLs (Hickory)         Hickory DNS at 10.0.0.1 is down — falling back to WG IPs [WARNING]
  ⚠️  alerts-api.app:5050            alerts-api.app:5050 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (11.4s) [WARNING]
  ⚠️  authelia.app:9091              authelia.app:9091 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.4s) [WARNING]
  ✅ backup-gitea.app:3002          backup-gitea.app:3002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (9.0s)
  ✅ c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (23.0s)
  ✅ c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (9.0s)
  ⚠️  c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=FAIL HTTP=skip (22.5s) [WARNING]
  ✅ caddy.app:443                  caddy.app:443 DNS=ok(204.69.207.1) TCP=ok HTTP=n/a (8.2s)
  ✅ cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (12.0s)
  ✅ cloud-spec.app:3080            c3-spec.app:3080 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (12.0s)
  ✅ code-server.app:8443           code-server.app:8443 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=302 (12.0s)
  ✅ crawlee-cloud.app:3000         crawlee.app:3000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (12.0s)
  ✅ dagu.app:8070                  dagu.app:8070 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=ok HTTP=200 (12.0s)
  ✅ dbgate.app:8086                dbgate.app:8086 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (9.0s)
  ✅ dozzle.app:9999                dozzle.app:9999 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=ok HTTP=200 (12.0s)
  ✅ etherpad.app:3012              etherpad.app:3012 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (18.3s)
  ⚠️  filebrowser.app:3015           filebrowser.app:3015 DNS=ok(3.33.251.168) TCP=FAIL HTTP=skip (8.3s) [WARNING]
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (11.4s) [WARNING]
  ✅ gitea.app:3002                 gitea.app:3002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (27.1s)
  ✅ google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (12.0s)
  ⚠️  grist.app:3011                 grist.app:3011 DNS=ok(35.188.208.237) TCP=FAIL HTTP=skip (11.2s) [WARNING]
  ⚠️  hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(192.145.46.12) TCP=FAIL HTTP=skip (22.2s) [WARNING]
  ⚠️  hickory-dns.app:53             hickory-dns.app:53 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=FAIL HTTP=skip (11.4s) [WARNING]
  ✅ introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=ok HTTP=404 (22.7s)
  ⚠️  lgtm.app:3200                  grafana.app:3200 DNS=ok(216.40.34.41) TCP=FAIL HTTP=skip (8.4s) [WARNING]
  ✅ maddy.app:443                  maddy.app:443 DNS=ok(37.97.254.27) TCP=ok HTTP=n/a (5.4s)
  ✅ mail-mcp.app:3103              mail-mcp.app:3103 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (12.0s)
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(213.186.33.5) TCP=ok HTTP=err: error sending request for url (http://213.186.33.5:8084/) (16.3s) [WARNING]
  ⚠️  mattermost-bots.app:8065       mattermost.app:8065 DNS=ok(172.234.24.211) TCP=FAIL HTTP=skip (8.3s) [WARNING]
  ✅ mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (12.0s)
  ⚠️  ntfy.app:8090                  ntfy.app:8090 DNS=ok(35.176.213.46) TCP=FAIL HTTP=skip (22.2s) [WARNING]
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=SYS-FAIL→hickory(10.0.0.8) TCP=FAIL HTTP=skip (11.3s) [WARNING]
  ✅ ollama-hai.app:11435           ollama-hai.app:11435 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (9.0s)
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(176.9.111.126) TCP=FAIL HTTP=skip (8.3s) [WARNING]
  ✅ photos-webhook.app:5002        photos-webhook.app:5002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (23.0s)
  ✅ radicale.app:5232              radicale.app:5232 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=302 (8.9s)
  ✅ redis.app:6379                 redis.app:6379 DNS=SYS-FAIL→hickory(10.0.0.1) TCP=ok HTTP=n/a (11.3s)
  ✅ rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (17.9s)
  ✅ smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=ok HTTP=404 (9.0s)
  ✅ snappymail.app:8888            snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=ok HTTP=200 (9.0s)
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip (11.2s) [WARNING]
  ⚠️  vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip (14.3s) [WARNING]
  ✅ windmill.app:8000              windmill-app.app:8000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (9.0s)

  Summary: 26/43 passed, 17 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ⚠️  Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502) (8.6s) [WARNING]
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (8.8s)
  ✅ Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (8.6s)
  ✅ Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://mcp.diegonmarcos.com/c3-infra-mcp), auth=200) (11.9s)
  ⚠️  Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://proxy.diegonmarcos.com/), auth=502) (11.9s) [WARNING]
  ⚠️  Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=502 AUTH=502 (no-auth=502, auth=502) (11.2s) [WARNING]
  ✅ Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://api.diegonmarcos.com/), auth=200) (13.5s)
  ⚠️  Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://workflows.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ⚠️  Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://db.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ✅ Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://logs.diegonmarcos.com/), auth=200) (13.4s)
  ✅ Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://pad.diegonmarcos.com/), auth=200) (13.5s)
  ✅ Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://files.diegonmarcos.com/), auth=200) (13.5s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://git.diegonmarcos.com/), auth=200) (13.4s)
  ⚠️  Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://sheets.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ✅ Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://doc.diegonmarcos.com/), auth=200) (13.4s)
  ✅ Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://grafana.diegonmarcos.com/), auth=200) (13.5s)
  ✅ Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://mail.diegonmarcos.com/), auth=200) (13.5s)
  ⚠️  Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ⚠️  Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://chat.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ⚠️  Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://rss.diegonmarcos.com/), auth=502) (13.4s) [WARNING]
  ✅ Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://photos.diegonmarcos.com/), auth=200) (13.6s)
  ⚠️  Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://cal.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ✅ Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=0 AUTH=404 (no-auth=err: error sending request for url (https://smtp.diegonmarcos.com/), auth=404) (13.7s)
  ✅ Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=0 AUTH=301 (no-auth=err: error sending request for url (https://webmail.diegonmarcos.com/), auth=301) (13.5s)
  ⚠️  Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=0 AUTH=502 (no-auth=err: error sending request for url (https://analytics.diegonmarcos.com/), auth=502) (13.5s) [WARNING]
  ✅ Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://vault.diegonmarcos.com/), auth=200) (13.4s)
  ✅ Public windmill.diegonmarcos.com windmill.diegonmarcos.com: HTTPS=0 AUTH=200 (no-auth=err: error sending request for url (https://windmill.diegonmarcos.com/), auth=200) (13.5s)

  Summary: 16/27 passed, 11 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ✅ Cross backup-gitea             backup-gitea: container=ok public=ok private=n/a
  ✅ Cross c3-infra-api             c3-infra-api: container=ok public=ok private=n/a
  ✅ Cross c3-infra-mcp             c3-infra-mcp: container=ok public=ok private=n/a
  ⚠️  Cross c3-services-api: container up, public down c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross caddy: container up, public down caddy: containers healthy but public URL proxy.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross code-server: container up, public down code-server: containers healthy but public URL ide.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ✅ Cross crawlee-cloud            crawlee-cloud: container=ok public=ok private=n/a
  ⚠️  Cross dagu: container up, public down dagu: containers healthy but public URL workflows.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross dbgate: container up, public down dbgate: containers healthy but public URL db.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross dozzle: public up, container down dozzle: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross etherpad                 etherpad: container=ok public=ok private=n/a
  ✅ Cross filebrowser              filebrowser: container=ok public=ok private=n/a
  ✅ Cross gitea                    gitea: container=ok public=ok private=n/a
  ⚠️  Cross grist: container up, public down grist: containers healthy but public URL sheets.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ✅ Cross hedgedoc                 hedgedoc: container=ok public=ok private=n/a
  ✅ Cross lgtm                     lgtm: container=ok public=ok private=n/a
  ✅ Cross maddy                    maddy: container=ok public=ok private=n/a
  ⚠️  Cross mattermost-bots: container up, public down mattermost-bots: containers healthy but public URL chat.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross ntfy: container up, public down ntfy: containers healthy but public URL rss.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ✅ Cross photoprism               photoprism: container=ok public=ok private=n/a
  ⚠️  Cross radicale: container up, public down radicale: containers healthy but public URL cal.diegonmarcos.com unreachable — check Caddy/Cloudflare [WARNING]
  ✅ Cross smtp-proxy               smtp-proxy: container=ok public=ok private=n/a
  ✅ Cross snappymail               snappymail: container=ok public=ok private=n/a
  ✅ Cross vaultwarden              vaultwarden: container=ok public=ok private=n/a
  ✅ Cross windmill                 windmill: container=ok public=ok private=n/a

  Summary: 15/25 passed, 10 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.0s)
  ⚠️  GHCR registry                  ghcr.io/v2/ -> err: error sending request for url (https://ghcr.io/v2/) (8.0s) [WARNING]
  ⚠️  GHA workflows                  5 recent runs, 2 failed (6.1s) [WARNING]
  ✅ GitHub API                     api.github.com/zen -> 403 (0.2s)
  ✅ MX record                      MX diegonmarcos.com -> 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net., 22 route1.mx.cloudflare.net. (0.0s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.0s)
  ✅ DKIM dkim._domainkey           DKIM: present (0.0s)
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (5.6s)

  Summary: 8/10 passed, 2 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ❌ Drift missing: oci-apps/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 declared in topology but not found in docker on oci-apps [CRITICAL]
  ❌ Drift missing: oci-mail/syslog-forwarder syslog-forwarder declared in topology but not found in docker on oci-mail [CRITICAL]
  ⚠️  Drift unmanaged: oci-apps/trusting_herschel trusting_herschel running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/revealmd_app revealmd_app running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/nocodb nocodb running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/photos-webhook photos-webhook running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift unmanaged: oci-apps/nocodb-db nocodb-db running on oci-apps but not declared in topology [WARNING]
  ⚠️  Drift exited: gcp-proxy/authelia authelia on gcp-proxy is exited: Exited (1) 9 minutes ago [WARNING]
  ⚠️  Drift exited: gcp-proxy/hickory-dns hickory-dns on gcp-proxy is exited: Exited (1) 17 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/c3-services-mcp c3-services-mcp on oci-apps is exited: Exited (134) 6 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/nocodb  nocodb on oci-apps is exited: Exited (143) 17 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/nocodb-db nocodb-db on oci-apps is exited: Exited (0) 17 hours ago [WARNING]
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
  ℹ️  Drift no-domain: postlite      postlite has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-full quant-lab-full has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: quant-lab-light quant-lab-light has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: redis         redis has containers but no domain assigned [INFO]
  ℹ️  Drift no-domain: revealmd      revealmd has containers but no domain assigned [INFO]
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

  Summary: 0/79 passed, 79 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✅ TLS cert diegonmarcos.com      expires Jun 29 22:11:40 2026 GMT (84d) (3.9s)
  ✅ TLS cert api.diegonmarcos.com  expires Jun 29 22:10:39 2026 GMT (84d) (3.9s)
  ✅ TLS cert auth.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (84d) (11.5s)
  ✅ TLS cert mail.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (84d) (8.6s)
  ✅ TLS cert vault.diegonmarcos.com expires Jun 29 22:10:39 2026 GMT (84d) (3.9s)
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.0s)
  ❌ Authelia health                auth.diegonmarcos.com/api/health -> 502 (5.6s) [CRITICAL]
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
  ⚠️  Caddy TLS                      proxy.diegonmarcos.com -> 502 (0.9s) [WARNING]

  Summary: 17/19 passed, 2 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    70.2s
  L4-L11_parallel          48.2s
  L5_private_urls          48.2s
  L6_public_urls           48.2s
  L10_security             48.2s
  L11_email_e2e            48.2s
  L8_external              48.2s
  L2_wg_mesh               15.3s
  L1_self_check            4.4s
  L3_platform              2.3s
  L7_cross_checks          0.0s
  L4_containers            0.0s
  L9_drift                 0.0s

  Total: 70.2s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 151/290 passed, 9 critical, 63 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
