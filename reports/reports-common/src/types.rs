use serde::Serialize;
use std::collections::HashMap;

/// Severity of a check failure
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

/// A single diagnostic check result
#[derive(Debug, Clone, Serialize)]
pub struct Check {
    pub name: String,
    pub passed: bool,
    pub details: String,
    pub duration_ms: u64,
    pub error: Option<String>,
    pub severity: Severity,
}

/// Summary of all checks
#[derive(Debug, Serialize)]
pub struct Summary {
    pub total_checks: usize,
    pub passed: usize,
    pub failed: usize,
    pub warnings: usize,
    pub critical: usize,
}

impl Summary {
    pub fn from_checks(checks: &[&Check]) -> Self {
        let total_checks = checks.len();
        let passed = checks.iter().filter(|c| c.passed).count();
        let failed = total_checks - passed;
        let warnings = checks
            .iter()
            .filter(|c| !c.passed && c.severity == Severity::Warning)
            .count();
        let critical = checks
            .iter()
            .filter(|c| !c.passed && c.severity == Severity::Critical)
            .count();
        Summary {
            total_checks,
            passed,
            failed,
            warnings,
            critical,
        }
    }
}

/// VM information from cloud-data consolidated JSON
#[derive(Clone, Debug)]
pub struct VmInfo {
    pub vm_id: String,
    pub alias: String,
    pub pub_ip: String,
    pub wg_ip: String,
    pub cloud_name: String,
    pub cloud_zone: String,
    pub rescue_port: u16,
    pub cpus: u32,
    pub ram_gb: f64,
    pub shape: String,
    pub provider: String,
    pub cost: String,
    pub declared_services: Vec<String>,
    pub public_ports: Vec<PublicPort>,
}

#[derive(Clone, Debug)]
pub struct PublicPort {
    pub port: u16,
    pub proto: String,
    pub desc: String,
}

/// Service information from cloud-data consolidated JSON
#[derive(Clone, Debug)]
pub struct ServiceInfo {
    pub name: String,
    pub category: String,
    pub vm_id: String,
    pub vm_alias: String,
    pub folder: String,
    pub domain: Option<String>,
    pub port: Option<u16>,
    pub dns: Option<String>,
    pub upstream: Option<String>,
    pub containers: Vec<ContainerDecl>,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub struct ContainerDecl {
    pub key: String,
    pub container_name: String,
    pub image: String,
    pub port: Option<u16>,
    pub dns: Option<String>,
    pub healthcheck: Option<String>,
}

/// Caddy route from build-proxy-caddy-routes.json
#[derive(Clone, Debug)]
pub struct CaddyRoute {
    pub domain: String,
    pub upstream: String,
    pub comment: String,
    pub auth: Option<String>,
}

/// Timer map for performance tracking
pub type Timers = HashMap<String, u64>;
