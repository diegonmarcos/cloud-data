# reports-logs — evidence collector for all reports

**Purpose**: single, data-driven collector that captures every raw signal the 6
reports need for debugging. Anything a report flags as an issue should have a
corresponding artefact here so the root cause is one `cat`/`less` away.

## Config (source of truth)

`cloud-data/cloud-data-reports-logs.json` — declares:
- VMs (cross-referenced against `cloud-data-topology.json`)
- systemd units to capture per VM
- network commands to run per VM
- DNS domains + resolvers + record types
- TLS endpoints to probe (host/port/SNI/STARTTLS)
- mail service locations (Maddy / Stalwart)
- Cloudflare zone + record types

Zero hardcoded data in scripts. Changing what is collected = edit that JSON.

## Usage

```sh
reports-logs/build.sh all          # run every enabled module
reports-logs/build.sh docker       # just docker evidence
reports-logs/build.sh systemd      # journalctl per declared unit
reports-logs/build.sh network      # ss / ip / wg / firewall
reports-logs/build.sh dns          # dig all declared domains
reports-logs/build.sh tls          # openssl s_client per endpoint
reports-logs/build.sh mail         # Maddy + Stalwart diagnostics
reports-logs/build.sh cloudflare   # zone records via CF API
reports-logs/build.sh list         # modules + last manifest
reports-logs/build.sh clean        # wipe dist/
```

## Output tree

```
dist/
├── vms/<vm>/
│   ├── docker_ps.json             # docker ps --format '{{json .}}'
│   ├── journal_<unit>.log         # per declared unit
│   ├── systemd_failed.txt         # systemctl --failed
│   ├── network.txt                # ss + ip + wg + firewall
│   └── meta.json                  # vm reachability + timestamp
├── containers/<service>/<container>/
│   ├── inspect.json
│   └── logs.txt
├── dns/<domain>.txt                # per-resolver × per-type results
├── tls/<host>_<port>.txt           # cert chain + cipher + protocol
├── mail/
│   ├── maddy_cert.txt
│   ├── maddy_listeners.txt
│   ├── maddy_creds.txt
│   └── stalwart_listeners.txt
├── cloudflare/
│   ├── all_records.json
│   └── <TYPE>.json                 # filtered per type
└── latest.json                     # top-level manifest
```

## How reports consume this

Rust crates that need evidence should read `dist/latest.json` for freshness +
hit the per-kind paths directly. Example:

```rust
let path = "reports-logs/dist/vms/oci-apps/docker_ps.json";
let containers: Vec<serde_json::Value> = serde_json::from_reader(File::open(path)?)?;
```

No shelling-out from inside reports — the collector is run independently (cron /
Dagu / manual) and reports just read the snapshot.

## Suggested cadence

- Lightweight (`network`, `dns`): every 5 min
- Medium (`docker`, `systemd`): every 15 min
- Heavy (`tls`, `cloudflare`, `mail`): every hour

Declare in Dagu DAG or systemd timer (follow `cloud-data-reports-logs.json`
retention_days for rotation).

## Tester

```sh
reports-logs/build.sh docker
jq . reports-logs/dist/latest.json
ls reports-logs/dist/vms/*/docker_ps.json | xargs -I{} sh -c 'echo "== {} =="; jq "length" {}'
```

Expected: one directory per reachable VM, container count ≥ 1 for each, non-empty
`inspect.json` + `logs.txt` per container.
