```

  ███╗   ██╗███████╗████████╗██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗
  ████╗  ██║██╔════╝╚══██╔══╝██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝
  ██╔██╗ ██║█████╗     ██║   ██║ █╗ ██║██║   ██║██████╔╝█████╔╝
  ██║╚██╗██║██╔══╝     ██║   ██║███╗██║██║   ██║██╔══██╗██╔═██╗
  ██║ ╚████║███████╗   ██║   ╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗
  ╚═╝  ╚═══╝╚══════╝   ╚═╝    ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
  CLOUD SECURITY: NETWORK — 2026-04-16 11:53 UTC
══════════════════════════════════════════════════════════════

  ISSUES SUMMARY
══════════════════════════════════════════════════════════════
  35 issues: 17 critical, 18 warnings, 0 info

  CRITICAL:
    ❌ ext:tls:ide.diegonmarcos.com: TLS handshake failed for ide.diegonmarcos.com
    ❌ ext:tls:sheets.diegonmarcos.com: TLS handshake failed for sheets.diegonmarcos.com
    ❌ ext:tls:chat.diegonmarcos.com: TLS handshake failed for chat.diegonmarcos.com
    ❌ ext:tls:photos.diegonmarcos.com: TLS handshake failed for photos.diegonmarcos.com
    ❌ ext:tls:cal.diegonmarcos.com: TLS handshake failed for cal.diegonmarcos.com
    ❌ ext:tls:webmail.diegonmarcos.com: TLS handshake failed for webmail.diegonmarcos.com
    ❌ ext:tls:mail.diegonmarcos.com: TLS handshake failed for mail.diegonmarcos.com
    ❌ ext:tls:smtp.diegonmarcos.com: TLS handshake failed for smtp.diegonmarcos.com
    ❌ ext:tls:vault.diegonmarcos.com: TLS handshake failed for vault.diegonmarcos.com
    ❌ ext:tls:auth.diegonmarcos.com: TLS handshake failed for auth.diegonmarcos.com
    ❌ ext:tls:workflows.diegonmarcos.com: TLS handshake failed for workflows.diegonmarcos.com
    ❌ ext:tls:db.diegonmarcos.com: TLS handshake failed for db.diegonmarcos.com
    ❌ ext:tls:grafana.diegonmarcos.com: TLS handshake failed for grafana.diegonmarcos.com
    ❌ ext:tls:windmill.diegonmarcos.com: TLS handshake failed for windmill.diegonmarcos.com
    ❌ ext:tls:git.diegonmarcos.com: TLS handshake failed for git.diegonmarcos.com
    ❌ wg:gcp-t4: gcp-t4 (10.0.0.8) never connected
    ❌ wg:peer-1: peer-1 (10.0.0.200) handshake 80737s ago, rx=8.1MiB tx=19.6MiB
  WARNINGS:
    ⚠️  ext:port-scan:gcp-proxy:undeclared: Undeclared open ports on 35.226.147.64: [22]
    ⚠️  ext:port-scan:gcp-proxy:closed: Declared ports closed on 35.226.147.64: [80, 443, 443, 993, 465, 587]
    ⚠️  ext:port-scan:oci-apps:closed: Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099]
    ⚠️  ext:port-scan:oci-mail:closed: Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027]
    ⚠️  int:port-scan:gcp-proxy:closed: WG 10.0.0.1 — service ports unreachable: [443]
    ⚠️  int:port-scan:gcp-t4:closed: WG 10.0.0.8 — service ports unreachable: [22, 11434]
    ⚠️  int:port-scan:oci-apps:closed: WG 10.0.0.6 — service ports unreachable: [8443, 3012, 5436, 3015, 3104, 3011, 3018, 5439, 3103, 8065, 5435, 3102, 3013, 5002, 5232, 3014, 8880, 3000, 3001, 9000, 6381, 5437, 8890, 5443, 5001, 8889, 11435, 8091, 8081, 3100, 8082, 3101, 3105, 3080, 8000, 5440, 3002]
    ⚠️  int:port-scan:oci-mail:closed: WG 10.0.0.3 — service ports unreachable: [443]
    ⚠️  int:port-scan:oci-analytics:closed: WG 10.0.0.4 — service ports unreachable: [2020, 8084, 3006, 5442]
    ⚠️  ext:dns:MX:webmail.diegonmarcos.com: No MX records found
    ⚠️  ext:dns:SPF:webmail.diegonmarcos.com: No SPF record
    ⚠️  ext:dns:DMARC:webmail.diegonmarcos.com: No DMARC record
    ⚠️  ext:dns:DMARC:mail.diegonmarcos.com: No DMARC record
    ⚠️  firewall:gcp-proxy:rogue: Rogue listeners on 0.0.0.0: [4182, 5355, 5433, 5434, 5435, 5436, 6379, 6380, 7680, 7681, 8090, 8198, 8199, 8880, 8890, 8891, 8892, 8893, 9091]
    ⚠️  firewall:gcp-t4: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-apps:rogue: Rogue listeners on 0.0.0.0: [111, 3019, 3110, 3200, 3210, 5433, 5441, 7680, 7681, 7946, 8085, 8086, 8090, 9009, 9111, 9112, 9113]
    ⚠️  firewall:oci-mail:rogue: Rogue listeners on 0.0.0.0: [111, 143, 7681, 8070, 8088, 8199, 8888, 9000]
    ⚠️  firewall:oci-analytics:rogue: Rogue listeners on 0.0.0.0: [111, 7680, 8070, 8198, 8199, 9999]

1. PORT SCAN
──────────────────────────────────────────────────────────────
  ⚠️  ext:port-scan:gcp-proxy:undeclared Undeclared open ports on 35.226.147.64: [22] (3.0s) [WARNING]
  ⚠️  ext:port-scan:gcp-proxy:closed Declared ports closed on 35.226.147.64: [80, 443, 443, 993, 465, 587] (3.0s) [WARNING]
  ✅ ext:port-scan:gcp-t4           34.173.227.250 open=[] (all declared) (3.0s)
  ⚠️  ext:port-scan:oci-apps:closed  Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099] (3.0s) [WARNING]
  ⚠️  ext:port-scan:oci-mail:closed  Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027] (3.0s) [WARNING]
  ✅ ext:port-scan:oci-analytics    129.151.228.66 open=[] (all declared) (3.0s)
  ⚠️  int:port-scan:gcp-proxy:closed WG 10.0.0.1 — service ports unreachable: [443] (0.1s) [WARNING]
  ⚠️  int:port-scan:gcp-t4:closed    WG 10.0.0.8 — service ports unreachable: [22, 11434] (3.0s) [WARNING]
  ⚠️  int:port-scan:oci-apps:closed  WG 10.0.0.6 — service ports unreachable: [8443, 3012, 5436, 3015, 3104, 3011, 3018, 5439, 3103, 8065, 5435, 3102, 3013, 5002, 5232, 3014, 8880, 3000, 3001, 9000, 6381, 5437, 8890, 5443, 5001, 8889, 11435, 8091, 8081, 3100, 8082, 3101, 3105, 3080, 8000, 5440, 3002] (0.3s) [WARNING]
  ⚠️  int:port-scan:oci-mail:closed  WG 10.0.0.3 — service ports unreachable: [443] (0.2s) [WARNING]
  ⚠️  int:port-scan:oci-analytics:closed WG 10.0.0.4 — service ports unreachable: [2020, 8084, 3006, 5442] (0.2s) [WARNING]

  Summary: 2/11 passed, 9 failed

2. TLS CERTIFICATE AUDIT
──────────────────────────────────────────────────────────────
  ❌ ext:tls:ide.diegonmarcos.com   TLS handshake failed for ide.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:sheets.diegonmarcos.com TLS handshake failed for sheets.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:chat.diegonmarcos.com  TLS handshake failed for chat.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:photos.diegonmarcos.com TLS handshake failed for photos.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:cal.diegonmarcos.com   TLS handshake failed for cal.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:webmail.diegonmarcos.com TLS handshake failed for webmail.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:mail.diegonmarcos.com  TLS handshake failed for mail.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:smtp.diegonmarcos.com  TLS handshake failed for smtp.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:vault.diegonmarcos.com TLS handshake failed for vault.diegonmarcos.com (0.4s) [CRITICAL]
  ❌ ext:tls:auth.diegonmarcos.com  TLS handshake failed for auth.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:workflows.diegonmarcos.com TLS handshake failed for workflows.diegonmarcos.com (0.5s) [CRITICAL]
  ❌ ext:tls:db.diegonmarcos.com    TLS handshake failed for db.diegonmarcos.com (0.3s) [CRITICAL]
  ❌ ext:tls:grafana.diegonmarcos.com TLS handshake failed for grafana.diegonmarcos.com (0.4s) [CRITICAL]
  ❌ ext:tls:windmill.diegonmarcos.com TLS handshake failed for windmill.diegonmarcos.com (0.4s) [CRITICAL]
  ❌ ext:tls:git.diegonmarcos.com   TLS handshake failed for git.diegonmarcos.com (0.3s) [CRITICAL]
  ✅ int:tls:ide.diegonmarcos.com   WG 10.0.0.6:8443 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:sheets.diegonmarcos.com WG 10.0.0.6:3011 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:chat.diegonmarcos.com  WG 10.0.0.6:8065 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:photos.diegonmarcos.com WG 10.0.0.6:3013 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:cal.diegonmarcos.com   WG 10.0.0.6:5232 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:webmail.diegonmarcos.com WG 10.0.0.3:8888 — plain HTTP (Caddy terminates TLS) (0.6s)
  ✅ int:tls:mail.diegonmarcos.com  WG 10.0.0.3:443 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:smtp.diegonmarcos.com  WG 10.0.0.3:8080 — plain HTTP (Caddy terminates TLS) (0.6s)
  ✅ int:tls:vault.diegonmarcos.com WG 10.0.0.6:8880 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:auth.diegonmarcos.com  WG 10.0.0.1:9091 — plain HTTP (Caddy terminates TLS) (0.4s)
  ✅ int:tls:workflows.diegonmarcos.com WG 10.0.0.4:8070 — plain HTTP (Caddy terminates TLS) (0.6s)
  ✅ int:tls:db.diegonmarcos.com    WG 10.0.0.6:8086 — plain HTTP (Caddy terminates TLS) (0.6s)
  ✅ int:tls:grafana.diegonmarcos.com WG 10.0.0.6:3200 — plain HTTP (Caddy terminates TLS) (0.6s)
  ✅ int:tls:windmill.diegonmarcos.com WG 10.0.0.6:8000 — plain HTTP (Caddy terminates TLS) (0.3s)
  ✅ int:tls:git.diegonmarcos.com   WG 10.0.0.6:3002 — plain HTTP (Caddy terminates TLS) (0.5s)

  Summary: 15/30 passed, 15 failed

3. WIREGUARD HEALTH
──────────────────────────────────────────────────────────────
  ❌ wg:gcp-t4                      gcp-t4 (10.0.0.8) never connected [CRITICAL]
  ❌ wg:peer-1                      peer-1 (10.0.0.200) handshake 80737s ago, rx=8.1MiB tx=19.6MiB [CRITICAL]
  ✅ wg:oci-analytics               oci-analytics (10.0.0.4) handshake 16s ago, rx=39.3MiB tx=6.4MiB
  ✅ wg:oci-apps                    oci-apps (10.0.0.6) handshake 117s ago, rx=404.6MiB tx=87.0MiB
  ✅ wg:oci-mail                    oci-mail (10.0.0.3) handshake 100s ago, rx=133.1MiB tx=35.3MiB
  ✅ wg:peer-2                      peer-2 (10.0.0.5) handshake 78s ago, rx=15.5MiB tx=625.3MiB
  ✅ wg:peer-3                      peer-3 (10.0.0.9) handshake 150s ago, rx=3.9MiB tx=3.5MiB

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
  ✅ ext:dns:A:workflows.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:db.diegonmarcos.com  A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:grafana.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:windmill.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:git.diegonmarcos.com A=35.226.147.64 (0.0s)
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
  ✅ int:dns:A:windmill.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.2s)
  ✅ int:dns:A:git.diegonmarcos.com Hickory A=35.226.147.64 (public=35.226.147.64) (0.2s)
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
  ⚠️  firewall:gcp-proxy:rogue       Rogue listeners on 0.0.0.0: [4182, 5355, 5433, 5434, 5435, 5436, 6379, 6380, 7680, 7681, 8090, 8198, 8199, 8880, 8890, 8891, 8892, 8893, 9091] (0.8s) [WARNING]
  ⚠️  firewall:gcp-t4                SSH unreachable, cannot audit firewall (5.0s) [WARNING]
  ⚠️  firewall:oci-apps:rogue        Rogue listeners on 0.0.0.0: [111, 3019, 3110, 3200, 3210, 5433, 5441, 7680, 7681, 7946, 8085, 8086, 8090, 9009, 9111, 9112, 9113] (4.4s) [WARNING]
  ⚠️  firewall:oci-mail:rogue        Rogue listeners on 0.0.0.0: [111, 143, 7681, 8070, 8088, 8199, 8888, 9000] (4.1s) [WARNING]
  ⚠️  firewall:oci-analytics:rogue   Rogue listeners on 0.0.0.0: [111, 7680, 8070, 8198, 8199, 9999] (4.0s) [WARNING]

  Summary: 0/5 passed, 5 failed

══════════════════════════════════════════════════════════════
  PERFORMANCE
══════════════════════════════════════════════════════════════
  Port scan                18.9s
  Firewall audit           18.3s
  TLS+DNS+WG (parallel)    11.4s

  Total: 48.9s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 57/92 passed, 17 critical, 18 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-sec-network-report
Run: build.sh all
```
