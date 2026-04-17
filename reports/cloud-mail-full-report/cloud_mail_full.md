```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-16T21:15:46.207385029+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  8 issues: 4 critical, 4 warnings, 0 info

  CRITICAL:
    ❌ DKIM record: MISSING
    ❌ Hickory DNS: FAIL: 10.0.0.1
    ❌ TLS WG direct: 993 FAIL 465 FAIL 587 FAIL
    ❌ DKIM: missing
  WARNINGS:
    ⚠️  Webmail internal: HTTP 000
    ⚠️  SnappyMail internal: HTTP 000
    ⚠️  webmail.* redirect: 301 → https://mail.diegonmarcos.com/
    ⚠️  mcp->SMTP AUTH: SMTP_NO_AUTH
SMTP_NO_AUTH
ERR:write after end
TIMEOUT


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 200 (0.8s)
  ✅ webmail HTTPS                  HTTP 301 (0.7s)
  ✅ auth HTTPS                     HTTP 200 (0.5s)
  ✅ MCP endpoint                   HTTP 400 (0.8s)
  ✅ mail:993 TLS                   TLS OK (1.3s)
  ✅ mail:465 TLS                   TLS OK (1.4s)
  ✅ mail:587 STARTTLS              TLS OK (1.9s)
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.1s)
  ❌ DKIM record                    MISSING (0.0s) [CRITICAL]
  ✅ GHA health                     all green (1.1s)
  ✅ CF Worker alive                HTTP 500 (0.4s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (0.5s)
  ✅ IMAP direct (WG)               10.0.0.3:993 OK (0.2s)
  ✅ SMTP direct (WG)               10.0.0.3:25 OK (0.2s)

  Summary: 13/14 passed, 1 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ Cloud API oci-mail             oci-mail: RUNNING
  ✅ Cloud API oci-apps             oci-apps: RUNNING
  ✅ Cloud API gcp-proxy            gcp-proxy: RUNNING
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.2s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.2s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.1s)
  ✅ SSH batch oci-mail             Docker 27.5.1
  ✅ SSH batch oci-apps             mail-mcp: Up About an hour
  ✅ SSH batch gcp-proxy            Authelia OK
  ✅ Disk space                     78% used
  ✅ Memory                         688/954MB (72%)
  ✅ Load                           load: 0.19 0.24 0.23

  Summary: 12/12 passed, 0 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ maddy                          Up 10 hours
  ✅ smtp-proxy                     Up 7 hours
  ✅ snappymail                     Up 9 hours
  ✅ mail-mcp                       Up About an hour

  Summary: 4/4 passed, 0 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (10.0.0.1) (0.8s)
  ❌ Hickory DNS                    FAIL: 10.0.0.1 (0.1s) [CRITICAL]
  ❌ TLS WG direct                  993 FAIL 465 FAIL 587 FAIL (0.7s) [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ✅ mail:993 (IMAP)                TLS OK (0.9s)
  ✅ mail:465 (SMTPS)               TLS OK (0.9s)
  ✅ mail:587 (SMTP Sub)            TLS OK (1.6s)
  ✅ SMTP :25 relay                 220 mail.diegonmarcos.com ESMTP Service Ready
  ✅ SMTP :587 local TLS            STARTTLS OK
  ✅ Webmail HTTPS                  HTTP 200 (0.7s)
  ⚠️  Webmail internal               HTTP 000 [WARNING]
  ⚠️  SnappyMail internal            HTTP 000 [WARNING]
  ✅ mail.*/webmail/ route          HTTP 302 (0.5s)
  ⚠️  webmail.* redirect             301 → https://mail.diegonmarcos.com/ (0.7s) [WARNING]
  ✅ Authelia health                Authelia OK
  ✅ OIDC bearer -> webmail         Bearer auth -> 200 OK (full chain) (0.5s)
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
  ✅ mail-mcp MCP                   HTTP 400 (alive) (0.9s)
  ✅ All ports bound                all 6 ports bound

  Summary: 21/27 passed, 6 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.0s)
  ❌ DKIM                           missing (0.0s) [CRITICAL]
  ✅ SPF                            v=spf1 (0.0s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)

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
  TOTAL                    30.6s
  P1_preflight             23.1s
  P0_instant_kpis          3.5s
  T0_path_checker          2.5s
  P3_network               1.6s
  P4_dns_auth              1.6s
  P2-P5_parallel           1.6s
  P2_containers            0.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s

  Total: 30.6s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 79/87 passed, 4 critical, 4 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
