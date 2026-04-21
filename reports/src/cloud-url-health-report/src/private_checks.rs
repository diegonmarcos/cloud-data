//! Probe private/internal upstreams. Protocol choice (HTTP vs TCP vs Skip)
//! is data-driven — reads `.protocol` + `.public` from each container spec in
//! the consolidated JSON, falls back to a port heuristic for the 23/83
//! containers that omit `.protocol`.

use crate::config::Timeouts;
use futures::stream::{self, StreamExt};
use reports_common::context::find_cloud_data_file;
use reports_common::probe::{self, Protocol};
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Serialize, Clone)]
pub struct PrivateTarget {
    pub service: String,
    pub upstream: String,
    /// Protocol resolved from cloud-data spec at load time.
    #[serde(skip)]
    pub protocol: Protocol,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrivateResult {
    pub service: String,
    pub upstream: String,
    pub status: Option<u16>,
    pub latency_ms: u64,
    pub ok: bool,
    pub probe: &'static str,
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

    let mut vm_wg: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut vm_alias: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    if let Some(vms) = c["vms"].as_object() {
        for (id, vm) in vms {
            if let Some(ip) = vm["wg_ip"].as_str() {
                if !ip.is_empty() {
                    vm_wg.insert(id.clone(), ip.to_string());
                }
            }
            if let Some(a) = vm["ssh_alias"].as_str() {
                vm_alias.insert(id.clone(), a.to_string());
            }
        }
    }

    let mut out = Vec::new();
    if let Some(services) = c["services"].as_object() {
        for (name, svc) in services {
            let vm_id = svc["vm"].as_str().unwrap_or("").to_string();
            let wg_ip = match vm_wg.get(&vm_id) {
                Some(ip) => ip.clone(),
                None => continue,
            };

            if let Some(containers) = svc["containers"].as_object() {
                for (cname, ct) in containers {
                    let port = match ct["port"].as_u64() {
                        Some(p) if p > 0 => p as u16,
                        _ => continue,
                    };
                    let proto = probe::protocol_for_container(ct, port);
                    if proto == Protocol::Skip {
                        continue; // public:false — docker bridge only
                    }
                    let target_name = if containers.len() == 1 {
                        name.clone()
                    } else {
                        format!("{}/{}", name, cname)
                    };
                    out.push(PrivateTarget {
                        service: target_name,
                        upstream: format!("{}:{}", wg_ip, port),
                        protocol: proto,
                    });
                }
            }
        }
    }

    // VM-level public_ports — read proto hint from each entry.
    if let Some(vms) = c["vms"].as_object() {
        for (vm_id, vm) in vms {
            let ip = match vm_wg.get(vm_id) {
                Some(ip) => ip.clone(),
                None => continue,
            };
            let alias = vm_alias
                .get(vm_id)
                .cloned()
                .unwrap_or_else(|| vm_id.clone());
            if let Some(ports) = vm["public_ports"].as_array() {
                for p in ports {
                    let port = p["port"].as_u64().unwrap_or(0);
                    if port == 0 {
                        continue;
                    }
                    let port = port as u16;
                    let desc = p["desc"].as_str().unwrap_or("");
                    let proto_hint = p["proto"].as_str();
                    let proto = probe::protocol_for_public_port(port, proto_hint);
                    if proto == Protocol::Skip {
                        continue;
                    }
                    out.push(PrivateTarget {
                        service: format!(
                            "vm:{}/{}",
                            alias,
                            if desc.is_empty() { port.to_string() } else { desc.to_string() }
                        ),
                        upstream: format!("{}:{}", ip, port),
                        protocol: proto,
                    });
                }
            }
        }
    }

    out.sort_by(|a, b| a.service.cmp(&b.service).then(a.upstream.cmp(&b.upstream)));
    out.dedup_by(|a, b| a.service == b.service && a.upstream == b.upstream);
    out
}

pub async fn run(
    targets: Vec<PrivateTarget>,
    bearer: Option<&str>,
    parallel: usize,
    timeouts: &Timeouts,
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
    let tcp_timeout = Duration::from_secs(timeouts.tcp_secs);

    stream::iter(targets)
        .map(|t| {
            let client = client.clone();
            let bearer = bearer.clone();
            async move {
                let (ip, port) = split_upstream(&t.upstream);
                let res = probe::probe_endpoint(
                    &ip,
                    port,
                    t.protocol,
                    bearer.as_deref(),
                    &client,
                    tcp_timeout,
                )
                .await;
                PrivateResult {
                    service: t.service,
                    upstream: t.upstream,
                    status: res.status,
                    latency_ms: res.latency_ms,
                    ok: res.ok,
                    probe: res.probe,
                    error: res.error,
                }
            }
        })
        .buffer_unordered(parallel)
        .collect::<Vec<_>>()
        .await
}

fn split_upstream(s: &str) -> (String, u16) {
    let stripped = s
        .strip_prefix("http://")
        .or_else(|| s.strip_prefix("https://"))
        .unwrap_or(s);
    let mut parts = stripped.rsplitn(2, ':');
    let port_s = parts.next().unwrap_or("0");
    let host = parts.next().unwrap_or(stripped);
    (
        host.to_string(),
        port_s.trim_end_matches('/').parse().unwrap_or(0),
    )
}
