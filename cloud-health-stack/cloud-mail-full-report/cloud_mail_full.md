```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-03-30T14:39:39.934207595+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  12 issues: 6 critical, 6 warnings, 0 info

  CRITICAL:
    ❌ SSH batch oci-mail: SSH FAILED
    ❌ skipped: SSH unreachable
    ❌ TLS WG direct: SSH down
    ❌ All ports bound: no data
    ❌ skipped: SSH unreachable
    ❌ IMAP arrival: SSH down
  WARNINGS:
    ⚠️  SMTP :25 relay: no data
    ⚠️  SMTP :587 local TLS: no data
    ⚠️  Webmail internal: no data
    ⚠️  SnappyMail internal: no data
    ⚠️  ManageSieve :4190: no data
    ⚠️  smtp-proxy logs: SSH down


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 302 (0.5s)
  ✅ webmail HTTPS                  HTTP 200 (0.5s)
  ✅ auth HTTPS                     HTTP 200 (0.5s)
  ✅ MCP endpoint                   HTTP 400 (0.7s)
  ✅ mail:993 TLS                   TLS OK (0.9s)
  ✅ mail:465 TLS                   TLS OK (0.9s)
  ✅ mail:587 STARTTLS              TLS OK (1.8s)
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM record                    present (0.0s)
  ✅ GHA health                     all green (0.9s)

  Summary: 10/10 passed, 0 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.2s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.2s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.2s)
  ❌ SSH batch oci-mail             SSH FAILED [CRITICAL]
  ✅ SSH batch oci-apps             mail-mcp: Up 14 hours
  ✅ SSH batch gcp-proxy            Authelia OK

  Summary: 5/6 passed, 1 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (10.0.0.1) (0.6s)
  ✅ Hickory DNS                    stalwart.app -> 10.0.0.3 (0.1s)
  ❌ TLS WG direct                  SSH down [CRITICAL]
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ✅ mail:993 (IMAP)                TLS OK (1.9s)
  ✅ mail:465 (SMTPS)               TLS OK (1.9s)
  ✅ mail:587 (SMTP Sub)            TLS OK (3.3s)
  ⚠️  SMTP :25 relay                 no data [WARNING]
  ⚠️  SMTP :587 local TLS            no data [WARNING]
  ✅ Webmail HTTPS                  HTTP 302 (0.5s)
  ⚠️  Webmail internal               no data [WARNING]
  ⚠️  SnappyMail internal            no data [WARNING]
  ⚠️  ManageSieve :4190              no data [WARNING]
  ✅ Authelia health                Authelia OK
  ✅ OIDC bearer -> webmail         Bearer auth -> 200 OK (full chain) (3.0s)
  ✅ Stalwart Admin via Bearer      HTTP 401 (2.7s)
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

  Summary: 19/26 passed, 7 failed

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
  ✅ Resend API key                 found
  ✅ Send via Resend                id=867339e8-4201-4827-870d-c3e5652b85c1 (0.3s)
  ✅ Resend status                  sent (IMAP is truth) (3.6s)
  ❌ IMAP arrival                   SSH down [CRITICAL]
  ⚠️  smtp-proxy logs                SSH down [WARNING]
  ✅ CF Worker                      info: no CF creds

  Summary: 4/6 passed, 2 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    55.2s
  P1_preflight             45.3s
  P6_e2e_delivery          3.9s
  P2-P5_parallel           3.3s
  P3_network               3.3s
  P4_dns_auth              3.3s
  P0_instant_kpis          2.7s
  P5_internals             0.0s
  P2_containers            0.0s

  Total: 55.2s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 42/54 passed, 6 critical, 6 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
