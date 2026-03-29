```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-03-29T21:01:48.553348040+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  37 issues: 15 critical, 18 warnings, 4 info

  CRITICAL:
    [X] mail.* HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    [X] webmail HTTPS: HTTP err: error sending request for url (https://webmail.diegonmarcos.com/)
    [X] mail:993 TLS: FAIL
    [X] mail:465 TLS: FAIL
    [X] mail:587 STARTTLS: FAIL
    [X] WG oci-apps: WG DOWN
    [X] Caddy (gcp-proxy): Caddy DOWN
    [X] Hickory DNS: FAIL: no response
    [X] Caddy L4 -> IMAP: no proxy data
    [X] Caddy L4 -> SMTPS: no proxy data
    [X] Caddy L4 -> SMTP: no proxy data
    [X] mail:993 (IMAP): FAIL
    [X] mail:465 (SMTPS): FAIL
    [X] mail:587 (SMTP Sub): FAIL
    [X] All ports bound: missing: 8443
  WARNINGS:
    [!] auth HTTPS: HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health)
    [!] MCP endpoint: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp)
    [!] SSH batch oci-apps: SSH FAILED
    [!] SSH batch gcp-proxy: SSH FAILED
    [!] mail-mcp: no data
    [!] Webmail HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    [!] Authelia health: no proxy data
    [!] OIDC bearer -> webmail: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    [!] Stalwart Admin via Bearer: HTTP err: error sending request for url (https://mail.diegonmarcos.com/api/)
    [!] mcp->DNS resolve: no app data
    [!] mcp->IMAP TLS: no app data
    [!] mcp->SMTP TLS: no app data
    [!] mcp->IMAP WG direct: no app data
    [!] mcp->IMAP LOGIN: no app data
    [!] mcp->SMTP AUTH: no app data
    [!] mail-mcp MCP: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive)
    [!] admin panel: HTTP 000
    [!] User accounts: unknown ()
  INFO:
    [-] GHA health: 2 failing: Health → Mail (Full Check), Health → Mail (Full Check)
    [-] Admin API accounts: 
    [-] Admin API domains: 
    [-] Resend API key: not set (set RESEND_API_KEY to enable E2E)


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✗ HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (0.2s) [CRITICAL]
  ✗ HTTP err: error sending request for url (https://webmail.diegonmarcos.com/) (0.2s) [CRITICAL]
  ! HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health) (0.2s) [WARNING]
  ! HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (0.2s) [WARNING]
  ✗ FAIL (0.2s) [CRITICAL]
  ✗ FAIL (0.2s) [CRITICAL]
  ✗ FAIL (0.2s) [CRITICAL]
  ✓ 22 route1.mx.cloudflare.net. (0.1s)
  ✓ present (0.1s)
  - 2 failing: Health → Mail (Full Check), Health → Mail (Full Check) (1.0s) [INFO]

  Summary: 2/10 passed, 8 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✓ 10.0.0.3:22 OK (0.3s)
  ✗ WG DOWN (3.0s) [CRITICAL]
  ✓ 10.0.0.1:22 OK (0.2s)
  ✓ Docker 27.5.1
  ! SSH FAILED [WARNING]
  ! SSH FAILED [WARNING]
  ✓ 68% used
  ✓ 667/954MB (70%)
  ✓ load: 2.16 2.04 1.99 WARNING

  Summary: 6/9 passed, 3 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✓ Up 39 minutes
  ✓ Up 40 minutes
  ✓ Up 40 minutes
  ! no data [WARNING]

  Summary: 3/4 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✗ Caddy DOWN (3.2s) [CRITICAL]
  ✗ FAIL: no response (3.0s) [CRITICAL]
  ✓ 993 OK 465 OK 587 OK (0.6s)
  ✗ no proxy data [CRITICAL]
  ✗ no proxy data [CRITICAL]
  ✗ no proxy data [CRITICAL]
  ✗ FAIL (0.2s) [CRITICAL]
  ✗ FAIL (0.2s) [CRITICAL]
  ✗ FAIL (0.2s) [CRITICAL]
  ✓ 220 mail.diegonmarcos.com Stalwart ESMTP at your service
  ✓ STARTTLS OK
  ! HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (0.2s) [WARNING]
  ✓ HTTP 200
  ✓ HTTP 200
  ✓ ManageSieve OK
  ! no proxy data [WARNING]
  ! HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (0.2s) [WARNING]
  ! HTTP err: error sending request for url (https://mail.diegonmarcos.com/api/) (0.2s) [WARNING]
  ! no app data [WARNING]
  ! no app data [WARNING]
  ! no app data [WARNING]
  ! no app data [WARNING]
  ! no app data [WARNING]
  ! no app data [WARNING]
  ! HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive) (0.2s) [WARNING]
  ✗ missing: 8443 [CRITICAL]

  Summary: 6/26 passed, 20 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✓ 22 route1.mx.cloudflare.net. (0.0s)
  ✓ present (0.0s)
  ✓ v=spf1 (0.0s)
  ✓ v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)

  Summary: 4/4 passed, 0 failed

5. MAIL INTERNALS
──────────────────────────────────────────────────────────────
  ✓ Stalwart IMAP responding
  ✓ IMAP4rev1
  ✓ Stalwart built-in
  ✓ RocksDB
  ! HTTP 000 [WARNING]
  ✓ Stalwart ManageSieve
  ✓ stalwart-builtin-quota
  -  [INFO]
  -  [INFO]
  ✓ 
  ! unknown () [WARNING]

  Summary: 7/11 passed, 4 failed

6. E2E DELIVERY
──────────────────────────────────────────────────────────────
  - not set (set RESEND_API_KEY to enable E2E) [INFO]

  Summary: 0/1 passed, 1 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    42.4s
  P1_preflight             38.0s
  P3_network               3.2s
  P2-P5_parallel           3.2s
  P4_dns_auth              3.2s
  P0_instant_kpis          1.2s
  P6_e2e_delivery          0.0s
  P2_containers            0.0s
  P5_internals             0.0s

  Total: 42.4s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 28/65 passed, 15 critical, 18 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
