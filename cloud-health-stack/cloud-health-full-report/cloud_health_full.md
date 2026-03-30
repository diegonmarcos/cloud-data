```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-03-30T14:43:04.000855307+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  59 issues: 3 critical, 27 warnings, 29 info

  CRITICAL:
    ❌ WG oci-mail: oci-mail (10.0.0.3): TCP=ok SSH=fail
    ❌ Platform oci-mail: oci-mail: unreachable (WG down)
    ❌ Container umami/umami-setup: umami-setup on oci-analytics: Exited (1) About an hour ago (exited)
  WARNINGS:
    ⚠️  SSH agent: no SSH agent or no keys
    ⚠️  Container dagu/dagu: dagu on oci-mail: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container smtp-proxy/smtp-proxy: smtp-proxy on oci-mail: VM unreachable
    ⚠️  Container snappymail/snappymail: snappymail on oci-mail: VM unreachable
    ⚠️  Container stalwart/stalwart: stalwart on oci-mail: VM unreachable
    ⚠️  Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: VM unreachable
    ⚠️  Private alerts-api: alerts-api.app (10.0.0.4:5050) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private c3-services-api: c3-services-api.app (10.0.0.6:8082) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private fluent-bit: fluent-bit.app (10.0.0.4:2020) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private gitea: gitea.app (10.0.0.6:3017) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private matomo: matomo.app (10.0.0.4:8080) TCP=ok HTTP=fail [503]
    ⚠️  Private ollama: ollama.app (10.0.0.8:11434) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private photoprism: photoprism.app (10.0.0.6:3013) TCP=fail HTTP=fail [tcp-fail]
    ⚠️  Private stalwart: stalwart.app (10.0.0.3:443) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.3:443/)]
    ⚠️  Public dns.internal: dns.internal: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://dns.internal/), auth=0)
    ⚠️  Cross dagu: public up, container down: dagu: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross hickory-dns: container up, public down: hickory-dns: containers healthy but public URL dns.internal unreachable — check Caddy/Cloudflare
    ⚠️  Cross hickory-dns: private up, public down: hickory-dns: reachable via WG but public URL fails — Caddy/Cloudflare issue
    ⚠️  Cross smtp-proxy: public up, container down: smtp-proxy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross snappymail: public up, container down: snappymail: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross stalwart: public up, container down: stalwart: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross umami: public up, container down: umami: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Drift exited: oci-apps/rig: rig on oci-apps is exited: Exited (101) 17 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic: rig-agentic on oci-apps is exited: Exited (101) 17 hours ago
    ⚠️  Drift exited: oci-apps/surrealdb: surrealdb on oci-apps is exited: Exited (2) 17 hours ago
    ⚠️  Drift exited: oci-analytics/umami-setup: umami-setup on oci-analytics is exited: Exited (1) About an hour ago
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
    ℹ️  Drift no-port-in-build: fluent-bit: fluent-bit has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: hickory-dns: hickory-dns has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: lgtm: lgtm has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: photos-webhook: photos-webhook has port in topology but missing ports.app in build.json
    ℹ️  Drift no-port-in-build: rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 has port in topology but missing ports.app in build.json


0. TIER DASHBOARD
──────────────────────────────────────────────────────────────
    Layer                gcp-proxy        oci-mail         oci-apps         oci-analytics   
                         (front door)     (mail)           (apps)           (analytics)     
    ────────────────────────────────────────────────────────────────────────────────────
    1. Self-check        ⚠️ 6/7                                          
    2. WG Mesh           ✅ 1/1            ❌ 0/1            ✅ 1/1            ✅ 1/1           
    3. Platform          ✅ 1/1            ❌ 0/1            ✅ 1/1            ✅ 1/1           
    4. Containers        ✅ 11/11          ❌ 0/5            ✅ 47/47          ⚠️ 7/8          
    5. Private URLs      ✅ 7/7            ⚠️ 3/4           ⚠️ 24/27         ⚠️ 2/5          
    6. Public URLs       ⚠️ 29/30                                        
    7. Cross-checks      ⚠️ 20/27                                        
    8. External          ✅ 10/10                                         
    9. Drift             ⚠️ 1/34                                         
    10. Security         ✅ 19/19                                         
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✅ C3 API (mesh)                  http://10.0.0.6:8081/health -> 200 (0.6s)
  ✅ C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> 302 (0.5s)
  ✅ WireGuard interface            TCP 10.0.0.1:22 -> open (0.1s)
  ✅ Local docker                   Docker 27.5.1 (0.2s)
  ⚠️  SSH agent                      no SSH agent or no keys (0.0s) [WARNING]
  ✅ cloud-data freshness           generated 2026-03-30T14:40:34.019Z (0h ago)
  ✅ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> 10.0.0.1 (0.2s)

  Summary: 6/7 passed, 1 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ✅ WG gcp-proxy                   gcp-proxy (10.0.0.1): TCP=ok SSH=ok (2.5s)
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): TCP=fail SSH=fail [spot instance] (3.0s)
  ✅ WG oci-apps                    oci-apps (10.0.0.6): TCP=ok SSH=ok (3.8s)
  ❌ WG oci-mail                    oci-mail (10.0.0.3): TCP=ok SSH=fail (6.4s) [CRITICAL]
  ✅ WG oci-analytics               oci-analytics (10.0.0.4): TCP=ok SSH=ok (9.9s)

  Summary: 4/5 passed, 1 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ Platform gcp-proxy             gcp-proxy: mem 53%, disk 61%, load 0.20 0.42 0.62, 19/19 containers, up 0d 0h (1.6s)
  ✅ Platform oci-apps              oci-apps: mem 18%, disk 76%, load 0.58 0.56 0.65, 52/56 containers, up 1d 0h (2.8s)
  ✅ Platform oci-analytics         oci-analytics: mem 73%, disk 56%, load 2.08 2.12 2.09, 7/8 containers, up 0d 1h (3.6s)
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]
  ❌ Platform oci-mail              oci-mail: unreachable (WG down) [CRITICAL]

  Summary: 4/5 passed, 1 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ Container alerts-api/alerts-api alerts-api on oci-analytics: Up About an hour (healthy) (healthy)
  ✅ Container authelia/authelia    authelia on gcp-proxy: Up 21 minutes (healthy) (healthy)
  ✅ Container authelia/authelia-redis authelia-redis on gcp-proxy: Up 21 minutes (none)
  ✅ Container backup-gitea/gitea   gitea on oci-apps: Up 17 hours (none)
  ✅ Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: Up About an hour (healthy) (healthy)
  ✅ Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container caddy/caddy          caddy on gcp-proxy: Up 19 minutes (none)
  ✅ Container caddy/introspect-proxy introspect-proxy on gcp-proxy: Up 21 minutes (healthy) (healthy)
  ✅ Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container cloud-spec/cloud-spec cloud-spec on oci-apps: Up 17 hours (none)
  ✅ Container code-server/code-server code-server on oci-apps: Up 17 hours (none)
  ✅ Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: Up 41 seconds (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: Up 35 seconds (none)
  ✅ Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: Up 47 seconds (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: Up 47 seconds (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: Up 47 seconds (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: Up 35 seconds (none)
  ✅ Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: Up 47 seconds (none)
  ⚠️  Container dagu/dagu            dagu on oci-mail: VM unreachable [WARNING]
  ✅ Container dozzle/dozzle        dozzle on oci-analytics: Up About an hour (none)
  ✅ Container etherpad/etherpad_app etherpad_app on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container filebrowser/filebrowser_app filebrowser_app on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container fluent-bit/fluent-bit fluent-bit on oci-analytics: Up About an hour (none)
  ✅ Container gitea/gitea          gitea on oci-apps: Up 17 hours (none)
  ✅ Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container grist/grist_app      grist_app on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: Up 11 minutes (none)
  ✅ Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: Up 21 minutes (healthy) (healthy)
  ✅ Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_loki       lgtm_loki on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: Up 17 hours (none)
  ✅ Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: Up 17 hours (none)
  ✅ Container mail-mcp/mail-mcp    mail-mcp on oci-apps: Up 14 hours (none)
  ✅ Container matomo/matomo-hybrid matomo-hybrid on oci-analytics: Up About an hour (none)
  ✅ Container mattermost-bots/mattermost mattermost on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: Up 17 hours (none)
  ✅ Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: Up 17 hours (none)
  ✅ Container nocodb/nocodb        nocodb on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container nocodb/nocodb-db     nocodb-db on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container ntfy/ntfy            ntfy on gcp-proxy: Up 20 minutes (none)
  ✅ Container ntfy/github-rss      github-rss on gcp-proxy: Up 20 minutes (none)
  ✅ Container ntfy/syslog-bridge   syslog-bridge on gcp-proxy: Up 20 minutes (none)
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ✅ Container ollama-hai/ollama-hai ollama-hai on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container photoprism/photoprism_app photoprism_app on oci-apps: Up About a minute (health: starting) (starting)
  ✅ Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container photos-webhook/photos-webhook photos-webhook on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_db quant_light_db on oci-apps: Up About an hour (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: Up About an hour (none)
  ✅ Container quant-lab-light/quant_light_research quant_light_research on oci-apps: Up About an hour (healthy) (healthy)
  ✅ Container radicale/radicale    radicale on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container redis/redis          redis on gcp-proxy: Up 19 minutes (healthy) (healthy)
  ✅ Container revealmd/revealmd_app revealmd_app on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: Up About an hour (none)
  ⚠️  Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: VM unreachable [WARNING]
  ⚠️  Container snappymail/snappymail snappymail on oci-mail: VM unreachable [WARNING]
  ⚠️  Container stalwart/stalwart    stalwart on oci-mail: VM unreachable [WARNING]
  ⚠️  Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: VM unreachable [WARNING]
  ✅ Container umami/umami          umami on oci-analytics: Up About an hour (healthy) (healthy)
  ✅ Container umami/umami-db       umami-db on oci-analytics: Up About an hour (healthy) (healthy)
  ❌ Container umami/umami-setup    umami-setup on oci-analytics: Exited (1) About an hour ago (exited) [CRITICAL]
  ✅ Container vaultwarden/vaultwarden vaultwarden on gcp-proxy: Up 18 minutes (healthy) (healthy)
  ✅ Container windmill/windmill-server windmill-server on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container windmill/windmill-db windmill-db on oci-apps: Up 17 hours (healthy) (healthy)
  ✅ Container windmill/windmill-worker windmill-worker on oci-apps: Up 17 hours (none)

  Summary: 65/72 passed, 7 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ⚠️  Private alerts-api             alerts-api.app (10.0.0.4:5050) TCP=fail HTTP=fail [tcp-fail] (0.5s) [WARNING]
  ✅ Private authelia               authelia.app (10.0.0.1:9091) TCP=ok HTTP=ok [200] (0.6s)
  ✅ Private backup-gitea           backup-gitea.app (10.0.0.6:3002) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private c3-infra-api           c3-infra-api.app (10.0.0.6:8081) TCP=ok HTTP=ok [404] (0.9s)
  ✅ Private c3-infra-mcp           c3-infra-mcp.app (10.0.0.6:3100) TCP=ok HTTP=ok [404] (0.9s)
  ⚠️  Private c3-services-api        c3-services-api.app (10.0.0.6:8082) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ Private c3-services-mcp        c3-services-mcp.app (10.0.0.6:3101) TCP=ok HTTP=ok [404] (0.9s)
  ✅ Private caddy                  caddy.app (10.0.0.1:443) TCP=ok HTTP=ok [400] (0.4s)
  ✅ Private cloud-cgc-mcp          cloud-cgc-mcp.app (10.0.0.6:3105) TCP=ok HTTP=ok [404] (0.9s)
  ✅ Private cloud-spec             c3-spec.app (10.0.0.6:3080) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private code-server            code-server.app (10.0.0.6:8443) TCP=ok HTTP=ok [302] (0.9s)
  ✅ Private crawlee-cloud          crawlee.app (10.0.0.6:3000) TCP=ok HTTP=ok [404] (0.9s)
  ✅ Private dagu                   dagu.app (10.0.0.3:8070) TCP=ok HTTP=ok [200] (1.0s)
  ✅ Private dozzle                 dozzle.app (10.0.0.4:9999) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private etherpad               etherpad.app (10.0.0.6:3012) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private filebrowser            filebrowser.app (10.0.0.6:3015) TCP=ok HTTP=ok [200] (0.9s)
  ⚠️  Private fluent-bit             fluent-bit.app (10.0.0.4:2020) TCP=fail HTTP=fail [tcp-fail] (0.6s) [WARNING]
  ⚠️  Private gitea                  gitea.app (10.0.0.6:3017) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ Private google-workspace-mcp   g-workspace-mcp.app (10.0.0.6:3104) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private grist                  grist.app (10.0.0.6:3011) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private hedgedoc               hedgedoc.app (10.0.0.6:3018) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private hickory-dns            hickory-dns.app (10.0.0.1:53) TCP=ok TCP-ONLY=ok [tcp-only (non-HTTP protocol)] (0.3s)
  ✅ Private introspect-proxy       introspect-proxy.app (10.0.0.1:4182) TCP=ok HTTP=ok [404] (0.6s)
  ✅ Private lgtm                   grafana.app (10.0.0.6:3200) TCP=ok HTTP=ok [302] (0.9s)
  ✅ Private mail-mcp               mail-mcp.app (10.0.0.6:3103) TCP=ok HTTP=ok [404] (0.9s)
  ⚠️  Private matomo                 matomo.app (10.0.0.4:8080) TCP=ok HTTP=fail [503] (0.9s) [WARNING]
  ✅ Private mattermost-bots        mattermost.app (10.0.0.6:8065) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private mattermost-mcp         mattermost-mcp.app (10.0.0.6:3102) TCP=ok HTTP=ok [404] (0.9s)
  ✅ Private nocodb                 nocodb.app (10.0.0.6:8085) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private ntfy                   ntfy.app (10.0.0.1:8090) TCP=ok HTTP=ok [200] (0.6s)
  ⚠️  Private ollama                 ollama.app (10.0.0.8:11434) TCP=fail HTTP=fail [tcp-fail] (3.2s) [WARNING]
  ✅ Private ollama-hai             ollama-hai.app (10.0.0.6:11435) TCP=ok HTTP=ok [200] (0.9s)
  ⚠️  Private photoprism             photoprism.app (10.0.0.6:3013) TCP=fail HTTP=fail [tcp-fail] (0.4s) [WARNING]
  ✅ Private photos-webhook         photos-webhook.app (10.0.0.6:5002) TCP=ok HTTP=ok [404] (1.1s)
  ✅ Private radicale               radicale.app (10.0.0.6:5232) TCP=ok HTTP=ok [302] (0.9s)
  ✅ Private redis                  redis.app (10.0.0.1:6379) TCP=ok TCP-ONLY=ok [tcp-only (non-HTTP protocol)] (0.3s)
  ✅ Private revealmd               revealmd.app (10.0.0.6:3014) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8.app (10.0.0.6:8090) TCP=ok HTTP=ok [404] (1.0s)
  ✅ Private smtp-proxy             smtp-proxy.app (10.0.0.3:8080) TCP=ok HTTP=ok [404] (1.0s)
  ✅ Private snappymail             snappymail.app (10.0.0.3:8888) TCP=ok HTTP=ok [200] (1.0s)
  ⚠️  Private stalwart               stalwart.app (10.0.0.3:443) TCP=ok HTTP=fail [err: error sending request for url (http://10.0.0.3:443/)] (1.0s) [WARNING]
  ✅ Private umami                  umami.app (10.0.0.4:3006) TCP=ok HTTP=ok [200] (0.9s)
  ✅ Private vaultwarden            vaultwarden.app (10.0.0.1:8880) TCP=ok HTTP=ok [200] (0.6s)
  ✅ Private windmill               windmill-app.app (10.0.0.6:8000) TCP=ok HTTP=ok [200] (0.9s)

  Summary: 36/44 passed, 8 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ✅ Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.1s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.3s)
  ✅ Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (1.1s)
  ✅ Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (0.9s)
  ✅ Public api.diegonmarcos.com/services api.diegonmarcos.com/services: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (1.0s)
  ✅ Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.1s)
  ✅ Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.5s)
  ✅ Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.1s)
  ✅ Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.3s)
  ✅ Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.0s)
  ✅ Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.0s)
  ✅ Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.0s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.3s)
  ✅ Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=302 AUTH=401 (no-auth=302, auth=401) (1.3s)
  ✅ Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.0s)
  ⚠️  Public dns.internal            dns.internal: HTTPS=0 AUTH=0 (no-auth=err: error sending request for url (https://dns.internal/), auth=0) (0.2s) [WARNING]
  ✅ Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.6s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=503 (no-auth=302, auth=503) (1.3s)
  ✅ Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.3s)
  ✅ Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.3s)
  ✅ Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=302 AUTH=401 (no-auth=302, auth=401) (1.1s)
  ✅ Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.1s)
  ✅ Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (2.5s)
  ✅ Public slides.diegonmarcos.com slides.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.1s)
  ✅ Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=404 AUTH=404 (no-auth=404, auth=404) (1.6s)
  ✅ Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.1s)
  ✅ Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.5s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=503 (no-auth=302, auth=503) (1.3s)
  ✅ Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (1.1s)
  ✅ Public windmill.diegonmarcos.com windmill.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (1.3s)

  Summary: 29/30 passed, 1 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ✅ Cross authelia                 authelia: container=ok public=ok private=ok
  ✅ Cross backup-gitea             backup-gitea: container=ok public=ok private=ok
  ✅ Cross c3-infra-api             c3-infra-api: container=ok public=ok private=ok
  ✅ Cross c3-infra-mcp             c3-infra-mcp: container=ok public=ok private=ok
  ✅ Cross caddy                    caddy: container=ok public=ok private=ok
  ✅ Cross code-server              code-server: container=ok public=ok private=ok
  ✅ Cross crawlee-cloud            crawlee-cloud: container=ok public=ok private=ok
  ⚠️  Cross dagu: public up, container down dagu: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross dozzle                   dozzle: container=ok public=ok private=ok
  ✅ Cross etherpad                 etherpad: container=ok public=ok private=ok
  ✅ Cross filebrowser              filebrowser: container=ok public=ok private=ok
  ✅ Cross grist                    grist: container=ok public=ok private=ok
  ✅ Cross hedgedoc                 hedgedoc: container=ok public=ok private=ok
  ⚠️  Cross hickory-dns: container up, public down hickory-dns: containers healthy but public URL dns.internal unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross hickory-dns: private up, public down hickory-dns: reachable via WG but public URL fails — Caddy/Cloudflare issue [WARNING]
  ✅ Cross lgtm                     lgtm: container=ok public=ok private=ok
  ✅ Cross mattermost-bots          mattermost-bots: container=ok public=ok private=ok
  ✅ Cross nocodb                   nocodb: container=ok public=ok private=ok
  ✅ Cross ntfy                     ntfy: container=ok public=ok private=ok
  ✅ Cross radicale                 radicale: container=ok public=ok private=ok
  ✅ Cross revealmd                 revealmd: container=ok public=ok private=ok
  ⚠️  Cross smtp-proxy: public up, container down smtp-proxy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross snappymail: public up, container down snappymail: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross stalwart: public up, container down stalwart: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross umami: public up, container down umami: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross vaultwarden              vaultwarden: container=ok public=ok private=ok
  ✅ Cross windmill                 windmill: container=ok public=ok private=ok

  Summary: 20/27 passed, 7 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.0s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (0.4s)
  ✅ GHA workflows                  5 recent runs, 0 failed (0.8s)
  ✅ GitHub API                     api.github.com/zen -> 403 (0.1s)
  ✅ MX record                      MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.0s)
  ✅ DKIM dkim._domainkey           DKIM: present (0.0s)
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ Resend API                     api.resend.com -> 200 (0.2s)

  Summary: 10/10 passed, 0 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ✅ Drift exited: oci-apps/crawlee_minio_init crawlee_minio_init on oci-apps exited cleanly [completed init job]
  ⚠️  Drift exited: oci-apps/rig     rig on oci-apps is exited: Exited (101) 17 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/rig-agentic rig-agentic on oci-apps is exited: Exited (101) 17 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/surrealdb surrealdb on oci-apps is exited: Exited (2) 17 hours ago [WARNING]
  ⚠️  Drift exited: oci-analytics/umami-setup umami-setup on oci-analytics is exited: Exited (1) About an hour ago [WARNING]
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
  ℹ️  Drift no-port-in-build: fluent-bit fluent-bit has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: hickory-dns hickory-dns has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: lgtm   lgtm has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: photos-webhook photos-webhook has port in topology but missing ports.app in build.json [INFO]
  ℹ️  Drift no-port-in-build: rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 has port in topology but missing ports.app in build.json [INFO]

  Summary: 1/34 passed, 33 failed

10. SECURITY
──────────────────────────────────────────────────────────────
  ✅ TLS cert diegonmarcos.com      expires Jun 23 11:05:14 2026 GMT (84d) (0.6s)
  ✅ TLS cert api.diegonmarcos.com  expires Jun 23 11:05:21 2026 GMT (84d) (0.6s)
  ✅ TLS cert auth.diegonmarcos.com expires Jun 23 11:05:21 2026 GMT (84d) (0.7s)
  ✅ TLS cert mail.diegonmarcos.com expires Jun 23 11:05:21 2026 GMT (84d) (0.6s)
  ✅ TLS cert vault.diegonmarcos.com expires Jun 23 11:05:21 2026 GMT (84d) (0.7s)
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.0s)
  ✅ Authelia health                auth.diegonmarcos.com/api/health -> 200 (0.4s)
  ✅ Firewall gcp-proxy             gcp-proxy: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall gcp-t4                gcp-t4: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-apps              oci-apps: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-mail              oci-mail: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-analytics         oci-analytics: no unexpected dangerous ports exposed (3.0s)
  ✅ SSH ports gcp-proxy            gcp-proxy: SSH:22=open Dropbear:2200=closed (3.2s)
  ✅ SSH ports gcp-t4               gcp-t4: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (3.2s)
  ✅ SSH ports oci-apps             oci-apps: SSH:22=open Dropbear:2200=closed (3.0s)
  ✅ SSH ports oci-mail             oci-mail: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-analytics        oci-analytics: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ Caddy TLS                      proxy.diegonmarcos.com -> 302 (0.7s)

  Summary: 19/19 passed, 0 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    41.5s
  L10_security             26.3s
  L11_email_e2e            26.3s
  L4-L11_parallel          26.3s
  L8_external              26.3s
  L5_private_urls          26.3s
  L6_public_urls           26.3s
  L2_wg_mesh               9.9s
  L3_platform              3.6s
  L1_self_check            1.6s
  L9_drift                 0.0s
  L4_containers            0.0s
  L7_cross_checks          0.0s

  Total: 41.5s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 195/254 passed, 3 critical, 27 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
