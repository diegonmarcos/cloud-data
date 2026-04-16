use reports_common::output;
use reports_common::types::{Check, Summary};
use std::collections::HashMap;

/// Build template variables for the markdown report
pub fn build_template_vars(
    export_checks: &[Check],
    yara_checks: &[Check],
    siem_checks: &[Check],
    ti_checks: &[Check],
    corr_checks: &[Check],
    all_checks: &[&Check],
    summary: &Summary,
    timers: &HashMap<String, u64>,
    duration_ms: u64,
) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    // Issues summary
    vars.insert(
        "ISSUES_SUMMARY".into(),
        output::build_issues_summary(all_checks),
    );

    // Export status
    vars.insert(
        "EXPORT_STATUS".into(),
        output::format_checks(export_checks),
    );

    // YARA summary (checks only)
    vars.insert(
        "YARA_SUMMARY".into(),
        output::format_checks(yara_checks),
    );

    // YARA hits detail
    let yara_failed: Vec<&Check> = yara_checks.iter().filter(|c| !c.passed).collect();
    let yara_hits_text = if yara_failed.is_empty() {
        "  No YARA matches detected.".into()
    } else {
        yara_failed
            .iter()
            .map(|c| format!("  - {}: {}", c.name, c.details))
            .collect::<Vec<_>>()
            .join("\n")
    };
    vars.insert("YARA_HITS".into(), yara_hits_text);

    // SIEM summary
    vars.insert(
        "SIEM_SUMMARY".into(),
        output::format_checks(siem_checks),
    );

    // SIEM alerts detail
    let siem_failed: Vec<&Check> = siem_checks.iter().filter(|c| !c.passed).collect();
    let siem_alerts_text = if siem_failed.is_empty() {
        "  No critical SIEM alerts.".into()
    } else {
        siem_failed
            .iter()
            .map(|c| format!("  - {}: {}", c.name, c.details))
            .collect::<Vec<_>>()
            .join("\n")
    };
    vars.insert("SIEM_ALERTS".into(), siem_alerts_text);

    // Threat intel
    vars.insert(
        "THREAT_INTEL".into(),
        output::format_checks(ti_checks),
    );

    // Correlations
    vars.insert(
        "CORRELATIONS".into(),
        output::format_checks(corr_checks),
    );

    // Performance
    vars.insert(
        "PERFORMANCE".into(),
        output::build_performance(timers, duration_ms),
    );

    // Result summary
    vars.insert(
        "RESULT_SUMMARY".into(),
        output::build_result_summary(summary),
    );

    // Generated date
    vars.insert(
        "GENERATED_DATE".into(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    );

    vars
}
