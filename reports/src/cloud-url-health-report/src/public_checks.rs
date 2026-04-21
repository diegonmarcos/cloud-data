//! Probe public URLs (caddy-routed) with bearer token. Expect 200 OK.

use crate::config::Timeouts;
use futures::stream::{self, StreamExt};
use reports_common::context::find_cloud_data_file;
use serde::Serialize;
use serde_json::Value;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Clone)]
pub struct PublicResult {
    pub domain: String,
    pub status: Option<u16>,
    pub latency_ms: u64,
    pub ok: bool,
    pub error: Option<String>,
}

pub fn load_caddy_domains() -> Vec<String> {
    let path = match find_cloud_data_file("build-caddy.json") {
        Some(p) => p,
        None => return Vec::new(),
    };
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let json: Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<String> = json["routes"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|r| r["domain"].as_str().map(|s| s.to_string()))
                .filter(|d| !d.is_empty() && d.contains('.'))
                .collect()
        })
        .unwrap_or_default();
    if !out.iter().any(|d| d == "diegonmarcos.com") {
        out.push("diegonmarcos.com".to_string());
    }
    out.sort();
    out.dedup();
    out
}

pub async fn run(
    domains: Vec<String>,
    bearer: Option<&str>,
    parallel: usize,
    timeouts: &Timeouts,
) -> Vec<PublicResult> {
    let client = match build_client(timeouts) {
        Ok(c) => c,
        Err(e) => {
            return domains
                .into_iter()
                .map(|d| PublicResult {
                    domain: d,
                    status: None,
                    latency_ms: 0,
                    ok: false,
                    error: Some(format!("client build failed: {}", e)),
                })
                .collect();
        }
    };
    let bearer = bearer.map(|s| s.to_string());

    stream::iter(domains)
        .map(|d| {
            let client = client.clone();
            let bearer = bearer.clone();
            async move { probe_one(&client, d, bearer.as_deref()).await }
        })
        .buffer_unordered(parallel)
        .collect::<Vec<_>>()
        .await
}

fn build_client(t: &Timeouts) -> anyhow::Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(t.http_connect_secs))
        .timeout(Duration::from_secs(t.http_total_secs))
        .danger_accept_invalid_certs(false)
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()?;
    Ok(client)
}

async fn probe_one(client: &reqwest::Client, domain: String, bearer: Option<&str>) -> PublicResult {
    let url = format!("https://{}/", domain);
    let mut req = client.get(&url);
    if let Some(b) = bearer {
        req = req.bearer_auth(b);
    }
    let t = Instant::now();
    match req.send().await {
        Ok(r) => {
            let status = r.status().as_u16();
            // 404/405 = service responded (just not on root / wrong method).
            // 401/403 = auth-gated and reachable. All count as "up".
            let ok = r.status().is_success()
                || r.status().is_redirection()
                || matches!(status, 401 | 403 | 404 | 405);
            PublicResult {
                domain,
                status: Some(status),
                latency_ms: t.elapsed().as_millis() as u64,
                ok,
                error: None,
            }
        }
        Err(e) => PublicResult {
            domain,
            status: None,
            latency_ms: t.elapsed().as_millis() as u64,
            ok: false,
            error: Some(e.to_string()),
        },
    }
}
