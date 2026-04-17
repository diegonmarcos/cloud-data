use crate::types::*;
use std::fmt::Write;

// ── Color constants ─────────────────────────────────────────────────
const C_OK: &str = "#00d68f";
const C_WARN: &str = "#ffaa00";
const C_CRIT: &str = "#ff3d71";
const C_DIM: &str = "#8899aa";
const BG_BODY: &str = "#1a1a2e";
const BG_CARD: &str = "#16213e";
const BG_HEAD: &str = "#0f3460";
const BG_BAR: &str = "#2a2a4e";
const C_TEXT: &str = "#e0e0e0";
const FONT: &str = "'Courier New',Consolas,monospace";

fn pct_color(pct: u32) -> &'static str {
    if pct > 90 { C_CRIT } else if pct > 75 { C_WARN } else { C_OK }
}

fn progress_bar(pct: u32) -> String {
    let color = pct_color(pct);
    format!(
        r#"<div style="display:inline-block;background:{BG_BAR};border-radius:4px;height:14px;width:80px;vertical-align:middle;"><div style="background:{color};border-radius:4px;height:14px;width:{pct}%;"></div></div> <span style="color:{color};font-size:12px;">{pct}%</span>"#
    )
}

fn status_badge(status: &VmStatus) -> String {
    let (color, label) = match status {
        VmStatus::Healthy => (C_OK, "HEALTHY"),
        VmStatus::Warning => (C_WARN, "WARNING"),
        VmStatus::Critical => (C_CRIT, "CRITICAL"),
        VmStatus::Unknown => (C_DIM, "UNKNOWN"),
    };
    format!(
        r#"<span style="display:inline-block;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:bold;background:{color};color:{BG_BODY};">{label}</span>"#
    )
}

fn code_badge(code: u16) -> String {
    let color = match code {
        200..=399 => C_OK,
        400..=499 => C_WARN,
        _ => C_CRIT,
    };
    format!(
        r#"<span style="display:inline-block;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:bold;background:{color};color:{BG_BODY};font-family:{FONT};">{code}</span>"#
    )
}

fn label_badge(label: &str, color: &str) -> String {
    format!(
        r#"<span style="display:inline-block;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:bold;background:{color};color:{BG_BODY};font-family:{FONT};">{label}</span>"#
    )
}

fn render_latency_bar(latency_ms: u64) -> String {
    let color = if latency_ms < 100 { C_OK } else if latency_ms < 500 { C_WARN } else { C_CRIT };
    let pct = std::cmp::min(latency_ms * 100 / 500, 100);
    format!(
        r#"<div style="display:inline-block;background:{BG_BAR};border-radius:4px;height:12px;width:60px;vertical-align:middle;"><div style="background:{color};border-radius:4px;height:12px;width:{pct}%;"></div></div> <span style="color:{color};font-size:11px;">{latency_ms}ms</span>"#
    )
}

// ── Table helpers ───────────────────────────────────────────────────

fn section_title(h: &mut String, letter: &str, title: &str) {
    write!(h, r#"<tr><td style="padding:20px 8px 4px 8px;">
<table width="100%" cellpadding="0" cellspacing="0">
<tr><td style="padding:10px 16px;border-bottom:2px solid {C_OK};font-family:{FONT};">
<span style="color:{C_OK};font-size:16px;font-weight:bold;letter-spacing:2px;">{letter}</span>
<span style="color:{C_TEXT};font-size:16px;font-weight:bold;letter-spacing:1px;margin-left:8px;">{title}</span>
</td></tr></table></td></tr>"#).unwrap();
}

fn section_start(h: &mut String, title: &str, cols: u8) {
    write!(h, r#"<tr><td style="padding:8px;">
<table width="100%" cellpadding="0" cellspacing="0" style="background:{BG_CARD};border-radius:6px;">
<tr><td colspan="{cols}" style="padding:12px 16px;background:{BG_HEAD};border-radius:6px 6px 0 0;font-size:14px;font-weight:bold;color:{C_TEXT};font-family:{FONT};">{title}</td></tr>"#).unwrap();
}

fn section_end(h: &mut String) {
    h.push_str("</table></td></tr>\n");
}

fn th(label: &str, align: &str) -> String {
    format!(
        r#"<th style="text-align:{align};color:{C_DIM};font-size:10px;padding:6px 8px;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{label}</th>"#
    )
}

fn td(val: &str, color: &str, size: &str, align: &str) -> String {
    format!(
        r#"<td style="padding:3px 8px;color:{color};font-size:{size};text-align:{align};border-bottom:1px solid rgba(15,52,96,0.3);font-family:{FONT};">{val}</td>"#
    )
}

// ── Main render function ────────────────────────────────────────────

pub fn render(data: &ReportData) -> String {
    let mut h = String::with_capacity(96 * 1024);

    // HTML boilerplate
    write!(h, r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><style>
body{{margin:0;padding:0;background:{BG_BODY}}}
td,th{{font-family:{FONT}}}
</style></head>
<body style="margin:0;padding:0;background:{BG_BODY};">
<center>
<table width="100%" cellpadding="0" cellspacing="0" style="background:{BG_BODY};"><tr><td align="center">
<table width="700" cellpadding="0" cellspacing="0" style="max-width:700px;width:100%;">
<tr><td style="background:{BG_HEAD};padding:20px 24px;text-align:center;border-radius:8px 8px 0 0;">
<h1 style="margin:0;font-size:20px;color:{C_TEXT};font-family:{FONT};letter-spacing:1px;">C3 Daily Ops Report</h1>
<p style="margin:4px 0 0;color:{C_DIM};font-size:12px;font-family:{FONT};">{date} &mdash; Generated at {time}</p>
</td></tr>"#,
        date = data.date, time = data.time
    ).unwrap();

    // Executive Summary (NEW)
    render_exec_summary(&mut h, data);

    // Fleet Dashboard (enhanced with stat cards + swap)
    render_fleet_dashboard(&mut h, data);

    // ═══════════════════════════════════════════════════════════
    // A) CONTAINERS
    // ═══════════════════════════════════════════════════════════
    section_title(&mut h, "A", "CONTAINERS");
    render_container_inventory(&mut h, data);
    render_container_resources(&mut h, data);
    render_log_errors(&mut h, data);
    render_container_drift(&mut h, data);
    render_restarts(&mut h, data);
    render_docker_disk(&mut h, data);

    // ═══════════════════════════════════════════════════════════
    // B) DATABASES
    // ═══════════════════════════════════════════════════════════
    section_title(&mut h, "B", "DATABASES");
    render_database_report(&mut h, data);
    render_object_storage(&mut h, data);
    render_runtime_volumes(&mut h, data);
    render_drift(&mut h, data);
    render_backup_status(&mut h, data);

    // ═══════════════════════════════════════════════════════════
    // C) SECURITY
    // ═══════════════════════════════════════════════════════════
    section_title(&mut h, "C", "SECURITY");
    render_security(&mut h, data);
    render_oom_kills(&mut h, data);
    render_certs(&mut h, data);
    render_dns(&mut h, data);
    render_wireguard(&mut h, data);
    render_failed_units(&mut h, data);

    // ═══════════════════════════════════════════════════════════
    // D) WORKFLOWS
    // ═══════════════════════════════════════════════════════════
    section_title(&mut h, "D", "WORKFLOWS");
    render_dags(&mut h, data);
    render_gha(&mut h, data);
    render_ghcr(&mut h, data);
    render_repos(&mut h, data);

    // ═══════════════════════════════════════════════════════════
    // E) SERVICES
    // ═══════════════════════════════════════════════════════════
    section_title(&mut h, "E", "SERVICES");
    render_services_all_unified(&mut h, data);
    render_services_mcps(&mut h, data);

    // ═══════════════════════════════════════════════════════════
    // F) OTHERS
    // ═══════════════════════════════════════════════════════════
    section_title(&mut h, "F", "OTHERS");
    render_endpoints(&mut h, data);
    render_mail(&mut h, data);

    // ── Footer ──────────────────────────────────────────────────
    write!(h, r#"<tr><td style="text-align:center;padding:16px;color:{C_DIM};font-size:11px;font-family:{FONT};">
C3 Daily Ops Report &mdash; {date} {time}<br>
<a href="http://10.0.0.3:8070" style="color:{C_OK};">Dagu Dashboard</a>
</td></tr>
</table>
</td></tr></table>
</center>
</body>
</html>"#,
        date = data.date, time = data.time
    ).unwrap();

    h
}

// ── Executive Summary ───────────────────────────────────────────────

fn render_exec_summary(h: &mut String, data: &ReportData) {
    let es = &data.exec_summary;

    // Traffic light cards: 3-column layout
    write!(h, r#"<tr><td style="padding:8px;">
<table width="100%" cellpadding="0" cellspacing="0" style="background:{BG_CARD};border-radius:6px;">
<tr><td colspan="3" style="padding:12px 16px;background:{BG_HEAD};border-radius:6px 6px 0 0;font-size:14px;font-weight:bold;color:{C_TEXT};font-family:{FONT};">Executive Summary</td></tr>
<tr>"#).unwrap();

    // Critical card
    write!(h, r#"<td style="padding:8px;width:33%;">
<div style="border-left:4px solid {C_CRIT};background:{BG_CARD};padding:8px 12px;border-radius:0 4px 4px 0;">
<span style="font-size:20px;font-weight:bold;color:{C_CRIT};font-family:{FONT};">{}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">CRITICAL</span>
</div></td>"#, es.critical).unwrap();

    // Warning card
    write!(h, r#"<td style="padding:8px;width:33%;">
<div style="border-left:4px solid {C_WARN};background:{BG_CARD};padding:8px 12px;border-radius:0 4px 4px 0;">
<span style="font-size:20px;font-weight:bold;color:{C_WARN};font-family:{FONT};">{}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">WARNING</span>
</div></td>"#, es.warnings).unwrap();

    // OK card
    write!(h, r#"<td style="padding:8px;width:34%;">
<div style="border-left:4px solid {C_OK};background:{BG_CARD};padding:8px 12px;border-radius:0 4px 4px 0;">
<span style="font-size:20px;font-weight:bold;color:{C_OK};font-family:{FONT};">{}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">OK</span>
</div></td>"#, es.ok).unwrap();

    h.push_str("</tr>\n");

    // Top issues
    if !es.top_issues.is_empty() {
        write!(h, r#"<tr><td colspan="3" style="padding:4px 16px 8px;">"#).unwrap();
        for issue in &es.top_issues {
            let color = if issue.severity == "CRIT" { C_CRIT } else { C_WARN };
            let badge = label_badge(&issue.severity, color);
            write!(h, r#"<div style="padding:3px 0;font-family:{FONT};font-size:11px;color:{C_TEXT};">{badge} {msg}</div>"#,
                msg = issue.message).unwrap();
        }
        h.push_str("</td></tr>\n");
    }

    section_end(h);
}

// ── Stat Cards ──────────────────────────────────────────────────────

fn render_stat_cards(h: &mut String, data: &ReportData) {
    let vm_count = data.vms.len();
    let ep_ok = data.endpoints.iter().filter(|e| (200..=399).contains(&e.status_code)).count();
    let certs_ok = data.certs.iter().filter(|c| c.days_left >= 7).count();

    write!(h, r#"<tr>
<td style="padding:6px 4px;width:25%;"><div style="border-left:4px solid {C_OK};background:{BG_CARD};padding:6px 10px;border-radius:0 4px 4px 0;">
<span style="font-size:18px;font-weight:bold;color:{C_TEXT};font-family:{FONT};">{vm_count}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">VMs</span>
</div></td>
<td style="padding:6px 4px;width:25%;"><div style="border-left:4px solid {C_OK};background:{BG_CARD};padding:6px 10px;border-radius:0 4px 4px 0;">
<span style="font-size:18px;font-weight:bold;color:{C_TEXT};font-family:{FONT};">{run}/{tot}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">Containers</span>
</div></td>
<td style="padding:6px 4px;width:25%;"><div style="border-left:4px solid {C_OK};background:{BG_CARD};padding:6px 10px;border-radius:0 4px 4px 0;">
<span style="font-size:18px;font-weight:bold;color:{C_TEXT};font-family:{FONT};">{ep_ok}/{ep_tot}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">Endpoints</span>
</div></td>
<td style="padding:6px 4px;width:25%;"><div style="border-left:4px solid {C_OK};background:{BG_CARD};padding:6px 10px;border-radius:0 4px 4px 0;">
<span style="font-size:18px;font-weight:bold;color:{C_TEXT};font-family:{FONT};">{certs_ok}/{certs_tot}</span>
<br><span style="font-size:10px;color:{C_DIM};font-family:{FONT};">Certs</span>
</div></td>
</tr>"#,
        run = data.fleet_running, tot = data.fleet_total,
        ep_tot = data.endpoints.len(), certs_tot = data.certs.len(),
    ).unwrap();
}

fn render_fleet_dashboard(h: &mut String, data: &ReportData) {
    section_start(h, "Fleet Dashboard", 8);

    // Stat cards row
    render_stat_cards(h, data);

    // VM table header (8 columns with Swap)
    h.push_str("<tr>");
    for label in &["VM", "Status", "Uptime", "Load", "Mem", "Disk", "Swap", "Ctrs"] {
        write!(h, r#"<th style="text-align:left;color:{C_DIM};font-size:11px;padding:8px;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{label}</th>"#).unwrap();
    }
    h.push_str("</tr>\n");

    for vm in &data.vms {
        let badge = status_badge(&vm.status);
        let load1 = vm.load.split_whitespace().next().unwrap_or("?");
        let mbar = progress_bar(vm.mem_pct);
        let dbar = progress_bar(vm.disk_pct);
        let sbar = if vm.swap == "N/A" || vm.swap.is_empty() {
            format!(r#"<span style="color:{C_DIM};font-size:11px;font-family:{FONT};">N/A</span>"#)
        } else {
            progress_bar(vm.swap_pct)
        };
        write!(h, r#"<tr>
<td style="padding:6px 8px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid rgba(15,52,96,0.5);font-family:{FONT};">{name}</td>
<td style="padding:6px 8px;border-bottom:1px solid rgba(15,52,96,0.5);">{badge}</td>
<td style="padding:6px 8px;color:{C_DIM};font-size:11px;border-bottom:1px solid rgba(15,52,96,0.5);font-family:{FONT};">{uptime}</td>
<td style="padding:6px 8px;color:{C_TEXT};font-size:12px;border-bottom:1px solid rgba(15,52,96,0.5);font-family:{FONT};">{load1}</td>
<td style="padding:6px 8px;border-bottom:1px solid rgba(15,52,96,0.5);">{mbar}</td>
<td style="padding:6px 8px;border-bottom:1px solid rgba(15,52,96,0.5);">{dbar}</td>
<td style="padding:6px 8px;border-bottom:1px solid rgba(15,52,96,0.5);">{sbar}</td>
<td style="padding:6px 8px;color:{C_TEXT};font-size:12px;border-bottom:1px solid rgba(15,52,96,0.5);font-family:{FONT};">{run}/{tot}</td>
</tr>"#,
            name = vm.name, uptime = vm.uptime,
            run = vm.containers_running, tot = vm.containers_total,
        ).unwrap();
    }
    section_end(h);
}

fn render_container_inventory(h: &mut String, data: &ReportData) {
    section_start(h, "Full Container Inventory", 4);
    for vm in &data.vms {
        if vm.container_list.is_empty() { continue; }
        write!(h, r#"<tr><td colspan="4" style="padding:8px 8px 2px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{} ({}/{} running)</td></tr>"#,
            vm.name, vm.containers_running, vm.containers_total).unwrap();
        h.push_str("<tr>");
        for label in &["Name", "Image", "Status", "Uptime"] {
            write!(h, r#"<th style="text-align:left;color:{C_DIM};font-size:10px;padding:4px 8px;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{label}</th>"#).unwrap();
        }
        h.push_str("</tr>\n");
        for c in &vm.container_list {
            let scolor = if c.status.contains("Exited") || c.status.contains("dead") || c.status.contains("Created") {
                C_CRIT
            } else if c.status.contains("unhealthy") || c.status.contains("restarting") {
                C_WARN
            } else {
                C_OK
            };
            let short_image = if c.image.len() > 40 {
                format!("...{}", &c.image[c.image.len()-37..])
            } else {
                c.image.clone()
            };
            write!(h, "<tr>{}{}{}{}</tr>\n",
                td(&c.name, C_TEXT, "11px", "left"),
                td(&short_image, C_DIM, "10px", "left"),
                td(&c.status, scolor, "11px", "left"),
                td(&c.running_for, C_DIM, "11px", "left"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_database_report(h: &mut String, data: &ReportData) {
    let enabled: Vec<_> = data.databases.iter().filter(|d| d.enabled).collect();
    if enabled.is_empty() { return; }

    section_start(h, "Database Report", 5);
    h.push_str("<tr>");
    for (label, align) in &[("Service","left"),("Type","left"),("Container","left"),("Size","right"),("VM","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for db in &enabled {
        let type_color = match db.db_type.as_str() {
            "postgres" => "#4169E1",
            "mariadb" => "#C0765A",
            "sqlite" => "#57A6D6",
            "redis" => "#DC382D",
            "s3" => "#FF9900",
            "loki" => "#F2994A",
            "mimir" => "#E07C4F",
            "tempo" => "#D4633F",
            "grafana" => "#F46800",
            _ => C_DIM,
        };
        let size = data.vms.iter()
            .filter(|v| v.ip == db.wg_ip)
            .flat_map(|v| v.db_sizes.iter())
            .find(|(svc, _)| svc == &db.service)
            .map(|(_, s)| s.as_str())
            .unwrap_or("N/A");

        write!(h, "<tr>{}{}{}{}{}</tr>\n",
            td(&db.service, C_TEXT, "11px", "left"),
            td(&db.db_type, type_color, "11px", "left"),
            td(&db.container, C_DIM, "10px", "left"),
            td(size, C_TEXT, "11px", "right"),
            td(&db.vm_alias, C_DIM, "11px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_object_storage(h: &mut String, data: &ReportData) {
    // ── Cloud Provider Buckets (Terraform-declared) ─────────────────
    let has_buckets = !data.cloud_buckets.is_empty();
    // ── Docker S3/MinIO (from databases.json) ───────────────────────
    let s3_declared: Vec<_> = data.databases.iter()
        .filter(|d| d.enabled && d.db_type == "s3")
        .collect();

    if !has_buckets && s3_declared.is_empty() { return; }

    section_start(h, "Object &amp; File Storage", 5);

    // ── Cloud Provider Buckets (sorted by tier) ───────────────────
    if has_buckets {
        write!(h, r#"<tr><td colspan="5" style="padding:8px 8px 2px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">Cloud Provider Buckets ({} via Terraform)</td></tr>"#,
            data.cloud_buckets.len()).unwrap();
        h.push_str("<tr>");
        for (label, align) in &[("Bucket","left"),("Provider","left"),("Tier","left"),("Size","right"),("Retrieval","left")] {
            h.push_str(&th(label, align));
        }
        h.push_str("</tr>\n");

        // Sort by tier priority: Standard first, then Archive/InfrequentAccess/DeepArchive
        let tier_order = |t: &str| -> u8 {
            match t {
                "Standard" => 0,
                "InfrequentAccess" => 1,
                "Archive" => 2,
                "DeepArchive" => 3,
                _ => 4,
            }
        };
        let mut sorted_buckets: Vec<_> = data.cloud_buckets.iter().collect();
        sorted_buckets.sort_by_key(|b| tier_order(&b.tier));

        for bucket in &sorted_buckets {
            let (tier_color, retrieval) = match bucket.tier.as_str() {
                "Standard" => (C_OK, "Instant"),
                "InfrequentAccess" => ("#57A6D6", "Instant (higher cost)"),
                "Archive" => (C_WARN, "3-5 hours"),
                "DeepArchive" => (C_DIM, "12-48 hours"),
                other => (C_DIM, other),
            };
            let size_str = if bucket.size_bytes > 0 { human_size(bucket.size_bytes) } else { "—".into() };
            write!(h, "<tr>{}{}{}{}{}</tr>\n",
                td(&bucket.name, "#FF9900", "11px", "left"),
                td(&bucket.provider.to_uppercase(), C_DIM, "11px", "left"),
                td(&bucket.tier, tier_color, "11px", "left"),
                td(&size_str, C_TEXT, "11px", "right"),
                td(retrieval, tier_color, "10px", "left"),
            ).unwrap();
        }
    }

    // ── Docker S3/MinIO Containers ──────────────────────────────────
    if !s3_declared.is_empty() {
        write!(h, r#"<tr><td colspan="5" style="padding:8px 8px 2px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">Docker S3 / MinIO</td></tr>"#).unwrap();
        h.push_str("<tr>");
        for (label, align) in &[("Service","left"),("Container","left"),("Volume","left"),("Size","right")] {
            h.push_str(&th(label, align));
        }
        h.push_str("</tr>\n");

        for db in &s3_declared {
            let vol = data.vms.iter()
                .filter(|v| v.ip == db.wg_ip)
                .flat_map(|v| v.runtime_volumes.iter())
                .find(|v| v.container == db.container);

            let size = vol.map(|v| if v.size.is_empty() { "?" } else { &*v.size }).unwrap_or("N/A");
            let vol_name = vol.map(|v| v.name.as_str()).unwrap_or("—");

            write!(h, "<tr>{}{}{}{}</tr>\n",
                td(&db.service, C_TEXT, "11px", "left"),
                td(&db.container, C_DIM, "10px", "left"),
                td(vol_name, C_DIM, "10px", "left"),
                td(size, "#FF9900", "11px", "right"),
            ).unwrap();
        }
    }

    section_end(h);
}

fn render_backup_status(h: &mut String, data: &ReportData) {
    let has_backups = data.vms.iter().any(|v| !v.backups.is_empty());
    if !has_backups { return; }

    section_start(h, "Backup Status", 4);
    h.push_str("<tr>");
    for (label, align) in &[("VM","left"),("File","left"),("Size","right"),("Age","right")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    let now = chrono::Utc::now().timestamp() as f64;
    for vm in &data.vms {
        for bk in &vm.backups {
            let size = human_size(bk.size_bytes);
            let (age_str, age_color) = if bk.epoch > 0.0 {
                let hours = ((now - bk.epoch) / 3600.0) as u64;
                if hours < 24 { (format!("{}h ago", hours), C_OK) }
                else if hours < 72 { (format!("{}d", hours / 24), C_WARN) }
                else { (format!("{}d", hours / 24), C_CRIT) }
            } else {
                ("unknown".into(), C_DIM)
            };
            write!(h, "<tr>{}{}{}{}</tr>\n",
                td(&vm.name, C_TEXT, "11px", "left"),
                td(&bk.file, C_TEXT, "11px", "left"),
                td(&size, C_DIM, "11px", "right"),
                td(&age_str, age_color, "11px", "right"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_endpoints(h: &mut String, data: &ReportData) {
    if data.endpoints.is_empty() { return; }
    section_start(h, "Service Endpoints", 4);
    h.push_str("<tr>");
    for (label, align) in &[("Service","left"),("URL","left"),("Status","center"),("Latency","center")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for ep in &data.endpoints {
        let badge = code_badge(ep.status_code);
        let latency = render_latency_bar(ep.latency_ms);
        write!(h, r#"<tr>{}{}<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td><td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{latency}</td></tr>
"#,
            td(&ep.service, C_TEXT, "11px", "left"),
            td(&ep.url, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_certs(h: &mut String, data: &ReportData) {
    if data.certs.is_empty() { return; }
    section_start(h, "Certificate Expiry", 3);
    h.push_str("<tr>");
    for (label, align) in &[("Domain","left"),("Days Left","right"),("Expires","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for cert in &data.certs {
        let color = if cert.days_left < 0 { C_CRIT }
            else if cert.days_left < 7 { C_CRIT }
            else if cert.days_left < 30 { C_WARN }
            else { C_OK };
        write!(h, "<tr>{}{}{}</tr>\n",
            td(&cert.domain, C_TEXT, "11px", "left"),
            td(&format!("{}d", cert.days_left), color, "11px", "right"),
            td(&cert.expiry, C_DIM, "11px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_dns(h: &mut String, data: &ReportData) {
    if data.dns.is_empty() { return; }
    section_start(h, "DNS Status (diegonmarcos.com)", 3);
    h.push_str("<tr>");
    for (label, align) in &[("Record","left"),("Status","center"),("Value","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for rec in &data.dns {
        let (status, color) = if rec.value.is_empty() {
            ("FAIL", C_CRIT)
        } else {
            ("PASS", C_OK)
        };
        let short_val = if rec.value.len() > 60 {
            format!("{}...", &rec.value[..57])
        } else if rec.value.is_empty() {
            "not found".into()
        } else {
            rec.value.clone()
        };
        let badge = label_badge(status, color);
        write!(h, r#"<tr>
{rec_td}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
{val_td}
</tr>
"#,
            rec_td = td(&rec.record_type, C_TEXT, "11px", "left"),
            val_td = td(&short_val, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_mail(h: &mut String, data: &ReportData) {
    let mail_vm = data.vms.iter().find(|v| v.mail_queue.is_some());
    let Some(vm) = mail_vm else { return; };

    let mq = vm.mail_queue.unwrap_or(0);
    let md = vm.mail_delivered.unwrap_or(0);
    let mf = vm.mail_failed.unwrap_or(0);

    section_start(h, "Mail Queue &amp; Stats (24h)", 2);
    let mq_color = if mq > 20 { C_CRIT } else if mq > 5 { C_WARN } else { C_OK };
    let mf_color = if mf > 5 { C_CRIT } else if mf > 0 { C_WARN } else { C_OK };
    for (label, val, color) in &[("Queued", mq, mq_color), ("Delivered (24h)", md, C_OK), ("Failed (24h)", mf, mf_color)] {
        write!(h, "<tr>{}{}</tr>\n",
            td(label, C_DIM, "12px", "left"),
            td(&val.to_string(), color, "12px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_gha(h: &mut String, data: &ReportData) {
    if data.gha_runs.is_empty() { return; }
    section_start(h, "GHA Workflows (Last 10)", 3);
    h.push_str("<tr>");
    for (label, align) in &[("Workflow","left"),("Status","center"),("Time","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for run in &data.gha_runs {
        let (label, color) = match run.conclusion.as_str() {
            "success" => ("SUCCESS", C_OK),
            "failure" => ("FAILED", C_CRIT),
            "cancelled" => ("CANCEL", C_WARN),
            other => (other, C_DIM),
        };
        let badge = label_badge(label, color);
        let time = run.created_at.replace('T', " ").replace('Z', "");
        write!(h, r#"<tr>
{}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
{}
</tr>
"#,
            td(&run.name, C_TEXT, "11px", "left"),
            td(&time, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_ghcr(h: &mut String, data: &ReportData) {
    if data.ghcr_packages.is_empty() { return; }
    section_start(h, &format!("GHCR Registry ({} packages)", data.ghcr_total), 2);
    h.push_str("<tr>");
    h.push_str(&th("Package", "left"));
    h.push_str(&th("Updated", "left"));
    h.push_str("</tr>\n");

    for pkg in &data.ghcr_packages {
        let time = pkg.updated_at.replace('T', " ").replace('Z', "");
        write!(h, "<tr>{}{}</tr>\n",
            td(&pkg.name, C_TEXT, "11px", "left"),
            td(&time, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_dags(h: &mut String, data: &ReportData) {
    if data.dags.is_empty() { return; }
    section_start(h, &format!("Dagu DAGs ({} workflows)", data.dags.len()), 3);
    h.push_str("<tr>");
    for (label, align) in &[("DAG","left"),("Status","center"),("Last Run","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for dag in &data.dags {
        let (label, color) = match dag.status.as_str() {
            "succeeded" | "success" | "finished" => ("OK", C_OK),
            "failed" | "error" => ("FAIL", C_CRIT),
            "running" => ("RUN", C_WARN),
            "not_started" | "none" => ("NEVER", C_DIM),
            _ => (&*dag.status, C_DIM),
        };
        let badge = label_badge(label, color);
        let time = dag.started_at.replace('T', " ").replace('Z', "");
        write!(h, r#"<tr>
{}
<td style="padding:2px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
{}
</tr>
"#,
            td(&dag.name, C_TEXT, "11px", "left"),
            td(&time, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn render_container_resources(h: &mut String, data: &ReportData) {
    for vm in &data.vms {
        section_start(h, &format!("Container Resources — {}", vm.name), 4);

        for name in &vm.unhealthy_names {
            write!(h, r#"<tr><td colspan="4" style="padding:4px 8px;color:{C_CRIT};font-size:12px;font-family:{FONT};">UNHEALTHY: {name}</td></tr>"#).unwrap();
        }
        for name in &vm.exited_names {
            write!(h, r#"<tr><td colspan="4" style="padding:4px 8px;color:{C_WARN};font-size:12px;font-family:{FONT};">EXITED: {name}</td></tr>"#).unwrap();
        }

        if !vm.container_stats.is_empty() {
            h.push_str("<tr>");
            for (label, align) in &[("Container","left"),("CPU","right"),("Mem Usage","right"),("Mem %","right")] {
                h.push_str(&th(label, align));
            }
            h.push_str("</tr>\n");

            for s in &vm.container_stats {
                write!(h, "<tr>{}{}{}{}</tr>\n",
                    td(&s.name, C_TEXT, "11px", "left"),
                    td(&s.cpu, C_TEXT, "11px", "right"),
                    td(&s.mem_usage, C_TEXT, "11px", "right"),
                    td(&s.mem_pct, C_TEXT, "11px", "right"),
                ).unwrap();
            }
        }
        section_end(h);
    }
}

// ── Log Errors (NEW) ────────────────────────────────────────────────

fn render_log_errors(h: &mut String, data: &ReportData) {
    let has_errors = data.vms.iter().any(|v| !v.log_errors.is_empty());
    if !has_errors { return; }

    section_start(h, "Container Log Errors (24h)", 3);
    h.push_str("<tr>");
    for (label, align) in &[("Container","left"),("Error Count","right"),("Severity","center")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for vm in &data.vms {
        if vm.log_errors.is_empty() { continue; }
        write!(h, r#"<tr><td colspan="3" style="padding:6px 8px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{}</td></tr>"#, vm.name).unwrap();
        for (name, count) in &vm.log_errors {
            let (sev_label, sev_color) = if *count > 100 {
                ("HIGH", C_CRIT)
            } else if *count > 10 {
                ("MED", C_WARN)
            } else {
                ("LOW", C_DIM)
            };
            let badge = label_badge(sev_label, sev_color);
            write!(h, r#"<tr>{}{}<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td></tr>
"#,
                td(name, C_TEXT, "11px", "left"),
                td(&count.to_string(), sev_color, "11px", "right"),
            ).unwrap();
        }
    }
    section_end(h);
}

// ── OOM Kills (NEW) ─────────────────────────────────────────────────

fn render_oom_kills(h: &mut String, data: &ReportData) {
    let has_oom = data.vms.iter().any(|v| !v.oom_kills.is_empty());
    if !has_oom { return; }

    section_start(h, "OOM Kills", 1);
    for vm in &data.vms {
        if vm.oom_kills.is_empty() { continue; }
        write!(h, r#"<tr><td style="padding:6px 8px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{}</td></tr>"#, vm.name).unwrap();
        for line in &vm.oom_kills {
            let short = if line.len() > 80 { format!("{}...", &line[..77]) } else { line.clone() };
            write!(h, r#"<tr><td style="padding:4px 8px;">
<div style="border-left:4px solid {C_CRIT};background:{BG_CARD};padding:4px 10px;border-radius:0 4px 4px 0;">
<span style="font-size:10px;color:{C_CRIT};font-family:{FONT};">{short}</span>
</div></td></tr>"#).unwrap();
        }
    }
    section_end(h);
}

fn render_security(h: &mut String, data: &ReportData) {
    section_start(h, "Security Events (24h)", 4);
    h.push_str("<tr>");
    for (label, align) in &[("VM","left"),("SSH OK","right"),("SSH Fail","right"),("sudo","right")] {
        write!(h, r#"<th style="text-align:{align};color:{C_DIM};font-size:11px;padding:8px;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{label}</th>"#).unwrap();
    }
    h.push_str("</tr>\n");

    for vm in &data.vms {
        let sf_color = if vm.ssh_fails > 50 { C_CRIT } else if vm.ssh_fails > 10 { C_WARN } else { C_OK };
        write!(h, "<tr>{}{}{}{}</tr>\n",
            td(&vm.name, C_TEXT, "12px", "left"),
            td(&vm.ssh_accepts.to_string(), C_OK, "12px", "right"),
            td(&vm.ssh_fails.to_string(), sf_color, "12px", "right"),
            td(&vm.sudo_count.to_string(), C_TEXT, "12px", "right"),
        ).unwrap();
    }

    let has_fails = data.vms.iter().any(|v| !v.top_fail_ips.is_empty());
    if has_fails {
        write!(h, r#"<tr><td colspan="4" style="padding:8px 8px 4px;color:{C_DIM};font-size:11px;font-family:{FONT};">Top failed IPs:</td></tr>"#).unwrap();
        for vm in &data.vms {
            for (ip, count) in &vm.top_fail_ips {
                write!(h, r#"<tr><td colspan="4" style="padding:2px 16px;color:{C_WARN};font-size:11px;font-family:{FONT};">{}: {} ({} attempts)</td></tr>"#,
                    vm.name, ip, count).unwrap();
            }
        }
    }
    section_end(h);
}

fn render_docker_disk(h: &mut String, data: &ReportData) {
    section_start(h, "Docker Disk Usage", 4);
    for vm in &data.vms {
        if vm.docker_df.is_empty() { continue; }
        write!(h, r#"<tr><td colspan="4" style="padding:6px 8px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{}</td></tr>"#, vm.name).unwrap();
        for df in &vm.docker_df {
            write!(h, "<tr>{}{}{}{}</tr>\n",
                td(&df.dtype, C_DIM, "11px", "left"),
                td(&df.count, C_TEXT, "11px", "right"),
                td(&df.size, C_TEXT, "11px", "right"),
                td(&format!("reclaimable: {}", df.reclaimable), C_DIM, "11px", "right"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_wireguard(h: &mut String, data: &ReportData) {
    let has_wg = data.vms.iter().any(|v| !v.wg_peers.is_empty());
    if !has_wg { return; }

    section_start(h, "WireGuard Peers", 2);
    let now = chrono::Utc::now().timestamp() as u64;
    for vm in &data.vms {
        if vm.wg_peers.is_empty() { continue; }
        write!(h, r#"<tr><td colspan="2" style="padding:6px 8px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{}</td></tr>"#, vm.name).unwrap();
        for (peer, ts) in &vm.wg_peers {
            let short = if peer.len() > 8 { format!("{}...", &peer[..8]) } else { peer.clone() };
            let (age, color) = if *ts == 0 {
                ("never".into(), C_CRIT)
            } else {
                let diff = now.saturating_sub(*ts);
                let age = if diff < 120 { format!("{}s ago", diff) }
                    else if diff < 7200 { format!("{}m ago", diff / 60) }
                    else { format!("{}h ago", diff / 3600) };
                let color = if diff > 300 { C_WARN } else { C_OK };
                (age, color)
            };
            write!(h, "<tr>{}{}</tr>\n",
                td(&short, C_DIM, "11px", "left"),
                td(&age, color, "11px", "left"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_runtime_volumes(h: &mut String, data: &ReportData) {
    let has_vols = data.vms.iter().any(|v| !v.runtime_volumes.is_empty());
    if !has_vols { return; }

    section_start(h, "Runtime Volumes (Discovered via SSH)", 4);
    h.push_str("<tr>");
    for (label, align) in &[("Volume", "left"), ("Size", "right"), ("Container", "left"), ("Mount", "left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for vm in &data.vms {
        if vm.runtime_volumes.is_empty() { continue; }
        write!(h, r#"<tr><td colspan="4" style="padding:8px 8px 2px;color:{C_TEXT};font-size:12px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">{} ({} volumes)</td></tr>"#,
            vm.name, vm.runtime_volumes.len()).unwrap();

        // Sort by size descending (larger first), treat "?" and empty as smallest
        let mut sorted: Vec<_> = vm.runtime_volumes.iter().collect();
        sorted.sort_by(|a, b| {
            let parse_size = |s: &str| -> u64 {
                let s = s.trim();
                if s.is_empty() || s == "?" { return 0; }
                let (num_str, mult) = if s.ends_with('G') { (&s[..s.len()-1], 1_073_741_824u64) }
                    else if s.ends_with('M') { (&s[..s.len()-1], 1_048_576) }
                    else if s.ends_with('K') { (&s[..s.len()-1], 1024) }
                    else { (s, 1) };
                num_str.parse::<f64>().unwrap_or(0.0) as u64 * mult
            };
            parse_size(&b.size).cmp(&parse_size(&a.size))
        });

        for v in &sorted {
            let size_color = if v.size.is_empty() || v.size == "?" { C_DIM } else { C_TEXT };
            let size_display = if v.size.is_empty() { "?" } else { &v.size };
            let ctr_color = if v.container == "(orphan)" { C_WARN } else { C_DIM };
            let short_mount = if v.mount_point.len() > 25 {
                format!("...{}", &v.mount_point[v.mount_point.len()-22..])
            } else {
                v.mount_point.clone()
            };
            write!(h, "<tr>{}{}{}{}</tr>\n",
                td(&v.name, C_TEXT, "10px", "left"),
                td(size_display, size_color, "11px", "right"),
                td(&v.container, ctr_color, "10px", "left"),
                td(&short_mount, C_DIM, "10px", "left"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_failed_units(h: &mut String, data: &ReportData) {
    let has_failed = data.vms.iter().any(|v| !v.failed_units.is_empty());
    if !has_failed { return; }

    section_start(h, "Failed Systemd Units", 2);
    for vm in &data.vms {
        for unit in &vm.failed_units {
            write!(h, "<tr>{}{}</tr>\n",
                td(&vm.name, C_TEXT, "12px", "left"),
                td(unit, C_CRIT, "12px", "left"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_restarts(h: &mut String, data: &ReportData) {
    let has_restarts = data.vms.iter().any(|v| !v.restarts.is_empty());
    if !has_restarts { return; }

    section_start(h, "Container Restarts (24h)", 3);
    for vm in &data.vms {
        for (name, count) in &vm.restarts {
            let color = if *count > 10 { C_CRIT } else if *count > 3 { C_WARN } else { C_OK };
            write!(h, "<tr>{}{}{}</tr>\n",
                td(&vm.name, C_TEXT, "12px", "left"),
                td(name, C_TEXT, "11px", "left"),
                td(&count.to_string(), color, "11px", "right"),
            ).unwrap();
        }
    }
    section_end(h);
}

// ── Container Drift (rewritten to use manifests) ────────────────────

fn render_container_drift(h: &mut String, data: &ReportData) {
    if data.container_drift.is_empty() { return; }

    section_start(h, "Container Drift (Manifest vs Running)", 3);
    h.push_str("<tr>");
    for (label, align) in &[("VM","left"),("Containers","left"),("Status","center")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for drift in &data.container_drift {
        // Expected but not running (MISSING)
        if !drift.expected_not_running.is_empty() {
            let mut names = drift.expected_not_running.clone();
            names.sort();
            let badge = label_badge("MISSING", C_CRIT);
            write!(h, r#"<tr>
{vm_td}
{names_td}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
</tr>
"#,
                vm_td = td(&drift.vm_name, C_TEXT, "11px", "left"),
                names_td = td(&names.join(", "), C_CRIT, "10px", "left"),
            ).unwrap();
        }

        // Running but not declared (UNDECLARED)
        if !drift.running_not_declared.is_empty() {
            let mut names = drift.running_not_declared.clone();
            names.sort();
            let badge = label_badge("UNDECLARED", C_WARN);
            write!(h, r#"<tr>
{vm_td}
{names_td}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
</tr>
"#,
                vm_td = td(&drift.vm_name, C_TEXT, "11px", "left"),
                names_td = td(&names.join(", "), C_WARN, "10px", "left"),
            ).unwrap();
        }

        // Image mismatch
        if !drift.image_mismatch.is_empty() {
            for (ctr, running, declared) in &drift.image_mismatch {
                let badge = label_badge("MISMATCH", C_WARN);
                let detail = format!("{}: running={} declared={}", ctr, running, declared);
                write!(h, r#"<tr>
{vm_td}
{detail_td}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
</tr>
"#,
                    vm_td = td(&drift.vm_name, C_TEXT, "11px", "left"),
                    detail_td = td(&detail, C_WARN, "10px", "left"),
                ).unwrap();
            }
        }
    }
    section_end(h);
}

fn render_drift(h: &mut String, data: &ReportData) {
    if data.drift.is_empty() { return; }

    section_start(h, "Storage Drift (Declared vs Runtime)", 3);
    h.push_str("<tr>");
    for (label, align) in &[("Category","left"),("Containers","left"),("Status","center")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for d in &data.drift {
        if !d.declared_only.is_empty() {
            let badge = label_badge("MISSING", C_CRIT);
            write!(h, r#"<tr>
{}
{}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
</tr>
"#,
                td("Declared, not running", C_TEXT, "11px", "left"),
                td(&d.declared_only.join(", "), C_CRIT, "10px", "left"),
            ).unwrap();
        }

        if !d.runtime_only.is_empty() {
            let badge = label_badge("UNDECLARED", C_WARN);
            write!(h, r#"<tr>
{}
{}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
</tr>
"#,
                td("Running, not declared", C_TEXT, "11px", "left"),
                td(&d.runtime_only.join(", "), C_WARN, "10px", "left"),
            ).unwrap();
        }

        if !d.matched.is_empty() {
            let badge = label_badge("OK", C_OK);
            write!(h, r#"<tr>
{}
{}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{badge}</td>
</tr>
"#,
                td("Declared & running", C_TEXT, "11px", "left"),
                td(&d.matched.join(", "), C_OK, "10px", "left"),
            ).unwrap();
        }
    }
    section_end(h);
}

fn render_services_all_unified(h: &mut String, data: &ReportData) {
    // All enabled services — public (with domain) and internal (without)
    let mut public: Vec<_> = data.services.iter()
        .filter(|s| s.enabled && !s.domain.is_empty() && s.service_type != "mcp")
        .collect();
    let mut internal: Vec<_> = data.services.iter()
        .filter(|s| s.enabled && s.domain.is_empty() && s.service_type != "mcp" && s.service_type != "infra")
        .collect();
    public.sort_by_key(|s| &s.name);
    internal.sort_by_key(|s| &s.name);

    let total = public.len() + internal.len();
    section_start(h, &format!("All Services ({} enabled)", total), 5);
    h.push_str("<tr>");
    for (label, align) in &[("Service","left"),("Category","left"),("URL","left"),("Status","center"),("Latency","right")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    // Public services
    if !public.is_empty() {
        write!(h, r#"<tr><td colspan="5" style="padding:6px 8px 2px;color:{C_DIM};font-size:10px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">Public ({} with domains)</td></tr>"#,
            public.len()).unwrap();
        for svc in &public {
            let ep = data.endpoints.iter().find(|e| e.service == svc.name);
            let url = if svc.domain.starts_with("http") { svc.domain.clone() } else { format!("https://{}", svc.domain) };
            let url_link = format!(
                r#"<a href="{}" style="color:{};font-size:10px;text-decoration:none;font-family:{FONT};">{}</a>"#,
                url,
                ep.map(|e| if e.status_code >= 200 && e.status_code < 400 { C_OK } else { C_CRIT }).unwrap_or(C_TEXT),
                svc.domain
            );
            let status_html = match ep {
                Some(e) => code_badge(e.status_code),
                None => label_badge("—", C_DIM),
            };
            let latency_str = match ep {
                Some(e) => format!("{}ms", e.latency_ms),
                None => "—".into(),
            };
            let lat_color = ep
                .map(|e| if e.latency_ms < 200 { C_OK } else if e.latency_ms < 1000 { C_WARN } else { C_CRIT })
                .unwrap_or(C_DIM);
            let cat_color = category_color(&svc.category);
            write!(h, r#"<tr>
{}{}
<td style="padding:3px 8px;border-bottom:1px solid rgba(15,52,96,0.3);font-family:{FONT};">{url_link}</td>
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{status_html}</td>
{}
</tr>
"#,
                td(&svc.name, C_TEXT, "11px", "left"),
                td(&svc.category, cat_color, "10px", "left"),
                td(&latency_str, lat_color, "10px", "right"),
            ).unwrap();
        }
    }

    // Internal services
    if !internal.is_empty() {
        write!(h, r#"<tr><td colspan="5" style="padding:6px 8px 2px;color:{C_DIM};font-size:10px;font-weight:bold;border-bottom:1px solid {BG_HEAD};font-family:{FONT};">Internal ({} no public domain)</td></tr>"#,
            internal.len()).unwrap();
        for svc in &internal {
            let cat_color = category_color(&svc.category);
            write!(h, r#"<tr>
{}{}
{}
{}
{}
</tr>
"#,
                td(&svc.name, C_TEXT, "11px", "left"),
                td(&svc.category, cat_color, "10px", "left"),
                td(&format!("internal :{}", svc.port), C_DIM, "10px", "left"),
                td("—", C_DIM, "10px", "center"),
                td(&svc.vm, C_DIM, "10px", "right"),
            ).unwrap();
        }
    }

    section_end(h);
}

fn render_services_mcps(h: &mut String, data: &ReportData) {
    if data.mcp_servers.is_empty() { return; }

    section_start(h, &format!("MCP Servers ({} configured)", data.mcp_servers.len()), 4);
    h.push_str("<tr>");
    for (label, align) in &[("Server","left"),("Command","left"),("Transport","left"),("Source","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for mcp in &data.mcp_servers {
        let cmd_color = if mcp.command == "?" { C_WARN } else { C_OK };
        let short_source = if mcp.source_path.len() > 45 {
            format!("...{}", &mcp.source_path[mcp.source_path.len()-42..])
        } else if mcp.source_path.is_empty() {
            "(remote/docker)".into()
        } else {
            mcp.source_path.clone()
        };
        write!(h, "<tr>{}{}{}{}</tr>\n",
            td(&mcp.name, C_TEXT, "11px", "left"),
            td(&mcp.command, cmd_color, "10px", "left"),
            td(&mcp.transport, C_DIM, "10px", "left"),
            td(&short_source, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn category_color(cat: &str) -> &'static str {
    match cat {
        "app" => "#57A6D6",
        "sec" => "#DC382D",
        "tools" | "obs" => "#F46800",
        "data" => "#4169E1",
        "fin" => "#00d68f",
        "agi" => "#C0765A",
        "cloud" => "#8899aa",
        "mic" => "#9B59B6",
        _ => C_DIM,
    }
}

// render_service_row removed — inlined into render_services_all

fn render_repos(h: &mut String, data: &ReportData) {
    if data.repos.is_empty() { return; }

    section_start(h, &format!("GitHub Repositories ({} repos)", data.repos.len()), 5);
    h.push_str("<tr>");
    for (label, align) in &[("Repository","left"),("Visibility","center"),("Language","left"),("Size","right"),("Updated","left")] {
        h.push_str(&th(label, align));
    }
    h.push_str("</tr>\n");

    for repo in &data.repos {
        let vis_badge = match repo.visibility.as_str() {
            "PUBLIC" => label_badge("PUBLIC", C_OK),
            "PRIVATE" => label_badge("PRIVATE", C_WARN),
            _ => label_badge(&repo.visibility, C_DIM),
        };
        let size = if repo.disk_kb > 1024 {
            format!("{:.0} MB", repo.disk_kb as f64 / 1024.0)
        } else {
            format!("{} KB", repo.disk_kb)
        };
        let time = repo.updated_at.replace('T', " ").replace('Z', "");
        let time_short = if time.len() > 16 { &time[..16] } else { &time };
        write!(h, r#"<tr>
{}
<td style="padding:3px 8px;text-align:center;border-bottom:1px solid rgba(15,52,96,0.3);">{vis_badge}</td>
{}{}{}
</tr>
"#,
            td(&repo.name, C_TEXT, "11px", "left"),
            td(&repo.language, C_DIM, "10px", "left"),
            td(&size, C_DIM, "11px", "right"),
            td(time_short, C_DIM, "10px", "left"),
        ).unwrap();
    }
    section_end(h);
}

fn human_size(bytes: u64) -> String {
    if bytes > 1_073_741_824 {
        format!("{:.1}G", bytes as f64 / 1_073_741_824.0)
    } else if bytes > 1_048_576 {
        format!("{:.1}M", bytes as f64 / 1_048_576.0)
    } else if bytes > 1024 {
        format!("{}K", bytes / 1024)
    } else {
        format!("{}B", bytes)
    }
}
