use crate::checks::*;
use crate::ssh;
use crate::types::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 1: SELF-CHECK
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_self_check(ctx: &Context) -> Vec<Check> {
    let mut checks = Vec::new();

    // C3 API mesh (internal via WG)
    {
        let t = Instant::now();
        let client = http_client();
        let (ok, _code, detail) =
            http_get(&client, "http://10.0.0.6:8081/health").await;
        checks.push(Check {
            name: "C3 API (mesh)".into(),
            passed: ok,
            details: format!("http://10.0.0.6:8081/health -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Critical,
        });
    }

    // C3 API public
    {
        let t = Instant::now();
        let client = http_client();
        let (ok, _code, detail) =
            http_get(&client, "https://api.diegonmarcos.com/c3-api/health").await;
        checks.push(Check {
            name: "C3 API (public)".into(),
            passed: ok,
            details: format!(
                "https://api.diegonmarcos.com/c3-api/health -> {}",
                detail
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Warning,
        });
    }

    // WG interface — TCP to hub
    {
        let t = Instant::now();
        let ok = tcp("10.0.0.1", 22).await;
        checks.push(Check {
            name: "WireGuard interface".into(),
            passed: ok,
            details: format!("TCP 10.0.0.1:22 -> {}", if ok { "open" } else { "closed" }),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("WG tunnel down".into())
            },
            severity: Severity::Critical,
        });
    }

    // Local docker
    {
        let t = Instant::now();
        let out = tokio::process::Command::new("docker")
            .args(["info", "--format", "{{.ServerVersion}}"])
            .output()
            .await;
        let (ok, detail) = match out {
            Ok(o) if o.status.success() => {
                let ver = String::from_utf8_lossy(&o.stdout).trim().to_string();
                (true, format!("Docker {}", ver))
            }
            Ok(o) => (
                false,
                String::from_utf8_lossy(&o.stderr).trim().to_string(),
            ),
            Err(e) => (false, format!("error: {}", e)),
        };
        checks.push(Check {
            name: "Local docker".into(),
            passed: ok,
            details: detail.clone(),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Info,
        });
    }

    // SSH agent
    {
        let t = Instant::now();
        let out = tokio::process::Command::new("ssh-add")
            .arg("-l")
            .output()
            .await;
        let (ok, detail) = match out {
            Ok(o) if o.status.success() => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                let count = stdout.lines().count();
                (true, format!("{} keys loaded", count))
            }
            _ => (false, "no SSH agent or no keys".into()),
        };
        checks.push(Check {
            name: "SSH agent".into(),
            passed: ok,
            details: detail.clone(),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Warning,
        });
    }

    // cloud-data freshness
    {
        let t = Instant::now();
        let meta_ts = ctx.consolidated["_meta"]["generated_at"]
            .as_str()
            .unwrap_or("");
        let (ok, detail) = if meta_ts.is_empty() {
            (false, "no generated_at in cloud-data".to_string())
        } else if let Ok(gen_time) = chrono::DateTime::parse_from_rfc3339(meta_ts) {
            let age = chrono::Utc::now().signed_duration_since(gen_time);
            let hours = age.num_hours();
            (
                hours < 24,
                format!("generated {} ({}h ago)", meta_ts, hours),
            )
        } else {
            (false, format!("unparseable timestamp: {}", meta_ts))
        };
        checks.push(Check {
            name: "cloud-data freshness".into(),
            passed: ok,
            details: detail.clone(),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Warning,
        });
    }

    // DNS resolver (Hickory)
    {
        let t = Instant::now();
        let resolver = hickory_resolver();
        let ip = dns_resolve(&resolver, "caddy.app").await;
        let ok = ip.is_some();
        let detail = format!(
            "dig caddy.app @10.0.0.1 -> {}",
            ip.as_deref().unwrap_or("NXDOMAIN")
        );
        checks.push(Check {
            name: "Hickory DNS resolver".into(),
            passed: ok,
            details: detail.clone(),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Critical,
        });
    }

    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 2: WIREGUARD MESH
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_wg_mesh(ctx: &Context) -> (Vec<Check>, Vec<String>) {
    let mut checks = Vec::new();
    let mut reachable_vms = Vec::new();

    let futs: Vec<_> = ctx
        .vms
        .iter()
        .map(|vm| {
            let alias = vm.alias.clone();
            let wg_ip = vm.wg_ip.clone();
            let pub_ip = vm.pub_ip.clone();
            let cloud_name = vm.cloud_name.clone();
            let rescue_port = vm.rescue_port;
            let is_spot = vm.cost.to_lowercase() == "spot";
            let provider = vm.provider.clone();
            let vm_id = vm.vm_id.clone();
            async move {
                let t = Instant::now();

                // ── Tier 1: Cloud API (gcloud/oci CLI) ──
                let cloud_status = cloud_vm_status(&vm_id, &cloud_name, &provider);
                let cloud_ok = cloud_status.as_deref() == Some("RUNNING");

                // ── Tier 2: Dropbear rescue SSH ──
                let dropbear_ok = if !pub_ip.is_empty() && pub_ip != "?" && pub_ip != "dynamic" {
                    tcp(&pub_ip, rescue_port).await
                } else {
                    false
                };

                // ── Tier 3: SSH via WG ──
                let tcp_ok = tcp(&wg_ip, 22).await;
                let ssh_ok = if tcp_ok {
                    ssh::ssh_echo_test(&alias).await
                } else {
                    false
                };

                let raw_passed = tcp_ok && ssh_ok;

                // Build diagnostic detail with all tiers
                let detail = format!(
                    "{} ({}): VPS={} Dropbear={} WG:TCP={} SSH={}{}",
                    alias, wg_ip,
                    cloud_status.as_deref().unwrap_or("?"),
                    if dropbear_ok { "ok" } else { "fail" },
                    if tcp_ok { "ok" } else { "fail" },
                    if ssh_ok { "ok" } else { "fail" },
                    if is_spot && !raw_passed { " [spot instance]" } else { "" }
                );

                // Severity: spot→Info, cloud dead→Critical, SSH fail only→Warning
                let (passed, severity) = if !raw_passed && is_spot {
                    (true, Severity::Info)
                } else if !cloud_ok && !raw_passed {
                    (false, Severity::Critical) // VM itself is down
                } else if cloud_ok && !raw_passed {
                    (false, Severity::Warning) // VM alive but SSH unreachable
                } else {
                    (raw_passed, Severity::Info)
                };
                (
                    Check {
                        name: format!("WG {}", alias),
                        passed,
                        details: detail.clone(),
                        duration_ms: t.elapsed().as_millis() as u64,
                        error: if passed { None } else { Some(detail) },
                        severity,
                    },
                    alias,
                    raw_passed,
                )
            }
        })
        .collect();

    let results = futures::future::join_all(futs).await;
    for (check, alias, passed) in results {
        if passed {
            reachable_vms.push(alias);
        }
        checks.push(check);
    }

    (checks, reachable_vms)
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 3: PLATFORM (rsync agent + parse)
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_platform(
    ctx: &Context,
    reachable_vms: &[String],
) -> (Vec<Check>, HashMap<String, VmBatchData>, Vec<String>, Vec<String>) {
    let mut checks = Vec::new();
    let mut vm_batch: HashMap<String, VmBatchData> = HashMap::new();
    let mut ssh_ok_vms = Vec::new();
    let mut docker_ok_vms = Vec::new();

    // Filter to only reachable VMs, also skip terminated spot instances
    let active_vms: Vec<&VmInfo> = ctx
        .vms
        .iter()
        .filter(|vm| {
            if !reachable_vms.contains(&vm.alias) {
                return false;
            }
            if vm.vm_id.contains("-p_") && vm.vm_id.starts_with("gcp-") {
                return gcloud_status(&vm.cloud_name)
                    .map(|s| s == "RUNNING")
                    .unwrap_or(false);
            }
            true
        })
        .collect();

    let futs: Vec<_> = active_vms
        .iter()
        .map(|vm| {
            let alias = vm.alias.clone();
            async move {
                let t = Instant::now();
                let data = ssh::rsync_health(&alias).await;
                (alias, t.elapsed().as_millis() as u64, data)
            }
        })
        .collect();

    let results = futures::future::join_all(futs).await;

    for (alias, duration, data) in results {
        match data {
            Some(batch) => {
                ssh_ok_vms.push(alias.clone());
                let has_docker = batch.containers_total > 0 || !batch.docker_version.is_empty();
                if has_docker {
                    docker_ok_vms.push(alias.clone());
                }
                let detail = format!(
                    "{}: mem {}%, disk {}, load {}, {}/{} containers, up {}",
                    alias,
                    batch.mem_pct,
                    batch.disk_pct,
                    batch.load,
                    batch.containers_running,
                    batch.containers_total,
                    batch.uptime
                );
                // Warn on high resource usage
                let severity = if batch.mem_pct > 90
                    || batch
                        .disk_pct
                        .trim_end_matches('%')
                        .parse::<u32>()
                        .unwrap_or(0)
                        > 90
                {
                    Severity::Warning
                } else {
                    Severity::Info
                };
                checks.push(Check {
                    name: format!("Platform {}", alias),
                    passed: true,
                    details: detail,
                    duration_ms: duration,
                    error: None,
                    severity,
                });
                vm_batch.insert(alias, batch);
            }
            None => {
                checks.push(Check {
                    name: format!("Platform {}", alias),
                    passed: false,
                    details: format!("{}: rsync failed", alias),
                    duration_ms: duration,
                    error: Some("rsync /opt/health/latest.json failed".into()),
                    severity: Severity::Warning,
                });
                // Insert empty batch data for unreachable
                vm_batch.insert(
                    alias.clone(),
                    VmBatchData {
                        alias,
                        ..Default::default()
                    },
                );
            }
        }
    }

    // Add checks for unreachable VMs
    for vm in &ctx.vms {
        if !reachable_vms.contains(&vm.alias) {
            let is_spot = vm.cost.to_lowercase() == "spot";
            let (passed, severity) = if is_spot {
                (true, Severity::Info)
            } else {
                (false, Severity::Critical)
            };
            checks.push(Check {
                name: format!("Platform {}", vm.alias),
                passed,
                details: format!(
                    "{}: unreachable (WG down){}",
                    vm.alias,
                    if is_spot { " [spot instance]" } else { "" }
                ),
                duration_ms: 0,
                error: if passed { None } else { Some("VM not reachable via WireGuard".into()) },
                severity,
            });
        }
    }

    (checks, vm_batch, ssh_ok_vms, docker_ok_vms)
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 4: CONTAINERS
// ═══════════════════════════════════════════════════════════════════════════

pub fn layer_containers(ctx: &Context, vm_batch: &HashMap<String, VmBatchData>) -> Vec<Check> {
    let mut checks = Vec::new();

    // Collect valid VM aliases for filtering
    let valid_vm_aliases: HashSet<String> = ctx.vms.iter().map(|vm| vm.alias.clone()).collect();

    for svc in &ctx.services {
        if !svc.enabled { continue; }
        // Skip non-remote targets: "local", "all", or non-existent VM aliases
        if svc.vm_alias == "local" || svc.vm_alias == "all"
            || !valid_vm_aliases.contains(&svc.vm_alias)
        {
            continue;
        }
        for ct in &svc.containers {
            let vm_data = vm_batch.get(&svc.vm_alias);
            let container_found = vm_data.and_then(|vd| {
                vd.containers
                    .iter()
                    .find(|c| c.name == ct.container_name)
            });

            match container_found {
                Some(live_ct) => {
                    let is_init_job = ct.container_name.contains("_init")
                        || ct.container_name.contains("-setup")
                        || ct.container_name.contains("_setup");
                    let is_successful_init = is_init_job
                        && live_ct.status.contains("Exited")
                        && live_ct.status.contains("Exited (0)");
                    let passed = live_ct.up || is_successful_init;
                    let healthy = live_ct.healthy;
                    let severity = if is_successful_init {
                        Severity::Info
                    } else if !passed {
                        Severity::Critical
                    } else if !healthy && ct.healthcheck.is_some() {
                        Severity::Warning
                    } else {
                        Severity::Info
                    };
                    checks.push(Check {
                        name: format!("Container {}/{}", svc.name, ct.container_name),
                        passed,
                        details: format!(
                            "{} on {}: {} ({}){}",
                            ct.container_name, svc.vm_alias, live_ct.status, live_ct.health_state,
                            if is_successful_init { " [completed init job]" } else { "" }
                        ),
                        duration_ms: 0,
                        error: if passed {
                            None
                        } else {
                            Some(format!("container down: {}", live_ct.status))
                        },
                        severity,
                    });
                }
                None => {
                    let vm_reachable = vm_data.map(|vd| vd.reachable).unwrap_or(false);
                    if vm_reachable {
                        checks.push(Check {
                            name: format!("Container {}/{}", svc.name, ct.container_name),
                            passed: false,
                            details: format!(
                                "{} on {}: NOT FOUND in docker ps",
                                ct.container_name, svc.vm_alias
                            ),
                            duration_ms: 0,
                            error: Some("container missing from VM".into()),
                            severity: Severity::Critical,
                        });
                    } else {
                        checks.push(Check {
                            name: format!("Container {}/{}", svc.name, ct.container_name),
                            passed: false,
                            details: format!(
                                "{} on {}: VM unreachable",
                                ct.container_name, svc.vm_alias
                            ),
                            duration_ms: 0,
                            error: Some("VM unreachable".into()),
                            severity: Severity::Warning,
                        });
                    }
                }
            }
        }
    }

    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 5: PUBLIC URLS
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_public_urls(ctx: &Context) -> Vec<Check> {
    let client = http_client();
    let aclient = ctx
        .bearer_token
        .as_ref()
        .map(|t| auth_client(t))
        .unwrap_or_else(|| client.clone());

    let futs: Vec<_> = ctx
        .services
        .iter()
        .filter(|svc| svc.enabled)
        .filter_map(|svc| {
            // Skip internal-only domains (not routable via public HTTPS)
            let domain = svc.domain.as_ref()?;
            if !domain.contains('.') || domain.ends_with(".internal") || domain.ends_with(".local") {
                return None;
            }
            Some({
                let _name = svc.name.clone();
                let domain = domain.clone();
                let cl = client.clone();
                let acl = aclient.clone();
                async move {
                    let t = Instant::now();
                    let url = format!("https://{}", domain);
                    let (ok, code, detail) = http_get(&cl, &url).await;
                    let (auth_ok, auth_code, _) = http_get(&acl, &url).await;
                    let passed = ok || (code >= 200 && code < 400)
                        || code == 401
                        || code == 403
                        || auth_ok
                        || (auth_code >= 200 && auth_code < 400);
                    Check {
                        name: format!("Public {}", domain),
                        passed,
                        details: format!(
                            "{}: HTTPS={} AUTH={} (no-auth={}, auth={})",
                            domain, code, auth_code, detail, auth_code
                        ),
                        duration_ms: t.elapsed().as_millis() as u64,
                        error: if passed {
                            None
                        } else {
                            Some(format!("HTTPS {} auth {}", detail, auth_code))
                        },
                        severity: if passed {
                            Severity::Info
                        } else {
                            Severity::Warning
                        },
                    }
                }
            })
        })
        .collect();

    futures::future::join_all(futs).await
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 6: PRIVATE URLS
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_private_urls(ctx: &Context) -> Vec<Check> {
    let client = http_client();
    let resolver = hickory_resolver();

    // Test if Hickory is up (warn but don't bail — fallback to WG IPs)
    let hickory_up = dns_resolve(&resolver, "caddy.app").await.is_some();
    let mut checks = Vec::new();
    if !hickory_up {
        checks.push(Check {
            name: "Private URLs (Hickory)".into(),
            passed: false,
            details: "Hickory DNS at 10.0.0.1 is down — falling back to WG IPs".into(),
            duration_ms: 0,
            error: Some("Hickory DNS unreachable, using WG IP fallback".into()),
            severity: Severity::Warning,
        });
    }

    let futs: Vec<_> = ctx
        .services
        .iter()
        .filter(|svc| svc.enabled)
        .filter_map(|svc| {
            // Get port from service_ports (build.json) or from consolidated
            let port = ctx
                .service_ports
                .get(&svc.name)
                .copied()
                .or(svc.port);
            let dns = svc.dns.as_ref()?;
            let port = port?;
            let vm = ctx.vms.iter().find(|v| v.alias == svc.vm_alias)?;
            let wg_ip = vm.wg_ip.clone();

            let name = svc.name.clone();
            let dns = dns.clone();
            let cl = client.clone();
            let r = resolver.clone();

            Some(async move {
                let t = Instant::now();

                const NON_HTTP_PORTS: &[u16] = &[
                    53, 25, 465, 587, 993,
                    5432, 5433, 5434, 5435, 5436, 5437, 5438, 5439, 5440, 5441, 5442, 5443,
                    6379, 6380, 6381,
                ];
                let is_non_http = NON_HTTP_PORTS.contains(&port);

                // Step 1: DNS — try system resolver first (should hit Hickory via /etc/resolv.conf)
                let sys_resolve = tokio::net::lookup_host(format!("{}:{}", dns, port)).await.ok()
                    .and_then(|mut addrs| addrs.next().map(|a| a.ip().to_string()));
                let dns_ok = sys_resolve.is_some();

                // Fallback: if system DNS fails, try Hickory directly
                let (ip, used_fallback) = if let Some(ref ip) = sys_resolve {
                    (ip.clone(), false)
                } else {
                    let hickory = dns_resolve(&r, &dns).await;
                    if let Some(ip) = hickory {
                        (ip, true)
                    } else {
                        (wg_ip.clone(), true)
                    }
                };

                // Step 2: TCP
                let tcp_ok = tcp(&ip, port).await;

                // Step 3: HTTP
                let (http_ok, http_code, http_detail) = if tcp_ok && !is_non_http {
                    http_get(&cl, &format!("http://{}:{}", ip, port)).await
                } else if tcp_ok && is_non_http {
                    (true, 0, "n/a".to_string())
                } else {
                    (false, 0, "skip".to_string())
                };

                let passed = if is_non_http { tcp_ok } else { tcp_ok && http_ok };

                let dns_status = if dns_ok {
                    format!("ok({})", ip)
                } else if used_fallback {
                    format!("SYS-FAIL→hickory({})", ip)
                } else {
                    "FAIL".to_string()
                };

                let detail = format!(
                    "{}:{} DNS={} TCP={} HTTP={}",
                    dns, port, dns_status,
                    if tcp_ok { "ok" } else { "FAIL" },
                    if !tcp_ok { "skip".to_string() }
                    else if is_non_http { "n/a".to_string() }
                    else if http_ok { format!("{}", http_code) }
                    else { http_detail.clone() }
                );

                Check {
                    name: format!("{}.app:{}", name, port),
                    passed,
                    details: detail.clone(),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: if passed { None } else { Some(detail) },
                    severity: if passed { Severity::Info } else { Severity::Warning },
                }
            })
        })
        .collect();

    checks.extend(futures::future::join_all(futs).await);
    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 7: CROSS-CHECKS
// ═══════════════════════════════════════════════════════════════════════════

pub fn layer_cross_checks(
    ctx: &Context,
    _vm_batch: &HashMap<String, VmBatchData>,
    public_checks: &[Check],
    private_checks: &[Check],
    container_checks: &[Check],
) -> Vec<Check> {
    let mut checks = Vec::new();

    for svc in &ctx.services {
        // Find corresponding checks
        let container_ok = container_checks
            .iter()
            .filter(|c| c.name.starts_with(&format!("Container {}/", svc.name)))
            .all(|c| c.passed);

        let public_ok = svc.domain.as_ref().map(|d| {
            public_checks
                .iter()
                .find(|c| c.name == format!("Public {}", d))
                .map(|c| c.passed)
                .unwrap_or(false)
        });

        let private_ok = private_checks
            .iter()
            .find(|c| c.name == format!("Private {}", svc.name))
            .map(|c| c.passed);

        // Cross-check: container up but public URL down
        if container_ok && public_ok == Some(false) {
            checks.push(Check {
                name: format!("Cross {}: container up, public down", svc.name),
                passed: false,
                details: format!(
                    "{}: containers healthy but public URL {} unreachable — check Caddy/Cloudflare",
                    svc.name,
                    svc.domain.as_deref().unwrap_or("?")
                ),
                duration_ms: 0,
                error: Some("routing issue: container OK but public URL fails".into()),
                severity: Severity::Warning,
            });
        }

        // Cross-check: public URL up but container down
        if !container_ok && public_ok == Some(true) {
            checks.push(Check {
                name: format!("Cross {}: public up, container down", svc.name),
                passed: false,
                details: format!(
                    "{}: public URL responds but container is down — stale cache or wrong routing",
                    svc.name
                ),
                duration_ms: 0,
                error: Some("stale routing: public responds but container down".into()),
                severity: Severity::Warning,
            });
        }

        // Cross-check: private OK but public down
        if private_ok == Some(true) && public_ok == Some(false) {
            checks.push(Check {
                name: format!("Cross {}: private up, public down", svc.name),
                passed: false,
                details: format!(
                    "{}: reachable via WG but public URL fails — Caddy/Cloudflare issue",
                    svc.name
                ),
                duration_ms: 0,
                error: Some("reverse proxy issue".into()),
                severity: Severity::Warning,
            });
        }

        // All consistent
        let all_consistent = (container_ok || svc.containers.is_empty())
            && public_ok.unwrap_or(true)
            && private_ok.unwrap_or(true);

        if all_consistent && svc.domain.is_some() {
            checks.push(Check {
                name: format!("Cross {}", svc.name),
                passed: true,
                details: format!(
                    "{}: container={} public={} private={}",
                    svc.name,
                    if container_ok { "ok" } else { "fail" },
                    public_ok
                        .map(|b| if b { "ok" } else { "fail" })
                        .unwrap_or("n/a"),
                    private_ok
                        .map(|b| if b { "ok" } else { "fail" })
                        .unwrap_or("n/a"),
                ),
                duration_ms: 0,
                error: None,
                severity: Severity::Info,
            });
        }
    }

    if checks.is_empty() {
        checks.push(Check {
            name: "Cross-checks".into(),
            passed: true,
            details: "No cross-check issues detected".into(),
            duration_ms: 0,
            error: None,
            severity: Severity::Info,
        });
    }

    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 8: EXTERNAL
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_external(_ctx: &Context) -> Vec<Check> {
    let mut checks = Vec::new();
    let pub_resolver = public_resolver();
    let client = http_client();

    // Cloudflare DNS: dig diegonmarcos.com @1.1.1.1
    {
        let t = Instant::now();
        let ip = dns_resolve(&pub_resolver, "diegonmarcos.com").await;
        let ok = ip.is_some();
        checks.push(Check {
            name: "Cloudflare DNS A".into(),
            passed: ok,
            details: format!(
                "dig diegonmarcos.com @1.1.1.1 -> {}",
                ip.as_deref().unwrap_or("NXDOMAIN")
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("Cloudflare DNS resolution failed".into())
            },
            severity: Severity::Critical,
        });
    }

    // GHCR registry
    {
        let t = Instant::now();
        let (ok, _code, detail) = http_get(&client, "https://ghcr.io/v2/").await;
        checks.push(Check {
            name: "GHCR registry".into(),
            passed: ok,
            details: format!("ghcr.io/v2/ -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Warning,
        });
    }

    // GHA workflows (gh run list)
    {
        let t = Instant::now();
        let out = tokio::process::Command::new("gh")
            .args([
                "run",
                "list",
                "--repo",
                "diegonmarcos/cloud",
                "--limit",
                "5",
                "--json",
                "conclusion,name,status",
            ])
            .output()
            .await;
        let (ok, detail) = match out {
            Ok(o) if o.status.success() => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                let runs: Vec<serde_json::Value> =
                    serde_json::from_str(&stdout).unwrap_or_default();
                let failed = runs
                    .iter()
                    .filter(|r| r["conclusion"].as_str() == Some("failure"))
                    .count();
                (
                    failed == 0,
                    format!(
                        "{} recent runs, {} failed",
                        runs.len(),
                        failed
                    ),
                )
            }
            _ => (false, "gh CLI failed".into()),
        };
        checks.push(Check {
            name: "GHA workflows".into(),
            passed: ok,
            details: detail.clone(),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Warning,
        });
    }

    // GitHub API
    {
        let t = Instant::now();
        let (ok, _code, detail) = http_get(&client, "https://api.github.com/zen").await;
        checks.push(Check {
            name: "GitHub API".into(),
            passed: ok,
            details: format!("api.github.com/zen -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Info,
        });
    }

    // MX record for diegonmarcos.com
    {
        let t = Instant::now();
        let mx = dns_mx(&pub_resolver, "diegonmarcos.com").await;
        let ok = !mx.is_empty();
        let detail = if ok {
            mx.iter()
                .map(|(pri, srv)| format!("{} {}", pri, srv))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            "no MX records".to_string()
        };
        checks.push(Check {
            name: "MX record".into(),
            passed: ok,
            details: format!("MX diegonmarcos.com -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Warning,
        });
    }

    // A record for mail.diegonmarcos.com
    {
        let t = Instant::now();
        let ip = dns_resolve(&pub_resolver, "mail.diegonmarcos.com").await;
        let ok = ip.is_some();
        checks.push(Check {
            name: "A mail".into(),
            passed: ok,
            details: format!(
                "mail.diegonmarcos.com -> {}",
                ip.as_deref().unwrap_or("NXDOMAIN")
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("mail A record missing".into())
            },
            severity: Severity::Warning,
        });
    }

    // DKIM
    {
        let t = Instant::now();
        let txt = dns_txt(
            &pub_resolver,
            "dkim._domainkey.diegonmarcos.com",
        )
        .await;
        let ok = txt
            .as_ref()
            .map(|t| t.contains("DKIM1"))
            .unwrap_or(false);
        checks.push(Check {
            name: "DKIM dkim._domainkey".into(),
            passed: ok,
            details: format!(
                "DKIM: {}",
                if ok {
                    "present"
                } else {
                    "NOT FOUND"
                }
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("DKIM record missing".into())
            },
            severity: Severity::Warning,
        });
    }

    // SPF
    {
        let t = Instant::now();
        let txt = dns_txt(&pub_resolver, "diegonmarcos.com").await;
        let ok = txt
            .as_ref()
            .map(|t| t.contains("v=spf1"))
            .unwrap_or(false);
        checks.push(Check {
            name: "SPF record".into(),
            passed: ok,
            details: format!(
                "SPF: {}",
                txt.as_deref().unwrap_or("NOT FOUND")
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("SPF record missing".into())
            },
            severity: Severity::Warning,
        });
    }

    // DMARC
    {
        let t = Instant::now();
        let txt = dns_txt(&pub_resolver, "_dmarc.diegonmarcos.com").await;
        let ok = txt
            .as_ref()
            .map(|t| t.contains("v=DMARC1"))
            .unwrap_or(false);
        checks.push(Check {
            name: "DMARC record".into(),
            passed: ok,
            details: format!(
                "DMARC: {}",
                txt.as_deref().unwrap_or("NOT FOUND")
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("DMARC record missing".into())
            },
            severity: Severity::Warning,
        });
    }

    // Resend API health (optional)
    {
        let t = Instant::now();
        let (ok, _code, detail) = http_get(&client, "https://api.resend.com/").await;
        checks.push(Check {
            name: "Resend API".into(),
            passed: ok,
            details: format!("api.resend.com -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Info,
        });
    }

    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 9: DRIFT
// ═══════════════════════════════════════════════════════════════════════════

pub fn layer_drift(
    ctx: &Context,
    vm_batch: &HashMap<String, VmBatchData>,
) -> Vec<Check> {
    let mut checks = Vec::new();

    // Collect all declared container names by VM (skip disabled services)
    let mut declared_by_vm: HashMap<String, HashSet<String>> = HashMap::new();
    for svc in &ctx.services {
        if !svc.enabled { continue; }
        let entry = declared_by_vm
            .entry(svc.vm_alias.clone())
            .or_default();
        for ct in &svc.containers {
            entry.insert(ct.container_name.clone());
        }
    }

    // Collect all running container names by VM
    let mut running_by_vm: HashMap<String, HashSet<String>> = HashMap::new();
    for (alias, batch) in vm_batch {
        let entry = running_by_vm.entry(alias.clone()).or_default();
        for ct in &batch.containers {
            entry.insert(ct.name.clone());
        }
    }

    // Missing containers: declared in topology but not in docker
    for (vm_alias, declared) in &declared_by_vm {
        let running = running_by_vm.get(vm_alias);
        let vm_reachable = vm_batch
            .get(vm_alias)
            .map(|b| b.reachable)
            .unwrap_or(false);
        if !vm_reachable {
            continue;
        }
        for name in declared {
            let found = running
                .map(|r| r.contains(name))
                .unwrap_or(false);
            if !found {
                checks.push(Check {
                    name: format!("Drift missing: {}/{}", vm_alias, name),
                    passed: false,
                    details: format!(
                        "{} declared in topology but not found in docker on {}",
                        name, vm_alias
                    ),
                    duration_ms: 0,
                    error: Some("container missing from VM".into()),
                    severity: Severity::Critical,
                });
            }
        }
    }

    // Unmanaged containers: in docker but not in topology
    const INFRA_ALLOWLIST: &[&str] = &[
        "fluent-bit", "sqlite-", "postlite-", "introspect-proxy",
        "palantir-cron", "borg-server", "bup-server", "syslog-central",
        "siem-api", "crawlee_minio_init", "photos-db", "rig",
        "rig-agentic", "surrealdb",
    ];
    for (vm_alias, running) in &running_by_vm {
        let declared = declared_by_vm.get(vm_alias);
        for name in running {
            let in_topology = declared
                .map(|d| d.contains(name))
                .unwrap_or(false);
            if !in_topology {
                // Skip known infrastructure/sidecar containers
                let is_infra = INFRA_ALLOWLIST.iter().any(|prefix| {
                    name.starts_with(prefix) || name == *prefix
                });
                if is_infra {
                    continue;
                }
                checks.push(Check {
                    name: format!("Drift unmanaged: {}/{}", vm_alias, name),
                    passed: false,
                    details: format!(
                        "{} running on {} but not declared in topology",
                        name, vm_alias
                    ),
                    duration_ms: 0,
                    error: Some("unmanaged container".into()),
                    severity: Severity::Warning,
                });
            }
        }
    }

    // Caddy route orphans: routes to services that don't exist
    for route in &ctx.caddy_route_list {
        let domain = &route.domain;
        let has_service = ctx
            .services
            .iter()
            .any(|svc| svc.domain.as_deref() == Some(domain));
        if !has_service && !domain.is_empty() {
            // Check if it's a parent domain route or GitHub Pages proxy (not orphaned)
            let is_meta = ctx.consolidated["services"]
                .as_object()
                .map(|svcs| {
                    svcs.values().any(|svc| {
                        svc["proxy"]["primary"]["parent_domain"]
                            .as_str()
                            == Some(domain)
                    })
                })
                .unwrap_or(false);
            if !is_meta {
                checks.push(Check {
                    name: format!("Drift caddy orphan: {}", domain),
                    passed: false,
                    details: format!(
                        "Caddy route for {} -> {} has no matching service",
                        domain, route.upstream
                    ),
                    duration_ms: 0,
                    error: Some("orphan caddy route".into()),
                    severity: Severity::Info,
                });
            }
        }
    }

    // Exited containers
    for (vm_alias, batch) in vm_batch {
        for ct in &batch.containers {
            if ct.health_state == "exited" {
                let is_init_job = ct.name.contains("_init")
                    || ct.name.contains("-setup")
                    || ct.name.contains("_setup");
                let is_clean_exit = ct.status.contains("Exited (0)");
                if is_init_job && is_clean_exit {
                    checks.push(Check {
                        name: format!("Drift exited: {}/{}", vm_alias, ct.name),
                        passed: true,
                        details: format!(
                            "{} on {} exited cleanly [completed init job]",
                            ct.name, vm_alias
                        ),
                        duration_ms: 0,
                        error: None,
                        severity: Severity::Info,
                    });
                } else {
                    checks.push(Check {
                        name: format!("Drift exited: {}/{}", vm_alias, ct.name),
                        passed: false,
                        details: format!(
                            "{} on {} is exited: {}",
                            ct.name, vm_alias, ct.status
                        ),
                        duration_ms: 0,
                        error: Some("container exited".into()),
                        severity: Severity::Warning,
                    });
                }
            }
        }
    }

    // Services without containers declared
    for svc in &ctx.services {
        if svc.containers.is_empty() {
            checks.push(Check {
                name: format!("Drift no-containers: {}", svc.name),
                passed: false,
                details: format!(
                    "{} has no containers declared in topology",
                    svc.name
                ),
                duration_ms: 0,
                error: Some("service without containers".into()),
                severity: Severity::Info,
            });
        }
    }

    // Services without domain
    for svc in &ctx.services {
        if svc.domain.is_none() && !svc.containers.is_empty() {
            // Only flag if it's not an internal-only service
            let is_internal = svc.category == "sec"
                || svc.name.contains("syslog")
                || svc.name.contains("fluent")
                || svc.name.contains("backup")
                || svc.name.contains("forwarder")
                || svc.name.contains("alerts");
            if !is_internal {
                checks.push(Check {
                    name: format!("Drift no-domain: {}", svc.name),
                    passed: false,
                    details: format!(
                        "{} has containers but no domain assigned",
                        svc.name
                    ),
                    duration_ms: 0,
                    error: Some("service has no public domain".into()),
                    severity: Severity::Info,
                });
            }
        }
    }

    // Services missing port in build.json
    for svc in &ctx.services {
        if svc.port.is_some() && !ctx.service_ports.contains_key(&svc.name) {
            checks.push(Check {
                name: format!("Drift no-port-in-build: {}", svc.name),
                passed: false,
                details: format!(
                    "{} has port in topology but missing ports.app in build.json",
                    svc.name
                ),
                duration_ms: 0,
                error: Some("build.json missing ports.app".into()),
                severity: Severity::Info,
            });
        }
    }

    if checks.is_empty() {
        checks.push(Check {
            name: "Drift".into(),
            passed: true,
            details: "No drift detected".into(),
            duration_ms: 0,
            error: None,
            severity: Severity::Info,
        });
    }

    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 10: SECURITY
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_security(ctx: &Context) -> Vec<Check> {
    let mut checks = Vec::new();
    let client = http_client();
    let pub_resolver = public_resolver();

    // TLS cert expiry on key domains
    let key_domains = vec![
        "diegonmarcos.com",
        "api.diegonmarcos.com",
        "auth.diegonmarcos.com",
        "mail.diegonmarcos.com",
        "vault.diegonmarcos.com",
    ];

    let tls_futs: Vec<_> = key_domains
        .iter()
        .map(|&domain| {
            let _cl = client.clone();
            async move {
                let t = Instant::now();
                // Use openssl s_client to check cert expiry
                let out = tokio::process::Command::new("bash")
                    .args([
                        "-c",
                        &format!(
                            "echo | openssl s_client -servername {} -connect {}:443 2>/dev/null | openssl x509 -noout -dates 2>/dev/null",
                            domain, domain
                        ),
                    ])
                    .output()
                    .await;

                let (ok, detail) = match out {
                    Ok(o) if o.status.success() => {
                        let stdout = String::from_utf8_lossy(&o.stdout);
                        let expiry = stdout
                            .lines()
                            .find(|l| l.starts_with("notAfter="))
                            .map(|l| l.strip_prefix("notAfter=").unwrap_or("").trim().to_string())
                            .unwrap_or_default();

                        // Parse the date to check if expiring soon
                        let days_left = parse_openssl_date(&expiry)
                            .map(|exp| {
                                let now = chrono::Utc::now();
                                (exp - now).num_days()
                            })
                            .unwrap_or(-1);

                        if days_left > 14 {
                            (true, format!("expires {} ({}d)", expiry, days_left))
                        } else if days_left > 0 {
                            (false, format!("EXPIRING SOON: {} ({}d!)", expiry, days_left))
                        } else {
                            (false, format!("EXPIRED or parse error: {}", expiry))
                        }
                    }
                    _ => (false, "TLS connection failed".into()),
                };

                Check {
                    name: format!("TLS cert {}", domain),
                    passed: ok,
                    details: detail.clone(),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: if ok { None } else { Some(detail) },
                    severity: if ok {
                        Severity::Info
                    } else {
                        Severity::Critical
                    },
                }
            }
        })
        .collect();

    let tls_results = futures::future::join_all(tls_futs).await;
    checks.extend(tls_results);

    // DMARC policy check
    {
        let t = Instant::now();
        let txt = dns_txt(&pub_resolver, "_dmarc.diegonmarcos.com").await;
        let has_reject = txt
            .as_ref()
            .map(|t| t.contains("p=reject"))
            .unwrap_or(false);
        checks.push(Check {
            name: "DMARC policy".into(),
            passed: has_reject,
            details: format!(
                "DMARC: {}",
                txt.as_deref().unwrap_or("NOT FOUND")
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if has_reject {
                None
            } else {
                Some("DMARC policy should be p=reject".into())
            },
            severity: if has_reject {
                Severity::Info
            } else {
                Severity::Warning
            },
        });
    }

    // SPF strictness check
    {
        let t = Instant::now();
        let txt = dns_txt(&pub_resolver, "diegonmarcos.com").await;
        let has_spf = txt
            .as_ref()
            .map(|t| t.contains("v=spf1"))
            .unwrap_or(false);
        let has_all_fail = txt
            .as_ref()
            .map(|t| t.contains("-all"))
            .unwrap_or(false);
        let ok = has_spf && has_all_fail;
        checks.push(Check {
            name: "SPF strict (-all)".into(),
            passed: ok,
            details: format!(
                "SPF: {} (strict={})",
                if has_spf { "present" } else { "missing" },
                if has_all_fail { "-all" } else { "soft/missing" }
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok {
                None
            } else {
                Some("SPF should end with -all".into())
            },
            severity: Severity::Warning,
        });
    }

    // Authelia health
    {
        let t = Instant::now();
        let (ok, _code, detail) =
            http_get(&client, "https://auth.diegonmarcos.com/api/health").await;
        checks.push(Check {
            name: "Authelia health".into(),
            passed: ok,
            details: format!("auth.diegonmarcos.com/api/health -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: Severity::Critical,
        });
    }

    // Firewall ports scan: dangerous ports on public IPs
    let dangerous_ports: Vec<u16> = vec![
        3306, 5432, 6379, 27017, 9200, 2375, 2376, 8080, 9090, 3000, 5000,
    ];
    let scan_futs: Vec<_> = ctx
        .vms
        .iter()
        .filter(|v| !v.pub_ip.is_empty() && v.pub_ip != "?")
        .map(|vm| {
            let alias = vm.alias.clone();
            let ip = vm.pub_ip.clone();
            let ports = dangerous_ports.clone();
            let declared_ports: HashSet<u16> = vm.public_ports.iter().map(|p| p.port).collect();
            async move {
                let t = Instant::now();
                let open = tcp_scan(&ip, &ports).await;
                // Filter out ports that are declared in public_ports (expected)
                let unexpected: Vec<u16> = open.iter()
                    .filter(|p| !declared_ports.contains(p))
                    .copied()
                    .collect();
                let ok = unexpected.is_empty();
                let detail = if ok {
                    format!("{}: no unexpected dangerous ports exposed", alias)
                } else {
                    format!(
                        "{}: DANGEROUS ports open: {}",
                        alias,
                        unexpected.iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                Check {
                    name: format!("Firewall {}", alias),
                    passed: ok,
                    details: detail.clone(),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: if ok { None } else { Some(detail) },
                    severity: if ok {
                        Severity::Info
                    } else {
                        Severity::Critical
                    },
                }
            }
        })
        .collect();

    let scan_results = futures::future::join_all(scan_futs).await;
    checks.extend(scan_results);

    // SSH host key stability (just check SSH is reachable on standard port for each VM)
    for vm in &ctx.vms {
        if vm.pub_ip.is_empty() || vm.pub_ip == "?" {
            continue;
        }
        let t = Instant::now();
        let ssh_ok = tcp(&vm.pub_ip, 22).await;
        let dropbear_ok = tcp(&vm.pub_ip, vm.rescue_port).await;
        let ssh_declared = vm.public_ports.iter().any(|p| p.port == 22);
        let is_wg_only = !ssh_declared;
        // If VM doesn't declare port 22 in public_ports, it's WG-only SSH.
        // SSH being closed on the public IP is CORRECT behavior.
        let (passed, severity) = if is_wg_only && !ssh_ok && !dropbear_ok {
            (true, Severity::Info)
        } else {
            (
                ssh_ok || dropbear_ok,
                if ssh_ok || dropbear_ok {
                    Severity::Info
                } else {
                    Severity::Critical
                },
            )
        };
        checks.push(Check {
            name: format!("SSH ports {}", vm.alias),
            passed,
            details: format!(
                "{}: SSH:22={} Dropbear:{}={}{}",
                vm.alias,
                if ssh_ok { "open" } else { "closed" },
                vm.rescue_port,
                if dropbear_ok { "open" } else { "closed" },
                if is_wg_only && !ssh_ok { " [WG-only SSH - expected]" } else { "" }
            ),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if passed { None } else { Some("no SSH access".into()) },
            severity,
        });
    }

    // Caddy TLS (check proxy.diegonmarcos.com)
    {
        let t = Instant::now();
        let (ok, _code, detail) =
            http_get(&client, "https://proxy.diegonmarcos.com").await;
        checks.push(Check {
            name: "Caddy TLS".into(),
            passed: ok,
            details: format!("proxy.diegonmarcos.com -> {}", detail),
            duration_ms: t.elapsed().as_millis() as u64,
            error: if ok { None } else { Some(detail) },
            severity: if ok {
                Severity::Info
            } else {
                Severity::Warning
            },
        });
    }

    checks
}

// ═══════════════════════════════════════════════════════════════════════════
// LAYER 11: E2E EMAIL DELIVERY
// ═══════════════════════════════════════════════════════════════════════════

pub async fn layer_email_e2e(_ctx: &Context, reachable_vms: &[String]) -> Vec<Check> {
    let mut checks = Vec::new();
    let t = std::time::Instant::now();

    // Check for RESEND_API_KEY
    let api_key = match std::env::var("RESEND_API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            checks.push(Check {
                name: "Resend API key".into(),
                passed: true,
                details: "not set (set RESEND_API_KEY to enable E2E email test)".into(),
                duration_ms: 0,
                error: None,
                severity: Severity::Info,
            });
            return checks;
        }
    };

    // 1. Send test email via Resend API
    let send_body = serde_json::json!({
        "from": "health@mails.diegonmarcos.com",
        "to": ["me@diegonmarcos.com"],
        "subject": format!("Health check {}", chrono::Utc::now().format("%H:%M:%S")),
        "text": "E2E email delivery test from cloud-health-full-report"
    });

    let send_result = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(send_body.to_string())
        .send()
        .await;

    let email_id = match send_result {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 200 {
                let id = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| v["id"].as_str().map(|s| s.to_string()));
                checks.push(Check {
                    name: "Resend send".into(),
                    passed: true,
                    details: format!("sent (id: {})", id.as_deref().unwrap_or("?")),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: None,
                    severity: Severity::Info,
                });
                id
            } else {
                checks.push(Check {
                    name: "Resend send".into(),
                    passed: false,
                    details: format!("HTTP {} — {}", status, &body[..body.len().min(100)]),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: Some(format!("Resend API returned {}", status)),
                    severity: Severity::Warning,
                });
                None
            }
        }
        Err(e) => {
            checks.push(Check {
                name: "Resend send".into(),
                passed: false,
                details: format!("error: {}", e),
                duration_ms: t.elapsed().as_millis() as u64,
                error: Some(e.to_string()),
                severity: Severity::Warning,
            });
            None
        }
    };

    // 2. Poll delivery status if we got an email ID
    if let Some(id) = &email_id {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        let mut delivered = false;
        for attempt in 1..=3 {
            let url = format!("https://api.resend.com/emails/{}", id);
            if let Ok(resp) = reqwest::Client::new()
                .get(&url)
                .header("Authorization", format!("Bearer {}", api_key))
                .timeout(std::time::Duration::from_secs(5))
                .send()
                .await
            {
                if let Ok(body) = resp.text().await {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) {
                        let status = v["last_event"].as_str().unwrap_or("unknown");
                        if status == "delivered" || status == "opened" {
                            delivered = true;
                            checks.push(Check {
                                name: "Resend delivery".into(),
                                passed: true,
                                details: format!("status={} (poll #{})", status, attempt),
                                duration_ms: t.elapsed().as_millis() as u64,
                                error: None,
                                severity: Severity::Info,
                            });
                            break;
                        }
                    }
                }
            }
            if attempt < 3 {
                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            }
        }
        if !delivered {
            checks.push(Check {
                name: "Resend delivery".into(),
                passed: false,
                details: "not delivered after 3 polls (may still be in transit)".into(),
                duration_ms: t.elapsed().as_millis() as u64,
                error: Some("delivery not confirmed".into()),
                severity: Severity::Warning,
            });
        }
    }

    // 3. Check stalwart ingestion via SSH (if oci-mail reachable)
    if reachable_vms.contains(&"oci-mail".to_string()) {
        let ssh_result = ssh::ssh_exec(
            "oci-mail",
            "docker logs stalwart --since 2m 2>&1 | grep -c 'Message ingested' || echo 0",
            10,
        ).await;
        match ssh_result {
            Ok(output) => {
                let count: u32 = output.trim().parse().unwrap_or(0);
                checks.push(Check {
                    name: "Stalwart ingestion".into(),
                    passed: count > 0,
                    details: format!("{} messages ingested (last 2min)", count),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: if count == 0 { Some("no messages ingested".into()) } else { None },
                    severity: if count > 0 { Severity::Info } else { Severity::Warning },
                });
            }
            Err(e) => {
                checks.push(Check {
                    name: "Stalwart ingestion".into(),
                    passed: false,
                    details: format!("SSH failed: {}", e),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: Some(e.to_string()),
                    severity: Severity::Warning,
                });
            }
        }
    }

    checks
}

/// Parse openssl date format "Mar 29 12:00:00 2026 GMT"
fn parse_openssl_date(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    // Try common openssl date formats
    let formats = [
        "%b %d %H:%M:%S %Y GMT",
        "%b  %d %H:%M:%S %Y GMT",
        "%b %d %H:%M:%S %Y %Z",
    ];
    for fmt in &formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s.trim(), fmt) {
            return Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                dt,
                chrono::Utc,
            ));
        }
    }
    None
}
