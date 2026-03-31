```

  ██████╗██╗      ██████╗ ██╗   ██╗██████╗
  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ██║     ██║     ██║   ██║██║   ██║██║  ██║
  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
  CLOUD HEALTH FULL — 2026-03-30T20:20:42.619098717+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  89 issues: 4 critical, 56 warnings, 29 info

  CRITICAL:
    ❌ Platform gcp-proxy: gcp-proxy: unreachable (WG down)
    ❌ Platform oci-mail: oci-mail: unreachable (WG down)
    ❌ Platform oci-analytics: oci-analytics: unreachable (WG down)
    ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps: Exited (101) 5 hours ago (exited)
  WARNINGS:
    ⚠️  WG gcp-proxy: gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=fail
    ⚠️  WG oci-mail: oci-mail (10.0.0.3): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=fail
    ⚠️  WG oci-analytics: oci-analytics (10.0.0.4): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=fail
    ⚠️  Container alerts-api/alerts-api: alerts-api on oci-analytics: VM unreachable
    ⚠️  Container authelia/authelia: authelia on gcp-proxy: VM unreachable
    ⚠️  Container authelia/authelia-redis: authelia-redis on gcp-proxy: VM unreachable
    ⚠️  Container caddy/caddy: caddy on gcp-proxy: VM unreachable
    ⚠️  Container caddy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container dagu/dagu: dagu on oci-mail: VM unreachable
    ⚠️  Container dozzle/dozzle: dozzle on oci-analytics: VM unreachable
    ⚠️  Container fluent-bit/fluent-bit: fluent-bit on oci-analytics: VM unreachable
    ⚠️  Container hickory-dns/hickory-dns: hickory-dns on gcp-proxy: VM unreachable
    ⚠️  Container introspect-proxy/introspect-proxy: introspect-proxy on gcp-proxy: VM unreachable
    ⚠️  Container matomo/matomo-hybrid: matomo-hybrid on oci-analytics: VM unreachable
    ⚠️  Container ntfy/ntfy: ntfy on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/github-rss: github-rss on gcp-proxy: VM unreachable
    ⚠️  Container ntfy/syslog-bridge: syslog-bridge on gcp-proxy: VM unreachable
    ⚠️  Container ollama/ollama: ollama on gcp-t4: VM unreachable
    ⚠️  Container redis/redis: redis on gcp-proxy: VM unreachable
    ⚠️  Container sauron-forwarder/sauron-forwarder: sauron-forwarder on oci-analytics: VM unreachable
    ⚠️  Container smtp-proxy/smtp-proxy: smtp-proxy on oci-mail: VM unreachable
    ⚠️  Container snappymail/snappymail: snappymail on oci-mail: VM unreachable
    ⚠️  Container stalwart/stalwart: stalwart on oci-mail: VM unreachable
    ⚠️  Container syslog-forwarder/syslog-forwarder: syslog-forwarder on oci-mail: VM unreachable
    ⚠️  Container umami/umami: umami on oci-analytics: VM unreachable
    ⚠️  Container umami/umami-db: umami-db on oci-analytics: VM unreachable
    ⚠️  Container umami/umami-setup: umami-setup on oci-analytics: VM unreachable
    ⚠️  Container vaultwarden/vaultwarden: vaultwarden on gcp-proxy: VM unreachable
    ⚠️  alerts-api.app:5050: alerts-api.app:5050 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  dagu.app:8070: dagu.app:8070 DNS=ok(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  fluent-bit.app:2020: fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  matomo.app:8084: matomo.app:8084 DNS=ok(10.0.0.4) TCP=FAIL HTTP=skip
    ⚠️  ollama.app:11434: ollama.app:11434 DNS=ok(10.0.0.8) TCP=FAIL HTTP=skip
    ⚠️  photoprism.app:3013: photoprism.app:3013 DNS=ok(10.0.0.6) TCP=FAIL HTTP=skip
    ⚠️  smtp-proxy.app:8080: smtp-proxy.app:8080 DNS=ok(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  snappymail.app:8888: snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  stalwart.app:443: stalwart.app:443 DNS=ok(10.0.0.3) TCP=FAIL HTTP=skip
    ⚠️  umami.app:3006: umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip
    ⚠️  vaultwarden.app:8880: vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip
    ⚠️  Public smtp.diegonmarcos.com: smtp.diegonmarcos.com: HTTPS=502 AUTH=0 (no-auth=502, auth=0)
    ⚠️  Cross authelia: public up, container down: authelia: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross c3-services-api: container up, public down: c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare
    ⚠️  Cross caddy: public up, container down: caddy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross dagu: public up, container down: dagu: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross dozzle: public up, container down: dozzle: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross matomo: public up, container down: matomo: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross ntfy: public up, container down: ntfy: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross snappymail: public up, container down: snappymail: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross stalwart: public up, container down: stalwart: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross umami: public up, container down: umami: public URL responds but container is down — stale cache or wrong routing
    ⚠️  Cross vaultwarden: public up, container down: vaultwarden: public URL responds but container is down — stale cache or wrong routing
    ⚠️  GHA workflows: 5 recent runs, 3 failed
    ⚠️  Drift exited: oci-apps/rig-agentic-sonn-14bq8: rig-agentic-sonn-14bq8 on oci-apps is exited: Exited (101) 5 hours ago
    ⚠️  Drift exited: oci-apps/rig: rig on oci-apps is exited: Exited (101) 23 hours ago
    ⚠️  Drift exited: oci-apps/rig-agentic: rig-agentic on oci-apps is exited: Exited (101) 23 hours ago
    ⚠️  Drift exited: oci-apps/surrealdb: surrealdb on oci-apps is exited: Exited (2) 23 hours ago
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
    1. Self-check        ✅ 7/7                                           
    2. WG Mesh           ❌ 0/1            ❌ 0/1            ✅ 1/1            ❌ 0/1           
    3. Platform          ❌ 0/1            ❌ 0/1            ✅ 1/1            ❌ 0/1           
    4. Containers        ❌ 0/11           ❌ 0/5            ⚠️ 46/47         ❌ 0/8           
    5. Private URLs      ✅ 6/6            ❌ 0/4            ⚠️ 25/26         ⚠️ 1/4          
    6. Public URLs       ⚠️ 27/28                                        
    7. Cross-checks      ⚠️ 17/28                                        
    8. External          ⚠️ 9/10                                         
    9. Drift             ⚠️ 1/34                                         
    10. Security         ✅ 19/19                                         
    11. E2E Email        ✅ 1/1                                           

1. SELF-CHECK
──────────────────────────────────────────────────────────────
  ✅ C3 API (mesh)                  http://10.0.0.6:8081/health -> 200 (0.9s)
  ✅ C3 API (public)                https://api.diegonmarcos.com/c3-api/health -> 302 (1.8s)
  ✅ WireGuard interface            TCP 10.0.0.1:22 -> open (0.2s)
  ✅ Local docker                   Docker 27.5.1 (0.2s)
  ✅ SSH agent                      3 keys loaded (0.0s)
  ✅ cloud-data freshness           generated 2026-03-30T18:10:59.753Z (2h ago)
  ✅ Hickory DNS resolver           dig caddy.app @10.0.0.1 -> 10.0.0.1 (0.3s)

  Summary: 7/7 passed, 0 failed

2. WIREGUARD MESH
──────────────────────────────────────────────────────────────
  ⚠️  WG gcp-proxy                   gcp-proxy (10.0.0.1): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=fail (21.3s) [WARNING]
  ✅ WG gcp-t4                      gcp-t4 (10.0.0.8): VPS=TERMINATED Dropbear=fail WG:TCP=fail SSH=fail [spot instance] (13.0s)
  ✅ WG oci-apps                    oci-apps (10.0.0.6): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=ok (17.6s)
  ⚠️  WG oci-mail                    oci-mail (10.0.0.3): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=fail (11.9s) [WARNING]
  ⚠️  WG oci-analytics               oci-analytics (10.0.0.4): VPS=RUNNING Dropbear=fail WG:TCP=ok SSH=fail (14.9s) [WARNING]

  Summary: 2/5 passed, 3 failed

3. PLATFORM
──────────────────────────────────────────────────────────────
  ✅ Platform oci-apps              oci-apps: mem 18%, disk 76%, load 0.48 0.78 0.84, 51/56 containers, up 1d 6h (4.7s)
  ❌ Platform gcp-proxy             gcp-proxy: unreachable (WG down) [CRITICAL]
  ✅ Platform gcp-t4                gcp-t4: unreachable (WG down) [spot instance]
  ❌ Platform oci-mail              oci-mail: unreachable (WG down) [CRITICAL]
  ❌ Platform oci-analytics         oci-analytics: unreachable (WG down) [CRITICAL]

  Summary: 2/5 passed, 3 failed

4. CONTAINERS
──────────────────────────────────────────────────────────────
  ⚠️  Container alerts-api/alerts-api alerts-api on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container authelia/authelia    authelia on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container authelia/authelia-redis authelia-redis on gcp-proxy: VM unreachable [WARNING]
  ✅ Container backup-gitea/gitea   gitea on oci-apps: Up 23 hours (none)
  ✅ Container c3-infra-api/c3-infra-api c3-infra-api on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container c3-infra-mcp/c3-infra-mcp c3-infra-mcp on oci-apps: Up 2 hours (healthy) (healthy)
  ✅ Container c3-services-mcp/c3-services-mcp c3-services-mcp on oci-apps: Up 15 minutes (healthy) (healthy)
  ⚠️  Container caddy/caddy          caddy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container caddy/introspect-proxy introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ✅ Container cloud-cgc-mcp/cloud-cgc-mcp cloud-cgc-mcp on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container cloud-spec/cloud-spec cloud-spec on oci-apps: Up 23 hours (none)
  ✅ Container code-server/code-server code-server on oci-apps: Up 23 hours (none)
  ✅ Container crawlee-cloud/crawlee_api crawlee_api on oci-apps: Up 6 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_dashboard crawlee_dashboard on oci-apps: Up 6 hours (none)
  ✅ Container crawlee-cloud/crawlee_db crawlee_db on oci-apps: Up 6 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_minio crawlee_minio on oci-apps: Up 6 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_redis crawlee_redis on oci-apps: Up 6 hours (healthy) (healthy)
  ✅ Container crawlee-cloud/crawlee_runner crawlee_runner on oci-apps: Up 6 hours (none)
  ✅ Container crawlee-cloud/crawlee_scheduler crawlee_scheduler on oci-apps: Up 6 hours (none)
  ⚠️  Container dagu/dagu            dagu on oci-mail: VM unreachable [WARNING]
  ⚠️  Container dozzle/dozzle        dozzle on oci-analytics: VM unreachable [WARNING]
  ✅ Container etherpad/etherpad_app etherpad_app on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container etherpad/etherpad_postgres etherpad_postgres on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container filebrowser/filebrowser_app filebrowser_app on oci-apps: Up 23 hours (healthy) (healthy)
  ⚠️  Container fluent-bit/fluent-bit fluent-bit on oci-analytics: VM unreachable [WARNING]
  ✅ Container gitea/gitea          gitea on oci-apps: Up 23 hours (none)
  ✅ Container google-workspace-mcp/google-workspace-mcp google-workspace-mcp on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container grist/grist_app      grist_app on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_app hedgedoc_app on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container hedgedoc/hedgedoc_postgres hedgedoc_postgres on oci-apps: Up 23 hours (healthy) (healthy)
  ⚠️  Container hickory-dns/hickory-dns hickory-dns on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container introspect-proxy/introspect-proxy introspect-proxy on gcp-proxy: VM unreachable [WARNING]
  ✅ Container lgtm/lgtm_grafana    lgtm_grafana on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_loki       lgtm_loki on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container lgtm/lgtm_mimir      lgtm_mimir on oci-apps: Up 23 hours (none)
  ✅ Container lgtm/lgtm_tempo      lgtm_tempo on oci-apps: Up 23 hours (none)
  ✅ Container mail-mcp/mail-mcp    mail-mcp on oci-apps: Up 3 hours (none)
  ⚠️  Container matomo/matomo-hybrid matomo-hybrid on oci-analytics: VM unreachable [WARNING]
  ✅ Container mattermost-bots/mattermost mattermost on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container mattermost-bots/mattermost-bots mattermost-bots on oci-apps: Up 23 hours (none)
  ✅ Container mattermost-bots/mattermost-postgres mattermost-postgres on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container mattermost-mcp/mattermost-mcp mattermost-mcp on oci-apps: Up 23 hours (none)
  ✅ Container nocodb/nocodb        nocodb on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container nocodb/nocodb-db     nocodb-db on oci-apps: Up 23 hours (healthy) (healthy)
  ⚠️  Container ntfy/ntfy            ntfy on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container ntfy/github-rss      github-rss on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container ntfy/syslog-bridge   syslog-bridge on gcp-proxy: VM unreachable [WARNING]
  ⚠️  Container ollama/ollama        ollama on gcp-t4: VM unreachable [WARNING]
  ✅ Container ollama-hai/ollama-hai ollama-hai on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container photoprism/photoprism_app photoprism_app on oci-apps: Up About a minute (health: starting) (starting)
  ✅ Container photoprism/photoprism_mariadb photoprism_mariadb on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container photoprism/photoprism_rclone photoprism_rclone on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container photos-webhook/photos-webhook photos-webhook on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_db quant_light_db on oci-apps: Up 7 hours (healthy) (healthy)
  ✅ Container quant-lab-light/quant_light_engine quant_light_engine on oci-apps: Up 7 hours (none)
  ✅ Container quant-lab-light/quant_light_research quant_light_research on oci-apps: Up 7 hours (healthy) (healthy)
  ✅ Container radicale/radicale    radicale on oci-apps: Up 23 hours (healthy) (healthy)
  ⚠️  Container redis/redis          redis on gcp-proxy: VM unreachable [WARNING]
  ✅ Container revealmd/revealmd_app revealmd_app on oci-apps: Up 23 hours (healthy) (healthy)
  ❌ Container rig-agentic-sonn-14bq8/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps: Exited (101) 5 hours ago (exited) [CRITICAL]
  ⚠️  Container sauron-forwarder/sauron-forwarder sauron-forwarder on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container smtp-proxy/smtp-proxy smtp-proxy on oci-mail: VM unreachable [WARNING]
  ⚠️  Container snappymail/snappymail snappymail on oci-mail: VM unreachable [WARNING]
  ⚠️  Container stalwart/stalwart    stalwart on oci-mail: VM unreachable [WARNING]
  ⚠️  Container syslog-forwarder/syslog-forwarder syslog-forwarder on oci-mail: VM unreachable [WARNING]
  ⚠️  Container umami/umami          umami on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container umami/umami-db       umami-db on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container umami/umami-setup    umami-setup on oci-analytics: VM unreachable [WARNING]
  ⚠️  Container vaultwarden/vaultwarden vaultwarden on gcp-proxy: VM unreachable [WARNING]
  ✅ Container windmill/windmill-server windmill-server on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container windmill/windmill-db windmill-db on oci-apps: Up 23 hours (healthy) (healthy)
  ✅ Container windmill/windmill-worker windmill-worker on oci-apps: Up 23 hours (none)

  Summary: 46/72 passed, 26 failed

5. PRIVATE URLS
──────────────────────────────────────────────────────────────
  ⚠️  alerts-api.app:5050            alerts-api.app:5050 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (6.5s) [WARNING]
  ✅ authelia.app:9091              authelia.app:9091 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (6.2s)
  ✅ backup-gitea.app:3002          backup-gitea.app:3002 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (9.0s)
  ✅ c3-infra-api.app:8081          c3-infra-api.app:8081 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (9.0s)
  ✅ c3-infra-mcp.app:3100          c3-infra-mcp.app:3100 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (9.0s)
  ✅ c3-services-mcp.app:3101       c3-services-mcp.app:3101 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (9.0s)
  ✅ caddy.app:443                  caddy.app:443 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (5.0s)
  ✅ cloud-cgc-mcp.app:3105         cloud-cgc-mcp.app:3105 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (9.0s)
  ✅ cloud-spec.app:3080            c3-spec.app:3080 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (9.0s)
  ✅ code-server.app:8443           code-server.app:8443 DNS=ok(10.0.0.6) TCP=ok HTTP=302 (13.7s)
  ✅ crawlee-cloud.app:3000         crawlee.app:3000 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (10.5s)
  ⚠️  dagu.app:8070                  dagu.app:8070 DNS=ok(10.0.0.3) TCP=FAIL HTTP=skip (7.7s) [WARNING]
  ✅ dozzle.app:9999                dozzle.app:9999 DNS=ok(10.0.0.4) TCP=ok HTTP=200 (13.7s)
  ✅ etherpad.app:3012              etherpad.app:3012 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.2s)
  ✅ filebrowser.app:3015           filebrowser.app:3015 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.5s)
  ⚠️  fluent-bit.app:2020            fluent-bit.app:2020 DNS=SYS-FAIL→hickory(10.0.0.4) TCP=FAIL HTTP=skip (13.9s) [WARNING]
  ✅ gitea.app:3002                 gitea.app:3002 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.5s)
  ✅ google-workspace-mcp.app:3104  g-workspace-mcp.app:3104 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.5s)
  ✅ grist.app:3011                 grist.app:3011 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.5s)
  ✅ hedgedoc.app:3018              hedgedoc.app:3018 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (14.9s)
  ✅ hickory-dns.app:53             hickory-dns.app:53 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (6.4s)
  ✅ introspect-proxy.app:4182      introspect-proxy.app:4182 DNS=ok(10.0.0.1) TCP=ok HTTP=404 (11.8s)
  ✅ lgtm.app:3200                  grafana.app:3200 DNS=ok(10.0.0.6) TCP=ok HTTP=302 (10.5s)
  ✅ mail-mcp.app:3103              mail-mcp.app:3103 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (10.2s)
  ⚠️  matomo.app:8084                matomo.app:8084 DNS=ok(10.0.0.4) TCP=FAIL HTTP=skip (7.7s) [WARNING]
  ✅ mattermost-bots.app:8065       mattermost.app:8065 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.5s)
  ✅ mattermost-mcp.app:3102        mattermost-mcp.app:3102 DNS=ok(10.0.0.6) TCP=ok HTTP=404 (13.7s)
  ✅ nocodb.app:8085                nocodb.app:8085 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (14.9s)
  ✅ ntfy.app:8090                  ntfy.app:8090 DNS=ok(10.0.0.1) TCP=ok HTTP=200 (9.2s)
  ⚠️  ollama.app:11434               ollama.app:11434 DNS=ok(10.0.0.8) TCP=FAIL HTTP=skip (9.5s) [WARNING]
  ✅ ollama-hai.app:11435           ollama-hai.app:11435 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (10.4s)
  ⚠️  photoprism.app:3013            photoprism.app:3013 DNS=ok(10.0.0.6) TCP=FAIL HTTP=skip (7.7s) [WARNING]
  ✅ photos-webhook.app:5002        photos-webhook.app:5002 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (17.4s)
  ✅ radicale.app:5232              radicale.app:5232 DNS=ok(10.0.0.6) TCP=ok HTTP=302 (10.5s)
  ✅ redis.app:6379                 redis.app:6379 DNS=ok(10.0.0.1) TCP=ok HTTP=n/a (7.7s)
  ✅ revealmd.app:3014              revealmd.app:3014 DNS=ok(10.0.0.6) TCP=ok HTTP=200 (13.7s)
  ✅ rig-agentic-sonn-14bq8.app:8091 rig-agentic-sonn-14bq8.app:8091 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=404 (21.2s)
  ⚠️  smtp-proxy.app:8080            smtp-proxy.app:8080 DNS=ok(10.0.0.3) TCP=FAIL HTTP=skip (7.7s) [WARNING]
  ⚠️  snappymail.app:8888            snappymail.app:8888 DNS=SYS-FAIL→hickory(10.0.0.3) TCP=FAIL HTTP=skip (17.4s) [WARNING]
  ⚠️  stalwart.app:443               stalwart.app:443 DNS=ok(10.0.0.3) TCP=FAIL HTTP=skip (7.7s) [WARNING]
  ⚠️  umami.app:3006                 umami.app:3006 DNS=ok(76.76.21.21) TCP=FAIL HTTP=skip (13.1s) [WARNING]
  ⚠️  vaultwarden.app:8880           vaultwarden.app:8880 DNS=ok(78.46.170.179) TCP=FAIL HTTP=skip (10.2s) [WARNING]
  ✅ windmill.app:8000              windmill-app.app:8000 DNS=SYS-FAIL→hickory(10.0.0.6) TCP=ok HTTP=200 (16.6s)

  Summary: 32/43 passed, 11 failed

6. PUBLIC URLS
──────────────────────────────────────────────────────────────
  ✅ Public auth.diegonmarcos.com   auth.diegonmarcos.com: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (12.5s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (12.5s)
  ✅ Public api.diegonmarcos.com/c3-api api.diegonmarcos.com/c3-api: HTTPS=404 AUTH=0 (no-auth=404, auth=0) (12.5s)
  ✅ Public mcp.diegonmarcos.com/c3-infra-mcp mcp.diegonmarcos.com/c3-infra-mcp: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (9.5s)
  ✅ Public proxy.diegonmarcos.com  proxy.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (11.7s)
  ✅ Public ide.diegonmarcos.com    ide.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (11.3s)
  ✅ Public api.diegonmarcos.com    api.diegonmarcos.com: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (12.5s)
  ✅ Public workflows.diegonmarcos.com workflows.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (11.3s)
  ✅ Public logs.diegonmarcos.com   logs.diegonmarcos.com: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (11.3s)
  ✅ Public pad.diegonmarcos.com    pad.diegonmarcos.com: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (11.7s)
  ✅ Public files.diegonmarcos.com  files.diegonmarcos.com: HTTPS=200 AUTH=0 (no-auth=200, auth=0) (11.7s)
  ✅ Public git.diegonmarcos.com    git.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (13.7s)
  ✅ Public sheets.diegonmarcos.com sheets.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (12.5s)
  ✅ Public doc.diegonmarcos.com    doc.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (12.1s)
  ✅ Public grafana.diegonmarcos.com grafana.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (12.5s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (14.8s)
  ✅ Public chat.diegonmarcos.com   chat.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (12.5s)
  ✅ Public db.diegonmarcos.com     db.diegonmarcos.com: HTTPS=302 AUTH=200 (no-auth=302, auth=200) (14.2s)
  ✅ Public rss.diegonmarcos.com    rss.diegonmarcos.com: HTTPS=302 AUTH=401 (no-auth=302, auth=401) (13.8s)
  ✅ Public photos.diegonmarcos.com photos.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (11.5s)
  ✅ Public cal.diegonmarcos.com    cal.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (14.7s)
  ✅ Public slides.diegonmarcos.com slides.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (12.1s)
  ⚠️  Public smtp.diegonmarcos.com   smtp.diegonmarcos.com: HTTPS=502 AUTH=0 (no-auth=502, auth=0) (14.0s) [WARNING]
  ✅ Public webmail.diegonmarcos.com webmail.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (11.3s)
  ✅ Public mail.diegonmarcos.com   mail.diegonmarcos.com: HTTPS=302 AUTH=502 (no-auth=302, auth=502) (13.7s)
  ✅ Public analytics.diegonmarcos.com analytics.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (14.7s)
  ✅ Public vault.diegonmarcos.com  vault.diegonmarcos.com: HTTPS=200 AUTH=200 (no-auth=200, auth=200) (11.3s)
  ✅ Public windmill.diegonmarcos.com windmill.diegonmarcos.com: HTTPS=302 AUTH=0 (no-auth=302, auth=0) (13.3s)

  Summary: 27/28 passed, 1 failed

7. CROSS-CHECKS
──────────────────────────────────────────────────────────────
  ⚠️  Cross authelia: public up, container down authelia: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross backup-gitea             backup-gitea: container=ok public=ok private=n/a
  ✅ Cross c3-infra-api             c3-infra-api: container=ok public=ok private=n/a
  ✅ Cross c3-infra-mcp             c3-infra-mcp: container=ok public=ok private=n/a
  ⚠️  Cross c3-services-api: container up, public down c3-services-api: containers healthy but public URL api.diegonmarcos.com/services unreachable — check Caddy/Cloudflare [WARNING]
  ⚠️  Cross caddy: public up, container down caddy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross code-server              code-server: container=ok public=ok private=n/a
  ✅ Cross crawlee-cloud            crawlee-cloud: container=ok public=ok private=n/a
  ⚠️  Cross dagu: public up, container down dagu: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross dozzle: public up, container down dozzle: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross etherpad                 etherpad: container=ok public=ok private=n/a
  ✅ Cross filebrowser              filebrowser: container=ok public=ok private=n/a
  ✅ Cross gitea                    gitea: container=ok public=ok private=n/a
  ✅ Cross grist                    grist: container=ok public=ok private=n/a
  ✅ Cross hedgedoc                 hedgedoc: container=ok public=ok private=n/a
  ✅ Cross lgtm                     lgtm: container=ok public=ok private=n/a
  ⚠️  Cross matomo: public up, container down matomo: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross mattermost-bots          mattermost-bots: container=ok public=ok private=n/a
  ✅ Cross nocodb                   nocodb: container=ok public=ok private=n/a
  ⚠️  Cross ntfy: public up, container down ntfy: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross photoprism               photoprism: container=ok public=ok private=n/a
  ✅ Cross radicale                 radicale: container=ok public=ok private=n/a
  ✅ Cross revealmd                 revealmd: container=ok public=ok private=n/a
  ⚠️  Cross snappymail: public up, container down snappymail: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross stalwart: public up, container down stalwart: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross umami: public up, container down umami: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ⚠️  Cross vaultwarden: public up, container down vaultwarden: public URL responds but container is down — stale cache or wrong routing [WARNING]
  ✅ Cross windmill                 windmill: container=ok public=ok private=n/a

  Summary: 17/28 passed, 11 failed

8. EXTERNAL
──────────────────────────────────────────────────────────────
  ✅ Cloudflare DNS A               dig diegonmarcos.com @1.1.1.1 -> 35.226.147.64 (0.1s)
  ✅ GHCR registry                  ghcr.io/v2/ -> 401 (3.6s)
  ⚠️  GHA workflows                  5 recent runs, 3 failed (6.8s) [WARNING]
  ✅ GitHub API                     api.github.com/zen -> 403 (5.2s)
  ✅ MX record                      MX diegonmarcos.com -> 22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ A mail                         mail.diegonmarcos.com -> 35.226.147.64 (0.1s)
  ✅ DKIM dkim._domainkey           DKIM: present (0.0s)
  ✅ SPF record                     SPF: v=spf1 ip4:130.110.251.193 include:_spf.mx.cloudflare.net include:amazonses.com include:eu.rp.oracleemaildelivery.com -all (0.0s)
  ✅ DMARC record                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✅ Resend API                     api.resend.com -> 200 (1.5s)

  Summary: 9/10 passed, 1 failed

9. DRIFT
──────────────────────────────────────────────────────────────
  ⚠️  Drift exited: oci-apps/rig-agentic-sonn-14bq8 rig-agentic-sonn-14bq8 on oci-apps is exited: Exited (101) 5 hours ago [WARNING]
  ✅ Drift exited: oci-apps/crawlee_minio_init crawlee_minio_init on oci-apps exited cleanly [completed init job]
  ⚠️  Drift exited: oci-apps/rig     rig on oci-apps is exited: Exited (101) 23 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/rig-agentic rig-agentic on oci-apps is exited: Exited (101) 23 hours ago [WARNING]
  ⚠️  Drift exited: oci-apps/surrealdb surrealdb on oci-apps is exited: Exited (2) 23 hours ago [WARNING]
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
  ✅ TLS cert diegonmarcos.com      expires Jun 23 11:05:14 2026 GMT (84d) (10.5s)
  ✅ TLS cert api.diegonmarcos.com  expires Jun 23 11:05:21 2026 GMT (84d) (11.8s)
  ✅ TLS cert auth.diegonmarcos.com expires Jun 23 11:05:21 2026 GMT (84d) (15.5s)
  ✅ TLS cert mail.diegonmarcos.com expires Jun 23 11:05:21 2026 GMT (84d) (11.8s)
  ✅ TLS cert vault.diegonmarcos.com expires Jun 23 11:05:21 2026 GMT (84d) (12.5s)
  ✅ DMARC policy                   DMARC: v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)
  ✅ SPF strict (-all)              SPF: present (strict=-all) (0.1s)
  ✅ Authelia health                auth.diegonmarcos.com/api/health -> 200 (3.6s)
  ✅ Firewall gcp-proxy             gcp-proxy: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall gcp-t4                gcp-t4: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-apps              oci-apps: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-mail              oci-mail: no unexpected dangerous ports exposed (3.0s)
  ✅ Firewall oci-analytics         oci-analytics: no unexpected dangerous ports exposed (3.0s)
  ✅ SSH ports gcp-proxy            gcp-proxy: SSH:22=open Dropbear:2200=closed (3.3s)
  ✅ SSH ports gcp-t4               gcp-t4: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (3.2s)
  ✅ SSH ports oci-apps             oci-apps: SSH:22=open Dropbear:2200=closed (3.1s)
  ✅ SSH ports oci-mail             oci-mail: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ SSH ports oci-analytics        oci-analytics: SSH:22=closed Dropbear:2200=closed [WG-only SSH - expected] (6.0s)
  ✅ Caddy TLS                      proxy.diegonmarcos.com -> 302 (1.0s)

  Summary: 19/19 passed, 0 failed

11. E2E EMAIL
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E email test)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    82.1s
  L10_security             44.7s
  L8_external              44.7s
  L11_email_e2e            44.7s
  L4-L11_parallel          44.7s
  L6_public_urls           44.7s
  L5_private_urls          44.7s
  L2_wg_mesh               29.3s
  L3_platform              4.7s
  L1_self_check            3.4s
  L4_containers            0.0s
  L9_drift                 0.0s
  L7_cross_checks          0.0s

  Total: 82.1s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(rsync+mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 163/252 passed, 4 critical, 56 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-health-full-report
Run: cargo run --release
```
