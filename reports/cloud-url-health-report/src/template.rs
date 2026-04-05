//! Template renderer — $VAR substitution (same pattern as cloud-health-full)

use crate::{icon, truncate, Report};
use std::collections::HashMap;

const TEMPLATE_PATH: &str = "cloud_url_health.md.tpl";
const OUTPUT_PATH: &str = "cloud_url_health.md";

pub fn render(report: &Report) -> anyhow::Result<()> {
    let mut template = std::fs::read_to_string(TEMPLATE_PATH)?;

    let mut vars: HashMap<String, String> = HashMap::new();

    vars.insert("GENERATED_DATE".into(), report.generated.clone());
    vars.insert(
        "SUMMARY".into(),
        format!(
            "  Public:  {}/{} healthy\n  Private: {}/{} healthy\n  Duration: {:.1}s",
            report.summary.public_healthy,
            report.summary.public_total,
            report.summary.private_healthy,
            report.summary.private_total,
            report.duration_ms as f64 / 1000.0,
        ),
    );

    // Issues: only failures
    let mut issues = String::new();
    for r in report.public.iter().chain(report.private.iter()) {
        if !r.errors.is_empty() {
            issues.push_str(&format!("  FAIL  {}  {}\n", r.domain, r.errors.join("; ")));
        }
    }
    if issues.is_empty() {
        issues = "  No issues detected.\n".into();
    }
    vars.insert("ISSUES".into(), issues);

    // Public table
    let mut pub_table = format!(
        "  {:<40} {:>4} {:>4} {:>4} {:>5} {:>5}  {}\n",
        "DOMAIN", "DNS", "TCP", "TLS", "HTTP", "ms", "DETAIL"
    );
    pub_table.push_str(&format!("  {}\n", "-".repeat(90)));
    for r in &report.public {
        let detail = if r.errors.is_empty() {
            r.http_code.map(|c| c.to_string()).unwrap_or_default()
        } else {
            r.errors.last().cloned().unwrap_or_default()
        };
        pub_table.push_str(&format!(
            "  {:<40} {:>4} {:>4} {:>4} {:>5} {:>3}ms  {}\n",
            r.domain,
            icon(&r.dns),
            icon(&r.tcp),
            icon(&r.tls),
            icon(&r.http),
            r.latency_ms,
            truncate(&detail, 35),
        ));
    }
    vars.insert("PUBLIC_URLS".into(), pub_table);

    // Private table
    let mut priv_table = format!(
        "  {:<30} {:<22} {:>4} {:>5} {:>5}  {}\n",
        "SERVICE", "UPSTREAM", "TCP", "HTTP", "ms", "DETAIL"
    );
    priv_table.push_str(&format!("  {}\n", "-".repeat(90)));
    for r in &report.private {
        let detail = if r.errors.is_empty() {
            r.http_code.map(|c| c.to_string()).unwrap_or_default()
        } else {
            r.errors.last().cloned().unwrap_or_default()
        };
        priv_table.push_str(&format!(
            "  {:<30} {:<22} {:>4} {:>5} {:>3}ms  {}\n",
            r.domain,
            r.url,
            icon(&r.tcp),
            icon(&r.http),
            r.latency_ms,
            truncate(&detail, 35),
        ));
    }
    vars.insert("PRIVATE_URLS".into(), priv_table);

    // Result summary
    let total = report.summary.public_total + report.summary.private_total;
    let healthy = report.summary.public_healthy + report.summary.private_healthy;
    let failed = total - healthy;
    vars.insert(
        "RESULT_SUMMARY".into(),
        format!(
            "{}/{} healthy ({} failed) in {:.1}s",
            healthy,
            total,
            failed,
            report.duration_ms as f64 / 1000.0,
        ),
    );

    // Substitute $VARS (longest key first)
    let mut sorted_keys: Vec<&String> = vars.keys().collect();
    sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

    for key in sorted_keys {
        let placeholder = format!("${}", key);
        template = template.replace(&placeholder, &vars[key]);
    }

    std::fs::write(OUTPUT_PATH, template)?;
    println!("Wrote {}", OUTPUT_PATH);
    Ok(())
}
