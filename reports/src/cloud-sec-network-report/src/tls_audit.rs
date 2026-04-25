use crate::context::NetworkContext;
use crate::types::TlsCertResult;
use futures::stream::{self, StreamExt};
use reports_common::capabilities::RuntimeCapabilities;
use reports_common::types::{Check, Severity};
use std::collections::HashSet;
use std::time::Instant;

/// Concurrency cap for openssl s_client subprocesses. openssl is CPU-light
/// but each spawn forks a process; capping at 16 keeps fork-rate sane while
/// landing 50-domain audits in seconds rather than minutes.
const TLS_AUDIT_PARALLEL: usize = 16;

/// Dual-path TLS audit: external (public domain:443) + internal (WG upstream:port) when available
pub async fn audit_all_certs(
    ctx: &NetworkContext,
    caps: &RuntimeCapabilities,
) -> (Vec<Check>, Vec<TlsCertResult>) {
    let mut checks = Vec::new();
    let mut results = Vec::new();

    // Deduplicate domains
    let mut seen = HashSet::new();
    let routes: Vec<_> = ctx
        .caddy_routes
        .iter()
        .filter(|r| !r.domain.is_empty())
        .filter(|r| seen.insert(r.domain.clone()))
        .cloned()
        .collect();

    println!(
        "TLS audit: {} domains (external{}) ",
        routes.len(),
        if caps.wg_up { " + internal" } else { "" }
    );

    // ── External: public domain:443 (always). Parallel audit, ≤ TLS_AUDIT_PARALLEL.
    let ext_results: Vec<(TlsCertResult, Check)> = stream::iter(routes.iter().cloned())
        .map(|route| async move {
            let t = Instant::now();
            let (result, check) = audit_single_cert(&route.domain, &format!("{}:443", route.domain), "ext").await;
            let mut c = check;
            c.duration_ms = t.elapsed().as_millis() as u64;
            (result, c)
        })
        .buffer_unordered(TLS_AUDIT_PARALLEL)
        .collect()
        .await;
    for (result, check) in ext_results {
        checks.push(check);
        results.push(result);
    }

    // ── Internal: WG upstream (when WG is up). Parallel audit, ≤ TLS_AUDIT_PARALLEL.
    if caps.wg_up {
        let internal_routes: Vec<_> = routes.iter().filter(|r| !r.upstream.is_empty()).cloned().collect();
        let int_results: Vec<(TlsCertResult, Check)> = stream::iter(internal_routes.into_iter())
            .map(|route| async move {
                let t = Instant::now();
                let (result, check) = audit_single_cert_internal(&route.domain, &route.upstream).await;
                let mut c = check;
                c.duration_ms = t.elapsed().as_millis() as u64;
                (result, c)
            })
            .buffer_unordered(TLS_AUDIT_PARALLEL)
            .collect()
            .await;
        for (result, check) in int_results {
            checks.push(check);
            results.push(result);
        }
    }

    (checks, results)
}

/// Audit TLS cert via public internet — deadline-bound at 8s per domain.
async fn audit_single_cert(domain: &str, connect: &str, prefix: &str) -> (TlsCertResult, Check) {
    use std::time::Duration;
    use tokio::time::timeout;
    // openssl s_client against a non-responsive peer can hang indefinitely
    // (system default ~120s, sometimes never). Bound it explicitly so one
    // bad domain can't block the sequential cert-audit loop.
    let cmd_fut = tokio::process::Command::new("openssl")
        .args([
            "s_client",
            "-connect", connect,
            "-servername", domain,
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();
    let output = match timeout(Duration::from_secs(8), cmd_fut).await {
        Ok(inner) => inner,
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "openssl s_client exceeded 8s deadline",
        )),
    };

    let cert_text = match &output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => String::new(),
    };

    let brief_ok = output.as_ref().map(|o| {
        let combined = format!("{}\n{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
        o.status.success() || combined.contains("Certificate chain")
    }).unwrap_or(false);

    let (issuer, not_after, days_remaining, valid, protocol) = if brief_ok {
        let details = parse_cert_details(&cert_text).await;
        let proto = extract_protocol(&cert_text, &output.as_ref().map(|o| String::from_utf8_lossy(&o.stderr).to_string()).unwrap_or_default());
        (details.0, details.1, details.2, details.3, proto)
    } else {
        ("none".into(), "none".into(), -1i64, false, "error".into())
    };

    let error = if !valid {
        Some(format!("TLS handshake failed for {}", domain))
    } else {
        None
    };

    let result = TlsCertResult {
        domain: domain.to_string(),
        valid,
        issuer: issuer.clone(),
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
        format!("{} expires {} ({} days)", connect, not_after, days_remaining)
    } else {
        error.unwrap_or_else(|| "unknown error".into())
    };

    let check = Check {
        name: format!("{}:tls:{}", prefix, domain),
        passed,
        details,
        duration_ms: 0,
        error: None,
        severity,
    };

    (result, check)
}

/// Audit TLS on the internal upstream (WG path) — may be plain HTTP or TLS.
/// Deadline-bound at 6s per upstream.
async fn audit_single_cert_internal(domain: &str, upstream: &str) -> (TlsCertResult, Check) {
    use std::time::Duration;
    use tokio::time::timeout;
    let cmd_fut = tokio::process::Command::new("openssl")
        .args([
            "s_client",
            "-connect", upstream,
            "-servername", domain,
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();
    let output = match timeout(Duration::from_secs(6), cmd_fut).await {
        Ok(inner) => inner,
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "openssl s_client (internal) exceeded 6s deadline",
        )),
    };

    let is_tls = output.as_ref().map(|o| {
        let combined = format!("{}\n{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
        combined.contains("Certificate chain") || combined.contains("BEGIN CERTIFICATE")
    }).unwrap_or(false);

    if is_tls {
        let cert_text = output.as_ref().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default();
        let (issuer, not_after, days_remaining, valid) = parse_cert_details(&cert_text).await;

        let result = TlsCertResult {
            domain: domain.to_string(),
            valid,
            issuer,
            not_after: not_after.clone(),
            days_remaining,
            protocol: "internal-tls".into(),
            error: None,
        };

        let check = Check {
            name: format!("int:tls:{}", domain),
            passed: valid,
            details: if valid {
                format!("WG {} — TLS OK, expires {} ({} days)", upstream, not_after, days_remaining)
            } else {
                format!("WG {} — TLS cert invalid", upstream)
            },
            duration_ms: 0,
            error: None,
            severity: if valid { Severity::Info } else { Severity::Warning },
        };

        (result, check)
    } else {
        // Upstream is plain HTTP (expected for most services behind Caddy)
        let result = TlsCertResult {
            domain: domain.to_string(),
            valid: true,
            issuer: "n/a (plain HTTP upstream)".into(),
            not_after: "n/a".into(),
            days_remaining: 999,
            protocol: "http".into(),
            error: None,
        };

        let check = Check {
            name: format!("int:tls:{}", domain),
            passed: true,
            details: format!("WG {} — plain HTTP (Caddy terminates TLS)", upstream),
            duration_ms: 0,
            error: None,
            severity: Severity::Info,
        };

        (result, check)
    }
}

/// Extract protocol version from openssl output
fn extract_protocol(stdout: &str, stderr: &str) -> String {
    let combined = format!("{}\n{}", stdout, stderr);
    combined
        .lines()
        .find(|l| l.contains("Protocol") && l.contains("TLS"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

/// Pipe cert PEM through `openssl x509` to extract issuer/dates
async fn parse_cert_details(cert_text: &str) -> (String, String, i64, bool) {
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
        return ("none".into(), "none".into(), -1, false);
    }

    let text = String::from_utf8_lossy(&output.stdout);

    let issuer = text
        .lines()
        .find(|l| l.starts_with("issuer=") || l.starts_with("issuer= "))
        .map(|l| l.trim_start_matches("issuer=").trim_start_matches("issuer= ").trim().to_string())
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

/// Parse openssl date format and return days until expiry
fn parse_openssl_date(date_str: &str) -> i64 {
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
