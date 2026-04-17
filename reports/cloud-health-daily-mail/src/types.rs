use serde::{Deserialize, Serialize};

// ── Cloud-data JSON structures ──────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct MonitoringTargets {
    #[serde(default)]
    pub endpoint_checks: Vec<EndpointCheck>,
    #[serde(default)]
    pub tls_checks: Vec<TlsCheck>,
    #[serde(default)]
    pub vms: Vec<VmTarget>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EndpointCheck {
    pub service: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsCheck {
    pub service: String,
    pub domain: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VmTarget {
    pub ip: String,
    pub name: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabasesJson {
    #[serde(default)]
    pub databases: Vec<DatabaseEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseEntry {
    pub service: String,
    pub container: String,
    #[serde(rename = "type")]
    pub db_type: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub vm_alias: String,
    #[serde(default)]
    pub wg_ip: String,
    pub user: Option<String>,
    pub db: Option<String>,
}

fn default_true() -> bool {
    true
}

// ── Cloud object storage (from consolidated.storage / Terraform) ────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloudBucket {
    pub provider: String,
    pub name: String,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub size_bytes: u64,
}

// ── Consolidated JSON (partial — only fields we need) ───────────────

#[derive(Debug, Deserialize)]
pub struct ConsolidatedJson {
    #[serde(default)]
    pub storage: Vec<CloudBucket>,
    #[serde(default)]
    pub services: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub vms: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub vpss: std::collections::HashMap<String, serde_json::Value>,
}

// ── Service inventory (parsed from consolidated.services) ───────────

#[derive(Debug, Clone, Serialize)]
pub struct ServiceEntry {
    pub name: String,
    pub category: String,
    pub vm: String,
    pub domain: String,
    pub enabled: bool,
    pub containers: u32,
    pub port: u16,
    pub service_type: String, // "mcp", "app", "infra"
    pub has_api: bool,        // exposes REST/programmatic API (runtime-confirmed or declared)
    pub has_web_ui: bool,     // has browser-accessible web UI
    pub api_path: String,     // API base path (e.g. "/api/v1") or empty
    pub api_url: String,      // Full API URL (e.g. "https://git.diegonmarcos.com/api/v1") or empty
}

// ── MCP server config (from .mcp.json) ──────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub source_path: String,
    pub transport: String, // "stdio", "http"
}

// ── Cloud cost data ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CloudCost {
    pub provider: String,
    pub month: String,      // "2026-04"
    pub service: String,    // "Compute", "Storage", "Network", etc.
    pub amount: f64,
    pub usage: f64,         // usage quantity (hours, GB, requests)
    pub currency: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WgTransfer {
    pub peer: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

// ── Collected VM data ───────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct VmData {
    pub name: String,
    pub ip: String,
    pub status: VmStatus,
    pub uptime: String,
    pub load: String,
    pub disk: String,
    pub disk_pct: u32,
    pub mem: String,
    pub mem_pct: u32,
    pub kernel: String,
    pub wg_transfer: Vec<WgTransfer>,
    pub containers_running: u32,
    pub containers_total: u32,
    pub containers_unhealthy: u32,
    pub container_list: Vec<ContainerInfo>,
    pub container_stats: Vec<ContainerStat>,
    pub unhealthy_names: Vec<String>,
    pub exited_names: Vec<String>,
    pub ssh_accepts: u32,
    pub ssh_fails: u32,
    pub sudo_count: u32,
    pub top_fail_ips: Vec<(String, u32)>,
    pub restarts: Vec<(String, u32)>,
    pub backups: Vec<BackupEntry>,
    pub failed_units: Vec<String>,
    pub wg_peers: Vec<(String, u64)>,
    pub docker_df: Vec<DockerDfEntry>,
    pub db_sizes: Vec<(String, String)>,
    pub mail_queue: Option<u32>,
    pub mail_delivered: Option<u32>,
    pub mail_failed: Option<u32>,
    pub runtime_volumes: Vec<RuntimeVolume>,
    pub oom_kills: Vec<String>,
    pub swap: String,
    pub swap_pct: u32,
    pub log_errors: Vec<(String, u32)>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub enum VmStatus {
    Healthy,
    Warning,
    Critical,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContainerInfo {
    pub name: String,
    pub image: String,
    pub status: String,
    pub running_for: String,
    pub image_created: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContainerStat {
    pub name: String,
    pub cpu: String,
    pub mem_usage: String,
    pub mem_pct: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupEntry {
    pub file: String,
    pub size_bytes: u64,
    pub epoch: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DockerDfEntry {
    pub dtype: String,
    pub count: String,
    pub size: String,
    pub reclaimable: String,
}

// ── API-collected data ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct EndpointResult {
    pub service: String,
    pub url: String,
    pub status_code: u16,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CertResult {
    pub domain: String,
    pub days_left: i64,
    pub expiry: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DnsResult {
    pub record_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GhaRun {
    pub name: String,
    pub conclusion: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GhcrPackage {
    pub name: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DagStatus {
    pub name: String,
    pub status: String,
    pub started_at: String,
}

// ── GitHub repos ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct GithubRepo {
    pub name: String,
    pub visibility: String,
    pub updated_at: String,
    pub language: String,
    pub disk_kb: u64,
}

// ── FinOps / VPS provider data ───────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct VpsProvider {
    pub name: String,
    pub provider: String,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VmFinops {
    pub alias: String,
    pub provider: String,
    pub tier: String,
    pub cpu: u32,
    pub ram_gb: f64,
    pub shape: String,
    pub services: u32,
    pub containers: u32,
}

// ── Runtime volume data (discovered via SSH) ────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeVolume {
    pub name: String,
    pub size: String,
    pub container: String,
    pub mount_point: String,
}

// ── Drift detection ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct StorageDrift {
    pub declared_only: Vec<String>,
    pub runtime_only: Vec<String>,
    pub matched: Vec<String>,
}

// ── Executive summary ───────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ExecSummary {
    pub critical: u32,
    pub warnings: u32,
    pub ok: u32,
    pub top_issues: Vec<Issue>,
}

#[derive(Debug, Serialize)]
pub struct Issue {
    pub severity: String,
    pub message: String,
}

// ── Container manifest + drift ─────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct VmContainerManifest {
    pub vm: String,
    #[serde(default)]
    pub services: Vec<ManifestService>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManifestService {
    pub name: String,
    #[serde(default)]
    pub dir: String,
    #[serde(default)]
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContainerDrift {
    pub vm_name: String,
    pub expected_not_running: Vec<String>,
    pub running_not_declared: Vec<String>,
    pub image_mismatch: Vec<(String, String, String)>,
}

// ── AI usage data (from Claude stats-cache.json) ───────────────────

#[derive(Debug, Clone, Serialize)]
pub struct AiModelUsage {
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_create_tokens: u64,
    pub estimated_cost_usd: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiDailyActivity {
    pub date: String,
    pub messages: u64,
    pub sessions: u64,
    pub tool_calls: u64,
    pub tokens: u64,
}

#[derive(Debug, Serialize)]
pub struct AiSummary {
    pub models: Vec<AiModelUsage>,
    pub daily: Vec<AiDailyActivity>,
    pub total_sessions: u64,
    pub total_messages: u64,
    pub total_cost_usd: f64,
    pub first_session: String,
}

// ── Full report data ────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ReportData {
    pub date: String,
    pub time: String,
    pub vms: Vec<VmData>,
    pub endpoints: Vec<EndpointResult>,
    pub certs: Vec<CertResult>,
    pub dns: Vec<DnsResult>,
    pub gha_runs: Vec<GhaRun>,
    pub ghcr_packages: Vec<GhcrPackage>,
    pub ghcr_total: usize,
    pub github_disk_kb: u64,
    pub dags: Vec<DagStatus>,
    pub databases: Vec<DatabaseEntry>,
    pub fleet_running: u32,
    pub fleet_total: u32,
    pub fleet_unhealthy: u32,
    pub drift: Vec<StorageDrift>,
    pub exec_summary: ExecSummary,
    pub container_drift: Vec<ContainerDrift>,
    pub cloud_buckets: Vec<CloudBucket>,
    pub cloud_costs: Vec<CloudCost>,
    pub services: Vec<ServiceEntry>,
    pub repos: Vec<GithubRepo>,
    pub mcp_servers: Vec<McpServer>,
    pub vps_providers: Vec<VpsProvider>,
    pub vm_finops: Vec<VmFinops>,
    pub total_services: usize,
    pub total_containers: u32,
    pub total_domains: usize,
    pub generation_duration_ms: u64,
    pub ai: Option<AiSummary>,
}
