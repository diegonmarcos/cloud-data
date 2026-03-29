use crate::types::*;
use anyhow::Result;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

const CONSOLIDATED: &str = "../../_cloud-data-consolidated.json";

pub fn load_context() -> Result<Context> {
    let raw = std::fs::read_to_string(CONSOLIDATED)?;
    let c: Value = serde_json::from_str(&raw)?;

    let mut vm_alias_map: HashMap<String, String> = HashMap::new();
    let mut host_ports_by_vm: HashMap<String, HashSet<u16>> = HashMap::new();

    // VMs
    let vms: Vec<VmInfo> = c["vms"].as_object().unwrap_or(&serde_json::Map::new()).iter().map(|(id, vm)| {
        let alias = vm["ssh_alias"].as_str().unwrap_or(id).to_string();
        vm_alias_map.insert(id.clone(), alias.clone());
        // Host ports
        let mut ports = HashSet::new();
        if let Some(pp) = vm["public_ports"].as_array() {
            for p in pp { if let Some(port) = p["port"].as_u64() { ports.insert(port as u16); } }
        }
        let mut declared_ports: Vec<u16> = ports.iter().copied().collect();
        host_ports_by_vm.insert(alias.clone(), ports);

        let provider = if id.starts_with("oci-") { "OCI" } else if id.starts_with("gcp-") { "GCP" } else if id.starts_with("vast-") { "Vast.ai" } else { "?" };
        let cost = if id.contains("-f_") || id.contains("-f-") { "Free" } else if id.contains("-p_") { "Spot" } else { "?" };
        declared_ports.sort();
        VmInfo {
            alias, vm_id: id.clone(),
            pub_ip: vm["ip"].as_str().unwrap_or("?").to_string(),
            wg_ip: vm["wg_ip"].as_str().unwrap_or("?").to_string(),
            cloud_name: vm["specs"]["cloud_name"].as_str().unwrap_or("—").to_string(),
            cloud_zone: vm["specs"]["cloud_zone"].as_str().unwrap_or("?").to_string(),
            rescue_port: vm["rescue_port"].as_u64().unwrap_or(2200) as u16,
            cpus: vm["specs"]["cpu"].as_u64().unwrap_or(0) as u32,
            ram_gb: format!("{}G", vm["specs"]["ram_gb"].as_u64().unwrap_or(0)),
            disk_gb: format!("{}G", vm["specs"]["disk_gb"].as_u64().unwrap_or(0)),
            shape: vm["specs"]["shape"].as_str().or(vm["specs"]["machine_type"].as_str()).unwrap_or("?").to_string(),
            provider: provider.to_string(),
            cost: cost.to_string(),
            declared_ports,
        }
    }).filter(|v| !v.wg_ip.is_empty() && v.wg_ip != "?").collect();

    // Public URLs from caddy routes
    let mut public_urls: Vec<UrlInfo> = Vec::new();
    let mut seen = HashSet::new();
    let add = |urls: &mut Vec<UrlInfo>, seen: &mut HashSet<String>, url: String, upstream: String| {
        if !seen.contains(&url) { seen.insert(url.clone()); urls.push(UrlInfo { url, upstream }); }
    };

    // From services
    for (_, svc) in c["services"].as_object().unwrap_or(&serde_json::Map::new()) {
        if let Some(domain) = svc["domain"].as_str() {
            let upstream = svc["upstream"].as_str().unwrap_or("?");
            add(&mut public_urls, &mut seen, domain.to_string(), upstream.to_string());
        }
    }
    // From caddy routes
    for route in c["configs"]["caddy"]["routes"].as_array().unwrap_or(&vec![]) {
        if let Some(d) = route["domain"].as_str() {
            add(&mut public_urls, &mut seen, d.to_string(), route["upstream"].as_str().unwrap_or("?").to_string());
        }
    }

    // Private DNS from containers
    let mut private_dns: Vec<DnsEntry> = Vec::new();
    for (_, svc) in c["services"].as_object().unwrap_or(&serde_json::Map::new()) {
        let vm_alias = vm_alias_map.get(svc["vm"].as_str().unwrap_or("")).cloned().unwrap_or_default();
        for (_, ct) in svc["containers"].as_object().unwrap_or(&serde_json::Map::new()) {
            let dns = ct["dns"].as_str().unwrap_or("").to_string();
            let port = ct["port"].as_u64().unwrap_or(0) as u16;
            if dns.is_empty() || port == 0 { continue; }
            let host_port = host_ports_by_vm.get(&vm_alias).map(|s| s.contains(&port)).unwrap_or(false);
            private_dns.push(DnsEntry {
                dns, container: ct["container_name"].as_str().unwrap_or("?").to_string(),
                port, vm: vm_alias.clone(), host_port,
            });
        }
    }
    private_dns.sort_by(|a, b| a.vm.cmp(&b.vm).then(a.dns.cmp(&b.dns)));

    // Databases from containers with db_user/db_name
    let mut databases: Vec<DbEntry> = Vec::new();
    for (svc_name, svc) in c["services"].as_object().unwrap_or(&serde_json::Map::new()) {
        let vm_alias = vm_alias_map.get(svc["vm"].as_str().unwrap_or("")).cloned().unwrap_or_default();
        for (_, ct) in svc["containers"].as_object().unwrap_or(&serde_json::Map::new()) {
            if ct["db_user"].is_string() || ct["db_name"].is_string() || ct["db_path"].is_string() {
                let img = ct["image"].as_str().unwrap_or("").to_lowercase();
                let db_type = if img.contains("postgres") { "postgres" } else if img.contains("mariadb") || img.contains("mysql") { "mariadb" } else if img.contains("redis") { "redis" } else if img.contains("sqlite") { "sqlite" } else { "?" };
                let dns_entry = private_dns.iter().find(|d| d.container == ct["container_name"].as_str().unwrap_or(""));
                databases.push(DbEntry {
                    service: svc_name.to_string(), db_type: db_type.to_string(),
                    container: ct["container_name"].as_str().unwrap_or("?").to_string(),
                    db_name: ct["db_name"].as_str().or(ct["db_path"].as_str()).unwrap_or("custom").to_string(),
                    vm: vm_alias.clone(),
                    dns_access: dns_entry.map(|d| format!("{}:{}", d.dns, d.port)).unwrap_or("embedded".to_string()),
                });
            }
        }
    }
    databases.sort_by(|a, b| a.vm.cmp(&b.vm).then(a.service.cmp(&b.service)));

    // WG peers
    let wg = &c["native"]["wireguard"];
    let mut wg_peers: Vec<WgPeer> = Vec::new();
    for peer in wg["peers"].as_array().unwrap_or(&vec![]) {
        wg_peers.push(WgPeer {
            name: peer["name"].as_str().unwrap_or("?").to_string(),
            wg_ip: peer["wg_ip"].as_str().unwrap_or("?").to_string(),
            pub_ip: peer["endpoint"].as_str().unwrap_or("?").split(':').next().unwrap_or("?").to_string(),
            role: peer["role"].as_str().unwrap_or("spoke").to_string(),
        });
    }
    for (name, client) in wg["clients"].as_object().unwrap_or(&serde_json::Map::new()) {
        wg_peers.push(WgPeer {
            name: name.clone(),
            wg_ip: client["wg_ip"].as_str().unwrap_or("?").to_string(),
            pub_ip: "dynamic".to_string(),
            role: "client".to_string(),
        });
    }

    // Mail ports
    let mail_ports = vec![
        MailPort { host: "mail.diegonmarcos.com".into(), port: 465, proto: "SMTPS".into() },
        MailPort { host: "mail.diegonmarcos.com".into(), port: 587, proto: "Submission".into() },
        MailPort { host: "mail.diegonmarcos.com".into(), port: 993, proto: "IMAPS".into() },
        MailPort { host: "smtp.diegonmarcos.com".into(), port: 465, proto: "SMTPS".into() },
        MailPort { host: "smtp.diegonmarcos.com".into(), port: 587, proto: "Submission".into() },
        MailPort { host: "imap.diegonmarcos.com".into(), port: 993, proto: "IMAPS".into() },
    ];

    // Caddy L4
    let caddy_l4: Vec<L4Route> = c["configs"]["caddy"]["l4_routes"].as_array().unwrap_or(&vec![]).iter().map(|r| {
        L4Route {
            port: r["port"].as_u64().unwrap_or(0) as u16,
            upstream: r["upstream"].as_str().unwrap_or("?").to_string(),
            comment: r["comment"].as_str().unwrap_or("").to_string(),
        }
    }).collect();

    // Bearer token
    let home = std::env::var("HOME").unwrap_or("/home/diego".into());
    let bearer_token = std::fs::read_to_string(format!("{}/Mounts/Git/vault/A0_keys/providers/authelia/signed-bearer_jwt/tokens/cloud-admin.json", home))
        .ok().and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .and_then(|v| v["access_token"].as_str().map(|s| s.to_string()));

    Ok(Context { consolidated: c, vms, public_urls, private_dns, databases, wg_peers, mail_ports, caddy_l4, bearer_token })
}
