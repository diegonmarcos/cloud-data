use crate::context::NetworkContext;
use crate::types::PortScanResult;
use reports_common::checks;
use reports_common::types::{Check, Severity};
use std::time::Instant;

/// Dangerous ports that should never be open to the public internet
const DANGEROUS_PORTS: &[u16] = &[21, 23, 3306, 5432, 6379, 27017];

/// Scan all VMs with public IPs for open ports
pub async fn scan_all_vms(ctx: &NetworkContext) -> (Vec<Check>, Vec<PortScanResult>) {
    let mut all_checks = Vec::new();
    let mut all_results = Vec::new();

    let scannable: Vec<_> = ctx
        .vms
        .iter()
        .filter(|vm| !vm.pub_ip.is_empty() && vm.pub_ip != "?")
        .collect();

    println!("Port scan: {} VMs with public IPs", scannable.len());

    for vm in &scannable {
        let t = Instant::now();

        // Combine declared ports + dangerous ports for scanning
        let declared: Vec<u16> = vm.public_ports.iter().map(|p| p.port).collect();
        let mut scan_ports: Vec<u16> = declared.clone();
        for &dp in DANGEROUS_PORTS {
            if !scan_ports.contains(&dp) {
                scan_ports.push(dp);
            }
        }
        // Also add common service ports
        for &extra in &[22, 80, 443, 8080, 8443] {
            if !scan_ports.contains(&extra) {
                scan_ports.push(extra);
            }
        }

        let open = checks::tcp_scan(&vm.pub_ip, &scan_ports).await;
        let elapsed = t.elapsed().as_millis() as u64;

        let undeclared_open: Vec<u16> = open
            .iter()
            .filter(|p| !declared.contains(p))
            .copied()
            .collect();

        let declared_closed: Vec<u16> = declared
            .iter()
            .filter(|p| !open.contains(p))
            .copied()
            .collect();

        let result = PortScanResult {
            vm_alias: vm.alias.clone(),
            ip: vm.pub_ip.clone(),
            declared_ports: declared.clone(),
            open_ports: open.clone(),
            undeclared_open: undeclared_open.clone(),
            declared_closed: declared_closed.clone(),
        };

        // Generate checks
        if undeclared_open.is_empty() && declared_closed.is_empty() {
            all_checks.push(Check {
                name: format!("port-scan:{}", vm.alias),
                passed: true,
                details: format!(
                    "{} open={:?} (all declared)",
                    vm.pub_ip,
                    open
                ),
                duration_ms: elapsed,
                error: None,
                severity: Severity::Info,
            });
        } else {
            // Check for dangerous undeclared ports
            let dangerous_open: Vec<u16> = undeclared_open
                .iter()
                .filter(|p| DANGEROUS_PORTS.contains(p))
                .copied()
                .collect();

            if !dangerous_open.is_empty() {
                all_checks.push(Check {
                    name: format!("port-scan:{}:dangerous", vm.alias),
                    passed: false,
                    details: format!(
                        "DANGEROUS ports open on {}: {:?}",
                        vm.pub_ip, dangerous_open
                    ),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Critical,
                });
            }

            let non_dangerous: Vec<u16> = undeclared_open
                .iter()
                .filter(|p| !DANGEROUS_PORTS.contains(p))
                .copied()
                .collect();

            if !non_dangerous.is_empty() {
                all_checks.push(Check {
                    name: format!("port-scan:{}:undeclared", vm.alias),
                    passed: false,
                    details: format!(
                        "Undeclared open ports on {}: {:?}",
                        vm.pub_ip, non_dangerous
                    ),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Warning,
                });
            }

            if !declared_closed.is_empty() {
                all_checks.push(Check {
                    name: format!("port-scan:{}:closed", vm.alias),
                    passed: false,
                    details: format!(
                        "Declared ports closed on {}: {:?}",
                        vm.pub_ip, declared_closed
                    ),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Warning,
                });
            }
        }

        all_results.push(result);
    }

    (all_checks, all_results)
}
