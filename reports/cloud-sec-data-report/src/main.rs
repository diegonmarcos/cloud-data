mod types;
mod context;
mod export;
mod yara_scan;
mod siem;
mod threat_intel;
mod correlate;
mod output;

use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("Cloud Security: Data Scan Report");
    println!("================================\n");

    // Load context
    let ctx = context::load_context()?;
    let caps = reports_common::capabilities::RuntimeCapabilities::detect().await;

    // Phase 1: Export container filesystems (parallel across VMs, sequential per VM)
    let t = Instant::now();
    let (export_checks, exports) = export::export_all(&ctx, &caps).await;
    let export_ms = t.elapsed().as_millis() as u64;

    // Phase 2: Parallel — YARA scan + SIEM alerts + Threat intel
    let t = Instant::now();
    let (yara_result, siem_result, ti_result) = tokio::join!(
        yara_scan::scan_all(&exports, &caps),
        siem::fetch_all(&ctx, &caps),
        threat_intel::fetch_all(),
    );
    let (yara_checks, yara_hits) = yara_result;
    let (siem_checks, siem_alerts) = siem_result;
    let (ti_checks, ti_matches) = ti_result;
    let scan_ms = t.elapsed().as_millis() as u64;

    // Phase 3: Cross-correlate
    let t = Instant::now();
    let (corr_checks, _correlations) = correlate::correlate(&yara_hits, &siem_alerts, &ti_matches);
    let corr_ms = t.elapsed().as_millis() as u64;

    let duration_ms = start.elapsed().as_millis() as u64;

    // Collect all checks
    let all_checks: Vec<&reports_common::types::Check> = export_checks
        .iter()
        .chain(&yara_checks)
        .chain(&siem_checks)
        .chain(&ti_checks)
        .chain(&corr_checks)
        .collect();

    let summary = reports_common::types::Summary::from_checks(&all_checks);

    // Timers
    let mut timers = HashMap::new();
    timers.insert("Container export".into(), export_ms);
    timers.insert("YARA+SIEM+ThreatIntel".into(), scan_ms);
    timers.insert("Correlation".into(), corr_ms);

    // Build template vars
    let vars = output::build_template_vars(
        &export_checks,
        &yara_checks,
        &siem_checks,
        &ti_checks,
        &corr_checks,
        &all_checks,
        &summary,
        &timers,
        duration_ms,
    );

    // Render template
    reports_common::template::render(
        "cloud_sec_data.md.tpl",
        "cloud_sec_data.md",
        &vars,
    )?;

    // Write JSON
    let json_out = serde_json::json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "duration_ms": duration_ms,
        "yara_hits": yara_hits,
        "siem_alerts": siem_alerts,
        "threat_intel_matches": ti_matches,
        "summary": summary,
    });
    std::fs::write(
        "cloud_sec_data.json",
        serde_json::to_string_pretty(&json_out)?,
    )?;
    println!("Wrote cloud_sec_data.json");

    // Cleanup temp dirs
    export::cleanup(&exports);

    Ok(())
}
