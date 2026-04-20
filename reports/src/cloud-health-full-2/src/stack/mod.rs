//! Stack report — topology / resources / databases / storage / security sections.
//! Absorbed from former cloud-stack-report crate (2026-04-20).
//! Runs as a submodule of cloud-health-full-2; appends its rendered content to
//! the main cloud_health_full.md output.

pub mod checks;
pub mod collectors;
pub mod parsers;
pub mod sections;
pub mod template;
pub mod types;

use anyhow::Result;

/// Run the stack pipeline. Returns the rendered markdown (to append to main report).
/// Also writes `cloud_stack.json` to cwd (dist/) for programmatic access.
pub async fn run() -> Result<String> {
    eprintln!("[stack] collecting topology + live data");
    let ctx = parsers::load_context()?;
    eprintln!(
        "[stack] loaded: {} VMs, {} URLs, {} DNS, {} DBs",
        ctx.vms.len(),
        ctx.public_urls.len(),
        ctx.private_dns.len(),
        ctx.databases.len()
    );
    let live = collectors::collect_all(&ctx).await;
    let vars = sections::build_all_vars(&ctx, &live);
    let md = template::render_string(&vars)?;
    let json = serde_json::to_string_pretty(&live)?;
    std::fs::write("cloud_stack.json", &json)?;
    eprintln!("[stack] Wrote cloud_stack.json");
    Ok(md)
}
