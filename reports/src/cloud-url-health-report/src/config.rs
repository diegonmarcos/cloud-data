use anyhow::{Context, Result};
use reports_common::context::find_cloud_data_file;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UrlHealthConfig {
    pub concurrency: Concurrency,
    pub timeouts: Timeouts,
    pub email: EmailConfig,
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp: SmtpConfig,
    pub imap: ImapConfig,
    pub timeout_secs: u64,
    pub poll_interval_ms: u64,
    pub subject_prefix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImapConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
}

pub fn load() -> Result<UrlHealthConfig> {
    let path = find_cloud_data_file("cloud-data-url-health.json")
        .context("cloud-data-url-health.json not found (walked up from cwd)")?;
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
