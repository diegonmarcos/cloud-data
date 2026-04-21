//! Probe private/internal upstreams (services with `upstream` set in consolidated JSON).
//! HTTP probe with bearer auth for HTTP ports; TCP-only `connect` for non-HTTP ports
//! (DNS 53, Redis 6379, IMAP/SMTP, raw TCP services).

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
    pub probe: &'static str, // "http" or "tcp"
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

    // Build vm_id → wg_ip map
    let mut vm_wg: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    if let Some(vms) = c["vms"].as_object() {
        for (id, vm) in vms {
            if let Some(ip) = vm["wg_ip"].as_str() {
                if !ip.is_empty() {
                    vm_wg.insert(id.clone(), ip.to_string());
                }
            }
        }
    }

    let mut out = Vec::new();
    if let Some(services) = c["services"].as_object() {
        for (name, svc) in services {
            let vm_id = svc["vm"].as_str().unwrap_or("").to_string();
            let wg_ip = vm_wg.get(&vm_id).cloned();

            // Container-level probes — one per (service, container) with declared port.
            if let Some(containers) = svc["containers"].as_object() {
                for (cname, ct) in containers {
                    let port = ct["port"].as_u64();
                    let Some(port) = port else { continue };
                    if port == 0 {
                        continue;
                    }
                    let ip = match &wg_ip {
                        Some(ip) => ip.clone(),
                        None => continue,
                    };
                    let target_name = if containers.len() == 1 {
                        name.clone()
                    } else {
                        format!("{}/{}", name, cname)
                    };
                    out.push(PrivateTarget {
                        service: target_name,
                        upstream: format!("{}:{}", ip, port),
                    });
                }
            }

            // Fallback: service-level upstream if no containers produced a probe
            // (covers services whose containers.* have no port but upstream is set elsewhere).
            if !out.iter().any(|t| t.service.starts_with(&format!("{}/", name)) || t.service == *name)
            {
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
    }

    // Also probe every declared public_port per VM (docker host-level bindings).
    // These cover raw TCP ports not tied to a specific service container.
    if let Some(vms) = c["vms"].as_object() {
        for (vm_id, vm) in vms {
            let ip = match vm_wg.get(vm_id) {
                Some(ip) => ip.clone(),
                None => continue,
            };
            let alias = vm["ssh_alias"].as_str().unwrap_or(vm_id);
            if let Some(ports) = vm["public_ports"].as_array() {
                for p in ports {
                    let port = p["port"].as_u64().unwrap_or(0);
                    if port == 0 {
                        continue;
                    }
                    let desc = p["desc"].as_str().unwrap_or("");
                    out.push(PrivateTarget {
                        service: format!("vm:{}/{}", alias, if desc.is_empty() { port.to_string() } else { desc.to_string() }),
                        upstream: format!("{}:{}", ip, port),
                    });
                }
            }
        }
    }

    // Dedupe by (service, upstream)
    out.sort_by(|a, b| a.service.cmp(&b.service).then(a.upstream.cmp(&b.upstream)));
    out.dedup_by(|a, b| a.service == b.service && a.upstream == b.upstream);
    out
}

pub async fn run(
    targets: Vec<PrivateTarget>,
    bearer: Option<&str>,
    parallel: usize,
    timeouts: &Timeouts,
    tcp_only_ports: &[u16],
) -> Vec<PrivateResult> {
    let client = match reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(timeouts.http_connect_secs))
        .timeout(Duration::from_secs(timeouts.http_total_secs))
        .danger_accept_invalid_certs(true)
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
                    probe: "http",
                    error: Some(format!("client build failed: {}", e)),
                })
                .collect();
        }
    };
    let bearer = bearer.map(|s| s.to_string());
    let tcp_secs = timeouts.tcp_secs;
    let tcp_set: std::collections::HashSet<u16> = tcp_only_ports.iter().copied().collect();

    stream::iter(targets)
        .map(|t| {
            let client = client.clone();
            let bearer = bearer.clone();
            let tcp_set = tcp_set.clone();
            async move {
                let port = parse_port(&t.upstream);
                if let Some(p) = port {
                    if tcp_set.contains(&p) {
                        return tcp_probe(&t, tcp_secs).await;
                    }
                }
                http_probe(&client, t, bearer.as_deref()).await
            }
        })
        .buffer_unordered(parallel)
        .collect::<Vec<_>>()
        .await
}

fn parse_port(upstream: &str) -> Option<u16> {
    upstream.rsplit(':').next()?.trim_end_matches('/').parse().ok()
}

fn parse_host(upstream: &str) -> &str {
    let stripped = upstream
        .strip_prefix("http://")
        .or_else(|| upstream.strip_prefix("https://"))
        .unwrap_or(upstream);
    stripped.split(':').next().unwrap_or(stripped)
}

fn normalize_url(upstream: &str) -> String {
    if upstream.starts_with("http://") || upstream.starts_with("https://") {
        upstream.to_string()
    } else {
        format!("http://{}", upstream)
    }
}

async fn tcp_probe(t: &PrivateTarget, timeout_secs: u64) -> PrivateResult {
    let host = parse_host(&t.upstream).to_string();
    let port = parse_port(&t.upstream).unwrap_or(0);
    let start = Instant::now();
    let res = tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        tokio::net::TcpStream::connect((host.as_str(), port)),
    )
    .await;
    match res {
        Ok(Ok(_)) => PrivateResult {
            service: t.service.clone(),
            upstream: t.upstream.clone(),
            status: None,
            latency_ms: start.elapsed().as_millis() as u64,
            ok: true,
            probe: "tcp",
            error: None,
        },
        Ok(Err(e)) => PrivateResult {
            service: t.service.clone(),
            upstream: t.upstream.clone(),
            status: None,
            latency_ms: start.elapsed().as_millis() as u64,
            ok: false,
            probe: "tcp",
            error: Some(e.to_string()),
        },
        Err(_) => PrivateResult {
            service: t.service.clone(),
            upstream: t.upstream.clone(),
            status: None,
            latency_ms: start.elapsed().as_millis() as u64,
            ok: false,
            probe: "tcp",
            error: Some(format!("tcp connect timeout {}s", timeout_secs)),
        },
    }
}

async fn http_probe(
    client: &reqwest::Client,
    t: PrivateTarget,
    bearer: Option<&str>,
) -> PrivateResult {
    let url = normalize_url(&t.upstream);
    let mut req = client.get(&url);
    if let Some(b) = bearer {
        req = req.bearer_auth(b);
    }
    let start = Instant::now();
    match req.send().await {
        Ok(r) => {
            let status = r.status().as_u16();
            // 404 = service up, no root handler. 405 = wrong method but service responding.
            // 401/403 = auth-gated and reachable. All count as "up".
            let ok = r.status().is_success()
                || r.status().is_redirection()
                || matches!(status, 401 | 403 | 404 | 405);
            PrivateResult {
                service: t.service,
                upstream: t.upstream,
                status: Some(status),
                latency_ms: start.elapsed().as_millis() as u64,
                ok,
                probe: "http",
                error: None,
            }
        }
        Err(e) => PrivateResult {
            service: t.service,
            upstream: t.upstream,
            status: None,
            latency_ms: start.elapsed().as_millis() as u64,
            ok: false,
            probe: "http",
            error: Some(e.to_string()),
        },
    }
}
