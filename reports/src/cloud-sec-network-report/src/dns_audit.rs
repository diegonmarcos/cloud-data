use crate::context::NetworkContext;
use crate::types::DnsValidationResult;
use reports_common::capabilities::RuntimeCapabilities;
use reports_common::checks;
use reports_common::types::{Check, Severity};
use std::collections::HashSet;
use std::time::Instant;

/// Dual-path DNS validation: public resolvers (always) + Hickory internal (when WG up)
pub async fn validate_dns(
    ctx: &NetworkContext,
    caps: &RuntimeCapabilities,
) -> (Vec<Check>, Vec<DnsValidationResult>) {
    let mut all_checks = Vec::new();
    let mut all_results = Vec::new();

    let public = checks::public_resolver();
    let google = checks::google_resolver();

    // Deduplicate domains
    let mut seen = HashSet::new();
    let domains: Vec<String> = ctx
        .caddy_routes
        .iter()
        .filter(|r| !r.domain.is_empty())
        .filter(|r| seen.insert(r.domain.clone()))
        .map(|r| r.domain.clone())
        .collect();

    println!(
        "DNS audit: {} domains (public{}) ",
        domains.len(),
        if caps.hickory_up { " + Hickory internal" } else { "" }
    );

    // ── External: public resolvers (Cloudflare + Google cross-check) ──
    for domain in &domains {
        let t = Instant::now();

        let cf_result = checks::dns_resolve(&public, domain).await;
        let google_result = checks::dns_resolve(&google, domain).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let cf_ip = cf_result.unwrap_or_else(|| "NXDOMAIN".into());
        let google_ip = google_result.unwrap_or_else(|| "NXDOMAIN".into());
        let matches = cf_ip == google_ip && cf_ip != "NXDOMAIN";

        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "A".into(),
            resolver: "external".into(),
            expected: cf_ip.clone(),
            actual: google_ip.clone(),
            matches,
        });

        let passed = cf_ip != "NXDOMAIN";
        let severity = if cf_ip == "NXDOMAIN" {
            Severity::Critical
        } else if !matches {
            Severity::Warning
        } else {
            Severity::Info
        };

        let details = if matches {
            format!("A={}", cf_ip)
        } else if cf_ip == "NXDOMAIN" {
            "NXDOMAIN (no A record)".into()
        } else {
            format!("CF={} Google={} (mismatch)", cf_ip, google_ip)
        };

        all_checks.push(Check {
            name: format!("ext:dns:A:{}", domain),
            passed,
            details,
            duration_ms: elapsed,
            error: None,
            severity,
        });
    }

    // ── Internal: Hickory DNS at 10.0.0.1 (when WG is up) ──
    if caps.hickory_up {
        let hickory = checks::hickory_resolver();

        for domain in &domains {
            let t = Instant::now();
            let hickory_result = checks::dns_resolve(&hickory, domain).await;
            let elapsed = t.elapsed().as_millis() as u64;

            let hickory_ip = hickory_result.unwrap_or_else(|| "NXDOMAIN".into());

            // Compare with public result
            let public_ip = all_results
                .iter()
                .find(|r| r.domain == *domain && r.record_type == "A" && r.resolver == "external")
                .map(|r| r.expected.clone())
                .unwrap_or_else(|| "unknown".into());

            let matches = hickory_ip != "NXDOMAIN";

            all_results.push(DnsValidationResult {
                domain: domain.clone(),
                record_type: "A".into(),
                resolver: "hickory".into(),
                expected: public_ip.clone(),
                actual: hickory_ip.clone(),
                matches,
            });

            let passed = hickory_ip != "NXDOMAIN";
            let details = if passed {
                format!("Hickory A={} (public={})", hickory_ip, public_ip)
            } else {
                format!("Hickory NXDOMAIN (public={})", public_ip)
            };

            all_checks.push(Check {
                name: format!("int:dns:A:{}", domain),
                passed,
                details,
                duration_ms: elapsed,
                error: None,
                severity: if passed { Severity::Info } else { Severity::Warning },
            });
        }
    }

    // ── Mail DNS: MX/SPF/DMARC (external only — authoritative check) ──
    let base_domain = "diegonmarcos.com";
    let mut mail_domains: Vec<String> = domains
        .iter()
        .filter(|d| d.contains("mail"))
        .cloned()
        .collect();
    if !mail_domains.contains(&base_domain.to_string()) {
        mail_domains.push(base_domain.to_string());
    }

    for domain in &mail_domains {
        // MX
        let t = Instant::now();
        let mx_records = checks::dns_mx(&public, domain).await;
        let elapsed = t.elapsed().as_millis() as u64;
        let mx_str = if mx_records.is_empty() {
            "none".into()
        } else {
            mx_records.iter().map(|(p, e)| format!("{} {}", p, e)).collect::<Vec<_>>().join(", ")
        };
        let mx_passed = !mx_records.is_empty();
        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "MX".into(),
            resolver: "external".into(),
            expected: "(any)".into(),
            actual: mx_str.clone(),
            matches: mx_passed,
        });
        all_checks.push(Check {
            name: format!("ext:dns:MX:{}", domain),
            passed: mx_passed,
            details: if mx_passed { format!("MX={}", mx_str) } else { "No MX records found".into() },
            duration_ms: elapsed,
            error: None,
            severity: if mx_passed { Severity::Info } else { Severity::Warning },
        });

        // SPF
        let t = Instant::now();
        let txt = checks::dns_txt(&public, domain).await;
        let elapsed = t.elapsed().as_millis() as u64;
        let has_spf = txt.as_ref().map(|t| t.contains("v=spf1")).unwrap_or(false);
        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "SPF".into(),
            resolver: "external".into(),
            expected: "v=spf1 ...".into(),
            actual: if has_spf { txt.unwrap_or_default() } else { "missing".into() },
            matches: has_spf,
        });
        all_checks.push(Check {
            name: format!("ext:dns:SPF:{}", domain),
            passed: has_spf,
            details: if has_spf { "SPF record present".into() } else { "No SPF record".into() },
            duration_ms: elapsed,
            error: None,
            severity: if has_spf { Severity::Info } else { Severity::Warning },
        });

        // DMARC
        let dkim_domain = format!("_dmarc.{}", domain);
        let t = Instant::now();
        let dmarc_txt = checks::dns_txt(&public, &dkim_domain).await;
        let elapsed = t.elapsed().as_millis() as u64;
        let has_dmarc = dmarc_txt.as_ref().map(|t| t.contains("v=DMARC1")).unwrap_or(false);
        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "DMARC".into(),
            resolver: "external".into(),
            expected: "v=DMARC1 ...".into(),
            actual: dmarc_txt.unwrap_or_else(|| "missing".into()),
            matches: has_dmarc,
        });
        all_checks.push(Check {
            name: format!("ext:dns:DMARC:{}", domain),
            passed: has_dmarc,
            details: if has_dmarc { "DMARC record present".into() } else { "No DMARC record".into() },
            duration_ms: elapsed,
            error: None,
            severity: if has_dmarc { Severity::Info } else { Severity::Warning },
        });
    }

    (all_checks, all_results)
}
