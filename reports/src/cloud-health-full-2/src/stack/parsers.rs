use super::types::*;
use anyhow::{Context as _, Result};
use reports_common::context::find_cloud_data_file;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub fn load_context() -> Result<Context> {
    let path = find_cloud_data_file("_cloud-data-consolidated.json")
        .context("_cloud-data-consolidated.json not found (walked up from cwd)")?;
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
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

    // Databases from cloud-data-databases.json (authoritative source — 18 declared DBs)
    let mut databases: Vec<DbEntry> = Vec::new();
    if let Some(db_path) = find_cloud_data_file("cloud-data-databases.json") {
      if let Ok(db_raw) = std::fs::read_to_string(&db_path) {
        if let Ok(db_json) = serde_json::from_str::<Value>(&db_raw) {
            for db in db_json["databases"].as_array().unwrap_or(&vec![]) {
                let container = db["container"].as_str().unwrap_or("?").to_string();
                let db_type = db["type"].as_str().unwrap_or("?").to_string();
                let vm_alias = db["vm_alias"].as_str().unwrap_or("").to_string();
                let port = db["port"].as_u64().unwrap_or(0) as u16;
                let dns = db["dns"].as_str().unwrap_or("").to_string();
                let dns_access = if dns.is_empty() { "embedded".to_string() } else if port > 0 { format!("{}:{}", dns, port) } else { dns.clone() };
                databases.push(DbEntry {
                    service: db["service"].as_str().unwrap_or("?").to_string(),
                    db_type,
                    container,
                    db_name: db["db"].as_str().or(db["path"].as_str()).unwrap_or("custom").to_string(),
                    db_user: db["user"].as_str().unwrap_or("").to_string(),
                    port,
                    vm: vm_alias,
                    dns_access,
                    enabled: db["enabled"].as_bool().unwrap_or(true),
                    healthcheck: db["healthcheck"].as_str().unwrap_or("").to_string(),
                    backup: db["backup"].as_bool().unwrap_or(false),
                });
            }
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
    let bearer_token = std::fs::read_to_string(format!("{}/git/vault/A0_keys/providers/authelia/signed-bearer_jwt/tokens/cloud-admin.json", home))
        .ok().and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .and_then(|v| v["access_token"].as_str().map(|s| s.to_string()));

    Ok(Context { consolidated: c, vms, public_urls, private_dns, databases, wg_peers, mail_ports, caddy_l4, bearer_token })
}
