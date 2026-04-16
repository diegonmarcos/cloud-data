use crate::context::SecDataContext;
use crate::types::ExportedContainer;
use reports_common::capabilities::RuntimeCapabilities;
use reports_common::ssh;
use reports_common::types::{Check, Severity};
use std::time::Instant;
use tempfile::TempDir;
use walkdir::WalkDir;

/// Database container name patterns to skip
const DB_PATTERNS: &[&str] = &["db", "redis", "postgres", "mysql", "mariadb", "mongo"];

/// Check if a container name looks like a database
fn is_database(name: &str) -> bool {
    let lower = name.to_lowercase();
    DB_PATTERNS.iter().any(|p| lower.contains(p))
}

/// Export container filesystems from all VMs
pub async fn export_all(
    ctx: &SecDataContext,
    caps: &RuntimeCapabilities,
) -> (Vec<Check>, Vec<ExportedContainer>) {
    let mut checks = Vec::new();
    let mut exports = Vec::new();

    if !caps.ssh_available || !caps.wg_up {
        checks.push(Check {
            name: "Container export".into(),
            passed: true,
            details: "Skipped — no SSH/WG connectivity".into(),
            duration_ms: 0,
            error: None,
            severity: Severity::Info,
        });
        return (checks, exports);
    }

    for vm in &ctx.vms {
        let t = Instant::now();
        match export_vm(&vm.alias).await {
            Ok((vm_checks, vm_exports)) => {
                let ms = t.elapsed().as_millis() as u64;
                checks.push(Check {
                    name: format!("Export {}", vm.alias),
                    passed: true,
                    details: format!(
                        "{} containers exported ({} files)",
                        vm_exports.len(),
                        vm_exports.iter().map(|e| e.file_count).sum::<usize>()
                    ),
                    duration_ms: ms,
                    error: None,
                    severity: Severity::Info,
                });
                checks.extend(vm_checks);
                exports.extend(vm_exports);
            }
            Err(e) => {
                let ms = t.elapsed().as_millis() as u64;
                checks.push(Check {
                    name: format!("Export {}", vm.alias),
                    passed: false,
                    details: format!("Failed: {}", e),
                    duration_ms: ms,
                    error: Some(e.to_string()),
                    severity: Severity::Warning,
                });
            }
        }
    }

    (checks, exports)
}

/// Export containers from a single VM
async fn export_vm(vm_alias: &str) -> anyhow::Result<(Vec<Check>, Vec<ExportedContainer>)> {
    let mut checks = Vec::new();
    let mut exports = Vec::new();

    // List running containers
    let output = ssh::ssh_exec(vm_alias, "docker ps --format '{{.Names}}'", 15).await?;
    let containers: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();

    if containers.is_empty() {
        checks.push(Check {
            name: format!("{}: no containers", vm_alias),
            passed: true,
            details: "No running containers found".into(),
            duration_ms: 0,
            error: None,
            severity: Severity::Info,
        });
        return Ok((checks, exports));
    }

    for container in containers {
        let container = container.trim();
        if is_database(container) {
            checks.push(Check {
                name: format!("{}:{}", vm_alias, container),
                passed: true,
                details: "Skipped (database container)".into(),
                duration_ms: 0,
                error: None,
                severity: Severity::Info,
            });
            continue;
        }

        let t = Instant::now();
        match export_container(vm_alias, container).await {
            Ok(exported) => {
                let ms = t.elapsed().as_millis() as u64;
                checks.push(Check {
                    name: format!("{}:{}", vm_alias, container),
                    passed: true,
                    details: format!(
                        "{} files, {} bytes",
                        exported.file_count, exported.total_bytes
                    ),
                    duration_ms: ms,
                    error: None,
                    severity: Severity::Info,
                });
                exports.push(exported);
            }
            Err(e) => {
                let ms = t.elapsed().as_millis() as u64;
                checks.push(Check {
                    name: format!("{}:{}", vm_alias, container),
                    passed: false,
                    details: format!("Export failed: {}", e),
                    duration_ms: ms,
                    error: Some(e.to_string()),
                    severity: Severity::Warning,
                });
            }
        }
    }

    Ok((checks, exports))
}

/// Export a single container's key config files to a tempdir.
/// Uses `docker cp` for selective extraction (avoids full `docker export` which OOMs E2 Micros).
async fn export_container(
    vm_alias: &str,
    container: &str,
) -> anyhow::Result<ExportedContainer> {
    let tmpdir = TempDir::new()?;
    let tmppath = tmpdir.path().to_string_lossy().to_string();

    // Use `docker cp` for specific directories — much lighter than `docker export`
    let dirs_to_scan = ["etc", "opt", "app", "usr/local/bin", "usr/local/sbin"];
    let mut total_file_count = 0usize;
    let mut total_bytes = 0u64;

    for dir in &dirs_to_scan {
        let local_dir = format!("{}/{}", tmppath, dir.replace('/', "_"));
        let _ = std::fs::create_dir_all(&local_dir);

        // docker cp <container>:/<dir> to a tar stream, pipe via SSH to local
        let cmd = format!(
            "docker cp {}:/{} - 2>/dev/null | head -c 52428800 || true",
            container, dir
        );
        let raw_data = ssh::ssh_exec_raw(vm_alias, &cmd, 30).await.unwrap_or_default();

        if raw_data.len() > 10 {
            let tar_path = format!("{}/tmp.tar", local_dir);
            std::fs::write(&tar_path, &raw_data)?;

            let _ = std::process::Command::new("tar")
                .args(["xf", &tar_path, "-C", &local_dir])
                .status();

            let _ = std::fs::remove_file(&tar_path);

            let (fc, fb) = count_dir_stats(&local_dir);
            total_file_count += fc;
            total_bytes += fb;
        }
    }

    let path = tmpdir.path().to_string_lossy().to_string();
    let _ = tmpdir.keep();

    Ok(ExportedContainer {
        vm_alias: vm_alias.to_string(),
        container_name: container.to_string(),
        export_path: path,
        file_count: total_file_count,
        total_bytes,
    })
}

/// Count files and total bytes in a directory
fn count_dir_stats(dir: &str) -> (usize, u64) {
    let mut count = 0usize;
    let mut bytes = 0u64;
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            count += 1;
            bytes += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }
    (count, bytes)
}

/// Clean up all exported temp directories
pub fn cleanup(exports: &[ExportedContainer]) {
    for export in exports {
        if std::path::Path::new(&export.export_path).exists() {
            if let Err(e) = std::fs::remove_dir_all(&export.export_path) {
                eprintln!(
                    "  Warning: failed to cleanup {}: {}",
                    export.export_path, e
                );
            }
        }
    }
}
