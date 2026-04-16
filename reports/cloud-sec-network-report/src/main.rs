mod types;
mod context;
mod port_scan;
mod tls_audit;
mod wireguard;
mod dns_audit;
mod firewall;
mod output;

use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("Cloud Security: Network Report");
    println!("==============================\n");

    // Load context
    let ctx = context::load_context()?;
    let caps = reports_common::capabilities::RuntimeCapabilities::detect().await;

    // Phase 1: Port scan all VMs (parallel)
    let t = Instant::now();
    let (port_checks, port_data) = port_scan::scan_all_vms(&ctx, &caps).await;
    let port_ms = t.elapsed().as_millis() as u64;

    // Phase 2: Parallel — TLS, DNS, WireGuard
    let t = Instant::now();
    let (tls_result, dns_result, wg_result) = tokio::join!(
        tls_audit::audit_all_certs(&ctx, &caps),
        dns_audit::validate_dns(&ctx, &caps),
        wireguard::check_wg(&ctx, &caps),
    );
    let (tls_checks, _tls_data) = tls_result;
    let (dns_checks, _dns_data) = dns_result;
    let (wg_checks, _wg_data) = wg_result;
    let parallel_ms = t.elapsed().as_millis() as u64;

    // Phase 3: Firewall audit (needs port_data + SSH)
    let t = Instant::now();
    let (fw_checks, _fw_data) = firewall::audit_firewalls(&ctx, &port_data, &caps).await;
    let fw_ms = t.elapsed().as_millis() as u64;

    let duration_ms = start.elapsed().as_millis() as u64;

    // Collect all checks
    let all_checks: Vec<&reports_common::types::Check> = port_checks
        .iter()
        .chain(&tls_checks)
        .chain(&dns_checks)
        .chain(&wg_checks)
        .chain(&fw_checks)
        .collect();

    let summary = reports_common::types::Summary::from_checks(&all_checks);

    // Timers
    let mut timers = HashMap::new();
    timers.insert("Port scan".into(), port_ms);
    timers.insert("TLS+DNS+WG (parallel)".into(), parallel_ms);
    timers.insert("Firewall audit".into(), fw_ms);

    // Build template vars
    let vars = output::build_template_vars(
        &port_checks,
        &tls_checks,
        &dns_checks,
        &wg_checks,
        &fw_checks,
        &all_checks,
        &summary,
        &timers,
        duration_ms,
    );

    // Render template
    reports_common::template::render(
        "cloud_sec_network.md.tpl",
        "cloud_sec_network.md",
        &vars,
    )?;

    // Write JSON
    let json_out = serde_json::json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "duration_ms": duration_ms,
        "port_scan": port_data,
        "summary": summary,
    });
    std::fs::write(
        "cloud_sec_network.json",
        serde_json::to_string_pretty(&json_out)?,
    )?;
    println!("Wrote cloud_sec_network.json");

    Ok(())
}
