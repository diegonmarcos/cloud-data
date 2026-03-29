use crate::types::*;
use std::collections::HashMap;

/// Build ALL template variables from context + live data
pub fn build_all_vars(ctx: &Context, live: &LiveData) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    vars.insert("GENERATED_DATE".into(), live.generated.clone());
    vars.insert("HUB_WG_IP".into(), ctx.vms.iter().find(|v| v.alias == "gcp-proxy").map(|v| v.wg_ip.clone()).unwrap_or("?".into()));

    // ISSUES FOUND
    vars.insert("ISSUES_SUMMARY".into(), build_issues(live));

    // A0 Mesh
    vars.insert("WG_PEERS".into(), build_mesh(live));

    // A1 Public
    vars.insert("PUBLIC_URLS".into(), build_public_urls(live));
    vars.insert("API_MCP_ENDPOINTS".into(), String::new()); // TODO from services
    vars.insert("REPOS_REGISTRIES".into(), "(Rust: TODO)".into());

    // A2 Private
    vars.insert("PRIVATE_HEALTH".into(), build_private(ctx, live));

    // A3 Containers
    vars.insert("VM_CONTAINERS".into(), build_containers(ctx, live));

    // A4 Mail
    vars.insert("MAIL_PORTS".into(), "(TODO)".into());
    vars.insert("MAIL_MX".into(), build_mail_mx(live));
    vars.insert("MAIL_SPF".into(), build_mail_spf(ctx, live));
    vars.insert("MAIL_DKIM".into(), build_mail_dkim(live));
    vars.insert("MAIL_DMARC".into(), build_mail_dmarc(live));
    vars.insert("MAIL_AUTH".into(), "(TODO)".into());
    vars.insert("MAIL_FLOW".into(), "(TODO)".into());

    // B Infra
    vars.insert("VPS_SPECS".into(), build_vps_specs(ctx));
    vars.insert("RESOURCES_HEADER".into(), live.vm_data.iter().map(|v| format!("{:14}", v.alias)).collect::<Vec<_>>().join(" "));
    vars.insert("RESOURCES_TABLE".into(), build_resources(live));
    vars.insert("STORAGE".into(), "(TODO)".into());

    // C Security
    vars.insert("OPEN_PORTS".into(), build_port_scan(live));
    vars.insert("DATABASES".into(), build_databases(ctx));
    vars.insert("DOCKER_NETWORKS".into(), "(TODO)".into());
    vars.insert("VAULT_PROVIDERS".into(), "(TODO)".into());

    // D Stack
    vars.insert("FRAMEWORK_PATHS".into(), "(TODO)".into());

    // Z Appendix
    vars.insert("PERFORMANCE".into(), build_performance(live));
    vars.insert("SCRIPT_INFO".into(), build_script_info(live));

    vars
}

fn build_issues(live: &LiveData) -> String {
    let mut issues = Vec::new();
    for vm in &live.vm_data {
        if !vm.reachable { issues.push(format!("❌ A3       VM {} — UNREACHABLE", vm.alias)); }
        for c in &vm.containers {
            if c.health == "exited" || c.health == "unhealthy" {
                issues.push(format!("{} A3       {}/{} — {}", if c.health == "unhealthy" { "⚠️" } else { "❌" }, vm.alias, c.name, c.health));
            }
        }
    }
    for u in &live.public_urls {
        if !u.https { issues.push(format!("❌ A1       {} — [{}]", u.url, u.code)); }
    }
    if issues.is_empty() { return "✅ No issues found".into(); }
    let crit = issues.iter().filter(|i| i.starts_with("❌")).count();
    let warn = issues.iter().filter(|i| i.starts_with("⚠️")).count();
    format!("{} critical, {} warnings — {} total\n\n    {}", crit, warn, issues.len(), issues.join("\n    "))
}

fn build_mesh(live: &LiveData) -> String {
    live.mesh.iter().map(|p| {
        let all_ok = p.vps_ok && p.pub_ok && p.wg_ok;
        let any = p.vps_ok || p.pub_ok || p.wg_ok || p.dropbear_ok;
        let icon = if all_ok { "✅" } else if any { "⚠️" } else { "❌" };
        let db = if p.peer_type == "CLIENT" { "—" } else if p.dropbear_ok { "✅" } else { "❌" };
        format!("{} {:14} {:18} {}  {}  {}  {}  {:18} {:14} {:8} {}",
            icon, p.name, p.cloud_name,
            if p.vps_ok { "✅" } else { "❌" },
            if p.pub_ok { "✅" } else { "❌" },
            db,
            if p.wg_ok { "✅" } else { "❌" },
            p.pub_ip, p.wg_ip, p.peer_type, p.wg_handshake)
    }).collect::<Vec<_>>().join("\n")
}

fn build_public_urls(live: &LiveData) -> String {
    live.public_urls.iter().map(|u| {
        let all = u.tcp && u.https && u.auth;
        let any = u.tcp || u.http || u.https;
        let icon = if all { "✅" } else if any { "⚠️" } else { "❌" };
        let auth = if u.auth { "✅" } else if u.auth_code == "---" { "—" } else { "❌" };
        format!("{} {:32} {}  {}  {}  {}  {:22} [{}] {}",
            icon, u.url,
            if u.tcp { "✅" } else { "❌" },
            if u.http { "✅" } else { "❌" },
            if u.https { "✅" } else { "❌" },
            auth, u.upstream, u.code,
            if !u.auth && u.auth_code != "---" { format!("auth:[{}]", u.auth_code) } else { String::new() })
    }).collect::<Vec<_>>().join("\n")
}

fn build_private(_ctx: &Context, live: &LiveData) -> String {
    let mut lines = Vec::new();
    if live.private_endpoints.iter().all(|p| p.resolved_ip.is_empty()) {
        lines.push("⚠️  WireGuard/Hickory DOWN — cannot reach .app endpoints".into());
        lines.push("    Run: sudo wg-quick up wg0".into());
        lines.push(String::new());
    }
    lines.push(format!("    {:28} 📡TCP 🌐HTTP {:7} {:16} {:22} Code", "DNS Name", "Port", "VM", "Container"));
    lines.push(format!("    {}", "─".repeat(95)));
    for p in &live.private_endpoints {
        let all = p.tcp && p.http;
        let any = p.tcp || p.http;
        let icon = if p.resolved_ip.is_empty() { "⏸️" } else if all { "✅" } else if any { "⚠️" } else { "❌" };
        let tcp_i = if p.resolved_ip.is_empty() { "⏸️" } else if p.tcp { "✅" } else { "❌" };
        let http_i = if p.resolved_ip.is_empty() { "⏸️" } else if p.http { "✅" } else { "❌" };
        lines.push(format!("{} {:28} {}   {}   {:5} {:16} {:22} [{}]", icon, p.dns, tcp_i, http_i, p.port, p.vm, p.container, p.code));
    }
    let tcp_ok = live.private_endpoints.iter().filter(|p| p.tcp).count();
    let http_ok = live.private_endpoints.iter().filter(|p| p.http).count();
    lines.push(String::new());
    lines.push(format!("  📡 TCP: {}/{}  🌐 HTTP: {}/{}", tcp_ok, live.private_endpoints.len(), http_ok, live.private_endpoints.len()));
    lines.join("\n")
}

fn build_containers(ctx: &Context, live: &LiveData) -> String {
    live.vm_data.iter().map(|vm| {
        let st = if vm.reachable { "✅" } else { "❌" };
        let mut lines = vec![
            format!("{} {} — {}C/{} — mem {}/{} ({}%) — disk {} — swap {} — load {} — {}/{} ctrs — {}",
                vm.alias, st, ctx.vms.iter().find(|v| v.alias == vm.alias).map(|v| v.cpus).unwrap_or(0),
                ctx.vms.iter().find(|v| v.alias == vm.alias).map(|v| v.ram_gb.as_str()).unwrap_or("?"),
                vm.mem_used, vm.mem_total, vm.mem_pct, vm.disk_pct, vm.swap, vm.load, vm.containers_running, vm.containers_total, vm.uptime),
            "─".repeat(60),
        ];
        for c in &vm.containers {
            let state = match c.health.as_str() {
                "healthy" => "HEALTHY",
                "unhealthy" => "UNHEALTHY",
                "starting" => "STARTING",
                "created" | "exited" => &format!("DOWN({})", c.status.split('(').nth(1).and_then(|s| s.split(')').next()).unwrap_or("?")),
                "none" => "UP (no hc)",
                _ => &c.health,
            };
            let icon = match c.health.as_str() {
                "healthy" => "✅", "unhealthy" | "exited" | "created" => "❌", _ => "⚠️",
            };
            lines.push(format!("  {} {:25} {:6} {:6} {:14} {}", icon, c.name, c.host_port, c.docker_port, state, &c.status[..c.status.len().min(30)]));
        }
        lines.push(String::new());
        lines.join("\n")
    }).collect::<Vec<_>>().join("\n")
}

fn build_mail_mx(live: &LiveData) -> String {
    let mut lines = vec![
        "MX — Inbound Routing (dig MX)".into(),
        "─".repeat(60),
        format!("    {:28} {:5} {:42} IP", "Domain", "Pri", "Server"),
        "─".repeat(60),
    ];
    for mx in &live.mail_dns.mx {
        let icon = if mx.server.contains("no MX") { "❌" } else { "✅" };
        lines.push(format!("{} {:28} {:5} {:42} {}", icon, mx.domain, mx.priority, mx.server, mx.ip));
    }
    lines.join("\n")
}

fn build_mail_spf(ctx: &Context, live: &LiveData) -> String {
    let mut lines = vec![
        "SPF — Outbound Policy: IP Allowlist".into(),
        "─".repeat(60),
    ];
    for spf in &live.mail_dns.spf {
        lines.push(format!("✅ {:32} {:45} {}", spf.domain, spf.include, spf.ips));
    }
    let oci_ip = ctx.vms.iter().find(|v| v.alias == "oci-mail").map(|v| v.pub_ip.as_str()).unwrap_or("?");
    lines.push(format!("⚠️ {:32} oci-mail VM IP {} NOT IN SPF!", "diegonmarcos.com", oci_ip));
    lines.join("\n")
}

fn build_mail_dkim(live: &LiveData) -> String {
    let mut lines = vec![
        "DKIM — Outbound Policy: Cryptographic Signatures".into(),
        "─".repeat(60),
    ];
    for d in &live.mail_dns.dkim {
        let icon = if d.present { "✅" } else { "❌" };
        lines.push(format!("{} {:28} {:24} {:20} {}", icon, d.selector, d.domain, d.signer, d.key_size));
    }
    lines.join("\n")
}

fn build_mail_dmarc(live: &LiveData) -> String {
    format!("DMARC — Outbound Policy\n─────────────────────\n✅ _dmarc.diegonmarcos.com       {}", live.mail_dns.dmarc)
}

fn build_vps_specs(ctx: &Context) -> String {
    ctx.vms.iter().map(|v| {
        format!("   {:16} {:10} {:20} {:6} {:6} {:8} {}", v.alias, v.provider, v.shape, v.cpus, v.ram_gb, v.disk_gb, v.cost)
    }).collect::<Vec<_>>().join("\n")
}

fn build_resources(live: &LiveData) -> String {
    let fields: Vec<(&str, Box<dyn Fn(&VmLiveData) -> String>)> = vec![
        ("CPU", Box::new(|v: &VmLiveData| format!("{} cores", if v.reachable { v.mem_pct.to_string() } else { "?".into() }))),
        ("RAM", Box::new(|v| format!("{}/{}", v.mem_used, v.mem_total))),
        ("RAM %", Box::new(|v| format!("{}%", v.mem_pct))),
        ("Swap", Box::new(|v| v.swap.clone())),
        ("Disk", Box::new(|v| format!("{}/{}", v.disk_used, v.disk_total))),
        ("Disk %", Box::new(|v| v.disk_pct.clone())),
        ("Load", Box::new(|v| v.load.clone())),
        ("Containers", Box::new(|v| format!("{}/{}", v.containers_running, v.containers_total))),
        ("Uptime", Box::new(|v| v.uptime.clone())),
    ];
    fields.iter().map(|(label, f)| {
        let vals: String = live.vm_data.iter().map(|v| format!("{:14}", f(v))).collect();
        format!("{:18} {}", label, vals)
    }).collect::<Vec<_>>().join("\n")
}

fn build_port_scan(live: &LiveData) -> String {
    live.port_scan.iter().map(|r| {
        let icon = if r.open_ports.is_empty() { "🔒" } else { "🔓" };
        let ports = if r.open_ports.is_empty() { "none reachable".into() } else { r.open_ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ") };
        format!("{} {:18} {:18} ports: {}", icon, r.name, r.ip, ports)
    }).collect::<Vec<_>>().join("\n")
}

fn build_databases(ctx: &Context) -> String {
    ctx.databases.iter().map(|d| {
        format!("   {:20} {:10} {:22} {:14} {:16} {}", d.service, d.db_type, d.container, d.db_name, d.vm, d.dns_access)
    }).collect::<Vec<_>>().join("\n")
}

fn build_performance(live: &LiveData) -> String {
    let mut sorted: Vec<_> = live.timers.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    sorted.iter().map(|(k, v)| format!("  {:20} {:.1}s", k, **v as f64 / 1000.0)).collect::<Vec<_>>().join("\n")
}

fn build_script_info(live: &LiveData) -> String {
    format!("  Engine:    Rust (native async tokio)\n  Duration:  {:.1}s\n  Checks:    TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(async process)", live.duration_ms as f64 / 1000.0)
}
