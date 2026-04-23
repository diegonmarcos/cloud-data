//! Probe public URLs (caddy-routed) with bearer token.
//! Source: `build-caddy.json` — merges `.routes[]`, `public_A_mcp`,
//! `public_B_apis`, `public_C_app_paths`, `public_D_others`.
//! No hard-coded domains.

use crate::config::Timeouts;
use futures::stream::{self, StreamExt};
use reports_common::caddy::{self, CaddyTarget};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Clone)]
pub struct PublicResult {
    pub domain: String,
    pub path: Option<String>,
    pub url: String,
    pub upstream: String,
    pub category: String,
    pub status: Option<u16>,
    pub latency_ms: u64,
    pub ok: bool,
    pub error: Option<String>,
}

/// Load public targets from build-caddy.json. Declarative — no hard-coded URLs.
pub fn load_public_targets() -> Vec<CaddyTarget> {
    caddy::load_public_targets()
}

pub async fn run(
    targets: Vec<CaddyTarget>,
    bearer: Option<&str>,
    parallel: usize,
    timeouts: &Timeouts,
) -> Vec<PublicResult> {
    let client = match build_client(timeouts) {
        Ok(c) => c,
        Err(e) => {
            return targets
                .into_iter()
                .map(|t| PublicResult {
                    domain: t.host.clone(),
                    path: t.path.clone(),
                    url: t.url.clone(),
                    upstream: t.upstream.clone(),
                    category: t.category.clone(),
                    status: None,
                    latency_ms: 0,
                    ok: false,
                    error: Some(format!("client build failed: {}", e)),
                })
                .collect();
        }
    };
    let bearer = bearer.map(|s| s.to_string());

    stream::iter(targets)
        .map(|t| {
            let client = client.clone();
            let bearer = bearer.clone();
            async move { probe_one(&client, t, bearer.as_deref()).await }
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

async fn probe_one(
    client: &reqwest::Client,
    target: CaddyTarget,
    bearer: Option<&str>,
) -> PublicResult {
    let mut req = client.get(&target.url);
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
                domain: target.host,
                path: target.path,
                url: target.url,
                upstream: target.upstream,
                category: target.category,
                status: Some(status),
                latency_ms: t.elapsed().as_millis() as u64,
                ok,
                error: None,
            }
        }
        Err(e) => PublicResult {
            domain: target.host,
            path: target.path,
            url: target.url,
            upstream: target.upstream,
            category: target.category,
            status: None,
            latency_ms: t.elapsed().as_millis() as u64,
            ok: false,
            error: Some(e.to_string()),
        },
    }
}
