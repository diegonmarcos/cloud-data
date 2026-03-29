//! Cloud Mail Full Report -- 7-Phase (0-6) Stalwart Mail Diagnostic
//! Native async: TCP, HTTP, DNS, SSH batch, TLS probing -- all parallel
//!
//! Phase 0: INSTANT KPIs (public URLs + DNS, no SSH, <2s)
//! Phase 1: PRE-FLIGHT (3-VM parallel WG + batch SSH)
//! Phase 2: CONTAINERS (from cached batch data)
//! Phase 3: NETWORK + AUTH (~30 checks: TLS, HTTP, OIDC, Caddy L4, MCP)
//! Phase 4: DNS AUTH (MX, DKIM, SPF, DMARC)
//! Phase 5: MAIL INTERNALS (IMAP, queue, spam, sieve, quota, Admin API)
//! Phase 6: E2E DELIVERY (optional, requires RESEND_API_KEY)
//!
//! Usage: cargo run --release (from cloud-mail-full-report/)

mod checks;
mod constants;
mod output;
mod phases;
mod ssh;
mod template;
mod types;

use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::time::Instant;
use types::*;

fn load_bearer_token() -> Option<String> {
    let home = std::env::var("HOME").unwrap_or("/home/diego".into());
    let path = format!("{}/{}", home, constants::BEARER_TOKEN_PATH);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["access_token"].as_str().map(|s| s.to_string()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("=== Cloud Mail Full Report (7-Phase Stalwart Diagnostic) ===");

    let bearer_token = load_bearer_token();
    if bearer_token.is_some() {
        println!("Bearer token: loaded");
    } else {
        println!("Bearer token: not found (OIDC checks will fail)");
    }

    let mut timers: HashMap<String, u64> = HashMap::new();

    // ════════════════════════════════════════════════════════════
    // PHASE 0: INSTANT KPIs (no SSH, <2s)
    // ════════════════════════════════════════════════════════════
    let t0 = Instant::now();
    println!("\n  Phase 0: Instant KPIs...");
    let instant_kpis = phases::phase0_instant_kpis().await;
    let p0_ms = t0.elapsed().as_millis() as u64;
    timers.insert("P0_instant_kpis".into(), p0_ms);
    println!(
        "  Phase 0: {}/{} in {:.1}s",
        instant_kpis.iter().filter(|c| c.passed).count(),
        instant_kpis.len(),
        p0_ms as f64 / 1000.0
    );

    // ════════════════════════════════════════════════════════════
    // PHASE 1: PRE-FLIGHT (3-VM parallel WG + batch SSH)
    // ════════════════════════════════════════════════════════════
    let t1 = Instant::now();
    println!("\n  Phase 1: Pre-flight (3-VM parallel)...");
    let (preflight_checks, mail_data, apps_data, proxy_data) = phases::preflight().await;
    let p1_ms = t1.elapsed().as_millis() as u64;
    timers.insert("P1_preflight".into(), p1_ms);
    println!(
        "  Phase 1: {}/{} in {:.1}s (mail={} apps={} proxy={})",
        preflight_checks.iter().filter(|c| c.passed).count(),
        preflight_checks.len(),
        p1_ms as f64 / 1000.0,
        if mail_data.is_some() { "OK" } else { "FAIL" },
        if apps_data.is_some() { "OK" } else { "FAIL" },
        if proxy_data.is_some() { "OK" } else { "FAIL" },
    );

    let ssh_ok = mail_data.is_some();

    // ════════════════════════════════════════════════════════════
    // PHASES 2-5: PARALLEL where possible
    // ════════════════════════════════════════════════════════════
    let t_par = Instant::now();

    // Phase 2: CONTAINERS (sync, reads cache)
    let containers = if ssh_ok {
        phases::container_health(&mail_data, &apps_data)
    } else {
        vec![Check {
            name: "skipped".into(),
            passed: false,
            details: "SSH unreachable".into(),
            duration_ms: 0,
            error: Some("SSH to oci-mail failed".into()),
            severity: Severity::Critical,
        }]
    };

    // Phase 3, 4 are async -- run in parallel
    let (network, dns_auth_checks) = tokio::join!(
        phases::network_checks(&mail_data, &apps_data, &proxy_data, &bearer_token),
        phases::dns_auth(),
    );

    // Phase 5: MAIL INTERNALS (sync, reads cache)
    let internals = if ssh_ok {
        phases::mail_internals(&mail_data)
    } else {
        vec![Check {
            name: "skipped".into(),
            passed: false,
            details: "SSH unreachable".into(),
            duration_ms: 0,
            error: Some("SSH to oci-mail failed".into()),
            severity: Severity::Critical,
        }]
    };

    let par_ms = t_par.elapsed().as_millis() as u64;
    timers.insert("P2_containers".into(), 0);
    timers.insert("P3_network".into(), par_ms);
    timers.insert("P4_dns_auth".into(), par_ms);
    timers.insert("P5_internals".into(), 0);
    timers.insert("P2-P5_parallel".into(), par_ms);

    println!(
        "\n  Phases 2-5 parallel: {:.1}s",
        par_ms as f64 / 1000.0
    );
    println!(
        "    P2 Containers: {}/{}",
        containers.iter().filter(|c| c.passed).count(),
        containers.len()
    );
    println!(
        "    P3 Network: {}/{}",
        network.iter().filter(|c| c.passed).count(),
        network.len()
    );
    println!(
        "    P4 DNS Auth: {}/{}",
        dns_auth_checks.iter().filter(|c| c.passed).count(),
        dns_auth_checks.len()
    );
    println!(
        "    P5 Internals: {}/{}",
        internals.iter().filter(|c| c.passed).count(),
        internals.len()
    );

    // ════════════════════════════════════════════════════════════
    // PHASE 6: E2E DELIVERY (optional)
    // ════════════════════════════════════════════════════════════
    let t6 = Instant::now();
    println!("\n  Phase 6: E2E Delivery...");
    let e2e_delivery = phases::e2e_delivery(&mail_data).await;
    let p6_ms = t6.elapsed().as_millis() as u64;
    timers.insert("P6_e2e_delivery".into(), p6_ms);
    println!(
        "  Phase 6: {}/{} in {:.1}s",
        e2e_delivery.iter().filter(|c| c.passed).count(),
        e2e_delivery.len(),
        p6_ms as f64 / 1000.0
    );

    // ════════════════════════════════════════════════════════════
    // BUILD RESULTS
    // ════════════════════════════════════════════════════════════
    let total_ms = start.elapsed().as_millis() as u64;
    timers.insert("TOTAL".into(), total_ms);

    let all_checks: Vec<&Check> = instant_kpis
        .iter()
        .chain(&preflight_checks)
        .chain(&containers)
        .chain(&network)
        .chain(&dns_auth_checks)
        .chain(&internals)
        .chain(&e2e_delivery)
        .collect();

    let total_count = all_checks.len();
    let passed_count = all_checks.iter().filter(|c| c.passed).count();
    let failed_count = total_count - passed_count;
    let critical_count = all_checks
        .iter()
        .filter(|c| !c.passed && c.severity == Severity::Critical)
        .count();
    let warning_count = all_checks
        .iter()
        .filter(|c| !c.passed && c.severity == Severity::Warning)
        .count();

    let results = MailHealthResult {
        generated: Utc::now().to_rfc3339(),
        duration_ms: total_ms,
        instant_kpis,
        preflight: preflight_checks,
        containers,
        network,
        dns_auth: dns_auth_checks,
        internals,
        e2e_delivery,
        summary: Summary {
            total_checks: total_count,
            passed: passed_count,
            failed: failed_count,
            warnings: warning_count,
            critical: critical_count,
        },
        timers,
    };

    // Render template
    let vars = output::build_template_vars(&results);
    template::render(&vars)?;

    // Write JSON
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("cloud_mail_full.json", &json)?;

    println!(
        "\n=== DONE in {:.1}s === {}/{} passed, {} critical, {} warnings",
        total_ms as f64 / 1000.0,
        passed_count,
        total_count,
        critical_count,
        warning_count,
    );
    println!("-> cloud_mail_full.json + cloud_mail_full.md");

    Ok(())
}
