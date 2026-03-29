use crate::checks::*;
use crate::types::*;
use chrono::Utc;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Instant;

/// Collect ALL live data in parallel
pub async fn collect_all(ctx: &Context) -> LiveData {
    let start = Instant::now();
    let mut timers: HashMap<String, u64> = HashMap::new();

    let client = http_client();
    let aclient = ctx.bearer_token.as_ref().map(|t| auth_client(t)).unwrap_or_else(|| client.clone());
    let resolver = hickory_resolver();

    // Pre-check: Hickory reachable?
    let hickory_up = dns_resolve(&resolver, "authelia.app").await.is_some();
    println!("  Hickory: {}", if hickory_up { "✅" } else { "❌" });

    // Launch ALL parallel groups
    let (mesh, urls, priv_eps, vm_data, mail_dns, port_scan) = tokio::join!(
        collect_mesh(ctx),
        collect_public_urls(ctx, &client, &aclient),
        collect_private(ctx, &client, &resolver, hickory_up),
        collect_vms(ctx),
        collect_mail_dns(&resolver),
        collect_port_scan(ctx),
    );

    timers.insert("mesh".into(), mesh.1);
    timers.insert("public_urls".into(), urls.1);
    timers.insert("private".into(), priv_eps.1);
    timers.insert("vm_ssh".into(), vm_data.1);
    timers.insert("mail_dns".into(), mail_dns.1);
    timers.insert("port_scan".into(), port_scan.1);
    timers.insert("TOTAL".into(), start.elapsed().as_millis() as u64);

    LiveData {
        generated: Utc::now().to_rfc3339(),
        duration_ms: start.elapsed().as_millis() as u64,
        mesh: mesh.0, public_urls: urls.0, private_endpoints: priv_eps.0,
        vm_data: vm_data.0, mail_dns: mail_dns.0, port_scan: port_scan.0,
        timers,
    }
}

async fn collect_mesh(ctx: &Context) -> (Vec<MeshResult>, u64) {
    let t = Instant::now();
    let futs: Vec<_> = ctx.wg_peers.iter().map(|peer| {
        let vm = ctx.vms.iter().find(|v| v.alias == peer.name).cloned();
        let name = peer.name.clone();
        let pub_ip = peer.pub_ip.clone();
        let wg_ip = peer.wg_ip.clone();
        let role = peer.role.clone();
        let cloud_name = vm.as_ref().map(|v| v.cloud_name.clone()).unwrap_or("—".into());
        let rescue_port = vm.as_ref().map(|v| v.rescue_port).unwrap_or(2200);
        let vm_id = vm.as_ref().map(|v| v.vm_id.clone()).unwrap_or_default();
        let is_client = role == "client";

        async move {
            if is_client || pub_ip == "?" || pub_ip == "dynamic" {
                return MeshResult {
                    name, cloud_name, pub_ip, wg_ip, peer_type: if role == "hub" { "HUB" } else if is_client { "CLIENT" } else { "VM" }.into(),
                    vps_ok: is_client, vps_status: if is_client { "client" } else { "?" }.into(),
                    pub_ok: is_client, dropbear_ok: false, wg_ok: false, wg_handshake: "no data".into(),
                };
            }
            let (pub_ok, db_ok) = tokio::join!(tcp(&pub_ip, 22), tcp(&pub_ip, rescue_port));
            // VPS check via gcloud if GCP
            let (vps_ok, vps_status) = if vm_id.starts_with("gcp-") {
                match gcloud_status(&cloud_name) {
                    Some(s) => (s == "RUNNING", s),
                    None => (pub_ok, if pub_ok { "SSH OK" } else { "SSH fail" }.into()),
                }
            } else {
                (pub_ok, if pub_ok { "SSH OK" } else { "SSH fail" }.into())
            };
            MeshResult {
                name, cloud_name, pub_ip, wg_ip,
                peer_type: if role == "hub" { "HUB" } else { "VM" }.into(),
                vps_ok, vps_status, pub_ok, dropbear_ok: db_ok, wg_ok: false, wg_handshake: "no data".into(),
            }
        }
    }).collect();
    let results = futures::future::join_all(futs).await;
    println!("  A0 Mesh: {} peers in {}ms", results.len(), t.elapsed().as_millis());
    (results, t.elapsed().as_millis() as u64)
}

async fn collect_public_urls(ctx: &Context, client: &Client, aclient: &Client) -> (Vec<UrlResult>, u64) {
    let t = Instant::now();
    let futs: Vec<_> = ctx.public_urls.iter().map(|u| {
        let url = u.url.clone();
        let upstream = u.upstream.clone();
        let cl = client.clone();
        let acl = aclient.clone();
        async move {
            let http_url = format!("http://{}", url);
            let https_url = format!("https://{}", url);
            let (tcp_ok, http_r, https_r, auth_r) = tokio::join!(
                tcp(&url, 443),
                http_get(&cl, &http_url),
                http_get(&cl, &https_url),
                http_get(&acl, &https_url),
            );
            UrlResult {
                url, upstream, tcp: tcp_ok,
                http: http_r.0, https: https_r.0,
                auth: auth_r.0 && auth_r.1 != "401" && auth_r.1 != "403",
                code: if https_r.0 { https_r.1 } else { http_r.1 },
                auth_code: auth_r.1,
            }
        }
    }).collect();
    let results = futures::future::join_all(futs).await;
    let https_ok = results.iter().filter(|u| u.https).count();
    let auth_ok = results.iter().filter(|u| u.auth).count();
    println!("  A1 Public: {}/{} HTTPS, {}/{} AUTH in {}ms", https_ok, results.len(), auth_ok, results.len(), t.elapsed().as_millis());
    (results, t.elapsed().as_millis() as u64)
}

async fn collect_private(ctx: &Context, client: &Client, resolver: &trust_dns_resolver::TokioAsyncResolver, hickory_up: bool) -> (Vec<PrivateResult>, u64) {
    let t = Instant::now();
    if !hickory_up {
        let results: Vec<PrivateResult> = ctx.private_dns.iter().map(|d| PrivateResult {
            dns: d.dns.clone(), container: d.container.clone(), port: d.port, vm: d.vm.clone(),
            tcp: false, http: false, code: "---".into(), resolved_ip: String::new(),
        }).collect();
        println!("  A2 Private: SKIPPED (Hickory down)");
        return (results, t.elapsed().as_millis() as u64);
    }
    let futs: Vec<_> = ctx.private_dns.iter().map(|d| {
        let dns = d.dns.clone();
        let port = d.port;
        let container = d.container.clone();
        let vm = d.vm.clone();
        let r = resolver.clone();
        let cl = client.clone();
        async move {
            let ip = dns_resolve(&r, &dns).await;
            match ip {
                Some(resolved) => {
                    let http_url = format!("http://{}:{}", resolved, port);
                    let (tcp_ok, http_r) = tokio::join!(tcp(&resolved, port), http_get(&cl, &http_url));
                    PrivateResult { dns, container, port, vm, tcp: tcp_ok, http: http_r.0, code: http_r.1, resolved_ip: resolved }
                }
                None => PrivateResult { dns, container, port, vm, tcp: false, http: false, code: "---".into(), resolved_ip: String::new() },
            }
        }
    }).collect();
    let results = futures::future::join_all(futs).await;
    let tcp_ok = results.iter().filter(|p| p.tcp).count();
    println!("  A2 Private: {}/{} TCP in {}ms", tcp_ok, results.len(), t.elapsed().as_millis());
    (results, t.elapsed().as_millis() as u64)
}

async fn collect_vms(ctx: &Context) -> (Vec<VmLiveData>, u64) {
    let t = Instant::now();
    // Try rsync from /opt/health/latest.json first (fast, via Dropbear if needed)
    // Fallback to SSH commands if rsync fails
    let futs: Vec<_> = ctx.vms.iter().map(|vm| {
        let alias = vm.alias.clone();
        let pub_ip = vm.pub_ip.clone();
        let rescue_port = vm.rescue_port;
        let dns_entries: Vec<_> = ctx.private_dns.iter().filter(|d| d.vm == alias).cloned().collect();
        async move {
            let mut data = VmLiveData {
                alias: alias.clone(), reachable: false,
                mem_used: "?".into(), mem_total: "?".into(), mem_pct: 0, swap: "?".into(),
                disk_used: "?".into(), disk_total: "?".into(), disk_pct: "?".into(),
                load: "?".into(), uptime: "?".into(),
                containers: vec![], containers_running: 0, containers_total: 0,
            };

            // Strategy 1: rsync /opt/health/latest.json (fast, works via Dropbear too)
            let cache_dir = format!("cache/{}", alias);
            let _ = std::fs::create_dir_all(&cache_dir);
            let cache_file = format!("{}/latest.json", cache_dir);

            // Try normal SSH first, then Dropbear port
            let rsync_ok = {
                let out = tokio::process::Command::new("rsync")
                    .args(["-az", "-e", "ssh -o ConnectTimeout=5 -o ControlPath=none -o BatchMode=yes",
                           &format!("{}@{}:/opt/health/latest.json", "ubuntu", pub_ip), &cache_file])
                    .output().await;
                out.map(|o| o.status.success()).unwrap_or(false)
            } || {
                // Fallback: Dropbear port
                let out = tokio::process::Command::new("rsync")
                    .args(["-az", "-e", &format!("ssh -p {} -o ConnectTimeout=5 -o ControlPath=none -o BatchMode=yes", rescue_port),
                           &format!("{}@{}:/opt/health/latest.json", "ubuntu", pub_ip), &cache_file])
                    .output().await;
                out.map(|o| o.status.success()).unwrap_or(false)
            };

            if rsync_ok {
                // Parse cached JSON
                if let Ok(raw) = std::fs::read_to_string(&cache_file) {
                    if let Ok(j) = serde_json::from_str::<serde_json::Value>(&raw) {
                        data.reachable = true;
                        data.mem_used = format!("{}M", j["mem"]["used"].as_u64().unwrap_or(0));
                        data.mem_total = format!("{}M", j["mem"]["total"].as_u64().unwrap_or(0));
                        data.mem_pct = j["mem"]["pct"].as_u64().unwrap_or(0) as u32;
                        data.swap = j["swap"].as_str().map(|s| s.to_string()).unwrap_or_else(||
                            format!("{}M/{}M", j["swap"]["used"].as_u64().unwrap_or(0), j["swap"]["total"].as_u64().unwrap_or(0)));
                        data.disk_used = j["disk"]["used"].as_str().unwrap_or("?").to_string();
                        data.disk_total = j["disk"]["total"].as_str().unwrap_or("?").to_string();
                        data.disk_pct = j["disk"]["pct"].as_str().unwrap_or("?").to_string();
                        data.load = j["load"].as_str().unwrap_or("?").to_string();
                        data.uptime = j["uptime"].as_str().unwrap_or("?").to_string();
                        data.containers_total = j["containers_total"].as_u64().unwrap_or(0) as u32;
                        data.containers_running = j["containers_running"].as_u64().unwrap_or(0) as u32;
                        if let Some(ctrs) = j["containers"].as_array() {
                            for c in ctrs {
                                let name = c["name"].as_str().unwrap_or("?").to_string();
                                let status = c["status"].as_str().unwrap_or("?").to_string();
                                let health = c["health"].as_str().unwrap_or("none").to_string();
                                let dns_entry = dns_entries.iter().find(|d| d.container == name);
                                data.containers.push(ContainerState {
                                    name, status, health,
                                    docker_port: dns_entry.map(|d| d.port.to_string()).unwrap_or("—".into()),
                                    host_port: dns_entry.filter(|d| d.host_port).map(|d| d.port.to_string()).unwrap_or("—".into()),
                                });
                            }
                        }
                        data.containers.sort_by(|a, b| {
                            let order = |h: &str| -> u8 { match h { "created" => 0, "exited" => 1, "unhealthy" => 2, "starting" => 3, "none" => 4, "healthy" => 5, _ => 9 } };
                            order(&a.health).cmp(&order(&b.health))
                        });
                        return data;
                    }
                }
            }

            // Strategy 2: SSH all-in-one command (fallback if rsync fails — agent not deployed yet)
            let cmd = r#"echo "MEM:$(free -m | awk '/Mem/{printf "%d %d %d", $3, $2, $3*100/$2}')";echo "SWAP:$(free -m | awk '/Swap/{printf "%dM/%dM", $3, $2}')";echo "DISK:$(df -h / | awk 'NR==2{printf "%s %s %s", $3, $2, $5}')";echo "LOAD:$(cut -d' ' -f1-3 /proc/loadavg)";echo "UPTIME:$(uptime -p 2>/dev/null || awk '{printf "up %dd %dh", $1/86400, ($1%86400)/3600}' /proc/uptime)";docker ps -a --format '{{.Names}}|{{.Status}}' 2>/dev/null | sed 's/^/CTR:/' "#;

            let output = tokio::process::Command::new("ssh")
                .args(["-o", "ConnectTimeout=8", "-o", "ControlPath=none", "-o", "BatchMode=yes", &alias])
                .arg(cmd)
                .output().await;

            match output {
                Ok(out) if out.status.success() || !out.stdout.is_empty() => {
                    data.reachable = true;
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    for line in stdout.lines() {
                        if let Some(mem) = line.strip_prefix("MEM:") {
                            let parts: Vec<&str> = mem.split_whitespace().collect();
                            if parts.len() >= 3 {
                                data.mem_used = format!("{}M", parts[0]);
                                data.mem_total = format!("{}M", parts[1]);
                                data.mem_pct = parts[2].parse().unwrap_or(0);
                            }
                        } else if let Some(swap) = line.strip_prefix("SWAP:") {
                            data.swap = swap.to_string();
                        } else if let Some(disk) = line.strip_prefix("DISK:") {
                            let parts: Vec<&str> = disk.split_whitespace().collect();
                            if parts.len() >= 3 {
                                data.disk_used = parts[0].to_string();
                                data.disk_total = parts[1].to_string();
                                data.disk_pct = parts[2].to_string();
                            }
                        } else if let Some(load) = line.strip_prefix("LOAD:") {
                            data.load = load.to_string();
                        } else if let Some(uptime) = line.strip_prefix("UPTIME:") {
                            data.uptime = uptime.to_string();
                        } else if let Some(ctr) = line.strip_prefix("CTR:") {
                            let parts: Vec<&str> = ctr.splitn(2, '|').collect();
                            if parts.len() == 2 {
                                let name = parts[0].to_string();
                                let status = parts[1].to_string();
                                let health = if status.contains("(healthy)") { "healthy" }
                                    else if status.contains("(unhealthy)") { "unhealthy" }
                                    else if status.contains("health: starting") { "starting" }
                                    else if status.starts_with("Created") { "created" }
                                    else if status.starts_with("Exited") { "exited" }
                                    else { "none" };
                                let dns_entry = dns_entries.iter().find(|d| d.container == name);
                                let docker_port = dns_entry.map(|d| d.port.to_string()).unwrap_or("—".into());
                                let host_port = dns_entry.filter(|d| d.host_port).map(|d| d.port.to_string()).unwrap_or("—".into());
                                data.containers.push(ContainerState { name, status, health: health.into(), docker_port, host_port });
                                data.containers_total += 1;
                                if health != "exited" && health != "created" { data.containers_running += 1; }
                            }
                        }
                    }
                    // Sort containers: exited first, then unhealthy, then starting, then up, then healthy
                    data.containers.sort_by(|a, b| {
                        let order = |h: &str| -> u8 { match h { "created" => 0, "exited" => 1, "unhealthy" => 2, "starting" => 3, "none" => 4, "healthy" => 5, _ => 9 } };
                        order(&a.health).cmp(&order(&b.health))
                    });
                }
                _ => { /* unreachable */ }
            }
            data
        }
    }).collect();
    let results = futures::future::join_all(futs).await;
    let reachable = results.iter().filter(|v| v.reachable).count();
    println!("  A3 VMs: {}/{} reachable in {}ms", reachable, results.len(), t.elapsed().as_millis());
    (results, t.elapsed().as_millis() as u64)
}

async fn collect_mail_dns(resolver: &trust_dns_resolver::TokioAsyncResolver) -> (MailDnsData, u64) {
    let t = Instant::now();
    let mut data = MailDnsData::default();

    // MX
    for domain in &["diegonmarcos.com", "send.mails.diegonmarcos.com", "mails.diegonmarcos.com"] {
        let records = dns_mx(resolver, domain).await;
        if records.is_empty() {
            data.mx.push(MxRecord { domain: domain.to_string(), priority: "—".into(), server: "no MX record".into(), ip: String::new() });
        } else {
            for (pri, srv) in records {
                let ip = dns_resolve(resolver, &srv).await.unwrap_or_default();
                data.mx.push(MxRecord { domain: domain.to_string(), priority: pri, server: srv, ip });
            }
        }
    }

    // SPF
    if let Some(spf) = dns_txt(resolver, "diegonmarcos.com").await {
        for cap in spf.split_whitespace().filter(|s| s.starts_with("include:")) {
            let target = cap.strip_prefix("include:").unwrap_or("");
            let resolved = dns_txt(resolver, target).await.unwrap_or_default();
            let ips: Vec<&str> = resolved.split_whitespace().filter(|s| s.starts_with("ip4:")).take(3).collect();
            data.spf.push(SpfEntry { domain: "diegonmarcos.com".into(), include: cap.to_string(), ips: ips.join(", ") });
        }
    }
    if let Some(spf) = dns_txt(resolver, "send.mails.diegonmarcos.com").await {
        for cap in spf.split_whitespace().filter(|s| s.starts_with("include:")) {
            data.spf.push(SpfEntry { domain: "send.mails.diegonmarcos.com".into(), include: cap.to_string(), ips: "(same)".into() });
        }
    }

    // DKIM
    let selectors = vec![
        ("dkim._domainkey", "Stalwart"), ("mail._domainkey", "Legacy Mailu"),
        ("google._domainkey", "Google Workspace"), ("cf2024-1._domainkey", "Cloudflare"),
        ("resend._domainkey.mails", "Resend/SES"),
    ];
    for (sel, signer) in selectors {
        let fqdn = format!("{}.diegonmarcos.com", sel);
        let txt = dns_txt(resolver, &fqdn).await;
        let present = txt.as_ref().map(|t| t.contains("DKIM1")).unwrap_or(false);
        let key_size = if present {
            let b64_len = txt.as_ref().and_then(|t| t.find("p=").map(|i| t[i+2..].split_whitespace().next().unwrap_or("").len())).unwrap_or(0);
            if b64_len > 300 { "RSA 2048" } else if b64_len > 100 { "RSA 1024" } else { "?" }
        } else { "NOT FOUND" };
        data.dkim.push(DkimEntry { selector: sel.to_string(), domain: "diegonmarcos.com".into(), signer: signer.into(), present, key_size: key_size.into() });
    }

    // DMARC
    data.dmarc = dns_txt(resolver, "_dmarc.diegonmarcos.com").await.unwrap_or("NO DMARC".into());

    println!("  A4 Mail DNS: {} MX, {} SPF, {} DKIM in {}ms", data.mx.len(), data.spf.len(), data.dkim.len(), t.elapsed().as_millis());
    (data, t.elapsed().as_millis() as u64)
}

async fn collect_port_scan(ctx: &Context) -> (Vec<PortScanResult>, u64) {
    let t = Instant::now();
    let ports = vec![22, 25, 80, 443, 465, 587, 993, 2200, 4190, 5000, 8080, 8443, 8888, 51820];
    let futs: Vec<_> = ctx.vms.iter().filter(|v| !v.pub_ip.is_empty() && v.pub_ip != "?").map(|vm| {
        let name = vm.alias.clone();
        let ip = vm.pub_ip.clone();
        let ports = ports.clone();
        async move {
            let open = tcp_scan(&ip, &ports).await;
            println!("    {}: {}", name, if open.is_empty() { "none".to_string() } else { open.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ") });
            PortScanResult { name, ip, open_ports: open }
        }
    }).collect();
    let results = futures::future::join_all(futs).await;
    println!("  C Port scan: {} VMs in {}ms", results.len(), t.elapsed().as_millis());
    (results, t.elapsed().as_millis() as u64)
}
