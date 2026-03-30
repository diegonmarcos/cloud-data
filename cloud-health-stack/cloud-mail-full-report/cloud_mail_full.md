```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-03-30T00:19:36.237191290+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  19 issues: 5 critical, 12 warnings, 2 info

  CRITICAL:
    ❌ SSH batch oci-mail: SSH FAILED
    ❌ skipped: SSH unreachable
    ❌ TLS WG direct: SSH down
    ❌ All ports bound: no data
    ❌ skipped: SSH unreachable
  WARNINGS:
    ⚠️  SMTP :25 relay: no data
    ⚠️  SMTP :587 local TLS: no data
    ⚠️  Webmail internal: no data
    ⚠️  SnappyMail internal: no data
    ⚠️  ManageSieve :4190: no data
    ⚠️  OIDC bearer -> webmail: HTTP 429
    ⚠️  Stalwart Admin via Bearer: HTTP 429
    ⚠️  mcp->IMAP TLS: ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established
    ⚠️  mcp->SMTP TLS: ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established
    ⚠️  mcp->IMAP LOGIN: ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT
    ⚠️  mcp->SMTP AUTH: ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT
    ⚠️  mail-mcp MCP: HTTP 429 (alive)
  INFO:
    ℹ️  GHA health: 3 failing: Health → Mail (Full Check), Health → VMs
    ℹ️  Resend API key: not set (set RESEND_API_KEY to enable E2E)


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 302 (0.4s)
  ✅ webmail HTTPS                  HTTP 200 (0.5s)
  ✅ auth HTTPS                     HTTP 200 (0.5s)
  ✅ MCP endpoint                   HTTP 400 (0.7s)
  ✅ mail:993 TLS                   TLS OK (0.5s)
  ✅ mail:465 TLS                   TLS OK (0.5s)
  ✅ mail:587 STARTTLS              TLS OK (0.5s)
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM record                    present (0.1s)
  ℹ️  GHA health                     3 failing: Health → Mail (Full Check), Health → VMs (0.7s) [INFO]

  Summary: 9/10 passed, 1 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.3s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.3s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.1s)
  ❌ SSH batch oci-mail             SSH FAILED [CRITICAL]
  ✅ SSH batch oci-apps             mail-mcp: Up 3 hours
  ✅ SSH batch gcp-proxy            Authelia OK

  Summary: 5/6 passed, 1 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (10.0.0.1) (0.8s)
  ✅ Hickory DNS                    stalwart.app -> 10.0.0.3 (0.2s)
  ❌ TLS WG direct                  SSH down [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ✅ mail:993 (IMAP)                TLS OK (0.7s)
  ✅ mail:465 (SMTPS)               TLS OK (0.7s)
  ✅ mail:587 (SMTP Sub)            TLS OK (0.7s)
  ⚠️  SMTP :25 relay                 no data [WARNING]
  ⚠️  SMTP :587 local TLS            no data [WARNING]
  ✅ Webmail HTTPS                  HTTP 429 (0.6s)
  ⚠️  Webmail internal               no data [WARNING]
  ⚠️  SnappyMail internal            no data [WARNING]
  ⚠️  ManageSieve :4190              no data [WARNING]
  ✅ Authelia health                Authelia OK
  ⚠️  OIDC bearer -> webmail         HTTP 429 (0.6s) [WARNING]
  ⚠️  Stalwart Admin via Bearer      HTTP 429 (0.6s) [WARNING]
  ✅ mcp->DNS resolve               -> 35.226.147.64
  ⚠️  mcp->IMAP TLS                  ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established [WARNING]
  ⚠️  mcp->SMTP TLS                  ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established [WARNING]
  ✅ mcp->IMAP WG direct            10.0.0.3:993 OK proto=TLSv1.3
  ⚠️  mcp->IMAP LOGIN                ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT [WARNING]
  ⚠️  mcp->SMTP AUTH                 ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT [WARNING]
  ⚠️  mail-mcp MCP                   HTTP 429 (alive) (0.6s) [WARNING]
  ❌ All ports bound                no data [CRITICAL]

  Summary: 12/26 passed, 14 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM                           present (0.0s)
  ✅ SPF                            v=spf1 (0.0s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)

  Summary: 4/4 passed, 0 failed

5. MAIL INTERNALS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

6. E2E DELIVERY
──────────────────────────────────────────────────────────────
  ℹ️  Resend API key                 not set (set RESEND_API_KEY to enable E2E) [INFO]

  Summary: 0/1 passed, 1 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    27.2s
  P1_preflight             25.3s
  P0_instant_kpis          1.2s
  P3_network               0.8s
  P4_dns_auth              0.8s
  P2-P5_parallel           0.8s
  P2_containers            0.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s

  Total: 27.2s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 30/49 passed, 5 critical, 12 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
