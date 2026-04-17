use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

const MONITORING: &str = "../../cloud-data-monitoring-targets.json";
const DATABASES: &str = "../../cloud-data-databases.json";
const CONSOLIDATED: &str = "../../_cloud-data-consolidated.json";

pub struct Context {
    pub monitoring: MonitoringTargets,
    pub databases: DatabasesJson,
    pub manifests: HashMap<String, VmContainerManifest>,
    pub cloud_buckets: Vec<CloudBucket>,
    pub services: Vec<ServiceEntry>,
    pub mcp_servers: Vec<McpServer>,
    pub vps_providers: Vec<VpsProvider>,
    pub vm_finops: Vec<VmFinops>,
    pub total_domains: usize,
}

pub fn load_context() -> Result<Context> {
    let monitoring: MonitoringTargets = if Path::new(MONITORING).exists() {
        let raw = std::fs::read_to_string(MONITORING)?;
        serde_json::from_str(&raw)?
    } else {
        MonitoringTargets {
            endpoint_checks: vec![],
            tls_checks: vec![],
            vms: vec![
                VmTarget { ip: "10.0.0.1".into(), name: "gcp-proxy".into(), user: "diego".into() },
                VmTarget { ip: "10.0.0.3".into(), name: "oci-mail".into(), user: "ubuntu".into() },
                VmTarget { ip: "10.0.0.4".into(), name: "oci-analytics".into(), user: "ubuntu".into() },
                VmTarget { ip: "10.0.0.6".into(), name: "oci-apps".into(), user: "ubuntu".into() },
            ],
        }
    };

    let databases: DatabasesJson = if Path::new(DATABASES).exists() {
        let raw = std::fs::read_to_string(DATABASES)?;
        serde_json::from_str(&raw)?
    } else {
        DatabasesJson { databases: vec![] }
    };

    // Load cloud buckets + services from consolidated JSON
    let (cloud_buckets, services, vps_providers, vm_finops, total_domains) = if Path::new(CONSOLIDATED).exists() {
        match std::fs::read_to_string(CONSOLIDATED) {
            Ok(raw) => match serde_json::from_str::<ConsolidatedJson>(&raw) {
                Ok(c) => {
                    let svc_list: Vec<ServiceEntry> = c.services.iter().map(|(name, svc)| {
                        let stype = if name.contains("mcp") { "mcp" }
                            else if ["caddy","introspect-proxy","hickory-dns","redis","sauron-central","sauron-lite","sauron-forwarder","db-agent","fluent-bit"].contains(&name.as_str()) { "infra" }
                            else { "app" };
                        let domain = svc["domain"].as_str().unwrap_or("").to_string();
                        let port = svc["port"].as_u64().unwrap_or(0) as u16;
                        // API classification from consolidated (derived by gen-cloud-data.ts)
                        let api_info = &svc["api"];
                        let has_api_declared = api_info["has_api"].as_bool().unwrap_or(false);
                        let has_web_ui = api_info["has_web_ui"].as_bool().unwrap_or(!domain.is_empty());
                        let api_path = api_info["api_path"].as_str().unwrap_or("").to_string();
                        let api_url = api_info["api_url"].as_str().unwrap_or("").to_string();
                        // has_api starts from declared, then upgraded to true by runtime probing in main.rs
                        let has_api = has_api_declared;
                        ServiceEntry {
                            name: name.clone(),
                            category: svc["category"].as_str().unwrap_or("?").to_string(),
                            vm: svc["vm"].as_str().unwrap_or("?").to_string(),
                            domain,
                            enabled: svc["enabled"].as_bool().unwrap_or(true),
                            containers: svc["containers"].as_object().map(|o| o.len() as u32).unwrap_or(0),
                            port: svc["port"].as_u64().unwrap_or(0) as u16,
                            service_type: stype.to_string(),
                            has_api,
                            has_web_ui,
                            api_path,
                            api_url,
                        }
                    }).collect();
                    // Parse VPS providers
                    let vps_list: Vec<VpsProvider> = c.vpss.iter().map(|(name, v)| {
                        VpsProvider {
                            name: name.clone(),
                            provider: v["provider"].as_str().unwrap_or(name).to_string(),
                            tier: v["tier"].as_str().unwrap_or("?").to_string(),
                        }
                    }).collect();

                    // Parse VM finops
                    let vm_fin: Vec<VmFinops> = c.vms.iter().map(|(vm_id, vm)| {
                        let provider = if vm_id.starts_with("oci") { "OCI" }
                            else if vm_id.starts_with("gcp") { "GCP" }
                            else { "?" };
                        let tier = if vm_id.contains("-f_") || vm_id.contains("-f-") { "Free" } else { "Paid" };
                        let specs = &vm["specs"];
                        VmFinops {
                            alias: vm["ssh_alias"].as_str().unwrap_or(vm_id).to_string(),
                            provider: provider.to_string(),
                            tier: tier.to_string(),
                            cpu: specs["cpu"].as_u64().unwrap_or(0) as u32,
                            ram_gb: specs["ram_gb"].as_f64().unwrap_or(0.0),
                            shape: specs["shape"].as_str()
                                .or(specs["machine_type"].as_str())
                                .unwrap_or("?").to_string(),
                            services: vm["services"].as_array().map(|a| a.len() as u32).unwrap_or(0),
                            containers: vm["container_count"].as_u64().unwrap_or(0) as u32,
                        }
                    }).collect();

                    // Count unique domains
                    let total_domains = svc_list.iter()
                        .filter(|s| s.enabled && !s.domain.is_empty())
                        .map(|s| s.domain.clone())
                        .collect::<std::collections::HashSet<_>>()
                        .len();

                    (c.storage, svc_list, vps_list, vm_fin, total_domains)
                }
                Err(e) => { eprintln!("  Failed to parse consolidated: {}", e); (vec![], vec![], vec![], vec![], 0) }
            },
            Err(_) => (vec![], vec![], vec![], vec![], 0),
        }
    } else {
        (vec![], vec![], vec![], vec![], 0)
    };

    // Load per-VM container manifests
    let mut manifests = HashMap::new();
    for vm in &monitoring.vms {
        let manifest_path = format!("../../cloud-data-containers-{}.json", vm.name);
        if Path::new(&manifest_path).exists() {
            match std::fs::read_to_string(&manifest_path) {
                Ok(raw) => match serde_json::from_str::<VmContainerManifest>(&raw) {
                    Ok(manifest) => {
                        eprintln!("  Loaded manifest for {} ({} services)", vm.name, manifest.services.len());
                        manifests.insert(vm.name.clone(), manifest);
                    }
                    Err(e) => eprintln!("  Failed to parse manifest for {}: {}", vm.name, e),
                },
                Err(e) => eprintln!("  Failed to read manifest for {}: {}", vm.name, e),
            }
        }
    }

    // Load MCP servers from .mcp.json (actual configured MCPs)
    let mcp_servers = {
        let home = std::env::var("HOME").unwrap_or("/home/diego".into());
        let mcp_paths = [
            format!("{}/.mcp.json", home),
            format!("{}/.claude/.mcp.json", home),
        ];
        let mut servers = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for mcp_path in &mcp_paths {
            if let Ok(raw) = std::fs::read_to_string(mcp_path) {
                if let Ok(j) = serde_json::from_str::<serde_json::Value>(&raw) {
                    if let Some(mcps) = j["mcpServers"].as_object() {
                        for (name, cfg) in mcps {
                            if seen.contains(name) { continue; }
                            seen.insert(name.clone());
                            let command = cfg["command"].as_str().unwrap_or("?").to_string();
                            let args: Vec<String> = cfg["args"].as_array()
                                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                                .unwrap_or_default();
                            let source_path = args.first().cloned().unwrap_or_default();
                            let transport = if args.iter().any(|a| a.contains("http")) { "http" } else { "stdio" };
                            servers.push(McpServer {
                                name: name.clone(),
                                command,
                                source_path,
                                transport: transport.to_string(),
                            });
                        }
                    }
                }
            }
        }
        servers.sort_by(|a, b| a.name.cmp(&b.name));
        servers
    };

    Ok(Context { monitoring, databases, manifests, cloud_buckets, services, mcp_servers, vps_providers, vm_finops, total_domains })
}
