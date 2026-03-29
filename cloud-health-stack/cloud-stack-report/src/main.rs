//! Container Health Reporter — Full Rust Implementation
//! Native async: TCP, HTTP, DNS, SSH — no shell subprocesses
//!
//! Sections: ISSUES, A0-A4 Health, B Infra, C Security, D Stack, Z Appendix
//! Usage: cargo run --release (from cloud-health-stack/)

mod checks;
mod collectors;
mod parsers;
mod sections;
mod template;
mod types;

use anyhow::Result;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    println!("═══ Rust Health Reporter ═══");

    // 1. Load & parse consolidated JSON
    let ctx = parsers::load_context()?;
    println!("Loaded: {} VMs, {} URLs, {} DNS, {} DBs",
        ctx.vms.len(), ctx.public_urls.len(), ctx.private_dns.len(), ctx.databases.len());

    // 2. Collect live data (SSH + network checks — all parallel)
    let live = collectors::collect_all(&ctx).await;
    println!("Collected in {:.1}s", start.elapsed().as_secs_f64());

    // 3. Generate all template vars
    let vars = sections::build_all_vars(&ctx, &live);

    // 4. Render template → .md
    template::render(&vars)?;

    // 5. Write JSON
    let json = serde_json::to_string_pretty(&live)?;
    std::fs::write("container_health_rust.json", &json)?;

    let total = start.elapsed().as_secs_f64();
    println!("\n═══ DONE in {:.1}s ═══", total);
    println!("→ container_health_rust.json + container_health.md");

    Ok(())
}
