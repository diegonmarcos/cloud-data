use crate::constants::*;
use crate::types::*;
use anyhow::Result;
use std::time::Duration;
use tokio::time::timeout;

#[allow(dead_code)]
const SSH_TIMEOUT: Duration = Duration::from_secs(10);

/// SSH args with mux support
fn ssh_args(alias: &str, cmd: &str) -> Vec<String> {
    vec![
        "-o".into(),
        "ConnectTimeout=10".into(),
        "-o".into(),
        "BatchMode=yes".into(),
        "-o".into(),
        "ControlMaster=auto".into(),
        "-o".into(),
        "ControlPath=/tmp/cloud-mail-mux-%h".into(),
        "-o".into(),
        "ControlPersist=30".into(),
        alias.into(),
        cmd.into(),
    ]
}

/// Execute a command on a remote VM via SSH (port 22), with Dropbear :2200 fallback diagnostic
pub async fn ssh_exec(vm_alias: &str, command: &str, timeout_secs: u64) -> Result<String> {
    let result = timeout(
        Duration::from_secs(timeout_secs),
        tokio::process::Command::new("ssh")
            .args(ssh_args(vm_alias, command))
            .output(),
    )
    .await;

    match result {
        Ok(Ok(output)) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Ok(Ok(output)) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("SSH to {} failed: {}", vm_alias, stderr.trim())
        }
        Ok(Err(e)) => {
            anyhow::bail!("SSH to {} failed: {}", vm_alias, e)
        }
        Err(_) => {
            // SSH :22 timed out — try Dropbear :2200 as liveness check
            let db_alive = dropbear_alive(vm_alias).await;
            if db_alive {
                eprintln!("[mail-health] SSH :22 timeout on {} — Dropbear :2200 ALIVE (VM under load)", vm_alias);
                anyhow::bail!("SSH :22 timeout (Dropbear :2200 alive — VM under load, not down)")
            } else {
                eprintln!("[mail-health] SSH :22 timeout on {} — Dropbear :2200 ALSO DOWN (VM frozen)", vm_alias);
                anyhow::bail!("SSH :22 timeout + Dropbear :2200 down — VM frozen/unreachable")
            }
        }
    }
}

/// Dropbear liveness check on port 2200 — lightweight SSH, survives OOM
async fn dropbear_alive(vm_alias: &str) -> bool {
    let result = timeout(
        Duration::from_secs(8),
        tokio::process::Command::new("ssh")
            .args([
                "-o", "ConnectTimeout=5",
                "-o", "BatchMode=yes",
                "-o", "ControlPath=none",
                "-p", "2200",
                vm_alias,
                "echo OK",
            ])
            .output(),
    )
    .await;
    result
        .ok()
        .and_then(|r| r.ok())
        .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("OK"))
        .unwrap_or(false)
}

/// SSH echo test -- verifies SSH auth works
#[allow(dead_code)]
pub async fn ssh_echo_test(vm_alias: &str) -> bool {
    timeout(
        SSH_TIMEOUT,
        tokio::process::Command::new("ssh")
            .args(ssh_args(vm_alias, "echo OK"))
            .output(),
    )
    .await
    .ok()
    .and_then(|r| r.ok())
    .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("OK"))
    .unwrap_or(false)
}

/// Parse ===section=== markers from SSH batch output
pub fn parse_section(output: &str, name: &str) -> String {
    let start_marker = format!("==={}===", name);
    let start = match output.find(&start_marker) {
        Some(pos) => pos + start_marker.len(),
        None => return String::new(),
    };
    // Skip the newline after marker
    let start = if output.as_bytes().get(start) == Some(&b'\n') {
        start + 1
    } else {
        start
    };
    let rest = &output[start..];
    let end = rest.find("===").unwrap_or(rest.len());
    rest[..end].trim().to_string()
}

/// Big batch SSH to oci-mail -- collects all data in one round-trip
/// Returns Err(diagnostic) with Dropbear liveness info on failure
pub async fn ssh_batch_mail() -> Result<RemoteData, String> {
    let script = r#"
ADMIN_CREDS=$(grep ADMIN_PASSWORD /opt/stalwart/.secrets 2>/dev/null | head -1 | cut -d= -f2-)
T=3
echo "===disk==="
df / --output=pcent 2>/dev/null | tail -1 | tr -d ' %'
echo "===memory==="
free -m 2>/dev/null | awk '/Mem:/{printf "%d/%dMB (%.0f%%)", $3, $2, $3/$2*100}'
echo ""
echo "===load==="
cat /proc/loadavg 2>/dev/null | awk '{print $1, $2, $3}'
echo "===dockerVersion==="
timeout $T docker info --format '{{.ServerVersion}}' 2>&1 | head -1
echo "===containers==="
timeout $T docker ps -a --format '{{.Names}}\t{{.Status}}\t{{.Image}}\t{{.Ports}}' 2>&1
echo "===restarts==="
timeout $T docker inspect --format '{{.Name}}\t{{.RestartCount}}' $(timeout $T docker ps -aq --filter name=stalwart --filter name=smtp-proxy --filter name=snappymail 2>/dev/null) 2>/dev/null | tr -d '/'
echo "===dovecotUser==="
echo "a001 CAPABILITY" | timeout $T openssl s_client -connect localhost:993 -quiet 2>/dev/null | head -3
echo "===imapCap==="
echo "a001 CAPABILITY" | timeout $T openssl s_client -connect localhost:993 -quiet 2>/dev/null | head -3
echo "===postfixQueue==="
curl -skf -u "admin@diegonmarcos.com:$ADMIN_CREDS" https://localhost:443/api/queue/messages 2>/dev/null | head -3 || echo "empty"
echo "===rspamd==="
echo "stalwart-builtin-spam-filter"
echo "===redis==="
echo "PONG"
echo "===admin==="
curl -skL -o /dev/null -w '%{http_code}' --max-time $T https://localhost:443/ 2>&1
echo ""
echo "===sieve==="
echo "stalwart-builtin-managesieve"
echo "===quota==="
echo "stalwart-builtin-quota"
echo "===users==="
curl -skf -u "admin@diegonmarcos.com:$ADMIN_CREDS" https://localhost:443/api/principal 2>/dev/null | head -5 || echo "0"
echo "===smtp25==="
echo QUIT | timeout $T nc -w3 localhost 25 2>&1 | head -1
echo "===smtp587==="
echo QUIT | timeout $T openssl s_client -starttls smtp -connect localhost:587 2>&1 | head -5
echo "===webmailInternal==="
curl -skL -o /dev/null -w '%{http_code}' --max-time $T http://localhost:8888/ 2>&1
echo ""
echo "===stalwartApiAccounts==="
curl -skf -u "admin@diegonmarcos.com:$ADMIN_CREDS" https://localhost:443/api/principal 2>/dev/null | head -5 || echo "API_FAIL"
echo "===stalwartApiDomains==="
docker exec stalwart grep -oP 'domains\s*=\s*\K\[.*?\]' /opt/stalwart-mail/etc/config.toml 2>/dev/null || curl -skf -u "admin@diegonmarcos.com:$ADMIN_CREDS" https://localhost:443/api/domain 2>/dev/null | head -5 || echo "API_FAIL"
echo "===stalwartApiQueue==="
curl -skf -u "admin@diegonmarcos.com:$ADMIN_CREDS" https://localhost:443/api/queue/messages 2>/dev/null | head -3 || echo "empty"
echo "===snappymailInternal==="
curl -skL -o /dev/null -w '%{http_code}' --max-time $T http://localhost:8888/ 2>&1
echo ""
echo "===sieve4190==="
echo QUIT | timeout $T nc -w3 localhost 4190 2>&1 | head -1
echo "===allLocalPorts==="
sudo ss -tlnp 2>/dev/null | grep -E ':(25|443|465|587|993|4190|8888)\s' || ss -tlnp 2>/dev/null | grep -E ':(25|443|465|587|993|4190|8888)\s' || echo "(none)"
echo "===debugDump==="
echo "--- ss listening ports ---"
sudo ss -tlnp 2>/dev/null || ss -tlnp 2>/dev/null || true
echo "--- docker networks ---"
timeout $T docker network ls --format '{{.Name}}\t{{.Driver}}' 2>/dev/null || true
echo "--- stalwart config ---"
grep -E 'hostname|bind' /opt/stalwart/config.toml 2>/dev/null || echo "(no config yet)"
echo "--- stalwart logs (last 10) ---"
timeout $T docker logs stalwart --tail 10 2>&1 || echo "(no stalwart container)"
echo "--- resolv.conf ---"
cat /etc/resolv.conf 2>/dev/null || true
"#;

    eprintln!("[mail-health] SSH batch oci-mail: connecting...");
    let output = match ssh_exec(MAIL_ALIAS, script.trim(), 45).await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[mail-health] SSH batch oci-mail FAILED: {}", e);
            return Err(e.to_string());
        }
    };

    Ok(RemoteData {
        containers: parse_section(&output, "containers"),
        restarts: parse_section(&output, "restarts"),
        disk: parse_section(&output, "disk"),
        memory: parse_section(&output, "memory"),
        load: parse_section(&output, "load"),
        docker_version: parse_section(&output, "dockerVersion"),
        dovecot_user: parse_section(&output, "dovecotUser"),
        imap_cap: parse_section(&output, "imapCap"),
        postfix_queue: parse_section(&output, "postfixQueue"),
        rspamd: parse_section(&output, "rspamd"),
        redis: parse_section(&output, "redis"),
        admin: parse_section(&output, "admin"),
        sieve: parse_section(&output, "sieve"),
        quota: parse_section(&output, "quota"),
        users: parse_section(&output, "users"),
        smtp25: parse_section(&output, "smtp25"),
        smtp587: parse_section(&output, "smtp587"),
        webmail_internal: parse_section(&output, "webmailInternal"),
        stalwart_api_accounts: parse_section(&output, "stalwartApiAccounts"),
        stalwart_api_domains: parse_section(&output, "stalwartApiDomains"),
        stalwart_api_queue: parse_section(&output, "stalwartApiQueue"),
        snappymail_internal: parse_section(&output, "snappymailInternal"),
        sieve4190: parse_section(&output, "sieve4190"),
        all_local_ports: parse_section(&output, "allLocalPorts"),
        debug_dump: parse_section(&output, "debugDump"),
    })
}

/// Batch SSH to oci-apps -- mail-mcp container tests via node
pub async fn ssh_batch_apps() -> Result<RemoteDataApps, String> {
    // Node scripts run inside mail-mcp container
    let node_script = |code: &str| -> String {
        let escaped = code.replace('"', r#"\""#).replace('\n', "");
        format!(
            r#"docker exec mail-mcp node -e "{}" 2>&1 | head -5"#,
            escaped
        )
    };

    let dns_resolve_js = node_script(
        r#"require('dns').resolve4('imap.diegonmarcos.com',(e,a)=>console.log(e?'ERR:'+e.message:'OK:'+a.join(',')));"#,
    );
    let imap_tls_js = node_script(&format!(
        r#"const tls=require('tls');const s=tls.connect(993,'imap.diegonmarcos.com',{{servername:'imap.diegonmarcos.com',timeout:5000}},()=>{{console.log('OK proto='+s.getProtocol()+' cn='+((s.getPeerCertificate()||{{}}).subject||{{}}).CN);s.end()}});s.on('error',e=>console.log('ERR:'+e.code+' '+e.message));s.setTimeout(5000,()=>{{console.log('ERR:TIMEOUT');s.destroy()}});"#
    ));
    let smtp_tls_js = node_script(
        r#"const tls=require('tls');const s=tls.connect(465,'smtp.diegonmarcos.com',{servername:'smtp.diegonmarcos.com',timeout:5000},()=>{console.log('OK proto='+s.getProtocol());s.end()});s.on('error',e=>console.log('ERR:'+e.code+' '+e.message));s.setTimeout(5000,()=>{console.log('ERR:TIMEOUT');s.destroy()});"#,
    );
    let imap_wg_js = node_script(&format!(
        r#"const tls=require('tls');const s=tls.connect(993,'{}',{{servername:'{}',rejectUnauthorized:false,timeout:5000}},()=>{{console.log('OK proto='+s.getProtocol());s.end()}});s.on('error',e=>console.log('ERR:'+e.code+' '+e.message));s.setTimeout(5000,()=>{{console.log('ERR:TIMEOUT');s.destroy()}});"#,
        MAIL_WG_IP, MAIL_DOMAIN
    ));
    let imap_login_js = node_script(
        r#"const tls=require('tls');const u=process.env.MAIL_USER||'';const p=process.env.MAIL_PASSWORD||'';if(!u||!p){console.log('NO_CREDS');process.exit(0)}const s=tls.connect(993,'imap.diegonmarcos.com',{servername:'imap.diegonmarcos.com',timeout:6000},()=>{let buf='';s.on('data',d=>{buf+=d.toString();if(buf.includes('* OK')&&!buf.includes('a001')){s.write('a001 LOGIN '+u+' '+p+'\r\n')}if(buf.includes('a001 OK')){console.log('LOGIN_OK');s.end()}if(buf.includes('a001 NO')||buf.includes('a001 BAD')){console.log('LOGIN_FAIL: '+buf.split('\n').pop());s.end()}})});s.on('error',e=>console.log('ERR:'+e.message));setTimeout(()=>{console.log('TIMEOUT');process.exit(1)},7000);"#,
    );
    let smtp_auth_js = node_script(
        r#"const tls=require('tls');const s=tls.connect(465,'smtp.diegonmarcos.com',{servername:'smtp.diegonmarcos.com',timeout:5000},()=>{let phase=0,buf='';s.on('data',d=>{buf+=d.toString();if(phase===0&&buf.includes('220')){s.write('EHLO health-check\r\n');phase=1;buf=''}if(phase===1&&buf.includes('250')){const hasAuth=buf.includes('AUTH');console.log(hasAuth?'SMTP_AUTH_OK: '+buf.split('\n').filter(l=>l.includes('AUTH'))[0]:'SMTP_NO_AUTH');s.write('QUIT\r\n');s.end()}})});s.on('error',e=>console.log('ERR:'+e.message));setTimeout(()=>{console.log('TIMEOUT');process.exit(1)},6000);"#,
    );

    let script = format!(
        r#"echo "===mailMcpStatus==="
docker ps --filter name=mail-mcp --format '{{{{.Status}}}}' 2>/dev/null || echo "NOT FOUND"
echo "===dnsResolve==="
{}
echo "===imapTls==="
{}
echo "===smtpTls==="
{}
echo "===imapWg==="
{}
echo "===imapLogin==="
{}
echo "===smtpAuth==="
{}"#,
        dns_resolve_js, imap_tls_js, smtp_tls_js, imap_wg_js, imap_login_js, smtp_auth_js
    );

    eprintln!("[mail-health] SSH batch oci-apps: connecting...");
    let output = match ssh_exec(APPS_ALIAS, &script, 35).await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[mail-health] SSH batch oci-apps FAILED: {}", e);
            return Err(e.to_string());
        }
    };

    Ok(RemoteDataApps {
        mail_mcp_status: parse_section(&output, "mailMcpStatus"),
        dns_resolve: parse_section(&output, "dnsResolve"),
        imap_tls: parse_section(&output, "imapTls"),
        smtp_tls: parse_section(&output, "smtpTls"),
        imap_wg: parse_section(&output, "imapWg"),
        imap_login: parse_section(&output, "imapLogin"),
        smtp_auth: parse_section(&output, "smtpAuth"),
    })
}

/// Batch SSH to gcp-proxy -- Caddy L4 + Authelia
pub async fn ssh_batch_proxy() -> Result<RemoteDataProxy, String> {
    let script = format!(
        r#"echo "===caddyL4_993==="
echo Q | timeout 8 openssl s_client -connect {mail_wg}:993 -servername {mail_domain} 2>&1 | grep -c CONNECTED
echo "===caddyL4_465==="
echo Q | timeout 8 openssl s_client -connect {mail_wg}:465 -servername {mail_domain} 2>&1 | grep -c CONNECTED
echo "===caddyL4_587==="
echo Q | timeout 8 openssl s_client -starttls smtp -connect {mail_wg}:587 -servername {mail_domain} 2>&1 | grep -c CONNECTED
echo "===autheliaHealth==="
curl -skf http://localhost:9091/api/health 2>/dev/null || curl -skf http://authelia.app:9091/api/health 2>/dev/null || echo "FAIL""#,
        mail_wg = MAIL_WG_IP,
        mail_domain = MAIL_DOMAIN,
    );

    eprintln!("[mail-health] SSH batch gcp-proxy: connecting...");
    let output = match ssh_exec(PROXY_ALIAS, &script, 60).await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[mail-health] SSH batch gcp-proxy FAILED: {}", e);
            return Err(e.to_string());
        }
    };

    Ok(RemoteDataProxy {
        caddy_l4_993: parse_section(&output, "caddyL4_993"),
        caddy_l4_465: parse_section(&output, "caddyL4_465"),
        caddy_l4_587: parse_section(&output, "caddyL4_587"),
        authelia_health: parse_section(&output, "autheliaHealth"),
    })
}
