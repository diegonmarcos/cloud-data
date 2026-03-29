//! Cloud Health Full Report — 10-Layer Diagnostic
//! Native async: TCP, HTTP, DNS, SSH/rsync — no shell subprocesses (except SSH)
//!
//! Layers: L1 Self-Check, L2 WG Mesh, L3 Platform, L4 Containers,
//!         L5 Public URLs, L6 Private URLs, L7 Cross-Checks,
//!         L8 External, L9 Drift, L10 Security
//!
//! Usage: cargo run --release (from cloud-health-full-report/)

mod checks;
mod context;
mod layers;
mod output;
mod ssh;
mod template;
mod types;

use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::time::Instant;
use types::*;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("=== Cloud Health Full Report (10-Layer) ===");

    // 1. Load context from cloud-data JSONs
    let ctx = context::load_context()?;
    println!(
        "Loaded: {} VMs, {} services, {} caddy routes, {} build.json ports",
        ctx.vms.len(),
        ctx.services.len(),
        ctx.caddy_route_list.len(),
        ctx.service_ports.len()
    );

    let mut timers: HashMap<String, u64> = HashMap::new();

    // ════════════════════════════════════════════════════════════
    // SEQUENTIAL: L1 -> L2 -> L3 (each depends on previous)
    // ════════════════════════════════════════════════════════════

    // L1: Self-check
    let t1 = Instant::now();
    let self_check = layers::layer_self_check(&ctx).await;
    let l1_ms = t1.elapsed().as_millis() as u64;
    timers.insert("L1_self_check".into(), l1_ms);
    println!(
        "  L1 Self-check: {}/{} in {:.1}s",
        self_check.iter().filter(|c| c.passed).count(),
        self_check.len(),
        l1_ms as f64 / 1000.0
    );

    // L2: WireGuard mesh (needs L1 to know if WG is up)
    let t2 = Instant::now();
    let (wg_mesh, reachable_vms) = layers::layer_wg_mesh(&ctx).await;
    let l2_ms = t2.elapsed().as_millis() as u64;
    timers.insert("L2_wg_mesh".into(), l2_ms);
    println!(
        "  L2 WG Mesh: {}/{} reachable in {:.1}s",
        reachable_vms.len(),
        ctx.vms.len(),
        l2_ms as f64 / 1000.0
    );

    // L3: Platform (needs L2 reachable_vms)
    let t3 = Instant::now();
    let (platform, vm_batch, ssh_ok_vms, docker_ok_vms) =
        layers::layer_platform(&ctx, &reachable_vms).await;
    let l3_ms = t3.elapsed().as_millis() as u64;
    timers.insert("L3_platform".into(), l3_ms);
    println!(
        "  L3 Platform: ssh={}/{} docker={}/{} in {:.1}s",
        ssh_ok_vms.len(),
        ctx.vms.len(),
        docker_ok_vms.len(),
        ctx.vms.len(),
        l3_ms as f64 / 1000.0
    );

    // ════════════════════════════════════════════════════════════
    // PARALLEL: L4-L10 (all read cached data from L3)
    // ════════════════════════════════════════════════════════════

    let t_par = Instant::now();

    // L4 is sync (reads vm_batch), run it first
    let containers = layers::layer_containers(&ctx, &vm_batch);

    // L5, L6, L8, L10 are async — run them in parallel
    let (public_urls, private_urls, external, security) = tokio::join!(
        layers::layer_public_urls(&ctx),
        layers::layer_private_urls(&ctx),
        layers::layer_external(&ctx),
        layers::layer_security(&ctx),
    );

    // L7 cross-checks (needs L4, L5, L6 results)
    let cross_checks =
        layers::layer_cross_checks(&ctx, &vm_batch, &public_urls, &private_urls, &containers);

    // L9 drift (sync, reads vm_batch)
    let drift = layers::layer_drift(&ctx, &vm_batch);

    let par_ms = t_par.elapsed().as_millis() as u64;
    timers.insert("L4_containers".into(), 0);
    timers.insert("L5_public_urls".into(), par_ms);
    timers.insert("L6_private_urls".into(), par_ms);
    timers.insert("L7_cross_checks".into(), 0);
    timers.insert("L8_external".into(), par_ms);
    timers.insert("L9_drift".into(), 0);
    timers.insert("L10_security".into(), par_ms);
    timers.insert("L4-L10_parallel".into(), par_ms);

    println!(
        "  L4-L10 parallel: {:.1}s",
        par_ms as f64 / 1000.0
    );

    // ════════════════════════════════════════════════════════════
    // BUILD RESULTS
    // ════════════════════════════════════════════════════════════

    let total_ms = start.elapsed().as_millis() as u64;
    timers.insert("TOTAL".into(), total_ms);

    // Compute summary
    let all_checks: Vec<&Check> = self_check
        .iter()
        .chain(&wg_mesh)
        .chain(&platform)
        .chain(&containers)
        .chain(&public_urls)
        .chain(&private_urls)
        .chain(&cross_checks)
        .chain(&external)
        .chain(&drift)
        .chain(&security)
        .collect();

    let total_count = all_checks.len();
    let passed_count = all_checks.iter().filter(|c| c.passed).count();
    let failed_count = total_count - passed_count;
    let critical_count = all_checks
        .iter()
        .filter(|c| !c.passed && c.severity == Severity::Critical)
        .count();
    let warning_count = all_checks
        .iter()
        .filter(|c| !c.passed && c.severity == Severity::Warning)
        .count();

    let results = LayerResults {
        generated: Utc::now().to_rfc3339(),
        duration_ms: total_ms,
        self_check,
        wg_mesh,
        platform,
        containers,
        public_urls,
        private_urls,
        cross_checks,
        external,
        drift,
        security,
        summary: Summary {
            total_checks: total_count,
            passed: passed_count,
            failed: failed_count,
            warnings: warning_count,
            critical: critical_count,
        },
        timers,
    };

    // 3. Build template vars and render
    let vars = output::build_template_vars(&results);
    template::render(&vars)?;

    // 4. Write JSON
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("cloud_health_full.json", &json)?;

    println!(
        "\n=== DONE in {:.1}s === {}/{} passed, {} critical, {} warnings",
        total_ms as f64 / 1000.0,
        passed_count,
        total_count,
        critical_count,
        warning_count,
    );
    println!("-> cloud_health_full.json + cloud_health_full.md");

    Ok(())
}
