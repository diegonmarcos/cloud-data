use anyhow::Result;
use reports_common::context;
use reports_common::types::{CaddyRoute, VmInfo};

/// All data needed by network security checks
pub struct NetworkContext {
    pub vms: Vec<VmInfo>,
    pub caddy_routes: Vec<CaddyRoute>,
    pub bearer_token: Option<String>,
}

/// Load cloud-data JSONs and build network context
pub fn load_context() -> Result<NetworkContext> {
    let consolidated = context::load_consolidated()?;
    let vms = context::parse_vms(&consolidated);
    println!("  Loaded {} VMs", vms.len());

    let caddy_routes = context::load_caddy_routes()
        .map(|j| context::parse_caddy_routes(&j))
        .unwrap_or_default();
    println!("  Loaded {} Caddy routes", caddy_routes.len());

    let bearer_token = context::load_bearer_token();
    println!(
        "  Bearer token: {}",
        if bearer_token.is_some() {
            "present"
        } else {
            "absent"
        }
    );

    Ok(NetworkContext {
        vms,
        caddy_routes,
        bearer_token,
    })
}
