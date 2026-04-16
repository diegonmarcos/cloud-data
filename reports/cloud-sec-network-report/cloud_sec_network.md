```

  ███╗   ██╗███████╗████████╗██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗
  ████╗  ██║██╔════╝╚══██╔══╝██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝
  ██╔██╗ ██║█████╗     ██║   ██║ █╗ ██║██║   ██║██████╔╝█████╔╝
  ██║╚██╗██║██╔══╝     ██║   ██║███╗██║██║   ██║██╔══██╗██╔═██╗
  ██║ ╚████║███████╗   ██║   ╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗
  ╚═╝  ╚═══╝╚══════╝   ╚═╝    ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
  CLOUD SECURITY: NETWORK — 2026-04-16 11:04 UTC
══════════════════════════════════════════════════════════════

  ISSUES SUMMARY
══════════════════════════════════════════════════════════════
  29 issues: 16 critical, 13 warnings, 0 info

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
    ❌ wg:gcp-proxy: Failed to query WireGuard: SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Name or service not known
  WARNINGS:
    ⚠️  port-scan:gcp-proxy:undeclared: Undeclared open ports on 35.226.147.64: [22]
    ⚠️  port-scan:gcp-proxy:closed: Declared ports closed on 35.226.147.64: [80, 443, 443, 993, 465, 587]
    ⚠️  port-scan:oci-apps:closed: Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099]
    ⚠️  port-scan:oci-mail:closed: Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 22000, 21027]
    ⚠️  dns:MX:webmail.diegonmarcos.com: No MX records found
    ⚠️  dns:SPF:webmail.diegonmarcos.com: No SPF record
    ⚠️  dns:DMARC:webmail.diegonmarcos.com: No DMARC record
    ⚠️  dns:DMARC:mail.diegonmarcos.com: No DMARC record
    ⚠️  firewall:gcp-proxy: SSH unreachable, cannot audit firewall
    ⚠️  firewall:gcp-t4: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-apps: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-mail: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-analytics: SSH unreachable, cannot audit firewall

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
  ❌ tls:ide.diegonmarcos.com       TLS handshake failed for ide.diegonmarcos.com (0.6s) [CRITICAL]
  ❌ tls:sheets.diegonmarcos.com    TLS handshake failed for sheets.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:chat.diegonmarcos.com      TLS handshake failed for chat.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:photos.diegonmarcos.com    TLS handshake failed for photos.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:cal.diegonmarcos.com       TLS handshake failed for cal.diegonmarcos.com (0.9s) [CRITICAL]
  ❌ tls:webmail.diegonmarcos.com   TLS handshake failed for webmail.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:mail.diegonmarcos.com      TLS handshake failed for mail.diegonmarcos.com (0.8s) [CRITICAL]
  ❌ tls:smtp.diegonmarcos.com      TLS handshake failed for smtp.diegonmarcos.com (0.6s) [CRITICAL]
  ❌ tls:vault.diegonmarcos.com     TLS handshake failed for vault.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:auth.diegonmarcos.com      TLS handshake failed for auth.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:workflows.diegonmarcos.com TLS handshake failed for workflows.diegonmarcos.com (0.6s) [CRITICAL]
  ❌ tls:db.diegonmarcos.com        TLS handshake failed for db.diegonmarcos.com (0.6s) [CRITICAL]
  ❌ tls:grafana.diegonmarcos.com   TLS handshake failed for grafana.diegonmarcos.com (0.7s) [CRITICAL]
  ❌ tls:windmill.diegonmarcos.com  TLS handshake failed for windmill.diegonmarcos.com (0.6s) [CRITICAL]
  ❌ tls:git.diegonmarcos.com       TLS handshake failed for git.diegonmarcos.com (0.9s) [CRITICAL]

  Summary: 0/15 passed, 15 failed

3. WIREGUARD HEALTH
──────────────────────────────────────────────────────────────
  ❌ wg:gcp-proxy                   Failed to query WireGuard: SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Name or service not known [CRITICAL]

  Summary: 0/1 passed, 1 failed

4. DNS VALIDATION
──────────────────────────────────────────────────────────────
  ✅ dns:A:ide.diegonmarcos.com     A=35.226.147.64 (0.0s)
  ✅ dns:A:sheets.diegonmarcos.com  A=35.226.147.64 (0.0s)
  ✅ dns:A:chat.diegonmarcos.com    A=35.226.147.64 (0.1s)
  ✅ dns:A:photos.diegonmarcos.com  A=35.226.147.64 (0.0s)
  ✅ dns:A:cal.diegonmarcos.com     A=35.226.147.64 (0.1s)
  ✅ dns:A:webmail.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ dns:A:mail.diegonmarcos.com    A=35.226.147.64 (0.0s)
  ✅ dns:A:smtp.diegonmarcos.com    A=35.226.147.64 (0.0s)
  ✅ dns:A:vault.diegonmarcos.com   A=35.226.147.64 (0.0s)
  ✅ dns:A:auth.diegonmarcos.com    A=35.226.147.64 (0.0s)
  ✅ dns:A:workflows.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ dns:A:db.diegonmarcos.com      A=35.226.147.64 (0.0s)
  ✅ dns:A:grafana.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ dns:A:windmill.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ dns:A:git.diegonmarcos.com     A=35.226.147.64 (0.0s)
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
  ⚠️  firewall:gcp-proxy             SSH unreachable, cannot audit firewall (0.2s) [WARNING]
  ⚠️  firewall:gcp-t4                SSH unreachable, cannot audit firewall (0.2s) [WARNING]
  ⚠️  firewall:oci-apps              SSH unreachable, cannot audit firewall (0.2s) [WARNING]
  ⚠️  firewall:oci-mail              SSH unreachable, cannot audit firewall (0.1s) [WARNING]
  ⚠️  firewall:oci-analytics         SSH unreachable, cannot audit firewall (0.2s) [WARNING]

  Summary: 0/5 passed, 5 failed

══════════════════════════════════════════════════════════════
  PERFORMANCE
══════════════════════════════════════════════════════════════
  Port scan                15.0s
  TLS+DNS+WG (parallel)    10.5s
  Firewall audit           0.9s

  Total: 26.7s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 22/51 passed, 16 critical, 13 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-sec-network-report
Run: build.sh all
```
