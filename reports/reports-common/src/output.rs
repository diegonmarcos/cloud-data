use crate::types::*;
use std::collections::HashMap;

/// Format a section of checks with icons, duration, severity
pub fn format_checks(checks: &[Check]) -> String {
    if checks.is_empty() {
        return "  (no checks)".to_string();
    }

    let mut lines = Vec::new();
    for c in checks {
        let icon = if c.passed {
            "\u{2705}"
        } else {
            match c.severity {
                Severity::Critical => "\u{274c}",
                Severity::Warning => "\u{26a0}\u{fe0f} ",
                Severity::Info => "\u{2139}\u{fe0f} ",
            }
        };

        let duration = if c.duration_ms > 0 {
            format!(" ({:.1}s)", c.duration_ms as f64 / 1000.0)
        } else {
            String::new()
        };

        let severity_tag = if !c.passed {
            format!(" [{}]", c.severity)
        } else {
            String::new()
        };

        lines.push(format!(
            "  {} {:30} {}{}{}",
            icon, c.name, c.details, duration, severity_tag,
        ));
    }

    // Summary line
    let total = checks.len();
    let passed = checks.iter().filter(|c| c.passed).count();
    let failed = total - passed;
    lines.push(String::new());
    lines.push(format!(
        "  Summary: {}/{} passed, {} failed",
        passed, total, failed
    ));

    lines.join("\n")
}

/// Build issues summary from a flat list of all checks
pub fn build_issues_summary(all_checks: &[&Check]) -> String {
    let failed: Vec<&&Check> = all_checks.iter().filter(|c| !c.passed).collect();

    if failed.is_empty() {
        return "  No issues found \u{2014} all checks passed.".into();
    }

    let critical: Vec<_> = failed
        .iter()
        .filter(|c| c.severity == Severity::Critical)
        .collect();
    let warnings: Vec<_> = failed
        .iter()
        .filter(|c| c.severity == Severity::Warning)
        .collect();
    let info: Vec<_> = failed
        .iter()
        .filter(|c| c.severity == Severity::Info)
        .collect();

    let mut lines = vec![format!(
        "  {} issues: {} critical, {} warnings, {} info",
        failed.len(),
        critical.len(),
        warnings.len(),
        info.len()
    )];
    lines.push(String::new());

    if !critical.is_empty() {
        lines.push("  CRITICAL:".into());
        for c in &critical {
            lines.push(format!("    \u{274c} {}: {}", c.name, c.details));
        }
    }

    if !warnings.is_empty() {
        lines.push("  WARNINGS:".into());
        for c in &warnings {
            lines.push(format!("    \u{26a0}\u{fe0f}  {}: {}", c.name, c.details));
        }
    }

    if !info.is_empty() {
        lines.push("  INFO:".into());
        for c in &info {
            lines.push(format!("    \u{2139}\u{fe0f}  {}: {}", c.name, c.details));
        }
    }

    lines.join("\n")
}

/// Build performance section from timers
pub fn build_performance(timers: &HashMap<String, u64>, total_ms: u64) -> String {
    let mut sorted: Vec<_> = timers.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    let mut lines = Vec::new();
    for (k, v) in sorted {
        lines.push(format!("  {:24} {:.1}s", k, *v as f64 / 1000.0));
    }
    lines.push(String::new());
    lines.push(format!(
        "  Total: {:.1}s | Engine: Rust (native async tokio)",
        total_ms as f64 / 1000.0
    ));
    lines.push("  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)".to_string());
    lines.join("\n")
}

/// Build result summary string
pub fn build_result_summary(summary: &Summary) -> String {
    if summary.critical == 0 && summary.failed == 0 {
        format!(
            "ALL CLEAR -- {}/{} checks passed",
            summary.passed, summary.total_checks
        )
    } else if summary.critical > 0 {
        format!(
            "CRITICAL -- {}/{} passed, {} critical, {} warnings",
            summary.passed, summary.total_checks, summary.critical, summary.warnings
        )
    } else {
        format!(
            "DEGRADED -- {}/{} passed, {} warnings",
            summary.passed, summary.total_checks, summary.warnings
        )
    }
}
