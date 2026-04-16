use crate::context::NetworkContext;
use crate::types::{FirewallAuditResult, PortScanResult};
use reports_common::capabilities::RuntimeCapabilities;
use reports_common::ssh;
use reports_common::types::{Check, Severity};
use std::time::Instant;

/// Audit firewall state on each VM by comparing `ss -tlnp` with declared ports
pub async fn audit_firewalls(
    ctx: &NetworkContext,
    port_data: &[PortScanResult],
    caps: &RuntimeCapabilities,
) -> (Vec<Check>, Vec<FirewallAuditResult>) {
    let mut checks = Vec::new();
    let mut results = Vec::new();

    if !caps.ssh_available {
        checks.push(Check {
            name: "firewall:ssh-unavailable".into(),
            passed: false,
            details: "SSH not available, skipping firewall audit".into(),
            duration_ms: 0,
            error: Some("no SSH".into()),
            severity: Severity::Warning,
        });
        return (checks, results);
    }

    println!("Firewall audit: {} VMs", ctx.vms.len());

    for vm in &ctx.vms {
        let t = Instant::now();

        // SSH echo test first
        if !ssh::ssh_echo_test(&vm.alias).await {
            checks.push(Check {
                name: format!("firewall:{}", vm.alias),
                passed: false,
                details: format!("SSH unreachable, cannot audit firewall"),
                duration_ms: t.elapsed().as_millis() as u64,
                error: Some("SSH unreachable".into()),
                severity: Severity::Warning,
            });
            continue;
        }

        // Run ss -tlnp to list listening TCP sockets
        let ss_output = match ssh::ssh_exec(&vm.alias, "ss -tlnp", 10).await {
            Ok(out) => out,
            Err(e) => {
                checks.push(Check {
                    name: format!("firewall:{}", vm.alias),
                    passed: false,
                    details: format!("Failed to run ss: {}", e),
                    duration_ms: t.elapsed().as_millis() as u64,
                    error: Some(e.to_string()),
                    severity: Severity::Warning,
                });
                continue;
            }
        };

        let elapsed = t.elapsed().as_millis() as u64;

        // Parse ss output for listening ports on 0.0.0.0 or *
        let listening_ports = parse_ss_output(&ss_output);

        // Get declared ports from topology
        let declared: Vec<u16> = vm.public_ports.iter().map(|p| p.port).collect();

        // Also consider ports from port_scan results
        let scan_declared: Vec<u16> = port_data
            .iter()
            .find(|p| p.vm_alias == vm.alias)
            .map(|p| p.declared_ports.clone())
            .unwrap_or_default();

        let mut all_declared: Vec<u16> = declared.clone();
        for p in &scan_declared {
            if !all_declared.contains(p) {
                all_declared.push(*p);
            }
        }

        // Rogue = listening on 0.0.0.0 but not in declared ports
        // Exclude common system ports that are expected (22 SSH, etc.)
        let system_ports: Vec<u16> = vec![22];
        let rogue: Vec<u16> = listening_ports
            .iter()
            .filter(|p| !all_declared.contains(p) && !system_ports.contains(p))
            .copied()
            .collect();

        let result = FirewallAuditResult {
            vm_alias: vm.alias.clone(),
            open_ports: listening_ports.clone(),
            declared_ports: all_declared.clone(),
            rogue_ports: rogue.clone(),
        };

        if rogue.is_empty() {
            checks.push(Check {
                name: format!("firewall:{}", vm.alias),
                passed: true,
                details: format!(
                    "{} listening ports, all declared",
                    listening_ports.len()
                ),
                duration_ms: elapsed,
                error: None,
                severity: Severity::Info,
            });
        } else {
            checks.push(Check {
                name: format!("firewall:{}:rogue", vm.alias),
                passed: false,
                details: format!(
                    "Rogue listeners on 0.0.0.0: {:?}",
                    rogue
                ),
                duration_ms: elapsed,
                error: None,
                severity: Severity::Warning,
            });
        }

        results.push(result);
    }

    (checks, results)
}

/// Parse `ss -tlnp` output and extract ports listening on 0.0.0.0 or [::]
fn parse_ss_output(output: &str) -> Vec<u16> {
    let mut ports = Vec::new();

    for line in output.lines().skip(1) {
        // Lines look like:
        // LISTEN 0  4096  0.0.0.0:443  0.0.0.0:*  users:(("caddy",pid=...))
        // LISTEN 0  4096  [::]:80      [::]:*
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 5 {
            continue;
        }

        let state = fields[0];
        if state != "LISTEN" {
            continue;
        }

        let local_addr = fields[3];

        // Check if bound to all interfaces
        let is_wildcard = local_addr.starts_with("0.0.0.0:")
            || local_addr.starts_with("*:")
            || local_addr.starts_with("[::]:");

        if !is_wildcard {
            continue;
        }

        // Extract port (last part after the last colon)
        if let Some(port_str) = local_addr.rsplit(':').next() {
            if let Ok(port) = port_str.parse::<u16>() {
                if !ports.contains(&port) {
                    ports.push(port);
                }
            }
        }
    }

    ports.sort();
    ports
}
