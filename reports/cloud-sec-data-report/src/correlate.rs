use crate::types::{Correlation, SiemAlert, ThreatIntelMatch, YaraHit};
use reports_common::types::{Check, Severity};
use std::collections::{HashMap, HashSet};

/// Cross-correlate YARA hits, SIEM alerts, and threat intel matches
pub fn correlate(
    yara_hits: &[YaraHit],
    siem_alerts: &[SiemAlert],
    ti_matches: &[ThreatIntelMatch],
) -> (Vec<Check>, Vec<Correlation>) {
    let mut checks = Vec::new();
    let mut correlations = Vec::new();

    // Pattern 1: YARA hit + SIEM alert on same VM = confirmed threat
    let yara_vms: HashSet<&str> = yara_hits.iter().map(|h| h.vm.as_str()).collect();
    let siem_vms: HashSet<&str> = siem_alerts.iter().map(|a| a.vm.as_str()).collect();
    let overlap_vms: Vec<&&str> = yara_vms.intersection(&siem_vms).collect();

    for vm in &overlap_vms {
        let vm_yara: Vec<&YaraHit> = yara_hits.iter().filter(|h| h.vm == ***vm).collect();
        let vm_siem: Vec<&SiemAlert> = siem_alerts.iter().filter(|a| a.vm == ***vm).collect();

        let evidence: Vec<String> = vm_yara
            .iter()
            .map(|h| format!("YARA: {} in {}:{}", h.rule_name, h.container, h.file_path))
            .chain(
                vm_siem
                    .iter()
                    .map(|a| format!("SIEM: {} at {}", a.rule, a.timestamp)),
            )
            .collect();

        correlations.push(Correlation {
            description: format!(
                "Confirmed threat on {}: YARA hit + SIEM alert",
                vm
            ),
            severity: "critical".into(),
            sources: vec!["yara".into(), "siem".into()],
            evidence,
        });
    }

    // Pattern 2: YARA hit hash matches threat intel = known malware
    let yara_hashes: HashSet<&str> = yara_hits
        .iter()
        .map(|h| h.file_hash.as_str())
        .filter(|h| !h.is_empty() && *h != "(unreadable)")
        .collect();

    let ti_indicators: HashSet<&str> = ti_matches
        .iter()
        .filter(|m| m.indicator_type == "hash" || m.indicator_type == "hash_match")
        .map(|m| m.indicator.as_str())
        .collect();

    let hash_overlap: Vec<&&str> = yara_hashes.intersection(&ti_indicators).collect();
    if !hash_overlap.is_empty() {
        let evidence: Vec<String> = hash_overlap
            .iter()
            .map(|h| {
                let hit = yara_hits.iter().find(|y| y.file_hash == ***h);
                match hit {
                    Some(y) => format!(
                        "Hash {} matched in {}:{} (rule: {})",
                        h, y.vm, y.file_path, y.rule_name
                    ),
                    None => format!("Hash {} matched threat intel", h),
                }
            })
            .collect();

        correlations.push(Correlation {
            description: format!(
                "Known malware detected: {} hash(es) match threat intel",
                hash_overlap.len()
            ),
            severity: "critical".into(),
            sources: vec!["yara".into(), "threat_intel".into()],
            evidence,
        });
    }

    // Pattern 3: Multiple YARA hits on same VM (>3) = systematic compromise
    let mut yara_by_vm: HashMap<&str, Vec<&YaraHit>> = HashMap::new();
    for hit in yara_hits {
        yara_by_vm.entry(hit.vm.as_str()).or_default().push(hit);
    }

    for (vm, hits) in &yara_by_vm {
        if hits.len() > 3 {
            let evidence: Vec<String> = hits
                .iter()
                .map(|h| {
                    format!(
                        "{}: {} ({})",
                        h.container, h.rule_name, h.file_path
                    )
                })
                .collect();

            correlations.push(Correlation {
                description: format!(
                    "Systematic compromise suspected on {}: {} YARA hits",
                    vm,
                    hits.len()
                ),
                severity: "warning".into(),
                sources: vec!["yara".into()],
                evidence,
            });
        }
    }

    // Pattern 4: SIEM alert spike (>10 in last hour) = active attack
    if siem_alerts.len() > 10 {
        // Count alerts with recent timestamps (simple heuristic: if we got >10 total,
        // flag it since we already filtered to 24h window)
        let high_severity: Vec<&SiemAlert> = siem_alerts
            .iter()
            .filter(|a| {
                let s = a.severity.to_lowercase();
                s == "critical" || s == "high"
            })
            .collect();

        if high_severity.len() > 10 {
            let evidence: Vec<String> = high_severity
                .iter()
                .take(5)
                .map(|a| format!("{}: {} on {} ({})", a.timestamp, a.rule, a.vm, a.severity))
                .collect();

            correlations.push(Correlation {
                description: format!(
                    "Alert spike detected: {} high-severity SIEM alerts in 24h",
                    high_severity.len()
                ),
                severity: "warning".into(),
                sources: vec!["siem".into()],
                evidence,
            });
        }
    }

    // Summary check
    if correlations.is_empty() {
        checks.push(Check {
            name: "Cross-correlation".into(),
            passed: true,
            details: "No correlated threats found".into(),
            duration_ms: 0,
            error: None,
            severity: Severity::Info,
        });
    } else {
        let critical_count = correlations
            .iter()
            .filter(|c| c.severity == "critical")
            .count();
        let warning_count = correlations
            .iter()
            .filter(|c| c.severity == "warning")
            .count();

        checks.push(Check {
            name: "Cross-correlation".into(),
            passed: false,
            details: format!(
                "{} correlations: {} critical, {} warning",
                correlations.len(),
                critical_count,
                warning_count,
            ),
            duration_ms: 0,
            error: None,
            severity: if critical_count > 0 {
                Severity::Critical
            } else {
                Severity::Warning
            },
        });
    }

    (checks, correlations)
}
