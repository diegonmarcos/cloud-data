//! Cloud Health Daily Mail — HTML email report generator
//! Collects VM metrics (SSH), API data (HTTP), cloud-data (JSON), generates HTML to dist/
//!
//! Usage: cloud-health-daily-mail (from cloud-health-daily-mail/)
//!   Outputs: dist/cloud_health_daily.html

mod appendix;
mod collect;
mod context;
mod diagrams;
mod html;
mod health_full2;
mod mail;
mod mail_full;
mod md;
mod mermaid;
mod ssh;
mod types;

use anyhow::Result;
use chrono::Utc;
use std::collections::HashSet;
use std::time::Instant;
use types::*;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let now = Utc::now();
    let date = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H:%M %Z").to_string();

    println!("=== Cloud Health Daily Mail Report ===");

    // 1. Load context from cloud-data JSONs
    let ctx = context::load_context()?;
    println!(
        "Loaded: {} VMs, {} endpoints, {} TLS checks, {} databases, {} manifests",
        ctx.monitoring.vms.len(),
        ctx.monitoring.endpoint_checks.len(),
        ctx.monitoring.tls_checks.len(),
        ctx.databases.databases.len(),
        ctx.manifests.len(),
    );

    // 2. Parallel collection: SSH (per VM) + API calls
    // GitHub token: env var -> gh config -> empty
    let github_token = std::env::var("GITHUB_TOKEN")
        .ok()
        .filter(|t| !t.is_empty())
        .or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            std::fs::read_to_string(format!("{}/.config/gh/hosts.yml", home))
                .ok()
                .and_then(|s| {
                    s.lines()
                        .find(|l| l.trim().starts_with("oauth_token:"))
                        .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
                })
        })
        .unwrap_or_default();

    if github_token.is_empty() {
        eprintln!("  GITHUB_TOKEN not found (env or gh config), skipping GHA/GHCR");
    }

    // Launch all tasks concurrently
    let vm_futures: Vec<_> = ctx.monitoring.vms.iter()
        .map(|vm| ssh::collect_vm(vm, &ctx.databases.databases))
        .collect();

    // Build endpoint checks: monitoring-targets + build-caddy.json (public) +
    // service `.domain` declarations — deduplicated by URL.
    let mut all_endpoints: Vec<EndpointCheck> = ctx.monitoring.endpoint_checks.clone();
    let mut seen_urls: HashSet<String> = all_endpoints.iter().map(|e| e.url.clone()).collect();

    // Caddy-declared public URLs (source of truth).
    for t in reports_common::caddy::load_public_targets() {
        if seen_urls.insert(t.url.clone()) {
            let service = t.service.clone().unwrap_or_else(|| t.host.clone());
            all_endpoints.push(EndpointCheck { service, url: t.url });
        }
    }

    // Fallback: service.domain declarations not yet in Caddy (drift).
    for svc in &ctx.services {
        if !svc.enabled || svc.domain.is_empty() { continue; }
        let url = if svc.domain.starts_with("http") {
            svc.domain.clone()
        } else {
            format!("https://{}/", svc.domain)
        };
        if seen_urls.insert(url.clone()) {
            all_endpoints.push(EndpointCheck { service: svc.name.clone(), url });
        }
    }

    let ep_futures: Vec<_> = all_endpoints.iter()
        .map(|ep| collect::check_endpoint(ep))
        .collect();

    // Deduplicate TLS domains
    let tls_domains: Vec<String> = ctx.monitoring.tls_checks.iter()
        .map(|t| t.domain.split('/').next().unwrap_or(&t.domain).to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let cert_futures: Vec<_> = tls_domains.iter()
        .map(|d| collect::check_cert(d))
        .collect();

    // Run everything in parallel (mail health checks run alongside SSH+API collection).
    // Also spawn the absorbed `health_full2` + `mail_full` submodules here so their
    // SSH / DNS / HTTP probes multiplex with Daily's own collectors — the whole point
    // of consolidating into one binary.
    let (
        vms,
        endpoints,
        certs,
        dns,
        gha_runs,
        gha_workflows,
        (ghcr_packages, ghcr_total, github_disk_kb),
        dags,
        cloud_buckets,
        repos,
        cloud_costs,
        umami_sites,
        matomo_sites,
        mut mail_health,
        full2_report,
        mail_full_report,
    ) = tokio::join!(
        futures::future::join_all(vm_futures),
        futures::future::join_all(ep_futures),
        futures::future::join_all(cert_futures),
        collect::check_dns(),
        async {
            if github_token.is_empty() { vec![] }
            else { collect::fetch_gha(&github_token).await }
        },
        async {
            if github_token.is_empty() { vec![] }
            else { collect::fetch_gha_workflows(&github_token).await }
        },
        async {
            if github_token.is_empty() { (vec![], 0, 0) }
            else { collect::fetch_ghcr(&github_token).await }
        },
        collect::fetch_dags(),
        collect::fetch_bucket_sizes(&ctx.cloud_buckets),
        collect::fetch_repos(),
        collect::fetch_cloud_costs(),
        collect::fetch_umami_analytics(),
        collect::fetch_matomo_analytics(),
        mail::collect_mail_network(),
        health_full2::run(),
        mail_full::run(),
    );

    // Fill mail health with SSH-collected data (synergy: reuses VM data already collected)
    mail::fill_from_vmdata(&mut mail_health, &vms);

    // 3. Mark services that responded as has_api (runtime-proven)
    let mut services = ctx.services.clone();
    for svc in &mut services {
        if let Some(ep) = endpoints.iter().find(|e| e.service == svc.name) {
            svc.has_api = ep.status_code > 0;
        }
    }

    // 4. Compute fleet totals
    let fleet_running: u32 = vms.iter().map(|v| v.containers_running).sum();
    let fleet_total: u32 = vms.iter().map(|v| v.containers_total).sum();
    let fleet_unhealthy: u32 = vms.iter().map(|v| v.containers_unhealthy).sum();

    let elapsed = start.elapsed();
    println!(
        "Collected: {} VMs, {} endpoints, {} certs, {} DNS, {} GHA runs, {} GHA workflows, {} GHCR, {} DAGs in {:.1}s",
        vms.len(), endpoints.len(), certs.len(), dns.len(),
        gha_runs.len(), gha_workflows.len(), ghcr_packages.len(), dags.len(),
        elapsed.as_secs_f64(),
    );

    // 4. Compute storage drift per VM
    let drift: Vec<StorageDrift> = {
        let mut drifts = Vec::new();
        for vm in &vms {
            let declared: HashSet<String> = ctx.databases.databases.iter()
                .filter(|d| d.enabled && d.wg_ip == vm.ip)
                .map(|d| d.container.clone())
                .collect();

            let runtime: HashSet<String> = vm.runtime_volumes.iter()
                .map(|v| v.container.clone())
                .collect();

            let declared_only: Vec<String> = declared.difference(&runtime).cloned().collect();
            let runtime_only: Vec<String> = runtime.difference(&declared).cloned().collect();
            let matched: Vec<String> = declared.intersection(&runtime).cloned().collect();

            if !declared_only.is_empty() || !runtime_only.is_empty() {
                drifts.push(StorageDrift { declared_only, runtime_only, matched });
            }
        }
        drifts
    };

    // 5. Compute container drift from manifests
    let container_drift: Vec<ContainerDrift> = {
        let mut drifts = Vec::new();
        for vm in &vms {
            if let Some(manifest) = ctx.manifests.get(&vm.name) {
                let runtime_names: HashSet<String> = vm.container_list.iter()
                    .map(|c| c.name.clone())
                    .collect();

                // Build expected containers from manifest services
                // A container belongs to a service if its name starts with the service name
                let mut expected_not_running = Vec::new();
                let mut image_mismatch = Vec::new();

                for svc in &manifest.services {
                    // Check if any runtime container matches this service
                    let matching: Vec<&ContainerInfo> = vm.container_list.iter()
                        .filter(|c| c.name.starts_with(&svc.name))
                        .collect();

                    if matching.is_empty() {
                        expected_not_running.push(svc.name.clone());
                    } else {
                        // Check image mismatches
                        for c in &matching {
                            // Check if the running image matches any declared image
                            let image_ok = svc.images.iter().any(|declared_img| {
                                // Normalize: strip tag for comparison if needed
                                let running = &c.image;
                                running.contains(&declared_img.split(':').next().unwrap_or(""))
                            });
                            if !image_ok && !svc.images.is_empty() {
                                let declared = svc.images.join(", ");
                                image_mismatch.push((c.name.clone(), c.image.clone(), declared));
                            }
                        }
                    }
                }

                // Find running containers not declared in any service
                let declared_prefixes: Vec<&str> = manifest.services.iter()
                    .map(|s| s.name.as_str())
                    .collect();
                let running_not_declared: Vec<String> = runtime_names.iter()
                    .filter(|name| !declared_prefixes.iter().any(|prefix| name.starts_with(prefix)))
                    .cloned()
                    .collect();

                if !expected_not_running.is_empty() || !running_not_declared.is_empty() || !image_mismatch.is_empty() {
                    drifts.push(ContainerDrift {
                        vm_name: vm.name.clone(),
                        expected_not_running,
                        running_not_declared,
                        image_mismatch,
                    });
                }
            }
        }
        drifts
    };

    // 6. Compute executive summary
    let exec_summary = {
        let mut critical: u32 = 0;
        let mut warnings: u32 = 0;
        let mut ok: u32 = 0;
        let mut issues: Vec<Issue> = Vec::new();

        // VMs
        for vm in &vms {
            match vm.status {
                VmStatus::Critical => {
                    critical += 1;
                    issues.push(Issue {
                        severity: "CRIT".into(),
                        message: format!("{}: status CRITICAL", vm.name),
                    });
                }
                VmStatus::Warning => {
                    warnings += 1;
                    issues.push(Issue {
                        severity: "WARN".into(),
                        message: format!("{}: status WARNING", vm.name),
                    });
                }
                VmStatus::Healthy => { ok += 1; }
                VmStatus::Unknown => { warnings += 1; }
            }

            // Unhealthy containers
            for name in &vm.unhealthy_names {
                critical += 1;
                issues.push(Issue {
                    severity: "CRIT".into(),
                    message: format!("{}: {} unhealthy", vm.name, name),
                });
            }

            // Exited containers
            for name in &vm.exited_names {
                warnings += 1;
                issues.push(Issue {
                    severity: "WARN".into(),
                    message: format!("{}: {} exited", vm.name, name),
                });
            }

            // Disk > 75%
            if vm.disk_pct > 75 {
                warnings += 1;
                issues.push(Issue {
                    severity: "WARN".into(),
                    message: format!("{}: disk {}%", vm.name, vm.disk_pct),
                });
            }
        }

        // Endpoints
        for ep in &endpoints {
            match ep.status_code {
                200..=399 => { ok += 1; }
                0 | 500..=599 => {
                    critical += 1;
                    issues.push(Issue {
                        severity: "CRIT".into(),
                        message: format!("{}: HTTP {}", ep.service, ep.status_code),
                    });
                }
                400..=499 => { warnings += 1; }
                _ => { ok += 1; }
            }
        }

        // Certs
        for cert in &certs {
            if cert.days_left < 7 {
                critical += 1;
                issues.push(Issue {
                    severity: "CRIT".into(),
                    message: format!("cert {} expires in {}d", cert.domain, cert.days_left),
                });
            } else if cert.days_left < 30 {
                warnings += 1;
                issues.push(Issue {
                    severity: "WARN".into(),
                    message: format!("cert {} expires in {}d", cert.domain, cert.days_left),
                });
            } else {
                ok += 1;
            }
        }

        // GHA
        for run in &gha_runs {
            match run.conclusion.as_str() {
                "failure" => {
                    critical += 1;
                    issues.push(Issue {
                        severity: "CRIT".into(),
                        message: format!("GHA {} FAILED", run.name),
                    });
                }
                "success" => { ok += 1; }
                _ => { warnings += 1; }
            }
        }

        // Sort issues: CRIT first, then WARN
        issues.sort_by(|a, b| {
            let ord_a = if a.severity == "CRIT" { 0 } else { 1 };
            let ord_b = if b.severity == "CRIT" { 0 } else { 1 };
            ord_a.cmp(&ord_b)
        });
        issues.truncate(3);

        ExecSummary { critical, warnings, ok, top_issues: issues }
    };

    // 6b. Compute container CPU ranking (top 20 across all VMs)
    let container_cpu_ranking: Vec<ContainerCpuRank> = {
        let mut all_entries: Vec<ContainerCpuRank> = Vec::new();
        for vm in &vms {
            for stat in &vm.container_stats {
                // Find uptime from container_list by matching name
                let uptime_hours = vm.container_list.iter()
                    .find(|c| c.name == stat.name)
                    .map(|c| parse_running_for(&c.running_for))
                    .unwrap_or(0.0);

                let cpu_num: f64 = stat.cpu.trim_end_matches('%').trim().parse().unwrap_or(0.0);
                let mem_mib = parse_mem_mib(&stat.mem_usage);
                let cpu_hours = (cpu_num / 100.0) * uptime_hours;
                let mem_gb_hours = (mem_mib / 1024.0) * uptime_hours;

                all_entries.push(ContainerCpuRank {
                    rank: 0,
                    container: stat.name.clone(),
                    vm: vm.name.clone(),
                    cpu_pct: stat.cpu.clone(),
                    mem_usage: stat.mem_usage.clone(),
                    mem_pct: stat.mem_pct.clone(),
                    uptime_hours,
                    cpu_hours,
                    mem_gb_hours,
                });
            }
        }
        // Sort by CPU*hours descending (sustained load matters more than peak %)
        all_entries.sort_by(|a, b| {
            b.cpu_hours.partial_cmp(&a.cpu_hours).unwrap_or(std::cmp::Ordering::Equal)
        });
        // No truncation — list ALL containers
        // Assign ranks after sorting
        for (i, entry) in all_entries.iter_mut().enumerate() {
            entry.rank = (i + 1) as u32;
        }
        all_entries
    };

    // 6.5 Assemble Z-Appendix from in-process submodule results.
    // Both submodules have already written their .md + .json to cwd, but we
    // also pass their returned markdown/JSON directly into the appendix so we
    // don't re-read from disk.
    let apx = appendix::from_reports(
        full2_report.as_ref().ok(),
        mail_full_report.as_ref().ok(),
    );
    if let Err(ref e) = full2_report { eprintln!("[health_full2] failed: {}", e); }
    if let Err(ref e) = mail_full_report { eprintln!("[mail_full] failed: {}", e); }
    if !apx.is_empty() {
        println!("Appendix loaded: {}", apx.summary());
    } else {
        println!("Appendix: empty (submodules returned no results)");
    }

    // 7. Build report data
    let report = ReportData {
        date,
        time,
        vms,
        endpoints,
        certs,
        dns,
        gha_runs,
        gha_workflows,
        ghcr_packages,
        ghcr_total,
        github_disk_kb,
        dags,
        databases: ctx.databases.databases.clone(),
        fleet_running,
        fleet_total,
        fleet_unhealthy,
        drift,
        exec_summary,
        container_drift,
        cloud_buckets,
        cloud_costs,
        total_services: services.iter().filter(|s| s.enabled).count(),
        services,
        repos,
        mcp_servers: ctx.mcp_servers.clone(),
        vps_providers: ctx.vps_providers.clone(),
        vm_finops: ctx.vm_finops.clone(),
        total_containers: fleet_total,
        total_domains: ctx.total_domains,
        generation_duration_ms: start.elapsed().as_millis() as u64,
        ai: ctx.ai,
        umami_sites,
        container_cpu_ranking,
        firewalls: ctx.firewalls.clone(),
        global_firewall: ctx.global_firewall.clone(),
        mail_health: Some(mail_health),
        matomo_sites,
        appendix_md: apx.legacy_md(),
        appendix_stack: apx.stack.clone(),
        appendix_full: apx.full.clone(),
    };

    // 8. Render HTML (email + web)
    let html_email = html::render(&report, html::OutputMode::Email);
    let html_web = html::render(&report, html::OutputMode::Web);

    // 8b. Render Markdown (template-driven; parity with other report crates).
    let md_out = match md::render(&report) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("[md] render failed: {}", e);
            None
        }
    };

    // 9. Write outputs — engine invokes binary with cwd=dist/
    let output_path = std::path::PathBuf::from("cloud_health_daily.html");
    std::fs::write(&output_path, &html_email)?;
    std::fs::write("cloud_health_daily_web.html", &html_web)?;
    if let Some(ref md_s) = md_out {
        std::fs::write(md::output_path(), md_s)?;
    }

    // 10. Write JSON (for debugging / programmatic access)
    let json = serde_json::to_string_pretty(&report)?;
    std::fs::write("cloud_health_daily.json", &json)?;

    println!(
        "\n=== DONE in {:.1}s === HTML: {} ({} bytes email, {} bytes web){}",
        start.elapsed().as_secs_f64(),
        output_path.display(),
        html_email.len(),
        html_web.len(),
        md_out
            .as_ref()
            .map(|s| format!(", MD: {} bytes", s.len()))
            .unwrap_or_default(),
    );

    Ok(())
}

/// Parse Docker's "running_for" string (e.g. "12 hours", "2 days", "About an hour") to hours
fn parse_running_for(s: &str) -> f64 {
    let s = s.trim().to_lowercase();
    // "about a minute" / "about an hour"
    if s.contains("about a minute") || s.contains("less than a second") {
        return 0.0;
    }
    if s.contains("about an hour") {
        return 1.0;
    }

    // Try to extract a number + unit pattern
    let mut hours = 0.0;
    let parts: Vec<&str> = s.split_whitespace().collect();
    let mut i = 0;
    while i < parts.len() {
        if let Ok(num) = parts[i].parse::<f64>() {
            let unit = parts.get(i + 1).unwrap_or(&"");
            if unit.starts_with("second") {
                hours += num / 3600.0;
            } else if unit.starts_with("minute") {
                hours += num / 60.0;
            } else if unit.starts_with("hour") {
                hours += num;
            } else if unit.starts_with("day") {
                hours += num * 24.0;
            } else if unit.starts_with("week") {
                hours += num * 24.0 * 7.0;
            } else if unit.starts_with("month") {
                hours += num * 24.0 * 30.0;
            } else if unit.starts_with("year") {
                hours += num * 24.0 * 365.0;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    hours
}

/// Parse mem_usage string like "119.7MiB / 23.41GiB" to MiB (first part only)
fn parse_mem_mib(s: &str) -> f64 {
    let used_part = s.split('/').next().unwrap_or("").trim();
    if used_part.ends_with("GiB") {
        used_part.trim_end_matches("GiB").trim().parse::<f64>().unwrap_or(0.0) * 1024.0
    } else if used_part.ends_with("MiB") {
        used_part.trim_end_matches("MiB").trim().parse::<f64>().unwrap_or(0.0)
    } else if used_part.ends_with("KiB") {
        used_part.trim_end_matches("KiB").trim().parse::<f64>().unwrap_or(0.0) / 1024.0
    } else if used_part.ends_with("B") {
        used_part.trim_end_matches("B").trim().parse::<f64>().unwrap_or(0.0) / (1024.0 * 1024.0)
    } else {
        0.0
    }
}
