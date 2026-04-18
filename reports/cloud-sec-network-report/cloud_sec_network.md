```

  ███╗   ██╗███████╗████████╗██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗
  ████╗  ██║██╔════╝╚══██╔══╝██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝
  ██╔██╗ ██║█████╗     ██║   ██║ █╗ ██║██║   ██║██████╔╝█████╔╝
  ██║╚██╗██║██╔══╝     ██║   ██║███╗██║██║   ██║██╔══██╗██╔═██╗
  ██║ ╚████║███████╗   ██║   ╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗
  ╚═╝  ╚═══╝╚══════╝   ╚═╝    ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
  CLOUD SECURITY: NETWORK — 2026-04-18 13:08 UTC
══════════════════════════════════════════════════════════════

  ISSUES SUMMARY
══════════════════════════════════════════════════════════════
  16 issues: 1 critical, 15 warnings, 0 info

  CRITICAL:
    ❌ wg:gcp-proxy: Failed to query WireGuard: SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Temporary failure in name resolution
  WARNINGS:
    ⚠️  ext:port-scan:gcp-proxy:undeclared: Undeclared open ports on 35.226.147.64: [22]
    ⚠️  ext:port-scan:gcp-proxy:closed: Declared ports closed on 35.226.147.64: [80]
    ⚠️  ext:port-scan:oci-apps:closed: Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099]
    ⚠️  ext:port-scan:oci-mail:closed: Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 2025, 2465, 2587, 2993, 2443, 6190, 22000, 21027]
    ⚠️  ext:dns:MX:webmail.diegonmarcos.com: No MX records found
    ⚠️  ext:dns:SPF:webmail.diegonmarcos.com: No SPF record
    ⚠️  ext:dns:DMARC:webmail.diegonmarcos.com: No DMARC record
    ⚠️  ext:dns:MX:mail-stalwart.diegonmarcos.com: No MX records found
    ⚠️  ext:dns:SPF:mail-stalwart.diegonmarcos.com: No SPF record
    ⚠️  ext:dns:DMARC:mail-stalwart.diegonmarcos.com: No DMARC record
    ⚠️  firewall:gcp-proxy: SSH unreachable, cannot audit firewall
    ⚠️  firewall:gcp-t4: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-apps: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-mail: SSH unreachable, cannot audit firewall
    ⚠️  firewall:oci-analytics: SSH unreachable, cannot audit firewall

1. PORT SCAN
──────────────────────────────────────────────────────────────
  ⚠️  ext:port-scan:gcp-proxy:undeclared Undeclared open ports on 35.226.147.64: [22] (3.0s) [WARNING]
  ⚠️  ext:port-scan:gcp-proxy:closed Declared ports closed on 35.226.147.64: [80] (3.0s) [WARNING]
  ✅ ext:port-scan:gcp-t4           34.173.227.250 open=[] (all declared) (3.0s)
  ⚠️  ext:port-scan:oci-apps:closed  Declared ports closed on 82.70.229.129: [3010, 8081, 3000, 3001, 2222, 2223, 2224, 8099] (3.0s) [WARNING]
  ⚠️  ext:port-scan:oci-mail:closed  Declared ports closed on 130.110.251.193: [25, 465, 587, 993, 4190, 8080, 8443, 2025, 2465, 2587, 2993, 2443, 6190, 22000, 21027] (3.0s) [WARNING]
  ✅ ext:port-scan:oci-analytics    129.151.228.66 open=[] (all declared) (3.0s)

  Summary: 2/6 passed, 4 failed

2. TLS CERTIFICATE AUDIT
──────────────────────────────────────────────────────────────
  ✅ ext:tls:ide.diegonmarcos.com   ide.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:sheets.diegonmarcos.com sheets.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:chat.diegonmarcos.com  chat.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:photos.diegonmarcos.com photos.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:cal.diegonmarcos.com   cal.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:webmail.diegonmarcos.com webmail.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:smtp.diegonmarcos.com  smtp.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:mail-stalwart.diegonmarcos.com mail-stalwart.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:vault.diegonmarcos.com vault.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:auth.diegonmarcos.com  auth.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:workflows.diegonmarcos.com workflows.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:db.diegonmarcos.com    db.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:grafana.diegonmarcos.com grafana.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:windmill.diegonmarcos.com windmill.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)
  ✅ ext:tls:git.diegonmarcos.com   git.diegonmarcos.com:443 expires Jun 29 22:10:39 2026 GMT (72 days) (0.1s)

  Summary: 15/15 passed, 0 failed

3. WIREGUARD HEALTH
──────────────────────────────────────────────────────────────
  ❌ wg:gcp-proxy                   Failed to query WireGuard: SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Temporary failure in name resolution [CRITICAL]

  Summary: 0/1 passed, 1 failed

4. DNS VALIDATION
──────────────────────────────────────────────────────────────
  ✅ ext:dns:A:ide.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:sheets.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:chat.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:photos.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:cal.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:webmail.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:smtp.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:mail-stalwart.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:vault.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:auth.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:workflows.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:db.diegonmarcos.com  A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:grafana.diegonmarcos.com A=35.226.147.64 (0.0s)
  ✅ ext:dns:A:windmill.diegonmarcos.com A=35.226.147.64 (0.1s)
  ✅ ext:dns:A:git.diegonmarcos.com A=35.226.147.64 (0.0s)
  ⚠️  ext:dns:MX:webmail.diegonmarcos.com No MX records found (0.0s) [WARNING]
  ⚠️  ext:dns:SPF:webmail.diegonmarcos.com No SPF record (0.0s) [WARNING]
  ⚠️  ext:dns:DMARC:webmail.diegonmarcos.com No DMARC record (0.0s) [WARNING]
  ⚠️  ext:dns:MX:mail-stalwart.diegonmarcos.com No MX records found (0.0s) [WARNING]
  ⚠️  ext:dns:SPF:mail-stalwart.diegonmarcos.com No SPF record (0.0s) [WARNING]
  ⚠️  ext:dns:DMARC:mail-stalwart.diegonmarcos.com No DMARC record (0.0s) [WARNING]
  ✅ ext:dns:MX:diegonmarcos.com    MX=22 route1.mx.cloudflare.net., 85 route2.mx.cloudflare.net., 97 route3.mx.cloudflare.net. (0.0s)
  ✅ ext:dns:SPF:diegonmarcos.com   SPF record present (0.0s)
  ✅ ext:dns:DMARC:diegonmarcos.com DMARC record present (0.0s)

  Summary: 18/24 passed, 6 failed

5. FIREWALL AUDIT
──────────────────────────────────────────────────────────────
  ⚠️  firewall:gcp-proxy             SSH unreachable, cannot audit firewall (0.0s) [WARNING]
  ⚠️  firewall:gcp-t4                SSH unreachable, cannot audit firewall (0.0s) [WARNING]
  ⚠️  firewall:oci-apps              SSH unreachable, cannot audit firewall (0.0s) [WARNING]
  ⚠️  firewall:oci-mail              SSH unreachable, cannot audit firewall (0.0s) [WARNING]
  ⚠️  firewall:oci-analytics         SSH unreachable, cannot audit firewall (0.0s) [WARNING]

  Summary: 0/5 passed, 5 failed

══════════════════════════════════════════════════════════════
  PERFORMANCE
══════════════════════════════════════════════════════════════
  Port scan                15.0s
  TLS+DNS+WG (parallel)    1.6s
  Firewall audit           0.0s

  Total: 19.6s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 35/51 passed, 1 critical, 15 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/reports/cloud-sec-network-report
Run: build.sh all
```
