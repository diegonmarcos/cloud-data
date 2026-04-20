use crate::types::*;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

const CONSOLIDATED: &str = "../../_cloud-data-consolidated.json";
const TOPOLOGY: &str = "../../cloud-data-topology.json";
const CADDY_ROUTES: &str = "../../build-proxy-caddy-routes.json";
const DNS_SERVICES: &str = "../../cloud-data-dns-services.json";
const BUILD_JSON_GLOB: &str = "/home/diego/Mounts/Git/cloud/a_solutions";
const BEARER_TOKEN_PATH: &str =
    "Mounts/Git/vault/A0_keys/providers/authelia/signed-bearer_jwt/tokens/cloud-admin.json";

/// Load the consolidated cloud-data JSON
pub fn load_consolidated() -> Result<Value> {
    let raw = std::fs::read_to_string(CONSOLIDATED)?;
    Ok(serde_json::from_str(&raw)?)
}

/// Load topology JSON (optional)
pub fn load_topology() -> Option<Value> {
    std::fs::read_to_string(TOPOLOGY)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

/// Load caddy routes JSON (optional)
pub fn load_caddy_routes() -> Option<Value> {
    std::fs::read_to_string(CADDY_ROUTES)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

/// Load DNS services JSON (optional)
pub fn load_dns_services() -> Option<Value> {
    std::fs::read_to_string(DNS_SERVICES)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

/// Load bearer token from vault
pub fn load_bearer_token() -> Option<String> {
    // Try env var first (GHA), then file (local)
    if let Ok(token) = std::env::var("BEARER_TOKEN") {
        if !token.is_empty() {
            return Some(token);
        }
    }
    let home = std::env::var("HOME").unwrap_or("/home/diego".into());
    std::fs::read_to_string(format!("{}/{}", home, BEARER_TOKEN_PATH))
        .ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .and_then(|v| v["access_token"].as_str().map(|s| s.to_string()))
}

/// Parse VMs from consolidated JSON
pub fn parse_vms(consolidated: &Value) -> Vec<VmInfo> {
    let mut vms = Vec::new();
    let empty_map = serde_json::Map::new();
    let vm_map = consolidated["vms"].as_object().unwrap_or(&empty_map);

    for (id, vm) in vm_map {
        let wg_ip = vm["wg_ip"].as_str().unwrap_or("").to_string();
        if wg_ip.is_empty() || wg_ip == "?" {
            continue;
        }

        let provider = if id.starts_with("oci-") {
            "OCI"
        } else if id.starts_with("gcp-") {
            "GCP"
        } else {
            "?"
        };
        let cost = if id.contains("-f_") || id.contains("-f-") {
            "Free"
        } else if id.contains("-p_") {
            "Spot"
        } else {
            "?"
        };

        let declared_services: Vec<String> = vm["services"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let public_ports: Vec<PublicPort> = vm["public_ports"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|p| PublicPort {
                        port: p["port"].as_u64().unwrap_or(0) as u16,
                        proto: p["proto"].as_str().unwrap_or("tcp").to_string(),
                        desc: p["desc"].as_str().unwrap_or("").to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        vms.push(VmInfo {
            vm_id: id.clone(),
            alias: vm["ssh_alias"].as_str().unwrap_or(id).to_string(),
            pub_ip: vm["ip"].as_str().unwrap_or("?").to_string(),
            wg_ip,
            cloud_name: vm["specs"]["cloud_name"]
                .as_str()
                .unwrap_or("?")
                .to_string(),
            cloud_zone: vm["specs"]["cloud_zone"]
                .as_str()
                .unwrap_or("?")
                .to_string(),
            rescue_port: vm["rescue_port"].as_u64().unwrap_or(2200) as u16,
            cpus: vm["specs"]["cpu"].as_u64().unwrap_or(0) as u32,
            ram_gb: vm["specs"]["ram_gb"].as_f64().unwrap_or(0.0),
            shape: vm["specs"]["shape"]
                .as_str()
                .or(vm["specs"]["machine_type"].as_str())
                .unwrap_or("?")
                .to_string(),
            provider: provider.to_string(),
            cost: cost.to_string(),
            declared_services,
            public_ports,
        });
    }

    vms
}

/// Parse services from consolidated JSON
pub fn parse_services(consolidated: &Value) -> Vec<ServiceInfo> {
    let empty_map = serde_json::Map::new();
    let svc_map = consolidated["services"].as_object().unwrap_or(&empty_map);

    // Build VM alias map
    let mut vm_alias_map: HashMap<String, String> = HashMap::new();
    if let Some(vms) = consolidated["vms"].as_object() {
        for (id, vm) in vms {
            let alias = vm["ssh_alias"].as_str().unwrap_or(id).to_string();
            vm_alias_map.insert(id.clone(), alias);
        }
    }

    svc_map
        .iter()
        .map(|(name, svc)| {
            let vm_id = svc["vm"].as_str().unwrap_or("").to_string();
            let vm_alias = vm_alias_map
                .get(&vm_id)
                .cloned()
                .unwrap_or_else(|| vm_id.clone());

            let containers: Vec<ContainerDecl> = svc["containers"]
                .as_object()
                .unwrap_or(&empty_map)
                .iter()
                .map(|(key, ct)| ContainerDecl {
                    key: key.clone(),
                    container_name: ct["container_name"]
                        .as_str()
                        .unwrap_or("?")
                        .to_string(),
                    image: ct["image"].as_str().unwrap_or("?").to_string(),
                    port: ct["port"].as_u64().map(|p| p as u16),
                    dns: ct["dns"].as_str().map(|s| s.to_string()),
                    healthcheck: ct["healthcheck"].as_str().map(|s| s.to_string()),
                })
                .collect();

            ServiceInfo {
                name: name.clone(),
                category: svc["category"].as_str().unwrap_or("?").to_string(),
                vm_id,
                vm_alias,
                folder: svc["folder"].as_str().unwrap_or("").to_string(),
                domain: svc["domain"].as_str().map(|s| s.to_string()),
                port: svc["port"].as_u64().map(|p| p as u16),
                dns: svc["dns"].as_str().map(|s| s.to_string()),
                upstream: svc["upstream"].as_str().map(|s| s.to_string()),
                containers,
                enabled: svc["enabled"].as_bool().unwrap_or(true),
            }
        })
        .collect()
}

/// Parse caddy routes from caddy routes JSON
pub fn parse_caddy_routes(caddy_json: &Value) -> Vec<CaddyRoute> {
    caddy_json["routes"]
        .as_array()
        .map(|routes| {
            routes
                .iter()
                .map(|r| CaddyRoute {
                    domain: r["domain"].as_str().unwrap_or("").to_string(),
                    upstream: r["upstream"].as_str().unwrap_or("").to_string(),
                    comment: r["comment"].as_str().unwrap_or("").to_string(),
                    auth: r["auth"].as_str().map(|s| s.to_string()),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Scan build.json files for ports.app
pub fn scan_build_json_ports(base: &str) -> HashMap<String, u16> {
    let mut ports = HashMap::new();
    let Ok(entries) = std::fs::read_dir(base) else {
        return ports;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let build_json = path.join("build.json");
        if !build_json.exists() {
            continue;
        }
        let Ok(raw) = std::fs::read_to_string(&build_json) else {
            continue;
        };
        let Ok(j) = serde_json::from_str::<Value>(&raw) else {
            continue;
        };

        let folder_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let svc_name = j["name"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                folder_name
                    .split('_')
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("_")
            });

        if let Some(port) = j["ports"]["app"].as_u64() {
            ports.insert(svc_name, port as u16);
        }
    }
    ports
}
