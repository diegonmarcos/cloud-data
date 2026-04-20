```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-19T14:58:56.236724312+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  17 issues: 6 critical, 11 warnings, 0 info

  CRITICAL:
    ❌ mail:587 STARTTLS: timeout/error
    ❌ DKIM record: MISSING
    ❌ Cloud API gcp-proxy: gcp-proxy: API_FAIL
    ❌ Hickory DNS: FAIL: 10.0.0.1
    ❌ TLS WG direct: 993 FAIL 465 FAIL 587 FAIL
    ❌ DKIM: missing
  WARNINGS:
    ⚠️  SSH batch oci-apps: SSH :22 failed (tokio timeout) + Dropbear :2200 down — OCI status: RUNNING
    ⚠️  mail-mcp: no data
    ⚠️  Webmail internal: HTTP 000
    ⚠️  SnappyMail internal: HTTP 000
    ⚠️  webmail.* redirect: 302 → https://auth.diegonmarcos.com/?rd=https%3A%2F%2Fwebmail.diegonmarcos.com%2F&rm=GET
    ⚠️  mcp->DNS resolve: no app data
    ⚠️  mcp->IMAP TLS: no app data
    ⚠️  mcp->SMTP TLS: no app data
    ⚠️  mcp->IMAP WG direct: no app data
    ⚠️  mcp->IMAP LOGIN: no app data
    ⚠️  mcp->SMTP AUTH: no app data


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 301 (3.1s)
  ✅ webmail HTTPS                  HTTP 302 (4.1s)
  ✅ auth HTTPS                     HTTP 200 (4.1s)
  ✅ MCP endpoint                   HTTP 400 (4.1s)
  ✅ mail:993 TLS                   TLS OK (3.3s)
  ✅ mail:465 TLS                   TLS OK (3.3s)
  ❌ mail:587 STARTTLS              timeout/error (5.0s) [CRITICAL]
  ✅ MX record                      97 route3.mx.cloudflare.net. (2.8s)
  ❌ DKIM record                    MISSING (3.0s) [CRITICAL]
  ✅ GHA health                     1 failing: Ship (5.2s)
  ✅ CF Worker alive                HTTP 500 (1.0s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (2.0s)
  ✅ IMAP direct (WG)               10.0.0.3:993 OK (0.3s)
  ✅ SMTP direct (WG)               10.0.0.3:25 OK (0.3s)

  Summary: 12/14 passed, 2 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ Cloud API oci-mail             oci-mail: RUNNING
  ✅ Cloud API oci-apps             oci-apps: RUNNING
  ❌ Cloud API gcp-proxy            gcp-proxy: API_FAIL [CRITICAL]
  ✅ WG oci-mail                    10.0.0.3:22 OK (1.0s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (1.0s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (1.0s)
  ✅ SSH batch oci-mail             Docker 27.5.1
  ⚠️  SSH batch oci-apps             SSH :22 failed (tokio timeout) + Dropbear :2200 down — OCI status: RUNNING [WARNING]
  ✅ SSH batch gcp-proxy            Authelia OK
  ✅ Disk space                     77% used
  ✅ Memory                         661/954MB (69%)
  ✅ Load                           load: 0.11 0.27 0.29

  Summary: 10/12 passed, 2 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ maddy                          Up 6 hours
  ✅ smtp-proxy                     Up 6 hours
  ✅ snappymail                     Up 6 hours
  ⚠️  mail-mcp                       no data [WARNING]

  Summary: 3/4 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (10.0.0.1) (1.4s)
  ❌ Hickory DNS                    FAIL: 10.0.0.1 (0.8s) [CRITICAL]
  ❌ TLS WG direct                  993 FAIL 465 FAIL 587 FAIL (1.1s) [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ✅ mail:993 (IMAP)                TLS OK (1.6s)
  ✅ mail:465 (SMTPS)               TLS OK (1.6s)
  ✅ mail:587 (SMTP Sub)            TLS OK (2.6s)
  ✅ SMTP :25 relay                 220 mail.diegonmarcos.com ESMTP Service Ready
  ✅ SMTP :587 local TLS            STARTTLS OK
  ✅ Webmail HTTPS                  HTTP 301 (1.3s)
  ⚠️  Webmail internal               HTTP 000 [WARNING]
  ⚠️  SnappyMail internal            HTTP 000 [WARNING]
  ✅ mail.*/webmail/ route          HTTP 301 (1.3s)
  ⚠️  webmail.* redirect             302 → https://auth.diegonmarcos.com/?rd=https%3A%2F%2Fwebmail.diegonmarcos.com%2F&rm=GET (1.2s) [WARNING]
  ✅ Authelia health                Authelia OK
  ✅ OIDC bearer -> webmail         Bearer auth -> 200 OK (full chain) (2.9s)
  ✅ Mail Admin via Bearer          N/A — Maddy CLI-only (no web admin)
  ⚠️  mcp->DNS resolve               no app data [WARNING]
  ⚠️  mcp->IMAP TLS                  no app data [WARNING]
  ⚠️  mcp->SMTP TLS                  no app data [WARNING]
  ⚠️  mcp->IMAP WG direct            no app data [WARNING]
  ⚠️  mcp->IMAP LOGIN                no app data [WARNING]
  ⚠️  mcp->SMTP AUTH                 no app data [WARNING]
  ✅ mail-mcp MCP                   HTTP 400 (alive) (1.5s)
  ✅ All ports bound                all 6 ports bound

  Summary: 16/27 passed, 11 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             97 route3.mx.cloudflare.net. (1.0s)
  ❌ DKIM                           missing (0.8s) [CRITICAL]
  ✅ SPF                            v=spf1 (0.8s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.8s)

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
  TOTAL                    99.6s
  P1_preflight             60.2s
  T0_path_checker          24.3s
  P0_instant_kpis          12.2s
  P2-P5_parallel           2.9s
  P4_dns_auth              2.9s
  P3_network               2.9s
  P5_internals             0.0s
  P6_e2e_delivery          0.0s
  P2_containers            0.0s

  Total: 99.6s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 68/87 passed, 8 critical, 11 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
