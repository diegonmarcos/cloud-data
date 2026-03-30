```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-03-30T00:38:52.312875078+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  8 issues: 0 critical, 5 warnings, 3 info

  WARNINGS:
    ⚠️  mcp->IMAP TLS: ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established
    ⚠️  mcp->SMTP TLS: ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established
    ⚠️  mcp->IMAP LOGIN: ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT
    ⚠️  mcp->SMTP AUTH: ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT
    ⚠️  User accounts: unknown ({"data":{"items":[{"id":78,"ty)
  INFO:
    ℹ️  GHA health: 3 failing: Health → Mail (Full Check), Health → VMs
    ℹ️  Admin API domains: 
    ℹ️  Resend API key: not set (set RESEND_API_KEY to enable E2E)


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ mail.* HTTPS                   HTTP 302 (0.6s)
  ✅ webmail HTTPS                  HTTP 200 (0.5s)
  ✅ auth HTTPS                     HTTP 200 (0.6s)
  ✅ MCP endpoint                   HTTP 400 (0.7s)
  ✅ mail:993 TLS                   TLS OK (0.6s)
  ✅ mail:465 TLS                   TLS OK (0.6s)
  ✅ mail:587 STARTTLS              TLS OK (0.6s)
  ✅ MX record                      22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM record                    present (0.0s)
  ℹ️  GHA health                     3 failing: Health → Mail (Full Check), Health → VMs (0.7s) [INFO]

  Summary: 9/10 passed, 1 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ WG oci-mail                    10.0.0.3:22 OK (0.2s)
  ✅ WG oci-apps                    10.0.0.6:22 OK (0.2s)
  ✅ WG gcp-proxy                   10.0.0.1:22 OK (0.1s)
  ✅ SSH batch oci-mail             Docker 27.5.1
  ✅ SSH batch oci-apps             mail-mcp: Up About a minute
  ✅ SSH batch gcp-proxy            Authelia OK
  ✅ Disk space                     68% used
  ✅ Memory                         663/954MB (69%)
  ✅ Load                           load: 0.43 0.71 0.48

  Summary: 9/9 passed, 0 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ stalwart                       Up 6 minutes
  ✅ smtp-proxy                     Up 7 minutes
  ✅ snappymail                     Up 7 minutes
  ✅ mail-mcp                       Up About a minute

  Summary: 4/4 passed, 0 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ Caddy (gcp-proxy)              HTTPS OK (10.0.0.1) (0.7s)
  ✅ Hickory DNS                    stalwart.app -> 10.0.0.3 (0.2s)
  ✅ TLS WG direct                  993 OK 465 OK 587 OK (0.6s)
  ✅ Caddy L4 -> IMAP               993 forwarding OK
  ✅ Caddy L4 -> SMTPS              465 forwarding OK
  ✅ Caddy L4 -> SMTP               587 forwarding OK
  ✅ mail:993 (IMAP)                TLS OK (0.6s)
  ✅ mail:465 (SMTPS)               TLS OK (0.6s)
  ✅ mail:587 (SMTP Sub)            TLS OK (0.6s)
  ✅ SMTP :25 relay                 220 mail.diegonmarcos.com Stalwart ESMTP at your service
  ✅ SMTP :587 local TLS            STARTTLS OK
  ✅ Webmail HTTPS                  HTTP 302 (0.5s)
  ✅ Webmail internal               HTTP 200
  ✅ SnappyMail internal            HTTP 200
  ✅ ManageSieve :4190              ManageSieve OK
  ✅ Authelia health                Authelia OK
  ✅ OIDC bearer -> webmail         Bearer auth -> 200 OK (full chain) (0.9s)
  ✅ Stalwart Admin via Bearer      HTTP 401 (0.9s)
  ✅ mcp->DNS resolve               -> 35.226.147.64
  ⚠️  mcp->IMAP TLS                  ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established [WARNING]
  ⚠️  mcp->SMTP TLS                  ERR:ECONNRESET Client network socket disconnected before secure TLS connection was established [WARNING]
  ✅ mcp->IMAP WG direct            10.0.0.3:993 OK proto=TLSv1.3
  ⚠️  mcp->IMAP LOGIN                ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT [WARNING]
  ⚠️  mcp->SMTP AUTH                 ERR:Client network socket disconnected before secure TLS connection was established
TIMEOUT [WARNING]
  ✅ mail-mcp MCP                   HTTP 400 (alive) (0.7s)
  ✅ All ports bound                all 7 ports bound

  Summary: 22/26 passed, 4 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ MX                             22 route1.mx.cloudflare.net. (0.0s)
  ✅ DKIM                           present (0.0s)
  ✅ SPF                            v=spf1 (0.0s)
  ✅ DMARC                          v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)

  Summary: 4/4 passed, 0 failed

5. MAIL INTERNALS
──────────────────────────────────────────────────────────────
  ✅ IMAP auth                      Stalwart IMAP responding
  ✅ IMAP protocol                  IMAP4rev1
  ✅ spam filter                    Stalwart built-in
  ✅ data store                     RocksDB
  ✅ admin panel                    HTTP 200
  ✅ sieve filter                   Stalwart ManageSieve
  ✅ mailbox quota                  stalwart-builtin-quota
  ✅ Admin API accounts             {"data":{"items":[{"id":78,"type":"individual","name":"admin
  ℹ️  Admin API domains               [INFO]
  ✅ Mail queue                     empty
  ⚠️  User accounts                  unknown ({"data":{"items":[{"id":78,"ty) [WARNING]

  Summary: 9/11 passed, 2 failed

6. E2E DELIVERY
──────────────────────────────────────────────────────────────
  ℹ️  Resend API key                 not set (set RESEND_API_KEY to enable E2E) [INFO]

  Summary: 0/1 passed, 1 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    22.0s
  P1_preflight             19.8s
  P0_instant_kpis          1.3s
  P2-P5_parallel           0.9s
  P3_network               0.9s
  P4_dns_auth              0.9s
  P2_containers            0.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s

  Total: 22.0s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: DEGRADED -- 57/65 passed, 5 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
