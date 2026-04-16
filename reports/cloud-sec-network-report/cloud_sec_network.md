```

  ███╗   ██╗███████╗████████╗██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗
  ████╗  ██║██╔════╝╚══██╔══╝██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝
  ██╔██╗ ██║█████╗     ██║   ██║ █╗ ██║██║   ██║██████╔╝█████╔╝
  ██║╚██╗██║██╔══╝     ██║   ██║███╗██║██║   ██║██╔══██╗██╔═██╗
  ██║ ╚████║███████╗   ██║   ╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗
  ╚═╝  ╚═══╝╚══════╝   ╚═╝    ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
  CLOUD SECURITY: NETWORK — 2026-04-16 10:41 UTC
══════════════════════════════════════════════════════════════

  ISSUES SUMMARY
══════════════════════════════════════════════════════════════
  30 issues: 17 critical, 13 warnings, 0 info

  CRITICAL:
    ❌ tls:ide.diegonmarcos.com: TLS handshake failed for ide.diegonmarcos.com
    ❌ tls:sheets.diegonmarcos.com: TLS handshake failed for sheets.diegonmarcos.com
    ❌ tls:chat.diegonmarcos.com: TLS handshake failed for chat.diegonmarcos.com
    ❌ tls:photos.diegonmarcos.com: TLS handshake failed for photos.diegonmarcos.com
    ❌ tls:cal.diegonmarcos.com: TLS handshake failed for cal.diegonmarcos.com
    ❌ tls:webmail.diegonmarcos.com: TLS handshake failed for webmail.diegonmarcos.com
    ❌ tls:mail.diegonmarcos.com: TLS handshake failed for mail.diegonmarcos.com
    ❌ tls:smtp.diegonmarcos.com: TLS handshake failed for smtp.diegonmarcos.com
    ❌ tls:vault.diegonmarcos.com: TLS handshake failed for vault.diegonmarcos.com
    ❌ tls:auth.diegonmarcos.com: TLS handshake failed for auth.diegonmarcos.com
    ❌ tls:workflows.diegonmarcos.com: TLS handshake failed for workflows.diegonmarcos.com
    ❌ tls:db.diegonmarcos.com: TLS handshake failed for db.diegonmarcos.com
    ❌ tls:grafana.diegonmarcos.com: TLS handshake failed for grafana.diegonmarcos.com
    ❌ tls:windmill.diegonmarcos.com: TLS handshake failed for windmill.diegonmarcos.com
    ❌ tls:git.diegonmarcos.com: TLS handshake failed for git.diegonmarcos.com
    ❌ wg:gcp-t4: gcp-t4 (10.0.0.8) never connected
    ❌ wg:peer-1: peer-1 (10.0.0.200) handshake 76387s ago, rx=8.1MiB tx=19.5MiB
  WARNINGS:
    ⚠️  port-scan:gcp-proxy:undeclared: Undeclared open ports on 35.226.147.64: [22]
    ⚠️  port-scan:gcp-proxy:closed: Declared ports closed on 35.226.147.64: [80, 443, 443, 993, 465, 587]
    ⚠️  port-scan:oci-apps:closed: Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099]
    ⚠️  port-scan:oci-mail:closed: Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027]
    ⚠️  dns:MX:webmail.diegonmarcos.com: No MX records found
    ⚠️  dns:SPF:webmail.diegonmarcos.com: No SPF record
    ⚠️  dns:DMARC:webmail.diegonmarcos.com: No DMARC record
    ⚠️  dns:DMARC:mail.diegonmarcos.com: No DMARC record
    ⚠️  firewall:gcp-proxy:rogue: Rogue listeners on 0.0.0.0: [5355, 5433, 5434, 5435, 5436, 6379, 6380, 7680, 7681, 8090, 8198, 8199, 8880, 8890, 8891, 8892, 8893, 9091]
    ⚠️  firewall:gcp-t4: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-apps:rogue: Rogue listeners on 0.0.0.0: [111, 3019, 3110, 3200, 3210, 5433, 5441, 7680, 7681, 7946, 8085, 8086, 8090, 8198, 8199, 9009, 9111, 9112, 9113]
    ⚠️  firewall:oci-mail:rogue: Rogue listeners on 0.0.0.0: [111, 143, 7681, 8070, 8088, 8199, 8888, 9000]
    ⚠️  firewall:oci-analytics:rogue: Rogue listeners on 0.0.0.0: [111, 7680, 8070, 8198, 8199, 9999]

1. PORT SCAN
──────────────────────────────────────────────────────────────
  ⚠️  port-scan:gcp-proxy:undeclared Undeclared open ports on 35.226.147.64: [22] (3.0s) [WARNING]
  ⚠️  port-scan:gcp-proxy:closed     Declared ports closed on 35.226.147.64: [80, 443, 443, 993, 465, 587] (3.0s) [WARNING]
  ✅ port-scan:gcp-t4               34.173.227.250 open=[] (all declared) (3.0s)
  ⚠️  port-scan:oci-apps:closed      Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099] (3.0s) [WARNING]
  ⚠️  port-scan:oci-mail:closed      Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027] (3.0s) [WARNING]
  ✅ port-scan:oci-analytics        129.151.228.66 open=[] (all declared) (3.0s)

  Summary: 2/6 passed, 4 failed

2. TLS CERTIFICATE AUDIT
──────────────────────────────────────────────────────────────
  ❌ tls:ide.diegonmarcos.com       TLS handshake failed for ide.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:sheets.diegonmarcos.com    TLS handshake failed for sheets.diegonmarcos.com (0.9s) [CRITICAL]
  ❌ tls:chat.diegonmarcos.com      TLS handshake failed for chat.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:photos.diegonmarcos.com    TLS handshake failed for photos.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:cal.diegonmarcos.com       TLS handshake failed for cal.diegonmarcos.com (0.9s) [CRITICAL]
  ❌ tls:webmail.diegonmarcos.com   TLS handshake failed for webmail.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:mail.diegonmarcos.com      TLS handshake failed for mail.diegonmarcos.com (0.9s) [CRITICAL]
  ❌ tls:smtp.diegonmarcos.com      TLS handshake failed for smtp.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:vault.diegonmarcos.com     TLS handshake failed for vault.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:auth.diegonmarcos.com      TLS handshake failed for auth.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:workflows.diegonmarcos.com TLS handshake failed for workflows.diegonmarcos.com (0.8s) [CRITICAL]
  ❌ tls:db.diegonmarcos.com        TLS handshake failed for db.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:grafana.diegonmarcos.com   TLS handshake failed for grafana.diegonmarcos.com (0.6s) [CRITICAL]
  ❌ tls:windmill.diegonmarcos.com  TLS handshake failed for windmill.diegonmarcos.com (0.9s) [CRITICAL]
  ❌ tls:git.diegonmarcos.com       TLS handshake failed for git.diegonmarcos.com (0.6s) [CRITICAL]

  Summary: 0/15 passed, 15 failed

3. WIREGUARD HEALTH
──────────────────────────────────────────────────────────────
  ❌ wg:gcp-t4                      gcp-t4 (10.0.0.8) never connected [CRITICAL]
  ❌ wg:peer-1                      peer-1 (10.0.0.200) handshake 76387s ago, rx=8.1MiB tx=19.5MiB [CRITICAL]
  ✅ wg:oci-analytics               oci-analytics (10.0.0.4) handshake 68s ago, rx=5.8MiB tx=6.0MiB
  ✅ wg:oci-apps                    oci-apps (10.0.0.6) handshake 40s ago, rx=209.3MiB tx=85.0MiB
  ✅ wg:oci-mail                    oci-mail (10.0.0.3) handshake 147s ago, rx=29.7MiB tx=34.5MiB
  ✅ wg:peer-2                      peer-2 (10.0.0.5) handshake 61s ago, rx=10.1MiB tx=189.9MiB
  ✅ wg:peer-3                      peer-3 (10.0.0.9) handshake 165s ago, rx=3.9MiB tx=3.5MiB

  Summary: 5/7 passed, 2 failed

4. DNS VALIDATION
──────────────────────────────────────────────────────────────
  ✅ dns:A:ide.diegonmarcos.com     A=35.226.147.64 (0.0s)
  ✅ dns:A:sheets.diegonmarcos.com  A=35.226.147.64 (0.1s)
  ✅ dns:A:chat.diegonmarcos.com    A=35.226.147.64 (0.1s)
  ✅ dns:A:photos.diegonmarcos.com  A=35.226.147.64 (0.1s)
  ✅ dns:A:cal.diegonmarcos.com     A=35.226.147.64 (0.1s)
  ✅ dns:A:webmail.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ dns:A:mail.diegonmarcos.com    A=35.226.147.64 (0.0s)
  ✅ dns:A:smtp.diegonmarcos.com    A=35.226.147.64 (0.1s)
  ✅ dns:A:vault.diegonmarcos.com   A=35.226.147.64 (0.0s)
  ✅ dns:A:auth.diegonmarcos.com    A=35.226.147.64 (0.0s)
  ✅ dns:A:workflows.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ dns:A:db.diegonmarcos.com      A=35.226.147.64 (0.1s)
  ✅ dns:A:grafana.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ dns:A:windmill.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ dns:A:git.diegonmarcos.com     A=35.226.147.64 (0.1s)
  ⚠️  dns:MX:webmail.diegonmarcos.com No MX records found (0.0s) [WARNING]
  ⚠️  dns:SPF:webmail.diegonmarcos.com No SPF record (0.0s) [WARNING]
  ⚠️  dns:DMARC:webmail.diegonmarcos.com No DMARC record (0.0s) [WARNING]
  ✅ dns:MX:mail.diegonmarcos.com   MX=10 feedback-smtp.us-east-1.amazonses.com. (0.0s)
  ✅ dns:SPF:mail.diegonmarcos.com  SPF record present (0.0s)
  ⚠️  dns:DMARC:mail.diegonmarcos.com No DMARC record (0.0s) [WARNING]
  ✅ dns:MX:diegonmarcos.com        MX=22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ dns:SPF:diegonmarcos.com       SPF record present (0.0s)
  ✅ dns:DMARC:diegonmarcos.com     DMARC record present (0.0s)

  Summary: 20/24 passed, 4 failed

5. FIREWALL AUDIT
──────────────────────────────────────────────────────────────
  ⚠️  firewall:gcp-proxy:rogue       Rogue listeners on 0.0.0.0: [5355, 5433, 5434, 5435, 5436, 6379, 6380, 7680, 7681, 8090, 8198, 8199, 8880, 8890, 8891, 8892, 8893, 9091] (0.6s) [WARNING]
  ⚠️  firewall:gcp-t4                SSH unreachable, cannot audit firewall (5.0s) [WARNING]
  ⚠️  firewall:oci-apps:rogue        Rogue listeners on 0.0.0.0: [111, 3019, 3110, 3200, 3210, 5433, 5441, 7680, 7681, 7946, 8085, 8086, 8090, 8198, 8199, 9009, 9111, 9112, 9113] (3.7s) [WARNING]
  ⚠️  firewall:oci-mail:rogue        Rogue listeners on 0.0.0.0: [111, 143, 7681, 8070, 8088, 8199, 8888, 9000] (3.6s) [WARNING]
  ⚠️  firewall:oci-analytics:rogue   Rogue listeners on 0.0.0.0: [111, 7680, 8070, 8198, 8199, 9999] (8.3s) [WARNING]

  Summary: 0/5 passed, 5 failed

══════════════════════════════════════════════════════════════
  PERFORMANCE
══════════════════════════════════════════════════════════════
  Firewall audit           21.3s
  Port scan                15.0s
  TLS+DNS+WG (parallel)    11.3s

  Total: 48.0s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 27/57 passed, 17 critical, 13 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-sec-network-report
Run: build.sh all
```
