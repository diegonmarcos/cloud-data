```

  ███╗   ███╗ █████╗ ██╗██╗
  ████╗ ████║██╔══██╗██║██║
  ██╔████╔██║███████║██║██║
  ██║╚██╔╝██║██╔══██║██║██║
  ██║ ╚═╝ ██║██║  ██║██║███████╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
  CLOUD MAIL FULL — 2026-03-29T21:16:09.232200026+00:00
══════════════════════════════════════════════════════════════

  ISSUES FOUND
══════════════════════════════════════════════════════════════
  22 issues: 4 critical, 14 warnings, 4 info

  CRITICAL:
    ❌ Caddy L4 -> IMAP: no proxy data
    ❌ Caddy L4 -> SMTPS: no proxy data
    ❌ Caddy L4 -> SMTP: no proxy data
    ❌ All ports bound: missing: 8443
  WARNINGS:
    ⚠️  MCP endpoint: HTTP 502
    ⚠️  SSH batch oci-apps: mail-mcp: 
    ⚠️  SSH batch gcp-proxy: SSH FAILED
    ⚠️  mail-mcp: 
    ⚠️  Authelia health: no proxy data
    ⚠️  mcp->DNS resolve: Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running
    ⚠️  mcp->IMAP TLS: Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running
    ⚠️  mcp->SMTP TLS: Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running
    ⚠️  mcp->IMAP WG direct: 10.0.0.3:993 Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running
    ⚠️  mcp->IMAP LOGIN: Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running
    ⚠️  mcp->SMTP AUTH: Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running
    ⚠️  mail-mcp MCP: HTTP 502 (alive)
    ⚠️  admin panel: HTTP 000
    ⚠️  User accounts: unknown ()
  INFO:
    ℹ️  GHA health: 2 failing: Health → Mail (Full Check), Health → Mail (Full Check)
    ℹ️  Admin API accounts: 
    ℹ️  Admin API domains: 
    ℹ️  Resend API key: not set (set RESEND_API_KEY to enable E2E)


0. INSTANT KPIs
──────────────────────────────────────────────────────────────
  ✅ HTTP 302 (0.5s)
  ✅ HTTP 200 (0.5s)
  ✅ HTTP 200 (0.5s)
  ⚠️  HTTP 502 (0.6s) [WARNING]
  ✅ TLS OK (0.8s)
  ✅ TLS OK (1.1s)
  ✅ TLS OK (1.7s)
  ✅ 22 route1.mx.cloudflare.net. (0.0s)
  ✅ present (0.1s)
  ℹ️  2 failing: Health → Mail (Full Check), Health → Mail (Full Check) (0.7s) [INFO]

  Summary: 8/10 passed, 2 failed

1. PRE-FLIGHT
──────────────────────────────────────────────────────────────
  ✅ 10.0.0.3:22 OK (0.3s)
  ✅ 10.0.0.6:22 OK (0.3s)
  ✅ 10.0.0.1:22 OK (0.1s)
  ✅ Docker 27.5.1
  ⚠️  mail-mcp:  [WARNING]
  ⚠️  SSH FAILED [WARNING]
  ✅ 68% used
  ✅ 667/954MB (70%)
  ✅ load: 0.37 0.64 1.13

  Summary: 7/9 passed, 2 failed

2. CONTAINERS
──────────────────────────────────────────────────────────────
  ✅ Up 54 minutes
  ✅ Up 54 minutes
  ✅ Up 54 minutes
  ⚠️   [WARNING]

  Summary: 3/4 passed, 1 failed

3. NETWORK + AUTH
──────────────────────────────────────────────────────────────
  ✅ HTTPS OK (10.0.0.1) (0.6s)
  ✅ stalwart.app -> 10.0.0.3 (0.1s)
  ✅ 993 OK 465 OK 587 OK (1.3s)
  ❌ no proxy data [CRITICAL]
  ❌ no proxy data [CRITICAL]
  ❌ no proxy data [CRITICAL]
  ✅ TLS OK (1.2s)
  ✅ TLS OK (0.9s)
  ✅ TLS OK (2.2s)
  ✅ 220 mail.diegonmarcos.com Stalwart ESMTP at your service
  ✅ STARTTLS OK
  ✅ HTTP 302 (0.5s)
  ✅ HTTP 200
  ✅ HTTP 200
  ✅ ManageSieve OK
  ⚠️  no proxy data [WARNING]
  ✅ Bearer auth -> 200 OK (full chain) (0.7s)
  ✅ HTTP 401 (1.0s)
  ⚠️  Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running [WARNING]
  ⚠️  Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running [WARNING]
  ⚠️  Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running [WARNING]
  ⚠️  10.0.0.3:993 Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running [WARNING]
  ⚠️  Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running [WARNING]
  ⚠️  Error response from daemon: container 8c7c0c47d5bd305730a8cf5b6ed63092a040f07a82742de52df26662fed2e1c7 is not running [WARNING]
  ⚠️  HTTP 502 (alive) (0.6s) [WARNING]
  ❌ missing: 8443 [CRITICAL]

  Summary: 14/26 passed, 12 failed

4. DNS AUTH
──────────────────────────────────────────────────────────────
  ✅ 22 route1.mx.cloudflare.net. (0.1s)
  ✅ present (0.0s)
  ✅ v=spf1 (0.0s)
  ✅ v=DMARC1; p=reject; sp=reject; rua=mailto:postmaster@diegonmarcos.com; ruf=mailto:postmaster@diegonmarcos.com; fo=1 (0.0s)

  Summary: 4/4 passed, 0 failed

5. MAIL INTERNALS
──────────────────────────────────────────────────────────────
  ✅ Stalwart IMAP responding
  ✅ IMAP4rev1
  ✅ Stalwart built-in
  ✅ RocksDB
  ⚠️  HTTP 000 [WARNING]
  ✅ Stalwart ManageSieve
  ✅ stalwart-builtin-quota
  ℹ️   [INFO]
  ℹ️   [INFO]
  ✅ 
  ⚠️  unknown () [WARNING]

  Summary: 7/11 passed, 4 failed

6. E2E DELIVERY
──────────────────────────────────────────────────────────────
  ℹ️  not set (set RESEND_API_KEY to enable E2E) [INFO]

  Summary: 0/1 passed, 1 failed

══════════════════════════════════════════════════════════════
PERFORMANCE
══════════════════════════════════════════════════════════════
  TOTAL                    40.0s
  P1_preflight             35.3s
  P0_instant_kpis          2.5s
  P3_network               2.2s
  P4_dns_auth              2.2s
  P2-P5_parallel           2.2s
  P2_containers            0.0s
  P6_e2e_delivery          0.0s
  P5_internals             0.0s

  Total: 40.0s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux) TLS(openssl)

══════════════════════════════════════════════════════════════
RESULT: CRITICAL -- 43/65 passed, 4 critical, 14 warnings
══════════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
Generated by: cloud-data/cloud-health-stack/cloud-mail-full-report
Run: cargo run --release
```
