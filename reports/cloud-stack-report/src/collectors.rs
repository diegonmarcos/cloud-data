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
    let (mesh, urls, priv_eps, vm_data, mail_dns, port_scan, db_health) = tokio::join!(
        collect_mesh(ctx),
        collect_public_urls(ctx, &client, &aclient),
        collect_private(ctx, &client, &resolver, hickory_up),
        collect_vms(ctx),
        collect_mail_dns(&resolver),
        collect_port_scan(ctx),
        collect_databases(ctx),
    );

    timers.insert("mesh".into(), mesh.1);
    timers.insert("public_urls".into(), urls.1);
    timers.insert("private".into(), priv_eps.1);
    timers.insert("vm_ssh".into(), vm_data.1);
    timers.insert("mail_dns".into(), mail_dns.1);
    timers.insert("port_scan".into(), port_scan.1);
    timers.insert("databases".into(), db_health.1);
    timers.insert("TOTAL".into(), start.elapsed().as_millis() as u64);

    // Storage from consolidated JSON (static, no live check)
    let storage_health = parse_storage(ctx);

    LiveData {
        generated: Utc::now().to_rfc3339(),
        duration_ms: start.elapsed().as_millis() as u64,
        mesh: mesh.0, public_urls: urls.0, private_endpoints: priv_eps.0,
        vm_data: vm_data.0, mail_dns: mail_dns.0, port_scan: port_scan.0,
        db_health: db_health.0, storage_health,
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
                code: if https_r.0 { https_r.1 } else { https_r.1 },
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
    // Skip terminated spot instances (gcp-t4) — check via gcloud first
    let active_vms: Vec<_> = ctx.vms.iter().filter(|vm| {
        if vm.vm_id.contains("-p_") || vm.vm_id.contains("-p-") {
            // Spot/paid instance — check if running via gcloud
            if vm.vm_id.starts_with("gcp-") {
                return crate::checks::gcloud_status(&vm.cloud_name).map(|s| s == "RUNNING").unwrap_or(false);
            }
        }
        true
    }).collect();
    // Pre-warm SSH multiplexed connections (sequential — one handshake each)
    let mux_dir = "/tmp/ssh-mux-health";
    let _ = std::fs::create_dir_all(mux_dir);
    // Pre-warm: establish SSH multiplexed masters sequentially
    let mux_futs: Vec<_> = active_vms.iter().map(|vm| {
        let alias = vm.alias.clone();
        let mux = mux_dir.to_string();
        async move {
            let _ = tokio::process::Command::new("ssh")
                .args(["-o", "BatchMode=yes", "-o", "ConnectTimeout=15",
                       "-o", &format!("ControlPath={}/%r@%h:%p", mux),
                       "-o", "ControlMaster=yes", "-o", "ControlPersist=120",
                       "-fNM", &alias])
                .output().await;
        }
    }).collect();
    futures::future::join_all(mux_futs).await;
    // Brief settle for masters to fully initialize
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let mux_dir_owned = mux_dir.to_string();
    let futs: Vec<_> = active_vms.iter().map(|vm| {
        let alias = vm.alias.clone();
        let _pub_ip = vm.pub_ip.clone();
        let _rescue_port = vm.rescue_port;
        let dns_entries: Vec<_> = ctx.private_dns.iter().filter(|d| d.vm == alias).cloned().collect();
        let mux = mux_dir_owned.clone();
        async move {
            let mut data = VmLiveData {
                alias: alias.clone(), reachable: false,
                mem_used: "?".into(), mem_total: "?".into(), mem_pct: 0, swap: "?".into(),
                disk_used: "?".into(), disk_total: "?".into(), disk_pct: "?".into(),
                load: "?".into(), uptime: "?".into(),
                containers: vec![], containers_running: 0, containers_total: 0,
            };

            // Live SSH: system stats + docker ps -a + docker stats (via pre-warmed mux)
            let ssh_opts = format!("-o ConnectTimeout=30 -o ControlPath={}/%r@%h:%p -o ControlMaster=auto -o BatchMode=yes", mux);
            let cmd = r#"echo "MEM:$(free -m | awk '/Mem/{printf "%d %d %d", $3, $2, ($2>0?$3*100/$2:0)}')";echo "SWAP:$(free -m | awk '/Swap/{printf "%dM/%dM", $3, $2}')";echo "DISK:$(df -h / | awk 'NR==2{printf "%s %s %s", $3, $2, $5}')";echo "LOAD:$(cut -d' ' -f1-3 /proc/loadavg)";echo "UPTIME:$(awk '{printf "%dd %dh", $1/86400, ($1%86400)/3600}' /proc/uptime)";docker ps -a --format '{{.Names}}|{{.Status}}' 2>/dev/null | sed 's/^/CTR:/';docker stats --no-stream --format '{{.Name}}|{{.CPUPerc}}|{{.MemUsage}}' 2>/dev/null | sed 's/^/STATS:/'"#;

            let mut ssh_args: Vec<String> = ssh_opts.split_whitespace().map(|s| s.to_string()).collect();
            ssh_args.push(alias.clone());
            ssh_args.push(cmd.to_string());

            let output = tokio::process::Command::new("ssh")
                .args(&ssh_args)
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
                                    else if status.starts_with("Up") { "running" }
                                    else { "none" };
                                let dns_entry = dns_entries.iter().find(|d| d.container == name);
                                data.containers.push(ContainerState {
                                    name, status, health: health.to_string(),
                                    docker_port: dns_entry.map(|d| d.port.to_string()).unwrap_or("—".into()),
                                    host_port: dns_entry.filter(|d| d.host_port).map(|d| d.port.to_string()).unwrap_or("—".into()),
                                });
                            }
                        } else if let Some(_stats) = line.strip_prefix("STATS:") {
                            // docker stats: name|cpu%|mem usage — enrich existing container entries
                            let parts: Vec<&str> = _stats.splitn(3, '|').collect();
                            if parts.len() == 3 {
                                if let Some(c) = data.containers.iter_mut().find(|c| c.name == parts[0]) {
                                    c.status = format!("{} cpu={} mem={}", c.status, parts[1], parts[2]);
                                }
                            }
                        }
                    }
                    data.containers_total = data.containers.len() as u32;
                    data.containers_running = data.containers.iter().filter(|c| c.health != "exited" && c.health != "created").count() as u32;
                    data.containers.sort_by(|a, b| {
                        let order = |h: &str| -> u8 { match h { "created" => 0, "exited" => 1, "unhealthy" => 2, "starting" => 3, "none" => 4, "running" => 5, "healthy" => 6, _ => 9 } };
                        order(&a.health).cmp(&order(&b.health))
                    });
                }
                _ => { /* VM unreachable */ }
            }

            // (old rsync + commented SSH fallback removed — now using live SSH above)
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

async fn collect_databases(ctx: &Context) -> (Vec<DbHealthResult>, u64) {
    let t = Instant::now();
    if ctx.databases.is_empty() {
        println!("  B2 Databases: 0 declared");
        return (vec![], 0);
    }

    let mux_dir = "/tmp/ssh-mux-health";

    // Group databases by VM for batched SSH
    let mut by_vm: HashMap<String, Vec<&crate::types::DbEntry>> = HashMap::new();
    for db in &ctx.databases {
        by_vm.entry(db.vm.clone()).or_default().push(db);
    }

    // TCP checks in parallel (one per database with a port)
    let tcp_futs: Vec<_> = ctx.databases.iter().filter(|db| db.port > 0).map(|db| {
        let vm = ctx.vms.iter().find(|v| v.alias == db.vm);
        let wg_ip = vm.map(|v| v.wg_ip.clone()).unwrap_or_default();
        let port = db.port;
        let container = db.container.clone();
        async move {
            let ok = if !wg_ip.is_empty() { tcp(&wg_ip, port).await } else { false };
            (container, ok)
        }
    }).collect();
    let tcp_results: HashMap<String, bool> = futures::future::join_all(tcp_futs).await
        .into_iter().collect();

    // SSH batch per VM: health check + size query
    let vm_futs: Vec<_> = by_vm.iter().map(|(vm_alias, dbs)| {
        let alias = vm_alias.clone();
        let mux = mux_dir.to_string();
        let db_cmds: Vec<(String, String, String, String)> = dbs.iter().map(|db| {
            let health_cmd = match db.db_type.as_str() {
                "postgres" => format!(
                    "docker exec {} pg_isready -U {} 2>&1 && echo OK || echo FAIL",
                    db.container, if db.db_user.is_empty() { "postgres" } else { &db.db_user }
                ),
                "mariadb" => format!(
                    "docker exec {} mariadb-admin ping --silent 2>&1 && echo OK || echo FAIL",
                    db.container
                ),
                "redis" => format!(
                    "docker exec {} redis-cli ping 2>&1 | grep -q PONG && echo OK || echo FAIL",
                    db.container
                ),
                "sqlite" => format!(
                    "docker exec {} test -f {} 2>&1 && echo OK || echo FAIL",
                    db.container, if db.db_name.is_empty() { "/data/db.sqlite" } else { &db.db_name }
                ),
                _ => format!("docker inspect --format '{{{{.State.Running}}}}' {} 2>/dev/null | grep -q true && echo OK || echo FAIL", db.container),
            };
            let size_cmd = match db.db_type.as_str() {
                "postgres" => format!(
                    "docker exec {} psql -U {} -d {} -tAc \"SELECT pg_database_size('{}')\" 2>/dev/null || echo ?",
                    db.container,
                    if db.db_user.is_empty() { "postgres" } else { &db.db_user },
                    if db.db_name.is_empty() { "postgres" } else { &db.db_name },
                    if db.db_name.is_empty() { "postgres" } else { &db.db_name }
                ),
                "mariadb" => format!(
                    "docker exec {} mariadb -u {} -e \"SELECT SUM(data_length+index_length) FROM information_schema.tables WHERE table_schema='{}'\" --skip-column-names 2>/dev/null || echo ?",
                    db.container,
                    if db.db_user.is_empty() { "root" } else { &db.db_user },
                    if db.db_name.is_empty() { "mysql" } else { &db.db_name }
                ),
                "redis" => format!(
                    "docker exec {} redis-cli dbsize 2>/dev/null || echo ?",
                    db.container
                ),
                _ => "echo ?".to_string(),
            };
            (db.container.clone(), db.db_type.clone(), health_cmd, size_cmd)
        }).collect();

        async move {
            // Build a single batch SSH command for all DBs on this VM
            let mut batch_parts: Vec<String> = Vec::new();
            for (container, _db_type, health_cmd, size_cmd) in &db_cmds {
                batch_parts.push(format!(
                    "echo \"DB:{}:$({})::$({})\"",
                    container, health_cmd, size_cmd
                ));
            }
            // Also check which containers are running
            batch_parts.push("docker ps --format '{{.Names}}' 2>/dev/null | sed 's/^/RUNNING:/'".to_string());

            let batch_cmd = batch_parts.join("; ");
            let ssh_opts = format!(
                "-o ConnectTimeout=30 -o ControlPath={}/%r@%h:%p -o ControlMaster=auto -o BatchMode=yes",
                mux
            );
            let mut ssh_args: Vec<String> = ssh_opts.split_whitespace().map(|s| s.to_string()).collect();
            ssh_args.push(alias.clone());
            ssh_args.push(batch_cmd);

            let output = tokio::process::Command::new("ssh")
                .args(&ssh_args)
                .output().await;

            let mut results: HashMap<String, (bool, String)> = HashMap::new(); // container -> (healthy, size)
            let mut running_set: std::collections::HashSet<String> = std::collections::HashSet::new();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    if let Some(rest) = line.strip_prefix("DB:") {
                        // Format: container:health_output::size_output
                        let parts: Vec<&str> = rest.splitn(3, "::").collect();
                        if parts.len() >= 1 {
                            let first = parts[0];
                            // Split first part: container:health_output
                            if let Some(colon_pos) = first.find(':') {
                                let container = &first[..colon_pos];
                                let health_out = &first[colon_pos+1..];
                                let healthy = health_out.contains("OK");
                                let size_raw = if parts.len() >= 2 { parts[1].trim() } else { "?" };
                                let size = format_db_size(size_raw);
                                results.insert(container.to_string(), (healthy, size));
                            }
                        }
                    } else if let Some(name) = line.strip_prefix("RUNNING:") {
                        running_set.insert(name.trim().to_string());
                    }
                }
            }

            (alias, results, running_set)
        }
    }).collect();

    let vm_results: Vec<_> = futures::future::join_all(vm_futs).await;

    // Build results map: vm -> (db_results, running_set)
    let mut vm_map: HashMap<String, (HashMap<String, (bool, String)>, std::collections::HashSet<String>)> = HashMap::new();
    for (alias, results, running) in vm_results {
        vm_map.insert(alias, (results, running));
    }

    // Check backup declarations from consolidated JSON
    let backup_set = parse_backup_declared(ctx);

    // Assemble final results
    let db_health: Vec<DbHealthResult> = ctx.databases.iter().map(|db| {
        let (vm_results, running_set) = vm_map.get(&db.vm)
            .map(|(r, s)| (r.clone(), s.clone()))
            .unwrap_or_default();
        let (healthy, size) = vm_results.get(&db.container).cloned().unwrap_or((false, "?".into()));
        let running = running_set.contains(&db.container);
        let tcp_ok = tcp_results.get(&db.container).copied().unwrap_or(false);
        let dns = db.dns_access.split(':').next().unwrap_or("").to_string();
        let backup = backup_set.contains(&db.service);

        DbHealthResult {
            service: db.service.clone(),
            db_type: db.db_type.clone(),
            container: db.container.clone(),
            vm: db.vm.clone(),
            port: db.port,
            declared: true,
            running,
            healthy,
            size,
            tcp_ok,
            dns,
            backup,
        }
    }).collect();

    let healthy_count = db_health.iter().filter(|d| d.healthy).count();
    println!("  B2 Databases: {}/{} healthy in {}ms", healthy_count, db_health.len(), t.elapsed().as_millis());
    (db_health, t.elapsed().as_millis() as u64)
}

fn format_db_size(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed == "?" || trimmed.is_empty() {
        return "?".into();
    }
    // Redis dbsize output: "keys=123"
    if trimmed.starts_with("keys=") || trimmed.contains("keys") {
        return trimmed.to_string();
    }
    // Postgres/MariaDB: raw bytes
    if let Ok(bytes) = trimmed.parse::<u64>() {
        if bytes >= 1_073_741_824 {
            return format!("{:.1}GB", bytes as f64 / 1_073_741_824.0);
        } else if bytes >= 1_048_576 {
            return format!("{:.0}MB", bytes as f64 / 1_048_576.0);
        } else if bytes >= 1024 {
            return format!("{:.0}KB", bytes as f64 / 1024.0);
        } else {
            return format!("{}B", bytes);
        }
    }
    trimmed.to_string()
}

fn parse_backup_declared(ctx: &Context) -> std::collections::HashSet<String> {
    let mut set = std::collections::HashSet::new();
    if let Some(svcs) = ctx.consolidated["services"].as_object() {
        for (name, svc) in svcs {
            if svc["backup"].is_object() || svc["backup"].is_string() || svc["backup"].as_bool().unwrap_or(false) {
                set.insert(name.to_string());
            }
        }
    }
    set
}

fn parse_storage(ctx: &Context) -> Vec<StorageHealthResult> {
    let mut results = Vec::new();
    // Try topology.storage first, then top-level storage
    let storage = ctx.consolidated["topology"]["storage"].as_array()
        .or_else(|| ctx.consolidated["storage"].as_array());
    if let Some(buckets) = storage {
        for b in buckets {
            results.push(StorageHealthResult {
                name: b["name"].as_str().unwrap_or("?").to_string(),
                provider: b["provider"].as_str().unwrap_or("oci").to_string(),
                tier: b["tier"].as_str().unwrap_or("Standard").to_string(),
                accessible: true, // static from JSON, no live check
                size: b["size"].as_str().unwrap_or("—").to_string(),
                objects: b["objects"].as_str().or(b["object_count"].as_str()).unwrap_or("—").to_string(),
            });
        }
    }
    results
}
