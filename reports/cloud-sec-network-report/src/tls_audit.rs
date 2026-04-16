use crate::context::NetworkContext;
use crate::types::TlsCertResult;
use reports_common::types::{Check, Severity};
use std::collections::HashSet;
use std::time::Instant;

/// Audit TLS certificates for all unique domains in caddy routes
pub async fn audit_all_certs(ctx: &NetworkContext) -> (Vec<Check>, Vec<TlsCertResult>) {
    let mut checks = Vec::new();
    let mut results = Vec::new();

    // Deduplicate domains
    let mut seen = HashSet::new();
    let domains: Vec<String> = ctx
        .caddy_routes
        .iter()
        .filter(|r| !r.domain.is_empty())
        .filter(|r| seen.insert(r.domain.clone()))
        .map(|r| r.domain.clone())
        .collect();

    println!("TLS audit: {} unique domains", domains.len());

    for domain in &domains {
        let t = Instant::now();
        let (result, check) = audit_single_cert(domain).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let mut c = check;
        c.duration_ms = elapsed;
        checks.push(c);
        results.push(result);
    }

    (checks, results)
}

async fn audit_single_cert(domain: &str) -> (TlsCertResult, Check) {
    // Run openssl s_client to get cert info
    let brief_output = tokio::process::Command::new("openssl")
        .args([
            "s_client",
            "-connect",
            &format!("{}:443", domain),
            "-servername",
            domain,
            "-brief",
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await;

    let (protocol, brief_ok) = match &brief_output {
        Ok(out) => {
            let combined = format!(
                "{}\n{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            let proto = combined
                .lines()
                .find(|l| l.contains("Protocol version:") || l.starts_with("Protocol"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "unknown".into());
            (proto, out.status.success() || combined.contains("Certificate chain"))
        }
        Err(_) => ("error".into(), false),
    };

    // Run openssl to get certificate dates
    let dates_output = tokio::process::Command::new("openssl")
        .args([
            "s_client",
            "-connect",
            &format!("{}:443", domain),
            "-servername",
            domain,
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await;

    let (issuer, not_after, days_remaining, valid) = match dates_output {
        Ok(out) => {
            let cert_pem = String::from_utf8_lossy(&out.stdout);
            parse_cert_details(&cert_pem, domain).await
        }
        Err(_) => ("error".into(), "error".into(), -1i64, false),
    };

    let error = if !valid && !brief_ok {
        Some(format!("TLS handshake failed for {}", domain))
    } else if !valid {
        Some(format!("Certificate validation issue for {}", domain))
    } else {
        None
    };

    let result = TlsCertResult {
        domain: domain.to_string(),
        valid,
        issuer,
        not_after: not_after.clone(),
        days_remaining,
        protocol,
        error: error.clone(),
    };

    let severity = if !valid || days_remaining < 7 {
        Severity::Critical
    } else if days_remaining < 14 {
        Severity::Warning
    } else {
        Severity::Info
    };

    let passed = valid && days_remaining >= 7;
    let details = if valid {
        format!("expires {} ({} days)", not_after, days_remaining)
    } else {
        error.unwrap_or_else(|| "unknown error".into())
    };

    let check = Check {
        name: format!("tls:{}", domain),
        passed,
        details,
        duration_ms: 0,
        error: None,
        severity,
    };

    (result, check)
}

/// Pipe the PEM cert text through `openssl x509` to extract issuer/dates
async fn parse_cert_details(cert_text: &str, _domain: &str) -> (String, String, i64, bool) {
    // Feed the certificate through openssl x509 to get details
    let child = tokio::process::Command::new("openssl")
        .args(["x509", "-noout", "-issuer", "-dates"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(_) => return ("unknown".into(), "unknown".into(), -1, false),
    };

    // Write cert text to stdin
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        let _ = stdin.write_all(cert_text.as_bytes()).await;
        drop(stdin);
    }

    let output = match child.wait_with_output().await {
        Ok(o) => o,
        Err(_) => return ("unknown".into(), "unknown".into(), -1, false),
    };

    if !output.status.success() {
        // If no valid cert was returned (e.g., connection failed), treat as invalid
        return ("none".into(), "none".into(), -1, false);
    }

    let text = String::from_utf8_lossy(&output.stdout);

    let issuer = text
        .lines()
        .find(|l| l.starts_with("issuer=") || l.starts_with("issuer= "))
        .map(|l| {
            l.trim_start_matches("issuer=")
                .trim_start_matches("issuer= ")
                .trim()
                .to_string()
        })
        .unwrap_or_else(|| "unknown".into());

    let not_after = text
        .lines()
        .find(|l| l.starts_with("notAfter="))
        .map(|l| l.trim_start_matches("notAfter=").trim().to_string())
        .unwrap_or_else(|| "unknown".into());

    let days_remaining = parse_openssl_date(&not_after);
    let valid = days_remaining >= 0;

    (issuer, not_after, days_remaining, valid)
}

/// Parse openssl date format "Mon DD HH:MM:SS YYYY GMT" and return days until expiry
fn parse_openssl_date(date_str: &str) -> i64 {
    // openssl dates look like: "Jan 15 12:00:00 2026 GMT"
    let formats = [
        "%b %d %H:%M:%S %Y GMT",
        "%b  %d %H:%M:%S %Y GMT",
        "%b %d %H:%M:%S %Y %Z",
    ];

    let trimmed = date_str.trim();
    for fmt in &formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(trimmed, fmt) {
            let expiry = dt.and_utc();
            let now = chrono::Utc::now();
            return (expiry - now).num_days();
        }
    }

    -1
}
