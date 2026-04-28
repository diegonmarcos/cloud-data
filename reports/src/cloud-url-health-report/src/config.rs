use anyhow::{Context, Result};
use reports_common::context::find_cloud_data_file;
use reports_common::email_e2e::EmailE2EConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UrlHealthConfig {
    pub concurrency: Concurrency,
    pub timeouts: Timeouts,
    #[serde(default)]
    pub targets: Targets,
    pub email: EmailE2EConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Concurrency {
    pub public: usize,
    pub private: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Timeouts {
    pub http_connect_secs: u64,
    pub http_total_secs: u64,
    #[serde(default = "default_tcp_secs")]
    pub tcp_secs: u64,
}

fn default_tcp_secs() -> u64 { 3 }

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Targets {
    #[serde(default)]
    pub tcp_only_ports: Vec<u16>,
}

pub fn load() -> Result<UrlHealthConfig> {
    // Migrated to build-reports.json:.url_health (single derived file at
    // cloud/2_configs/dist/, symlinked into cloud-data/). Falls back to
    // legacy cloud-data-url-health.json for back-compat during migration.
    if let Some(section) = reports_common::context::load_build_reports_section("url_health") {
        let cfg: UrlHealthConfig = serde_json::from_value(section)
            .context("parsing build-reports.json:.url_health")?;
        eprintln!(
            "[url-health] config loaded from build-reports.json:.url_health (public={}, private={}, email_timeout={}s)",
            cfg.concurrency.public,
            cfg.concurrency.private,
            cfg.email.timeout_secs,
        );
        return Ok(cfg);
    }
    let path = find_cloud_data_file("cloud-data-url-health.json")
        .context("neither build-reports.json:.url_health nor cloud-data-url-health.json found")?;
    let bytes = std::fs::read(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let cfg: UrlHealthConfig = serde_json::from_slice(&bytes)
        .with_context(|| format!("parsing {}", path.display()))?;
    eprintln!(
        "[url-health] config loaded from {} (public={}, private={}, email_timeout={}s)",
        path.display(),
        cfg.concurrency.public,
        cfg.concurrency.private,
        cfg.email.timeout_secs,
    );
    Ok(cfg)
}
