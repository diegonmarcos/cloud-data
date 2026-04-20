mod types;
mod context;
mod export;
mod scan_config;
mod yara_scan;
mod siem;
mod threat_intel;
mod journal;
mod runtime;
mod diff_analysis;
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

    // Phase 1: Export container filesystems + evidence vault (parallel across VMs and containers)
    let t = Instant::now();
    let (export_checks, exports, evidence_dirs) = export::export_all(&ctx, &caps).await;
    let export_ms = t.elapsed().as_millis() as u64;

    // Phase 2: Parallel — YARA scan + SIEM alerts + Threat intel + Journal + Runtime + Diff
    let t = Instant::now();
    let (yara_result, siem_result, ti_result, journal_result, runtime_result) = tokio::join!(
        yara_scan::scan_all(&exports, &caps, ctx.scan.phases.yara_enabled),
        siem::fetch_all(&ctx, &caps),
        threat_intel::fetch_all(),
        journal::analyze_journals(&evidence_dirs),
        runtime::analyze_runtime(&evidence_dirs),
    );
    let (yara_checks, yara_hits) = yara_result;
    let (siem_checks, siem_alerts) = siem_result;
    let (ti_checks, ti_matches) = ti_result;
    let (journal_checks, journal_alerts) = journal_result;
    let (runtime_checks, runtime_issues) = runtime_result;

    // Diff analysis (sync, runs fast on local files)
    let (diff_checks, diff_changes) = diff_analysis::analyze_diffs(&evidence_dirs);
    let scan_ms = t.elapsed().as_millis() as u64;

    // Phase 3: Cross-correlate (now with all 6 data sources)
    let t = Instant::now();
    let (corr_checks, _correlations) = correlate::correlate(
        &yara_hits,
        &siem_alerts,
        &ti_matches,
        &journal_alerts,
        &runtime_issues,
        &diff_changes,
    );
    let corr_ms = t.elapsed().as_millis() as u64;

    let duration_ms = start.elapsed().as_millis() as u64;

    // Collect all checks
    let all_checks: Vec<&reports_common::types::Check> = export_checks
        .iter()
        .chain(&yara_checks)
        .chain(&siem_checks)
        .chain(&ti_checks)
        .chain(&journal_checks)
        .chain(&runtime_checks)
        .chain(&diff_checks)
        .chain(&corr_checks)
        .collect();

    let summary = reports_common::types::Summary::from_checks(&all_checks);

    // Timers
    let mut timers = HashMap::new();
    timers.insert("Container export + evidence".into(), export_ms);
    timers.insert("YARA+SIEM+ThreatIntel+Journal+Runtime+Diff".into(), scan_ms);
    timers.insert("Correlation".into(), corr_ms);

    // Build template vars
    let vars = output::build_template_vars(
        &export_checks,
        &yara_checks,
        &siem_checks,
        &ti_checks,
        &journal_checks,
        &runtime_checks,
        &diff_checks,
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
        "journal_alerts": journal_alerts,
        "runtime_issues": runtime_issues,
        "diff_changes": diff_changes,
        "summary": summary,
    });
    std::fs::write(
        "cloud_sec_data.json",
        serde_json::to_string_pretty(&json_out)?,
    )?;
    println!("Wrote cloud_sec_data.json");

    // Cleanup temp dirs
    export::cleanup(&exports);
    export::cleanup_evidence(&evidence_dirs);

    Ok(())
}
