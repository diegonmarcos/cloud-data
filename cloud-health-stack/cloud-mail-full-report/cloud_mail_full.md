```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-03-31T16:43:18.316394554+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  42 issues: 22 critical, 20 warnings, 0 info

  CRITICAL:
    ❌ mail.* HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    ❌ webmail HTTPS: HTTP err: error sending request for url (https://webmail.diegonmarcos.com/)
    ❌ mail:993 TLS: timeout/error
    ❌ mail:465 TLS: timeout/error
    ❌ mail:587 STARTTLS: timeout/error
    ❌ Cloud API oci-mail: oci-mail: API_FAIL
    ❌ Cloud API oci-apps: oci-apps: API_FAIL
    ❌ Cloud API gcp-proxy: gcp-proxy: API_FAIL
    ❌ WG gcp-proxy: WG DOWN
    ❌ SSH batch oci-mail: SSH :22 failed (Connection timed out during banner exchange
Connection to 10.0.0.3 port 22 timed out) + Dropbear :2200 down — OCI status: API_FAIL
    ❌ skipped: SSH unreachable
    ❌ Caddy (gcp-proxy): Caddy DOWN
    ❌ Hickory DNS: FAIL: no response
    ❌ TLS WG direct: SSH down
    ❌ Caddy L4 -> IMAP: no proxy data
    ❌ Caddy L4 -> SMTPS: no proxy data
    ❌ Caddy L4 -> SMTP: no proxy data
    ❌ mail:993 (IMAP): timeout/error
    ❌ mail:465 (SMTPS): timeout/error
    ❌ mail:587 (SMTP Sub): timeout/error
    ❌ All ports bound: no data
    ❌ skipped: SSH unreachable
  WARNINGS:
    ⚠️  auth HTTPS: HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health)
    ⚠️  MCP endpoint: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp)
    ⚠️  SSH batch oci-apps: SSH :22 failed (Dropbear :2200 alive — VM under load, not down)
    ⚠️  SSH batch gcp-proxy: SSH :22 failed (ssh: connect to host 10.0.0.1 port 22: Connection timed out) + Dropbear :2200 down — GCP status: API_FAIL | SERIAL: gcloud timeout
    ⚠️  SMTP :25 relay: no data
    ⚠️  SMTP :587 local TLS: no data
    ⚠️  Webmail HTTPS: HTTP err: error sending request for url (https://mail.diegonmarcos.com/)
    ⚠️  Webmail internal: no data
    ⚠️  SnappyMail internal: no data
    ⚠️  ManageSieve :4190: no data
    ⚠️  Authelia health: no proxy data
    ⚠️  OIDC bearer -> webmail: no OIDC token
    ⚠️  Stalwart Admin via Bearer: no OIDC token
    ⚠️  mcp->DNS resolve: no app data
    ⚠️  mcp->IMAP TLS: no app data
    ⚠️  mcp->SMTP TLS: no app data
    ⚠️  mcp->IMAP WG direct: no app data
    ⚠️  mcp->IMAP LOGIN: no app data
    ⚠️  mcp->SMTP AUTH: no app data
    ⚠️  mail-mcp MCP: HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive)


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ❌ mail.* HTTPS                   HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (5.2s) [CRITICAL]
  ❌ webmail HTTPS                  HTTP err: error sending request for url (https://webmail.diegonmarcos.com/) (5.3s) [CRITICAL]
  ⚠️  auth HTTPS                     HTTP err: error sending request for url (https://auth.diegonmarcos.com/api/health) (5.2s) [WARNING]
  ⚠️  MCP endpoint                   HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (5.2s) [WARNING]
  ❌ mail:993 TLS                   timeout/error (5.0s) [CRITICAL]
  ❌ mail:465 TLS                   timeout/error (5.0s) [CRITICAL]
  ❌ mail:587 STARTTLS              timeout/error (5.0s) [CRITICAL]
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.1s)
  ✅ DKIM record                    present (0.1s)
  ✅ GHA health                     gh unavailable (skipped) (12.0s)

  Summary: 3/10 passed, 7 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ❌ Cloud API oci-mail             oci-mail: API_FAIL [CRITICAL]
  ❌ Cloud API oci-apps             oci-apps: API_FAIL [CRITICAL]
  ❌ Cloud API gcp-proxy            gcp-proxy: API_FAIL [CRITICAL]
  ✅ WG oci-mail                    10.0.0.3:22 OK (1.4s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.3s)
  ❌ WG gcp-proxy                   WG DOWN (3.0s) [CRITICAL]
  ❌ SSH batch oci-mail             SSH :22 failed (Connection timed out during banner exchange
Connection to 10.0.0.3 port 22 timed out) + Dropbear :2200 down — OCI status: API_FAIL [CRITICAL]
  ⚠️  SSH batch oci-apps             SSH :22 failed (Dropbear :2200 alive — VM under load, not down) [WARNING]
  ⚠️  SSH batch gcp-proxy            SSH :22 failed (ssh: connect to host 10.0.0.1 port 22: Connection timed out) + Dropbear :2200 down — GCP status: API_FAIL | SERIAL: gcloud timeout [WARNING]

  Summary: 2/9 passed, 7 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ❌ skipped                        SSH unreachable [CRITICAL]

  Summary: 0/1 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ❌ Caddy (gcp-proxy)              Caddy DOWN (8.0s) [CRITICAL]
  ❌ Hickory DNS                    FAIL: no response (3.0s) [CRITICAL]
  ❌ TLS WG direct                  SSH down [CRITICAL]
  ❌ Caddy L4 -> IMAP               no proxy data [CRITICAL]
  ❌ Caddy L4 -> SMTPS              no proxy data [CRITICAL]
  ❌ Caddy L4 -> SMTP               no proxy data [CRITICAL]
  ❌ mail:993 (IMAP)                timeout/error (5.0s) [CRITICAL]
  ❌ mail:465 (SMTPS)               timeout/error (5.0s) [CRITICAL]
  ❌ mail:587 (SMTP Sub)            timeout/error (5.0s) [CRITICAL]
  ⚠️  SMTP :25 relay                 no data [WARNING]
  ⚠️  SMTP :587 local TLS            no data [WARNING]
  ⚠️  Webmail HTTPS                  HTTP err: error sending request for url (https://mail.diegonmarcos.com/) (5.2s) [WARNING]
  ⚠️  Webmail internal               no data [WARNING]
  ⚠️  SnappyMail internal            no data [WARNING]
  ⚠️  ManageSieve :4190              no data [WARNING]
  ⚠️  Authelia health                no proxy data [WARNING]
  ⚠️  OIDC bearer -> webmail         no OIDC token [WARNING]
  ⚠️  Stalwart Admin via Bearer      no OIDC token [WARNING]
  ⚠️  mcp->DNS resolve               no app data [WARNING]
  ⚠️  mcp->IMAP TLS                  no app data [WARNING]
  ⚠️  mcp->SMTP TLS                  no app data [WARNING]
  ⚠️  mcp->IMAP WG direct            no app data [WARNING]
  ⚠️  mcp->IMAP LOGIN                no app data [WARNING]
  ⚠️  mcp->SMTP AUTH                 no app data [WARNING]
  ⚠️  mail-mcp MCP                   HTTP err: error sending request for url (https://mcp.diegonmarcos.com/mail-mcp/mcp) (alive) (5.5s) [WARNING]
  ❌ All ports bound                no data [CRITICAL]

  Summary: 0/26 passed, 26 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.1s)
  ✅ DKIM                           present (0.1s)
  ✅ SPF                            v=spf1 (0.0s)
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
  TOTAL                    137.3s
  T0_path_checker          59.2s
  P1_preflight             53.1s
  P0_instant_kpis          17.0s
  P4_dns_auth              8.0s
  P3_network               8.0s
  P2-P5_parallel           8.0s
  P6_e2e_delivery          0.0s
  P2_containers            0.0s
  P5_internals             0.0s

  Total: 137.3s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 15/62 passed, 27 critical, 20 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
