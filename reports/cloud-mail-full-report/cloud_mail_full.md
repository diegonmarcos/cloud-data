```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-20T08:13:07.271302528+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  34 issues: 19 critical, 15 warnings, 0 info

  CRITICAL:
    ❌ DKIM record: MISSING
    ❌ IMAP direct (WG): 10.0.0.3:993 FAIL
    ❌ SMTP direct (WG): 10.0.0.3:25 FAIL
    ❌ Cloud API oci-mail: oci-mail: API_FAIL
    ❌ Cloud API oci-apps: oci-apps: API_FAIL
    ❌ Cloud API gcp-proxy: gcp-proxy: API_FAIL
    ❌ WG oci-mail: WG DOWN
    ❌ WG oci-apps: WG DOWN
    ❌ WG gcp-proxy: WG DOWN
    ❌ SSH batch oci-mail: SSH to oci-mail failed: ssh: Could not resolve hostname oci-mail: Temporary failure in name resolution
    ❌ skipped: SSH unreachable
    ❌ Hickory DNS: FAIL: no response
    ❌ TLS WG direct: SSH down
    ❌ Caddy L4 -> IMAP: no proxy data
    ❌ Caddy L4 -> SMTPS: no proxy data
    ❌ Caddy L4 -> SMTP: no proxy data
    ❌ All ports bound: no data
    ❌ DKIM: missing
    ❌ skipped: SSH unreachable
  WARNINGS:
    ⚠️  SSH batch oci-apps: SSH to oci-apps failed: ssh: Could not resolve hostname oci-apps: Temporary failure in name resolution
    ⚠️  SSH batch gcp-proxy: SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Temporary failure in name resolution
    ⚠️  SMTP :25 relay: no data
    ⚠️  SMTP :587 local TLS: no data
    ⚠️  Webmail internal: no data
    ⚠️  SnappyMail internal: no data
    ⚠️  webmail.* redirect: 302 → https://auth.diegonmarcos.com/?rd=https%3A%2F%2Fwebmail.diegonmarcos.com%2F&rm=GET
    ⚠️  Authelia health: no proxy data
    ⚠️  OIDC bearer -> webmail: no OIDC token
    ⚠️  mcp->DNS resolve: no app data
    ⚠️  mcp->IMAP TLS: no app data
    ⚠️  mcp->SMTP TLS: no app data
    ⚠️  mcp->IMAP WG direct: no app data
    ⚠️  mcp->IMAP LOGIN: no app data
    ⚠️  mcp->SMTP AUTH: no app data


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 301 (0.2s)
  ✅ webmail HTTPS                  HTTP 302 (0.2s)
  ✅ auth HTTPS                     HTTP 200 (0.2s)
  ✅ MCP endpoint                   HTTP 400 (0.4s)
  ✅ mail:993 TLS                   TLS OK (1.4s)
  ✅ mail:465 TLS                   TLS OK (1.3s)
  ✅ mail:587 STARTTLS              TLS OK (2.1s)
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ❌ DKIM record                    MISSING (0.0s) [CRITICAL]
  ✅ GHA health                     gh unavailable (skipped)
  ✅ CF Worker alive                HTTP 500 (0.1s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (0.1s)
  ❌ IMAP direct (WG)               10.0.0.3:993 FAIL (3.0s) [CRITICAL]
  ❌ SMTP direct (WG)               10.0.0.3:25 FAIL (3.0s) [CRITICAL]

  Summary: 11/14 passed, 3 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ❌ Cloud API oci-mail             oci-mail: API_FAIL [CRITICAL]
  ❌ Cloud API oci-apps             oci-apps: API_FAIL [CRITICAL]
  ❌ Cloud API gcp-proxy            gcp-proxy: API_FAIL [CRITICAL]
  ❌ WG oci-mail                    WG DOWN (3.0s) [CRITICAL]
  ❌ WG oci-apps                    WG DOWN (3.0s) [CRITICAL]
  ❌ WG gcp-proxy                   WG DOWN (3.0s) [CRITICAL]
  ❌ SSH batch oci-mail             SSH to oci-mail failed: ssh: Could not resolve hostname oci-mail: Temporary failure in name resolution [CRITICAL]
  ⚠️  SSH batch oci-apps             SSH to oci-apps failed: ssh: Could not resolve hostname oci-apps: Temporary failure in name resolution [WARNING]
  ⚠️  SSH batch gcp-proxy            SSH to gcp-proxy failed: ssh: Could not resolve hostname gcp-proxy: Temporary failure in name resolution [WARNING]

  Summary: 0/9 passed, 9 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (no DNS) (3.2s)
  ❌ Hickory DNS                    FAIL: no response (3.0s) [CRITICAL]
  ❌ TLS WG direct                  SSH down [CRITICAL]
  ❌ Caddy L4 -> IMAP               no proxy data [CRITICAL]
  ❌ Caddy L4 -> SMTPS              no proxy data [CRITICAL]
  ❌ Caddy L4 -> SMTP               no proxy data [CRITICAL]
  ✅ mail:993 (IMAP)                TLS OK (0.7s)
  ✅ mail:465 (SMTPS)               TLS OK (0.7s)
  ✅ mail:587 (SMTP Sub)            TLS OK (1.3s)
  ⚠️  SMTP :25 relay                 no data [WARNING]
  ⚠️  SMTP :587 local TLS            no data [WARNING]
  ✅ Webmail HTTPS                  HTTP 301 (0.2s)
  ⚠️  Webmail internal               no data [WARNING]
  ⚠️  SnappyMail internal            no data [WARNING]
  ✅ mail.*/webmail/ route          HTTP 301 (0.2s)
  ⚠️  webmail.* redirect             302 → https://auth.diegonmarcos.com/?rd=https%3A%2F%2Fwebmail.diegonmarcos.com%2F&rm=GET (0.2s) [WARNING]
  ⚠️  Authelia health                no proxy data [WARNING]
  ⚠️  OIDC bearer -> webmail         no OIDC token [WARNING]
  ✅ Mail Admin via Bearer          N/A — Maddy CLI-only (no web admin)
  ⚠️  mcp->DNS resolve               no app data [WARNING]
  ⚠️  mcp->IMAP TLS                  no app data [WARNING]
  ⚠️  mcp->SMTP TLS                  no app data [WARNING]
  ⚠️  mcp->IMAP WG direct            no app data [WARNING]
  ⚠️  mcp->IMAP LOGIN                no app data [WARNING]
  ⚠️  mcp->SMTP AUTH                 no app data [WARNING]
  ✅ mail-mcp MCP                   HTTP 400 (alive) (0.4s)
  ❌ All ports bound                no data [CRITICAL]

  Summary: 8/27 passed, 19 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.0s)
  ❌ DKIM                           missing (0.0s) [CRITICAL]
  ✅ SPF                            v=spf1 (0.0s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)

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
  TOTAL                    51.4s
  T0_path_checker          39.7s
  P0_instant_kpis          5.5s
  P2-P5_parallel           3.2s
  P4_dns_auth              3.2s
  P3_network               3.2s
  P1_preflight             3.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s
  P2_containers            0.0s

  Total: 51.4s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 26/67 passed, 26 critical, 15 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
