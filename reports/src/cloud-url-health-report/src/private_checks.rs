//! Probe private/internal upstreams (services with `upstream` set in consolidated JSON).

use crate::config::Timeouts;
use futures::stream::{self, StreamExt};
use reports_common::context::find_cloud_data_file;
use serde::Serialize;
use serde_json::Value;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Clone)]
pub struct PrivateTarget {
    pub service: String,
    pub upstream: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrivateResult {
    pub service: String,
    pub upstream: String,
    pub status: Option<u16>,
    pub latency_ms: u64,
    pub ok: bool,
    pub error: Option<String>,
}

pub fn load_private_targets() -> Vec<PrivateTarget> {
    let path = match find_cloud_data_file("_cloud-data-consolidated.json") {
        Some(p) => p,
        None => return Vec::new(),
    };
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let c: Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    if let Some(services) = c["services"].as_object() {
        for (name, svc) in services {
            if let Some(up) = svc["upstream"].as_str() {
                if !up.is_empty() {
                    out.push(PrivateTarget {
                        service: name.clone(),
                        upstream: up.to_string(),
                    });
                }
            }
        }
    }
    out.sort_by(|a, b| a.service.cmp(&b.service));
    out
}

pub async fn run(
    targets: Vec<PrivateTarget>,
    parallel: usize,
    timeouts: &Timeouts,
) -> Vec<PrivateResult> {
    let client = match reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(timeouts.http_connect_secs))
        .timeout(Duration::from_secs(timeouts.http_total_secs))
        .danger_accept_invalid_certs(true) // internal upstreams often self-signed
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return targets
                .into_iter()
                .map(|t| PrivateResult {
                    service: t.service,
                    upstream: t.upstream,
                    status: None,
                    latency_ms: 0,
                    ok: false,
                    error: Some(format!("client build failed: {}", e)),
                })
                .collect();
        }
    };

    stream::iter(targets)
        .map(|t| {
            let client = client.clone();
            async move { probe_one(&client, t).await }
        })
        .buffer_unordered(parallel)
        .collect::<Vec<_>>()
        .await
}

fn normalize_url(upstream: &str) -> String {
    if upstream.starts_with("http://") || upstream.starts_with("https://") {
        upstream.to_string()
    } else {
        format!("http://{}", upstream)
    }
}

async fn probe_one(client: &reqwest::Client, t: PrivateTarget) -> PrivateResult {
    let url = normalize_url(&t.upstream);
    let start = Instant::now();
    match client.get(&url).send().await {
        Ok(r) => {
            let status = r.status().as_u16();
            // 401/403 from an auth-gated service still proves it's up.
            let ok = r.status().is_success()
                || r.status().is_redirection()
                || status == 401
                || status == 403;
            PrivateResult {
                service: t.service,
                upstream: t.upstream,
                status: Some(status),
                latency_ms: start.elapsed().as_millis() as u64,
                ok,
                error: None,
            }
        }
        Err(e) => PrivateResult {
            service: t.service,
            upstream: t.upstream,
            status: None,
            latency_ms: start.elapsed().as_millis() as u64,
            ok: false,
            error: Some(e.to_string()),
        },
    }
}
