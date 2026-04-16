```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-16T11:52:51.049524661+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  31 issues: 16 critical, 15 warnings, 0 info

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
    ❌ Caddy (gcp-proxy): Caddy DOWN
    ❌ Hickory DNS: FAIL: 10.0.0.1
    ❌ TLS WG direct: 993 FAIL 465 FAIL 587 FAIL
    ❌ mail:993 (IMAP): FAIL
    ❌ mail:465 (SMTPS): FAIL
    ❌ mail:587 (SMTP Sub): FAIL
    ❌ DKIM: missing
  WARNINGS:
    ⚠️  auth HTTPS: HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health)
    ⚠️  MCP endpoint: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp)
    ⚠️  SSH batch oci-apps: mail-mcp: 
    ⚠️  mail-mcp: 
    ⚠️  Webmail HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    ⚠️  mail.*/webmail/ route: HTTP err: error sending request for url (https://mail.diegonmarcos.com/webmail/)
    ⚠️  webmail.* redirect: ERR: error sending request for url (https://webmail.diegonmarcos.com/)
    ⚠️  OIDC bearer -> webmail: no OIDC token
    ⚠️  mcp->DNS resolve: Error response from daemon: No such container: mail-mcp
    ⚠️  mcp->IMAP TLS: Error response from daemon: No such container: mail-mcp
    ⚠️  mcp->SMTP TLS: Error response from daemon: No such container: mail-mcp
    ⚠️  mcp->IMAP WG direct: 10.0.0.3:993 Error response from daemon: No such container: mail-mcp
    ⚠️  mcp->IMAP LOGIN: Error response from daemon: No such container: mail-mcp
    ⚠️  mcp->SMTP AUTH: Error response from daemon: No such container: mail-mcp
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
  ✅ CF Worker alive                HTTP 500 (0.2s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (0.7s)
  ✅ IMAP direct (WG)               10.0.0.3:993 OK (0.2s)
  ✅ SMTP direct (WG)               10.0.0.3:25 OK (0.2s)

  Summary: 6/14 passed, 8 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ❌ Cloud API oci-mail             oci-mail: API_FAIL [CRITICAL]
  ❌ Cloud API oci-apps             oci-apps: API_FAIL [CRITICAL]
  ❌ Cloud API gcp-proxy            gcp-proxy: API_FAIL [CRITICAL]
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.3s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.3s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.1s)
  ✅ SSH batch oci-mail             Docker 27.5.1
  ⚠️  SSH batch oci-apps             mail-mcp:  [WARNING]
  ✅ SSH batch gcp-proxy            Authelia OK
  ✅ Disk space                     78% used
  ✅ Memory                         640/954MB (67%)
  ✅ Load                           load: 0.11 0.17 0.18

  Summary: 8/12 passed, 4 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ maddy                          Up 16 minutes
  ✅ smtp-proxy                     Up 39 hours
  ✅ snappymail                     Up 12 days
  ⚠️  mail-mcp                        [WARNING]

  Summary: 3/4 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ❌ Caddy (gcp-proxy)              Caddy DOWN (0.6s) [CRITICAL]
  ❌ Hickory DNS                    FAIL: 10.0.0.1 (0.2s) [CRITICAL]
  ❌ TLS WG direct                  993 FAIL 465 FAIL 587 FAIL (0.5s) [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ❌ mail:993 (IMAP)                FAIL (0.4s) [CRITICAL]
  ❌ mail:465 (SMTPS)               FAIL (0.4s) [CRITICAL]
  ❌ mail:587 (SMTP Sub)            FAIL (0.4s) [CRITICAL]
  ✅ SMTP :25 relay                 220 mail.diegonmarcos.com ESMTP Service Ready
  ✅ SMTP :587 local TLS            STARTTLS OK
  ⚠️  Webmail HTTPS                  HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (0.4s) [WARNING]
  ✅ Webmail internal               HTTP 200
  ✅ SnappyMail internal            HTTP 200
  ⚠️  mail.*/webmail/ route          HTTP err: error sending request for url (https://mail.diegonmarcos.com/webmail/) (0.4s) [WARNING]
  ⚠️  webmail.* redirect             ERR: error sending request for url (https://webmail.diegonmarcos.com/) (0.4s) [WARNING]
  ✅ Authelia health                Authelia OK
  ⚠️  OIDC bearer -> webmail         no OIDC token [WARNING]
  ✅ Mail Admin via Bearer          N/A — Maddy CLI-only (no web admin)
  ⚠️  mcp->DNS resolve               Error response from daemon: No such container: mail-mcp [WARNING]
  ⚠️  mcp->IMAP TLS                  Error response from daemon: No such container: mail-mcp [WARNING]
  ⚠️  mcp->SMTP TLS                  Error response from daemon: No such container: mail-mcp [WARNING]
  ⚠️  mcp->IMAP WG direct            10.0.0.3:993 Error response from daemon: No such container: mail-mcp [WARNING]
  ⚠️  mcp->IMAP LOGIN                Error response from daemon: No such container: mail-mcp [WARNING]
  ⚠️  mcp->SMTP AUTH                 Error response from daemon: No such container: mail-mcp [WARNING]
  ⚠️  mail-mcp MCP                   HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive) (0.4s) [WARNING]
  ✅ All ports bound                all 6 ports bound

  Summary: 10/27 passed, 17 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.0s)
  ❌ DKIM                           missing (0.0s) [CRITICAL]
  ✅ SPF                            v=spf1 (0.2s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)

  Summary: 3/4 passed, 1 failed

5. MAIL INTERNALS
──────────────────────────────────────────────────────────────
  ✅ IMAP auth                      Maddy IMAP responding
  ✅ IMAP protocol                  IMAP4rev1
  ✅ spam filter                    Maddy built-in DKIM/SPF
  ✅ data store                     Maddy SQLite
  ✅ admin panel                    Maddy CLI-only (no web admin)
  ✅ sieve filter                   Maddy built-in sieve (no ManageSieve server)
  ✅ mailbox quota                  me@diegonmarcos.com
no-reply@diegonmarcos.com
  ✅ Mail accounts                  2 accounts (maddy creds)
  ✅ Mail domains                   diegonmarcos.com configured
  ✅ Mail queue                     empty
  ✅ User accounts                  2 users

  Summary: 11/11 passed, 0 failed

6. E2E DELIVERY
──────────────────────────────────────────────────────────────
  ✅ Resend API key                 not set (set RESEND_API_KEY to enable E2E)

  Summary: 1/1 passed, 0 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    18.9s
  P1_preflight             14.7s
  T0_path_checker          2.4s
  P0_instant_kpis          1.3s
  P4_dns_auth              0.6s
  P2-P5_parallel           0.6s
  P3_network               0.6s
  P2_containers            0.0s
  P5_internals             0.0s
  P6_e2e_delivery          0.0s

  Total: 18.9s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 52/87 passed, 19 critical, 16 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
