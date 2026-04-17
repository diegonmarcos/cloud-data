```

  ███╗   ██╗███████╗████████╗██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗
  ████╗  ██║██╔════╝╚══██╔══╝██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝
  ██╔██╗ ██║█████╗     ██║   ██║ █╗ ██║██║   ██║██████╔╝█████╔╝
  ██║╚██╗██║██╔══╝     ██║   ██║███╗██║██║   ██║██╔══██╗██╔═██╗
  ██║ ╚████║███████╗   ██║   ╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗
  ╚═╝  ╚═══╝╚══════╝   ╚═╝    ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
  CLOUD SECURITY: NETWORK — 2026-04-16 21:17 UTC
══════════════════════════════════════════════════════════════

  ISSUES SUMMARY
══════════════════════════════════════════════════════════════
  20 issues: 2 critical, 18 warnings, 0 info

  CRITICAL:
    ❌ wg:gcp-t4: gcp-t4 (10.0.0.8) never connected
    ❌ wg:peer-1: peer-1 (10.0.0.200) handshake 114558s ago, rx=8.1MiB tx=20.4MiB
  WARNINGS:
    ⚠️  ext:port-scan:gcp-proxy:undeclared: Undeclared open ports on 35.226.147.64: [22]
    ⚠️  ext:port-scan:gcp-proxy:closed: Declared ports closed on 35.226.147.64: [80]
    ⚠️  ext:port-scan:oci-apps:closed: Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099]
    ⚠️  ext:port-scan:oci-mail:closed: Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027]
    ⚠️  int:port-scan:gcp-proxy:closed: WG 10.0.0.1 — service ports unreachable: [6380]
    ⚠️  int:port-scan:gcp-t4:closed: WG 10.0.0.8 — service ports unreachable: [22, 11434]
    ⚠️  int:port-scan:oci-apps:closed: WG 10.0.0.6 — service ports unreachable: [5002, 3014, 3001, 5437, 8890, 5001, 3105]
    ⚠️  int:port-scan:oci-mail:closed: WG 10.0.0.3 — service ports unreachable: [443]
    ⚠️  int:port-scan:oci-analytics:closed: WG 10.0.0.4 — service ports unreachable: [2020, 8084, 5442]
    ⚠️  ext:dns:MX:webmail.diegonmarcos.com: No MX records found
    ⚠️  ext:dns:SPF:webmail.diegonmarcos.com: No SPF record
    ⚠️  ext:dns:DMARC:webmail.diegonmarcos.com: No DMARC record
    ⚠️  ext:dns:DMARC:mail.diegonmarcos.com: No DMARC record
    ⚠️  firewall:gcp-proxy:rogue: Rogue listeners on 0.0.0.0: [4182, 5433, 5434, 5435, 5436, 7680, 7681, 8090, 8198, 8199, 8880]
    ⚠️  firewall:gcp-t4: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-apps:rogue: Rogue listeners on 0.0.0.0: [3002, 3011, 3012, 3013, 3018, 3019, 3080, 3100, 3110, 3200, 3210, 3306, 5433, 5435, 5436, 5439, 5440, 5443, 6381, 7680, 7946, 8000, 8065, 8086, 8090, 8443, 8889, 9000, 9001, 9009, 9111, 9112, 9113, 35077]
    ⚠️  firewall:oci-mail:rogue: Rogue listeners on 0.0.0.0: [143, 8070, 8199]
    ⚠️  firewall:oci-analytics:rogue: Rogue listeners on 0.0.0.0: [7680, 8070, 8198, 8199, 9999]

1. PORT SCAN
──────────────────────────────────────────────────────────────
  ⚠️  ext:port-scan:gcp-proxy:undeclared Undeclared open ports on 35.226.147.64: [22] (3.0s) [WARNING]
  ⚠️  ext:port-scan:gcp-proxy:closed Declared ports closed on 35.226.147.64: [80] (3.0s) [WARNING]
  ✅ ext:port-scan:gcp-t4           34.173.227.250 open=[] (all declared) (3.0s)
  ⚠️  ext:port-scan:oci-apps:closed  Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099] (3.0s) [WARNING]
  ⚠️  ext:port-scan:oci-mail:closed  Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027] (3.0s) [WARNING]
  ✅ ext:port-scan:oci-analytics    129.151.228.66 open=[] (all declared) (3.0s)
  ⚠️  int:port-scan:gcp-proxy:closed WG 10.0.0.1 — service ports unreachable: [6380] (0.1s) [WARNING]
  ⚠️  int:port-scan:gcp-t4:closed    WG 10.0.0.8 — service ports unreachable: [22, 11434] (3.0s) [WARNING]
  ⚠️  int:port-scan:oci-apps:closed  WG 10.0.0.6 — service ports unreachable: [5002, 3014, 3001, 5437, 8890, 5001, 3105] (0.2s) [WARNING]
  ⚠️  int:port-scan:oci-mail:closed  WG 10.0.0.3 — service ports unreachable: [443] (0.2s) [WARNING]
  ⚠️  int:port-scan:oci-analytics:closed WG 10.0.0.4 — service ports unreachable: [2020, 8084, 5442] (0.2s) [WARNING]

  Summary: 2/11 passed, 9 failed

2. TLS CERTIFICATE AUDIT
──────────────────────────────────────────────────────────────
  ✅ ext:tls:ide.diegonmarcos.com   ide.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:sheets.diegonmarcos.com sheets.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:chat.diegonmarcos.com  chat.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:photos.diegonmarcos.com photos.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:cal.diegonmarcos.com   cal.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:webmail.diegonmarcos.com webmail.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:mail.diegonmarcos.com  mail.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:smtp.diegonmarcos.com  smtp.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:vault.diegonmarcos.com vault.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:auth.diegonmarcos.com  auth.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:workflows.diegonmarcos.com workflows.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:db.diegonmarcos.com    db.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:grafana.diegonmarcos.com grafana.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:windmill.diegonmarcos.com windmill.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ ext:tls:git.diegonmarcos.com   git.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (74 days) (0.6s)
  ✅ int:tls:ide.diegonmarcos.com   WG 10.0.0.6:8443 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:sheets.diegonmarcos.com WG 10.0.0.6:3011 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:chat.diegonmarcos.com  WG 10.0.0.6:8065 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:photos.diegonmarcos.com WG 10.0.0.6:3013 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:cal.diegonmarcos.com   WG 10.0.0.6:5232 — plain HTTP (Caddy terminates TLS) (0.6s)
  ✅ int:tls:webmail.diegonmarcos.com WG 10.0.0.3:8888 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:mail.diegonmarcos.com  WG 10.0.0.3:443 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:smtp.diegonmarcos.com  WG 10.0.0.3:8080 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:vault.diegonmarcos.com WG 10.0.0.6:8880 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:auth.diegonmarcos.com  WG 10.0.0.1:9091 — plain HTTP (Caddy terminates TLS) (0.4s)
  ✅ int:tls:workflows.diegonmarcos.com WG 10.0.0.4:8070 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:db.diegonmarcos.com    WG 10.0.0.6:8086 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:grafana.diegonmarcos.com WG 10.0.0.6:3200 — plain HTTP (Caddy terminates TLS) (1.4s)
  ✅ int:tls:windmill.diegonmarcos.com WG 10.0.0.6:8000 — plain HTTP (Caddy terminates TLS) (0.5s)
  ✅ int:tls:git.diegonmarcos.com   WG 10.0.0.6:3002 — plain HTTP (Caddy terminates TLS) (0.5s)

  Summary: 30/30 passed, 0 failed

3. WIREGUARD HEALTH
──────────────────────────────────────────────────────────────
  ❌ wg:gcp-t4                      gcp-t4 (10.0.0.8) never connected [CRITICAL]
  ❌ wg:peer-1                      peer-1 (10.0.0.200) handshake 114558s ago, rx=8.1MiB tx=20.4MiB [CRITICAL]
  ✅ wg:oci-analytics               oci-analytics (10.0.0.4) handshake 96s ago, rx=59.2MiB tx=9.4MiB
  ✅ wg:oci-apps                    oci-apps (10.0.0.6) handshake 18s ago, rx=486.3MiB tx=108.0MiB
  ✅ wg:oci-mail                    oci-mail (10.0.0.3) handshake 8s ago, rx=189.0MiB tx=37.7MiB
  ✅ wg:peer-2                      peer-2 (10.0.0.5) handshake 113s ago, rx=40.8MiB tx=833.1MiB
  ✅ wg:peer-3                      peer-3 (10.0.0.9) handshake 104s ago, rx=4.0MiB tx=3.6MiB

  Summary: 5/7 passed, 2 failed

4. DNS VALIDATION
──────────────────────────────────────────────────────────────
  ✅ ext:dns:A:ide.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:sheets.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:chat.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:photos.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:cal.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:webmail.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:mail.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:smtp.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:vault.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:auth.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:workflows.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:db.diegonmarcos.com  A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:grafana.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:windmill.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:git.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ int:dns:A:ide.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:sheets.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:chat.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:photos.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:cal.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:webmail.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:mail.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:smtp.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:vault.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:auth.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:workflows.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:db.diegonmarcos.com  Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:grafana.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:windmill.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ✅ int:dns:A:git.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.1s)
  ⚠️  ext:dns:MX:webmail.diegonmarcos.com No MX records found (0.0s) [WARNING]
  ⚠️  ext:dns:SPF:webmail.diegonmarcos.com No SPF record (0.0s) [WARNING]
  ⚠️  ext:dns:DMARC:webmail.diegonmarcos.com No DMARC record (0.0s) [WARNING]
  ✅ ext:dns:MX:mail.diegonmarcos.com MX=10 feedback-smtp.us-east-1.amazonses.com. (0.0s)
  ✅ ext:dns:SPF:mail.diegonmarcos.com SPF record present (0.0s)
  ⚠️  ext:dns:DMARC:mail.diegonmarcos.com No DMARC record (0.0s) [WARNING]
  ✅ ext:dns:MX:diegonmarcos.com    MX=22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ ext:dns:SPF:diegonmarcos.com   SPF record present (0.0s)
  ✅ ext:dns:DMARC:diegonmarcos.com DMARC record present (0.0s)

  Summary: 35/39 passed, 4 failed

5. FIREWALL AUDIT
──────────────────────────────────────────────────────────────
  ⚠️  firewall:gcp-proxy:rogue       Rogue listeners on 0.0.0.0: [4182, 5433, 5434, 5435, 5436, 7680, 7681, 8090, 8198, 8199, 8880] (0.7s) [WARNING]
  ⚠️  firewall:gcp-t4                SSH unreachable, cannot audit firewall (5.0s) [WARNING]
  ⚠️  firewall:oci-apps:rogue        Rogue listeners on 0.0.0.0: [3002, 3011, 3012, 3013, 3018, 3019, 3080, 3100, 3110, 3200, 3210, 3306, 5433, 5435, 5436, 5439, 5440, 5443, 6381, 7680, 7946, 8000, 8065, 8086, 8090, 8443, 8889, 9000, 9001, 9009, 9111, 9112, 9113, 35077] (3.7s) [WARNING]
  ⚠️  firewall:oci-mail:rogue        Rogue listeners on 0.0.0.0: [143, 8070, 8199] (3.6s) [WARNING]
  ⚠️  firewall:oci-analytics:rogue   Rogue listeners on 0.0.0.0: [7680, 8070, 8198, 8199, 9999] (3.8s) [WARNING]

  Summary: 0/5 passed, 5 failed

══════════════════════════════════════════════════════════════
  PERFORMANCE
══════════════════════════════════════════════════════════════
  Port scan                18.9s
  TLS+DNS+WG (parallel)    17.3s
  Firewall audit           16.8s

  Total: 53.2s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 72/92 passed, 2 critical, 18 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-sec-network-report
Run: build.sh all
```
