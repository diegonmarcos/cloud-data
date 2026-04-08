```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-06T04:43:03.319552632+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  15 issues: 9 critical, 6 warnings, 0 info

  CRITICAL:
    ❌ mail:993 TLS: timeout/error
    ❌ mail:465 TLS: timeout/error
    ❌ mail:587 STARTTLS: timeout/error
    ❌ Caddy (gcp-proxy): Caddy DOWN
    ❌ Hickory DNS: FAIL: no response
    ❌ TLS WG direct: 993 FAIL 465 FAIL 587 FAIL
    ❌ mail:993 (IMAP): timeout/error
    ❌ mail:465 (SMTPS): timeout/error
    ❌ mail:587 (SMTP Sub): timeout/error
  WARNINGS:
    ⚠️  auth HTTPS: HTTP 502
    ⚠️  SSH batch gcp-proxy: Authelia FAILED
    ⚠️  mail.*/webmail/ route: HTTP 502
    ⚠️  webmail.* redirect: 301 → https://mail.diegonmarcos.com/
    ⚠️  Authelia health: FAIL
    ⚠️  mcp->SMTP AUTH: SMTP_NO_AUTH
SMTP_NO_AUTH
ERR:write after end
TIMEOUT


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 200 (0.6s)
  ✅ webmail HTTPS                  HTTP 301 (0.6s)
  ⚠️  auth HTTPS                     HTTP 502 (0.6s) [WARNING]
  ✅ MCP endpoint                   HTTP 400 (5.7s)
  ❌ mail:993 TLS                   timeout/error (5.0s) [CRITICAL]
  ❌ mail:465 TLS                   timeout/error (5.0s) [CRITICAL]
  ❌ mail:587 STARTTLS              timeout/error (5.0s) [CRITICAL]
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM record                    present (0.0s)
  ✅ GHA health                     2 failing: Health, Health (6.1s)
  ✅ CF Worker alive                HTTP 500 (0.3s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (0.2s)
  ✅ IMAP direct (WG)               10.0.0.3:993 OK (0.2s)
  ✅ SMTP direct (WG)               10.0.0.3:25 OK (0.2s)

  Summary: 10/14 passed, 4 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ Cloud API oci-mail             oci-mail: RUNNING
  ✅ Cloud API oci-apps             oci-apps: RUNNING
  ✅ Cloud API gcp-proxy            gcp-proxy: RUNNING
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.2s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.2s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.2s)
  ✅ SSH batch oci-mail             Docker 27.5.1
  ✅ SSH batch oci-apps             mail-mcp: Up 43 hours
  ⚠️  SSH batch gcp-proxy            Authelia FAILED [WARNING]
  ✅ Disk space                     78% used
  ✅ Memory                         649/954MB (68%)
  ✅ Load                           load: 0.12 0.16 0.12

  Summary: 11/12 passed, 1 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ maddy                          Up 43 hours
  ✅ smtp-proxy                     Up 4 days
  ✅ snappymail                     Up 42 hours
  ✅ mail-mcp                       Up 43 hours

  Summary: 4/4 passed, 0 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ❌ Caddy (gcp-proxy)              Caddy DOWN (8.0s) [CRITICAL]
  ❌ Hickory DNS                    FAIL: no response (3.0s) [CRITICAL]
  ❌ TLS WG direct                  993 FAIL 465 FAIL 587 FAIL (0.5s) [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ❌ mail:993 (IMAP)                timeout/error (5.0s) [CRITICAL]
  ❌ mail:465 (SMTPS)               timeout/error (5.0s) [CRITICAL]
  ❌ mail:587 (SMTP Sub)            timeout/error (5.0s) [CRITICAL]
  ✅ SMTP :25 relay                 220 mail.diegonmarcos.com ESMTP Service Ready
  ✅ SMTP :587 local TLS            STARTTLS OK
  ✅ Webmail HTTPS                  HTTP 200 (0.5s)
  ✅ Webmail internal               HTTP 200
  ✅ SnappyMail internal            HTTP 200
  ⚠️  mail.*/webmail/ route          HTTP 502 (0.5s) [WARNING]
  ⚠️  webmail.* redirect             301 → https://mail.diegonmarcos.com/ (0.5s) [WARNING]
  ⚠️  Authelia health                FAIL [WARNING]
  ✅ OIDC bearer -> webmail         Bearer auth -> 200 OK (full chain) (5.4s)
  ✅ Mail Admin via Bearer          N/A — Maddy CLI-only (no web admin)
  ✅ mcp->DNS resolve               -> 35.226.147.64
  ✅ mcp->IMAP TLS                  OK proto=TLSv1.3 cn=*.diegonmarcos.com
  ✅ mcp->SMTP TLS                  OK proto=TLSv1.3
  ✅ mcp->IMAP WG direct            10.0.0.3:993 OK proto=TLSv1.3
  ✅ mcp->IMAP LOGIN                LOGIN_OK
TIMEOUT
  ⚠️  mcp->SMTP AUTH                 SMTP_NO_AUTH
SMTP_NO_AUTH
ERR:write after end
TIMEOUT [WARNING]
  ✅ mail-mcp MCP                   HTTP 400 (alive) (5.7s)
  ✅ All ports bound                all 6 ports bound

  Summary: 17/27 passed, 10 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             97 route3.mx.cloudflare.net. (0.0s)
  ✅ DKIM                           present (0.0s)
  ✅ SPF                            v=spf1 (0.0s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.3s)

  Summary: 4/4 passed, 0 failed

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
  TOTAL                    76.3s
  T0_path_checker          30.4s
  P1_preflight             26.4s
  P0_instant_kpis          11.5s
  P2-P5_parallel           8.0s
  P3_network               8.0s
  P4_dns_auth              8.0s
  P6_e2e_delivery          0.0s
  P2_containers            0.0s
  P5_internals             0.0s

  Total: 76.3s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 70/87 passed, 11 critical, 6 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
