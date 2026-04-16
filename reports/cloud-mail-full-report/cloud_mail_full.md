```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-16T11:03:46.773253289+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  43 issues: 23 critical, 20 warnings, 0 info

  CRITICAL:
    ❌ mail.* HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    ❌ webmail HTTPS: HTTP err: error sending request for url (https://webmail.diegonmarcos.com/)
    ❌ mail:993 TLS: FAIL
    ❌ mail:465 TLS: FAIL
    ❌ mail:587 STARTTLS: FAIL
    ❌ DKIM record: MISSING
    ❌ Cloud API oci-mail: oci-mail: API_FAIL
    ❌ Cloud API oci-apps: oci-apps: API_FAIL
    ❌ Cloud API gcp-proxy: gcp-proxy: API_FAIL
    ❌ SSH batch oci-mail: SSH to oci-mail failed: ssh: Could not resolve hostname oci-mail: Name or service not known
    ❌ skipped: SSH unreachable
    ❌ Caddy (gcp-proxy): Caddy DOWN
    ❌ Hickory DNS: FAIL: 10.0.0.1
    ❌ TLS WG direct: SSH down
    ❌ Caddy L4 -> IMAP: no proxy data
    ❌ Caddy L4 -> SMTPS: no proxy data
    ❌ Caddy L4 -> SMTP: no proxy data
    ❌ mail:993 (IMAP): FAIL
    ❌ mail:465 (SMTPS): FAIL
    ❌ mail:587 (SMTP Sub): FAIL
    ❌ All ports bound: no data
    ❌ DKIM: missing
    ❌ skipped: SSH unreachable
  WARNINGS:
    ⚠️  auth HTTPS: HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health)
    ⚠️  MCP endpoint: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp)
    ⚠️  SSH batch oci-apps: SSH to oci-apps failed: ssh: Could not resolve hostname oci-apps: Name or service not known
    ⚠️  SSH batch gcp-proxy: SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Name or service not known
    ⚠️  SMTP :25 relay: no data
    ⚠️  SMTP :587 local TLS: no data
    ⚠️  Webmail HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    ⚠️  Webmail internal: no data
    ⚠️  SnappyMail internal: no data
    ⚠️  mail.*/webmail/ route: HTTP err: error sending request for url (https://mail.diegonmarcos.com/webmail/)
    ⚠️  webmail.* redirect: ERR: error sending request for url (https://webmail.diegonmarcos.com/)
    ⚠️  Authelia health: no proxy data
    ⚠️  OIDC bearer -> webmail: no OIDC token
    ⚠️  mcp->DNS resolve: no app data
    ⚠️  mcp->IMAP TLS: no app data
    ⚠️  mcp->SMTP TLS: no app data
    ⚠️  mcp->IMAP WG direct: no app data
    ⚠️  mcp->IMAP LOGIN: no app data
    ⚠️  mcp->SMTP AUTH: no app data
    ⚠️  mail-mcp MCP: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive)


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ❌ mail.* HTTPS                   HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (0.3s) [CRITICAL]
  ❌ webmail HTTPS                  HTTP err: error sending request for url (https://webmail.diegonmarcos.com/) (0.3s) [CRITICAL]
  ⚠️  auth HTTPS                     HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health) (0.3s) [WARNING]
  ⚠️  MCP endpoint                   HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (0.3s) [WARNING]
  ❌ mail:993 TLS                   FAIL (0.3s) [CRITICAL]
  ❌ mail:465 TLS                   FAIL (0.3s) [CRITICAL]
  ❌ mail:587 STARTTLS              FAIL (0.3s) [CRITICAL]
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ❌ DKIM record                    MISSING (0.0s) [CRITICAL]
  ✅ GHA health                     gh unavailable (skipped)
  ✅ CF Worker alive                HTTP 500 (1.2s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (0.8s)
  ✅ IMAP direct (WG)               10.0.0.3:993 OK (1.3s)
  ✅ SMTP direct (WG)               10.0.0.3:25 OK (1.3s)

  Summary: 6/14 passed, 8 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ❌ Cloud API oci-mail             oci-mail: API_FAIL [CRITICAL]
  ❌ Cloud API oci-apps             oci-apps: API_FAIL [CRITICAL]
  ❌ Cloud API gcp-proxy            gcp-proxy: API_FAIL [CRITICAL]
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.2s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.2s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.1s)
  ❌ SSH batch oci-mail             SSH to oci-mail failed: ssh: Could not resolve hostname oci-mail: Name or service not known [CRITICAL]
  ⚠️  SSH batch oci-apps             SSH to oci-apps failed: ssh: Could not resolve hostname oci-apps: Name or service not known [WARNING]
  ⚠️  SSH batch gcp-proxy            SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Name or service not known [WARNING]

  Summary: 3/9 passed, 6 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ❌ Caddy (gcp-proxy)              Caddy DOWN (5.1s) [CRITICAL]
  ❌ Hickory DNS                    FAIL: 10.0.0.1 (0.1s) [CRITICAL]
  ❌ TLS WG direct                  SSH down [CRITICAL]
  ❌ Caddy L4 -> IMAP               no proxy data [CRITICAL]
  ❌ Caddy L4 -> SMTPS              no proxy data [CRITICAL]
  ❌ Caddy L4 -> SMTP               no proxy data [CRITICAL]
  ❌ mail:993 (IMAP)                FAIL (0.3s) [CRITICAL]
  ❌ mail:465 (SMTPS)               FAIL (0.3s) [CRITICAL]
  ❌ mail:587 (SMTP Sub)            FAIL (0.3s) [CRITICAL]
  ⚠️  SMTP :25 relay                 no data [WARNING]
  ⚠️  SMTP :587 local TLS            no data [WARNING]
  ⚠️  Webmail HTTPS                  HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (0.3s) [WARNING]
  ⚠️  Webmail internal               no data [WARNING]
  ⚠️  SnappyMail internal            no data [WARNING]
  ⚠️  mail.*/webmail/ route          HTTP err: error sending request for url (https://mail.diegonmarcos.com/webmail/) (0.3s) [WARNING]
  ⚠️  webmail.* redirect             ERR: error sending request for url (https://webmail.diegonmarcos.com/) (0.3s) [WARNING]
  ⚠️  Authelia health                no proxy data [WARNING]
  ⚠️  OIDC bearer -> webmail         no OIDC token [WARNING]
  ✅ Mail Admin via Bearer          N/A — Maddy CLI-only (no web admin)
  ⚠️  mcp->DNS resolve               no app data [WARNING]
  ⚠️  mcp->IMAP TLS                  no app data [WARNING]
  ⚠️  mcp->SMTP TLS                  no app data [WARNING]
  ⚠️  mcp->IMAP WG direct            no app data [WARNING]
  ⚠️  mcp->IMAP LOGIN                no app data [WARNING]
  ⚠️  mcp->SMTP AUTH                 no app data [WARNING]
  ⚠️  mail-mcp MCP                   HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive) (0.3s) [WARNING]
  ❌ All ports bound                no data [CRITICAL]

  Summary: 1/27 passed, 26 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.1s)
  ❌ DKIM                           missing (3.2s) [CRITICAL]
  ✅ SPF                            v=spf1 (0.1s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)

  Summary: 3/4 passed, 1 failed

5. MAIL INTERNALS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

6. E2E DELIVERY
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    9.7s
  P4_dns_auth              5.1s
  P3_network               5.1s
  P2-P5_parallel           5.1s
  T0_path_checker          2.3s
  P0_instant_kpis          1.9s
  P1_preflight             0.4s
  P2_containers            0.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s

  Total: 9.7s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 22/67 passed, 25 critical, 20 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
