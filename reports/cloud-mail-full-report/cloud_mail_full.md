```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-04-03T14:19:21.986024745+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  14 issues: 6 critical, 8 warnings, 0 info

  CRITICAL:
    ❌ SSH batch oci-mail: SSH to oci-mail failed: fish: Unsupported use of '='. In fish, please use 'set ADMIN_CREDS $(grep ADMIN_PASSWORD /opt/stalwart/.secrets 2>/dev/null | head -1 | cut -d= -f2-)'.
ADMIN_CREDS=$(grep ADMIN_PASSWORD /opt/stalwart/.secrets 2>/dev/null | head -1 | cut -d= -f2-)
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
    ❌ skipped: SSH unreachable
    ❌ Hickory DNS: FAIL: 10.0.0.1
    ❌ TLS WG direct: SSH down
    ❌ All ports bound: no data
    ❌ skipped: SSH unreachable
  WARNINGS:
    ⚠️  Outbound queue: 4 messages stuck in queue
    ⚠️  SMTP :25 relay: no data
    ⚠️  SMTP :587 local TLS: no data
    ⚠️  Webmail internal: no data
    ⚠️  SnappyMail internal: no data
    ⚠️  ManageSieve :4190: no data
    ⚠️  OIDC bearer -> webmail: no OIDC token
    ⚠️  Stalwart Admin via Bearer: no OIDC token


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 302 (0.5s)
  ✅ webmail HTTPS                  HTTP 200 (0.5s)
  ✅ auth HTTPS                     HTTP 200 (0.5s)
  ✅ MCP endpoint                   HTTP 400 (0.7s)
  ✅ mail:993 TLS                   TLS OK (1.0s)
  ✅ mail:465 TLS                   TLS OK (1.0s)
  ✅ mail:587 STARTTLS              TLS OK (1.8s)
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM record                    present (0.1s)
  ✅ GHA health                     1 failing: Health (1.0s)
  ✅ CF Worker alive                HTTP 500 (0.3s)
  ✅ Google OAuth reachable         HTTP 404 (token endpoint) (0.5s)
  ✅ Domain principal               1 domains in RocksDB (0.8s)
  ⚠️  Outbound queue                 4 messages stuck in queue (0.8s) [WARNING]

  Summary: 13/14 passed, 1 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ Cloud API oci-mail             oci-mail: RUNNING
  ✅ Cloud API oci-apps             oci-apps: RUNNING
  ✅ Cloud API gcp-proxy            gcp-proxy: RUNNING
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.2s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.2s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.1s)
  ❌ SSH batch oci-mail             SSH to oci-mail failed: fish: Unsupported use of '='. In fish, please use 'set ADMIN_CREDS $(grep ADMIN_PASSWORD /opt/stalwart/.secrets 2>/dev/null | head -1 | cut -d= -f2-)'.
ADMIN_CREDS=$(grep ADMIN_PASSWORD /opt/stalwart/.secrets 2>/dev/null | head -1 | cut -d= -f2-)
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^ [CRITICAL]
  ✅ SSH batch oci-apps             mail-mcp: Up 2 days
  ✅ SSH batch gcp-proxy            Authelia OK

  Summary: 8/9 passed, 1 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (10.0.0.1) (0.7s)
  ❌ Hickory DNS                    FAIL: 10.0.0.1 (0.1s) [CRITICAL]
  ❌ TLS WG direct                  SSH down [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ✅ mail:993 (IMAP)                TLS OK (0.9s)
  ✅ mail:465 (SMTPS)               TLS OK (0.9s)
  ✅ mail:587 (SMTP Sub)            TLS OK (1.6s)
  ⚠️  SMTP :25 relay                 no data [WARNING]
  ⚠️  SMTP :587 local TLS            no data [WARNING]
  ✅ Webmail HTTPS                  HTTP 302 (0.5s)
  ⚠️  Webmail internal               no data [WARNING]
  ⚠️  SnappyMail internal            no data [WARNING]
  ⚠️  ManageSieve :4190              no data [WARNING]
  ✅ Authelia health                Authelia OK
  ⚠️  OIDC bearer -> webmail         no OIDC token [WARNING]
  ⚠️  Stalwart Admin via Bearer      no OIDC token [WARNING]
  ✅ mcp->DNS resolve               -> 35.226.147.64
  ✅ mcp->IMAP TLS                  OK proto=TLSv1.3 cn=mail.diegonmarcos.com
  ✅ mcp->SMTP TLS                  OK proto=TLSv1.3
  ✅ mcp->IMAP WG direct            10.0.0.3:993 OK proto=TLSv1.3
  ✅ mcp->IMAP LOGIN                LOGIN_OK
TIMEOUT
  ✅ mcp->SMTP AUTH                 SMTP_AUTH_OK: 250-AUTH PLAIN LOGIN
SMTP_AUTH_OK: 250-AUTH PLAIN LOGIN
ERR:write after end
TIMEOUT
  ✅ mail-mcp MCP                   HTTP 400 (alive) (0.7s)
  ❌ All ports bound                no data [CRITICAL]

  Summary: 16/26 passed, 10 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.1s)
  ✅ DKIM                           present (0.1s)
  ✅ SPF                            v=spf1 (0.1s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.1s)

  Summary: 4/4 passed, 0 failed

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
  TOTAL                    29.9s
  P1_preflight             22.4s
  P0_instant_kpis          3.5s
  T0_path_checker          2.5s
  P2-P5_parallel           1.6s
  P3_network               1.6s
  P4_dns_auth              1.6s
  P2_containers            0.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s

  Total: 29.9s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 51/66 passed, 7 critical, 8 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
