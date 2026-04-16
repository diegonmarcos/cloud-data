use crate::context::NetworkContext;
use crate::types::PortScanResult;
use reports_common::capabilities::RuntimeCapabilities;
use reports_common::checks;
use reports_common::types::{Check, Severity};
use std::time::Instant;

/// Dangerous ports that should never be open to the public internet
const DANGEROUS_PORTS: &[u16] = &[21, 23, 3306, 5432, 6379, 27017];

/// Dual-path port scan: external (public IP) + internal (WG IP) when available
pub async fn scan_all_vms(
    ctx: &NetworkContext,
    caps: &RuntimeCapabilities,
) -> (Vec<Check>, Vec<PortScanResult>) {
    let mut all_checks = Vec::new();
    let mut all_results = Vec::new();

    // ── External scan: public IPs ──
    let scannable: Vec<_> = ctx
        .vms
        .iter()
        .filter(|vm| !vm.pub_ip.is_empty() && vm.pub_ip != "?")
        .collect();

    println!("Port scan: {} VMs public + {} internal",
        scannable.len(),
        if caps.wg_up { ctx.vms.len().to_string() } else { "0 (no WG)".into() });

    for vm in &scannable {
        let t = Instant::now();

        let declared: Vec<u16> = vm.public_ports.iter().map(|p| p.port).collect();
        let mut scan_ports: Vec<u16> = declared.clone();
        for &dp in DANGEROUS_PORTS {
            if !scan_ports.contains(&dp) {
                scan_ports.push(dp);
            }
        }
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

        all_results.push(PortScanResult {
            vm_alias: vm.alias.clone(),
            ip: vm.pub_ip.clone(),
            scan_path: "external".into(),
            declared_ports: declared.clone(),
            open_ports: open.clone(),
            undeclared_open: undeclared_open.clone(),
            declared_closed: declared_closed.clone(),
        });

        if undeclared_open.is_empty() && declared_closed.is_empty() {
            all_checks.push(Check {
                name: format!("ext:port-scan:{}", vm.alias),
                passed: true,
                details: format!("{} open={:?} (all declared)", vm.pub_ip, open),
                duration_ms: elapsed,
                error: None,
                severity: Severity::Info,
            });
        } else {
            let dangerous_open: Vec<u16> = undeclared_open
                .iter()
                .filter(|p| DANGEROUS_PORTS.contains(p))
                .copied()
                .collect();
            if !dangerous_open.is_empty() {
                all_checks.push(Check {
                    name: format!("ext:port-scan:{}:dangerous", vm.alias),
                    passed: false,
                    details: format!("DANGEROUS ports open on {}: {:?}", vm.pub_ip, dangerous_open),
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
                    name: format!("ext:port-scan:{}:undeclared", vm.alias),
                    passed: false,
                    details: format!("Undeclared open ports on {}: {:?}", vm.pub_ip, non_dangerous),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Warning,
                });
            }
            if !declared_closed.is_empty() {
                all_checks.push(Check {
                    name: format!("ext:port-scan:{}:closed", vm.alias),
                    passed: false,
                    details: format!("Declared ports closed on {}: {:?}", vm.pub_ip, declared_closed),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Warning,
                });
            }
        }
    }

    // ── Internal scan: WG IPs (when WG mesh is up) ──
    if caps.wg_up {
        for vm in &ctx.vms {
            if vm.wg_ip.is_empty() || vm.wg_ip == "?" {
                continue;
            }
            let t = Instant::now();

            // Internal scan: check service ports that should be reachable via WG
            let mut internal_ports: Vec<u16> = vec![22]; // SSH always expected
            // Add all service ports for containers on this VM
            for svc_name in &vm.declared_services {
                if let Some(svc) = ctx.services.iter().find(|s| &s.name == svc_name) {
                    if let Some(port) = svc.port {
                        if !internal_ports.contains(&port) {
                            internal_ports.push(port);
                        }
                    }
                    for ct in &svc.containers {
                        if let Some(port) = ct.port {
                            if !internal_ports.contains(&port) {
                                internal_ports.push(port);
                            }
                        }
                    }
                }
            }

            let open = checks::tcp_scan(&vm.wg_ip, &internal_ports).await;
            let elapsed = t.elapsed().as_millis() as u64;

            let closed: Vec<u16> = internal_ports
                .iter()
                .filter(|p| !open.contains(p))
                .copied()
                .collect();

            all_results.push(PortScanResult {
                vm_alias: vm.alias.clone(),
                ip: vm.wg_ip.clone(),
                scan_path: "internal".into(),
                declared_ports: internal_ports.clone(),
                open_ports: open.clone(),
                undeclared_open: vec![],
                declared_closed: closed.clone(),
            });

            if closed.is_empty() {
                all_checks.push(Check {
                    name: format!("int:port-scan:{}", vm.alias),
                    passed: true,
                    details: format!("WG {} — {}/{} service ports open", vm.wg_ip, open.len(), internal_ports.len()),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Info,
                });
            } else {
                all_checks.push(Check {
                    name: format!("int:port-scan:{}:closed", vm.alias),
                    passed: false,
                    details: format!("WG {} — service ports unreachable: {:?}", vm.wg_ip, closed),
                    duration_ms: elapsed,
                    error: None,
                    severity: Severity::Warning,
                });
            }
        }
    }

    (all_checks, all_results)
}
