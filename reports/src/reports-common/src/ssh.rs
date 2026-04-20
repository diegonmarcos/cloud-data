use anyhow::Result;
use std::time::Duration;
use tokio::time::timeout;

const SSH_TIMEOUT: Duration = Duration::from_secs(10);

/// Execute a command on a remote VM via SSH with ControlMaster muxing
pub async fn ssh_exec(vm_alias: &str, command: &str, timeout_secs: u64) -> Result<String> {
    let output = timeout(
        Duration::from_secs(timeout_secs),
        tokio::process::Command::new("ssh")
            .args([
                "-o", "ConnectTimeout=5",
                "-o", "BatchMode=yes",
                "-o", "ControlMaster=auto",
                "-o", "ControlPath=/tmp/cloud-reports-mux-%h",
                "-o", "ControlPersist=30",
                vm_alias,
                command,
            ])
            .output(),
    )
    .await??;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("SSH to {} failed: {}", vm_alias, stderr.trim())
    }
}

/// Execute SSH command returning raw bytes (for binary data like docker export)
pub async fn ssh_exec_raw(vm_alias: &str, command: &str, timeout_secs: u64) -> Result<Vec<u8>> {
    let output = timeout(
        Duration::from_secs(timeout_secs),
        tokio::process::Command::new("ssh")
            .args([
                "-o", "ConnectTimeout=5",
                "-o", "BatchMode=yes",
                "-o", "ControlMaster=auto",
                "-o", "ControlPath=/tmp/cloud-reports-mux-%h",
                "-o", "ControlPersist=30",
                vm_alias,
                command,
            ])
            .output(),
    )
    .await??;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("SSH to {} failed: {}", vm_alias, stderr.trim())
    }
}

/// SSH echo test — verifies SSH auth works
pub async fn ssh_echo_test(vm_alias: &str) -> bool {
    timeout(
        SSH_TIMEOUT,
        tokio::process::Command::new("ssh")
            .args([
                "-o", "ConnectTimeout=5",
                "-o", "BatchMode=yes",
                "-o", "ControlMaster=auto",
                "-o", "ControlPath=/tmp/cloud-reports-mux-%h",
                "-o", "ControlPersist=30",
                vm_alias,
                "echo OK",
            ])
            .output(),
    )
    .await
    .ok()
    .and_then(|r| r.ok())
    .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("OK"))
    .unwrap_or(false)
}
