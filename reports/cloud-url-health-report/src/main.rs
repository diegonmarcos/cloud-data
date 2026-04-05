//! Cloud URL Health Report — 4-Layer Probe (Public + Private)
//!
//! Public URLs (from caddy routes):
//!   L1 DNS    — resolve domain via Cloudflare + system resolvers
//!   L2 TCP    — connect to resolved IP:443
//!   L3 TLS    — TLS handshake (implicit in HTTPS)
//!   L4 HTTP   — GET with OIDC bearer token auto-injection
//!
//! Private URLs (from consolidated services with upstream):
//!   L1 TCP    — connect to WireGuard IP:port
//!   L2 HTTP   — GET http://wg_ip:port (no TLS, no auth)
//!
//! Data sources:
//!   ../../cloud-data-caddy-routes.json     (public domains)
//!   ../../_cloud-data-consolidated.json    (private upstreams)
//!   ~/git/vault/.../cloud-admin.json       (bearer token)
//!
//! Output: cloud_url_health.json + cloud_url_health.md

mod probes;
mod template;

use anyhow::Result;
use chrono::Utc;
use futures::future::join_all;
use serde::Serialize;
use std::time::Instant;

use probes::{probe_private, probe_public, ProbeResult, ProbeStatus};

// ────────────────────────────────────────────────────────────────
// Types
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct Report {
    generated: String,
    duration_ms: u64,
    summary: Summary,
    public: Vec<ProbeResult>,
    private: Vec<ProbeResult>,
}

#[derive(Debug, Serialize)]
struct Summary {
    public_total: usize,
    public_healthy: usize,
    private_total: usize,
    private_healthy: usize,
}

#[derive(Debug, serde::Deserialize)]
struct CaddyRoutes {
    routes: Option<Vec<CaddyRoute>>,
}

#[derive(Debug, serde::Deserialize)]
struct CaddyRoute {
    domain: String,
    #[allow(dead_code)]
    #[serde(default)]
    comment: String,
}

#[derive(Debug)]
struct PrivateTarget {
    name: String,
    upstream: String,
    vm: String,
}

// ────────────────────────────────────────────────────────────────
// Main
// ────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("=== Cloud URL Health Report (4-Layer Probe) ===\n");

    let (domains, private_targets) = load_targets()?;
    let token = load_bearer_token();

    println!(
        "Targets: {} public, {} private | Bearer: {}",
        domains.len(),
        private_targets.len(),
        if token.is_some() { "loaded" } else { "missing" },
    );

    // Probe all concurrently
    let (public_results, private_results) = tokio::join!(
        probe_all_public(&domains, token.as_deref()),
        probe_all_private(&private_targets),
    );

    let duration_ms = start.elapsed().as_millis() as u64;

    // Print results
    print_table("PUBLIC URLS", &public_results, true);
    print_table("PRIVATE URLS", &private_results, false);

    let pub_healthy = public_results.iter().filter(|r| r.is_healthy()).count();
    let priv_healthy = private_results.iter().filter(|r| r.is_healthy()).count();

    let report = Report {
        generated: Utc::now().to_rfc3339(),
        duration_ms,
        summary: Summary {
            public_total: public_results.len(),
            public_healthy: pub_healthy,
            private_total: private_results.len(),
            private_healthy: priv_healthy,
        },
        public: public_results,
        private: private_results,
    };

    println!(
        "\n=== Done in {:.1}s | Public: {}/{} | Private: {}/{} ===",
        duration_ms as f64 / 1000.0,
        report.summary.public_healthy,
        report.summary.public_total,
        report.summary.private_healthy,
        report.summary.private_total,
    );

    // Write outputs
    std::fs::write(
        "cloud_url_health.json",
        serde_json::to_string_pretty(&report)?,
    )?;
    template::render(&report)?;

    println!("-> cloud_url_health.json + cloud_url_health.md");
    Ok(())
}

// ────────────────────────────────────────────────────────────────
// Target loading
// ────────────────────────────────────────────────────────────────

fn load_targets() -> Result<(Vec<String>, Vec<PrivateTarget>)> {
    // Public: from caddy routes
    let caddy_raw = std::fs::read_to_string("../../cloud-data-caddy-routes.json")?;
    let caddy: CaddyRoutes = serde_json::from_str(&caddy_raw)?;
    let mut domains: Vec<String> = caddy
        .routes
        .unwrap_or_default()
        .into_iter()
        .map(|r| r.domain)
        .filter(|d| !d.is_empty() && d.contains('.'))
        .collect();
    if !domains.iter().any(|d| d == "diegonmarcos.com") {
        domains.push("diegonmarcos.com".to_string());
    }
    domains.sort();
    domains.dedup();

    // Private: from consolidated services with upstream
    let cons_raw = std::fs::read_to_string("../../_cloud-data-consolidated.json")?;
    let cons: serde_json::Value = serde_json::from_str(&cons_raw)?;
    let mut privates: Vec<PrivateTarget> = Vec::new();

    if let Some(services) = cons["services"].as_object() {
        for (name, svc) in services {
            if let Some(upstream) = svc["upstream"].as_str() {
                if !upstream.is_empty() {
                    privates.push(PrivateTarget {
                        name: name.clone(),
                        upstream: upstream.to_string(),
                        vm: svc["vm"].as_str().unwrap_or("?").to_string(),
                    });
                }
            }
        }
    }
    privates.sort_by(|a, b| a.name.cmp(&b.name));

    Ok((domains, privates))
}

fn load_bearer_token() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let path = format!(
        "{}/git/vault/A0_keys/providers/authelia/signed-bearer_jwt/tokens/cloud-admin.json",
        home
    );
    let raw = std::fs::read_to_string(path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&raw).ok()?;
    v["access_token"].as_str().map(|s| s.to_string())
}

// ────────────────────────────────────────────────────────────────
// Concurrent probing
// ────────────────────────────────────────────────────────────────

async fn probe_all_public(domains: &[String], bearer: Option<&str>) -> Vec<ProbeResult> {
    let futs: Vec<_> = domains.iter().map(|d| probe_public(d, bearer)).collect();
    join_all(futs).await
}

async fn probe_all_private(targets: &[PrivateTarget]) -> Vec<ProbeResult> {
    let futs: Vec<_> = targets
        .iter()
        .map(|t| probe_private(&t.name, &t.upstream, &t.vm))
        .collect();
    join_all(futs).await
}

// ────────────────────────────────────────────────────────────────
// Display
// ────────────────────────────────────────────────────────────────

fn print_table(title: &str, results: &[ProbeResult], show_dns: bool) {
    println!("\n  {}", title);
    if show_dns {
        println!(
            "  {:<40} {:>4} {:>4} {:>4} {:>5} {:>5}  {}",
            "DOMAIN", "DNS", "TCP", "TLS", "HTTP", "ms", "DETAIL"
        );
    } else {
        println!(
            "  {:<30} {:<22} {:>4} {:>5} {:>5}  {}",
            "SERVICE", "UPSTREAM", "TCP", "HTTP", "ms", "DETAIL"
        );
    }
    println!("  {}", "-".repeat(95));

    for r in results {
        let detail = if r.errors.is_empty() {
            r.http_code.map(|c| c.to_string()).unwrap_or_default()
        } else {
            r.errors.last().cloned().unwrap_or_default()
        };

        if show_dns {
            println!(
                "  {:<40} {:>4} {:>4} {:>4} {:>5} {:>3}ms  {}",
                r.domain,
                icon(&r.dns),
                icon(&r.tcp),
                icon(&r.tls),
                icon(&r.http),
                r.latency_ms,
                truncate(&detail, 35),
            );
        } else {
            println!(
                "  {:<30} {:<22} {:>4} {:>5} {:>3}ms  {}",
                r.domain,
                r.url,
                icon(&r.tcp),
                icon(&r.http),
                r.latency_ms,
                truncate(&detail, 35),
            );
        }
    }
}

fn icon(s: &ProbeStatus) -> &'static str {
    match s {
        ProbeStatus::Ok => "OK",
        ProbeStatus::Fail => "FAIL",
        ProbeStatus::Skip => "--",
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
