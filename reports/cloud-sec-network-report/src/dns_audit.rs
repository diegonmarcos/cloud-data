use crate::context::NetworkContext;
use crate::types::DnsValidationResult;
use reports_common::checks;
use reports_common::types::{Check, Severity};
use std::collections::HashSet;
use std::time::Instant;

/// Validate DNS records for all caddy route domains
pub async fn validate_dns(ctx: &NetworkContext) -> (Vec<Check>, Vec<DnsValidationResult>) {
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

    println!("DNS audit: {} unique domains", domains.len());

    // Check A records for each domain
    for domain in &domains {
        let t = Instant::now();

        let cf_result = checks::dns_resolve(&public, domain).await;
        let google_result = checks::dns_resolve(&google, domain).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let cf_ip = cf_result.unwrap_or_else(|| "NXDOMAIN".into());
        let google_ip = google_result.unwrap_or_else(|| "NXDOMAIN".into());

        let matches = cf_ip == google_ip && cf_ip != "NXDOMAIN";

        let result = DnsValidationResult {
            domain: domain.clone(),
            record_type: "A".into(),
            expected: cf_ip.clone(),
            actual: google_ip.clone(),
            matches,
        };

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
            format!("NXDOMAIN (no A record)")
        } else {
            format!("CF={} Google={} (mismatch)", cf_ip, google_ip)
        };

        all_checks.push(Check {
            name: format!("dns:A:{}", domain),
            passed,
            details,
            duration_ms: elapsed,
            error: None,
            severity,
        });

        all_results.push(result);
    }

    // Check MX/SPF/DKIM for mail domains and the base domain
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
        // MX check
        let t = Instant::now();
        let mx_records = checks::dns_mx(&public, domain).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let mx_str = if mx_records.is_empty() {
            "none".into()
        } else {
            mx_records
                .iter()
                .map(|(pref, exch)| format!("{} {}", pref, exch))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let mx_passed = !mx_records.is_empty();
        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "MX".into(),
            expected: "(any)".into(),
            actual: mx_str.clone(),
            matches: mx_passed,
        });

        all_checks.push(Check {
            name: format!("dns:MX:{}", domain),
            passed: mx_passed,
            details: if mx_passed {
                format!("MX={}", mx_str)
            } else {
                "No MX records found".into()
            },
            duration_ms: elapsed,
            error: None,
            severity: if mx_passed {
                Severity::Info
            } else {
                Severity::Warning
            },
        });

        // SPF check
        let t = Instant::now();
        let txt = checks::dns_txt(&public, domain).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let has_spf = txt
            .as_ref()
            .map(|t| t.contains("v=spf1"))
            .unwrap_or(false);
        let spf_text = txt.unwrap_or_else(|| "none".into());

        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "SPF".into(),
            expected: "v=spf1 ...".into(),
            actual: if has_spf {
                spf_text.clone()
            } else {
                "missing".into()
            },
            matches: has_spf,
        });

        all_checks.push(Check {
            name: format!("dns:SPF:{}", domain),
            passed: has_spf,
            details: if has_spf {
                "SPF record present".into()
            } else {
                "No SPF record".into()
            },
            duration_ms: elapsed,
            error: None,
            severity: if has_spf {
                Severity::Info
            } else {
                Severity::Warning
            },
        });

        // DKIM check (_dmarc record as proxy)
        let dkim_domain = format!("_dmarc.{}", domain);
        let t = Instant::now();
        let dmarc_txt = checks::dns_txt(&public, &dkim_domain).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let has_dmarc = dmarc_txt
            .as_ref()
            .map(|t| t.contains("v=DMARC1"))
            .unwrap_or(false);

        all_results.push(DnsValidationResult {
            domain: domain.clone(),
            record_type: "DMARC".into(),
            expected: "v=DMARC1 ...".into(),
            actual: dmarc_txt.unwrap_or_else(|| "missing".into()),
            matches: has_dmarc,
        });

        all_checks.push(Check {
            name: format!("dns:DMARC:{}", domain),
            passed: has_dmarc,
            details: if has_dmarc {
                "DMARC record present".into()
            } else {
                "No DMARC record".into()
            },
            duration_ms: elapsed,
            error: None,
            severity: if has_dmarc {
                Severity::Info
            } else {
                Severity::Warning
            },
        });
    }

    (all_checks, all_results)
}
