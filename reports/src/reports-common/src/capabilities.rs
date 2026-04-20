use crate::checks;

/// Runtime capabilities detected at startup
pub struct RuntimeCapabilities {
    pub wg_up: bool,
    pub ssh_available: bool,
    pub bearer_token: Option<String>,
    pub hickory_up: bool,
    pub yara_available: bool,
}

impl RuntimeCapabilities {
    /// Detect what capabilities are available in the current environment
    pub async fn detect() -> Self {
        // Check WG mesh via Hickory DNS at 10.0.0.1
        let wg_up = checks::tcp("10.0.0.1", 53).await;

        // Check if SSH keys are loaded
        let ssh_available = std::path::Path::new("/root/.ssh").exists()
            || std::env::var("HOME")
                .ok()
                .map(|h| std::path::Path::new(&format!("{}/.ssh", h)).exists())
                .unwrap_or(false);

        // Load bearer token
        let bearer_token = crate::context::load_bearer_token();

        // Check Hickory DNS
        let hickory_up = if wg_up {
            let resolver = checks::hickory_resolver();
            checks::dns_resolve(&resolver, "caddy.app").await.is_some()
        } else {
            false
        };

        // Check if yara binary is available
        let yara_available = std::process::Command::new("yara")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        let caps = RuntimeCapabilities {
            wg_up,
            ssh_available,
            bearer_token,
            hickory_up,
            yara_available,
        };

        println!("Runtime capabilities:");
        println!("  WG mesh:    {}", if caps.wg_up { "UP" } else { "DOWN (GHA mode)" });
        println!("  SSH keys:   {}", if caps.ssh_available { "OK" } else { "MISSING" });
        println!("  Bearer:     {}", if caps.bearer_token.is_some() { "OK" } else { "MISSING" });
        println!("  Hickory:    {}", if caps.hickory_up { "OK" } else { "DOWN" });
        println!("  YARA:       {}", if caps.yara_available { "OK" } else { "NOT INSTALLED" });

        caps
    }
}
