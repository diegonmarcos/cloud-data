//! Container Health Reporter — Rust
//! Native async TCP/HTTP/DNS checks, no shell subprocess spawning
//!
//! Usage: cargo run --release

use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

const HICKORY_DNS: &str = "10.0.0.1";
const CONSOLIDATED_PATH: &str = "../_cloud-data-consolidated.json";
const TCP_TIMEOUT: Duration = Duration::from_secs(3);
const HTTP_TIMEOUT: Duration = Duration::from_secs(8);

#[derive(Debug, Serialize)]
struct Report {
    generated: String,
    duration_ms: u64,
    mesh: Vec<MeshPeer>,
    public_urls: Vec<UrlCheck>,
    private_endpoints: Vec<PrivateCheck>,
    timers: HashMap<String, u64>,
}

#[derive(Debug, Serialize)]
struct MeshPeer {
    name: String,
    cloud_name: String,
    pub_ip: String,
    wg_ip: String,
    tcp_22: bool,
    tcp_2200: bool,
}

#[derive(Debug, Serialize)]
struct UrlCheck {
    url: String,
    upstream: String,
    tcp: bool,
    http: bool,
    https: bool,
    auth: bool,
    code: String,
    auth_code: String,
}

#[derive(Debug, Serialize)]
struct PrivateCheck {
    dns: String,
    container: String,
    port: u16,
    vm: String,
    tcp: bool,
    http: bool,
    resolved_ip: String,
}

// ── Native checks (no shell, no nc, no curl) ─────────────

async fn tcp_check(ip: &str, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port);
    match addr.parse::<std::net::SocketAddr>() {
        Ok(sa) => timeout(TCP_TIMEOUT, TcpStream::connect(sa)).await.map(|r| r.is_ok()).unwrap_or(false),
        Err(_) => {
            // Try DNS resolve
            match tokio::net::lookup_host(&addr).await {
                Ok(mut addrs) => match addrs.next() {
                    Some(sa) => timeout(TCP_TIMEOUT, TcpStream::connect(sa)).await.map(|r| r.is_ok()).unwrap_or(false),
                    None => false,
                },
                Err(_) => false,
            }
        }
    }
}

async fn http_get(client: &Client, url: &str) -> (bool, String) {
    match timeout(HTTP_TIMEOUT, client.get(url).send()).await {
        Ok(Ok(resp)) => {
            let code = resp.status().as_u16();
            (code != 0 && code != 502, code.to_string())
        }
        _ => (false, "---".to_string()),
    }
}

async fn dns_resolve(resolver: &TokioAsyncResolver, name: &str) -> Option<String> {
    match timeout(Duration::from_secs(3), resolver.lookup_ip(name)).await {
        Ok(Ok(lookup)) => lookup.iter().next().map(|ip| ip.to_string()),
        _ => None,
    }
}

// ── Main ────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let mut timers: HashMap<String, u64> = HashMap::new();
    println!("═══ Rust Health Reporter ═══");

    // Load consolidated
    let raw = std::fs::read_to_string(CONSOLIDATED_PATH).unwrap_or_else(|_| "{}".to_string());
    let c: serde_json::Value = serde_json::from_str(&raw)?;
    println!("Loaded consolidated ({} keys)", c.as_object().map(|o| o.len()).unwrap_or(0));

    // HTTP clients (connection pooling built-in)
    let client = Client::builder()
        .timeout(HTTP_TIMEOUT)
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .pool_max_idle_per_host(20)
        .build()?;

    // Bearer token
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/diego".to_string());
    let token: Option<String> = std::fs::read_to_string(format!("{}/Mounts/Git/vault/A0_keys/providers/authelia/signed-bearer_jwt/tokens/cloud-admin.json", home))
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["access_token"].as_str().map(|s| s.to_string()));

    let auth_client = match &token {
        Some(t) => {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Authorization", format!("Bearer {}", t).parse()?);
            Client::builder()
                .timeout(HTTP_TIMEOUT)
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::limited(5))
                .default_headers(headers)
                .pool_max_idle_per_host(20)
                .build()?
        }
        None => client.clone(),
    };
    println!("Bearer: {}", if token.is_some() { "✅" } else { "❌" });

    // Hickory DNS resolver (native, no dig subprocess)
    let mut rc = ResolverConfig::new();
    rc.add_name_server(NameServerConfig::new(format!("{}:53", HICKORY_DNS).parse()?, Protocol::Udp));
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(3);
    opts.attempts = 2;
    let resolver = TokioAsyncResolver::tokio(rc, opts);

    // ── A0: Mesh ────────────────────────────────────────
    println!("A0) Mesh...");
    let t = Instant::now();
    let vms = c["vms"].as_object().cloned().unwrap_or_default();
    let mesh_futs: Vec<_> = vms.iter().map(|(_, vm)| {
        let alias = vm["ssh_alias"].as_str().unwrap_or("?").to_string();
        let pub_ip = vm["ip"].as_str().unwrap_or("?").to_string();
        let wg_ip = vm["wg_ip"].as_str().unwrap_or("?").to_string();
        let cloud_name = vm["specs"]["cloud_name"].as_str().unwrap_or("—").to_string();
        let rp = vm["rescue_port"].as_u64().unwrap_or(2200) as u16;
        let ip = pub_ip.clone();
        async move {
            let (t22, t2200) = tokio::join!(tcp_check(&ip, 22), tcp_check(&ip, rp));
            MeshPeer { name: alias, cloud_name, pub_ip, wg_ip, tcp_22: t22, tcp_2200: t2200 }
        }
    }).collect();
    let mesh: Vec<MeshPeer> = futures::future::join_all(mesh_futs).await;
    timers.insert("mesh".into(), t.elapsed().as_millis() as u64);
    println!("  {} peers in {}ms", mesh.len(), t.elapsed().as_millis());

    // ── A1: Public URLs ─────────────────────────────────
    println!("A1) Public URLs...");
    let t = Instant::now();
    let routes = c["configs"]["caddy"]["routes"].as_array().cloned().unwrap_or_default();
    let url_futs: Vec<_> = routes.iter().filter_map(|r| {
        let domain = r["domain"].as_str()?.to_string();
        let upstream = r["upstream"].as_str().unwrap_or("?").to_string();
        let cl = client.clone();
        let acl = auth_client.clone();
        Some(async move {
            let http_url = format!("http://{}", domain);
            let https_url = format!("https://{}", domain);
            let auth_url = https_url.clone();
            let (tcp, http_r, https_r, auth_r) = tokio::join!(
                tcp_check(&domain, 443),
                http_get(&cl, &http_url),
                http_get(&cl, &https_url),
                http_get(&acl, &auth_url),
            );
            UrlCheck {
                url: domain, upstream, tcp,
                http: http_r.0, https: https_r.0,
                auth: auth_r.0 && auth_r.1 != "401" && auth_r.1 != "403",
                code: if https_r.0 { https_r.1 } else { http_r.1 },
                auth_code: auth_r.1,
            }
        })
    }).collect();
    let public_urls: Vec<UrlCheck> = futures::future::join_all(url_futs).await;
    timers.insert("public_urls".into(), t.elapsed().as_millis() as u64);
    let https_ok = public_urls.iter().filter(|u| u.https).count();
    let auth_ok = public_urls.iter().filter(|u| u.auth).count();
    println!("  {}/{} HTTPS, {}/{} AUTH in {}ms", https_ok, public_urls.len(), auth_ok, public_urls.len(), t.elapsed().as_millis());

    // ── A2: Private endpoints ───────────────────────────
    println!("A2) Private (Hickory)...");
    let t = Instant::now();
    let hickory_up = dns_resolve(&resolver, "authelia.app").await.is_some();
    println!("  Hickory: {}", if hickory_up { "✅" } else { "❌" });

    let mut private_endpoints: Vec<PrivateCheck> = Vec::new();
    if hickory_up {
        let vm_alias: HashMap<String, String> = vms.iter()
            .map(|(id, v)| (id.clone(), v["ssh_alias"].as_str().unwrap_or(id).to_string()))
            .collect();

        let mut futs = Vec::new();
        for (_, svc) in c["services"].as_object().unwrap_or(&serde_json::Map::new()) {
            let vm = vm_alias.get(svc["vm"].as_str().unwrap_or("")).cloned().unwrap_or_default();
            for (_, ct) in svc["containers"].as_object().unwrap_or(&serde_json::Map::new()) {
                let dns = ct["dns"].as_str().unwrap_or("").to_string();
                let port = ct["port"].as_u64().unwrap_or(0) as u16;
                let cname = ct["container_name"].as_str().unwrap_or("?").to_string();
                if dns.is_empty() || port == 0 { continue; }
                let r = resolver.clone();
                let cl = client.clone();
                let v = vm.clone();
                futs.push(async move {
                    let ip = dns_resolve(&r, &dns).await;
                    match ip {
                        Some(ref resolved) => {
                            let http_url = format!("http://{}:{}", resolved, port);
                            let (tcp, http_r) = tokio::join!(
                                tcp_check(resolved, port),
                                http_get(&cl, &http_url),
                            );
                            PrivateCheck { dns, container: cname, port, vm: v, tcp, http: http_r.0, resolved_ip: resolved.clone() }
                        }
                        None => PrivateCheck { dns, container: cname, port, vm: v, tcp: false, http: false, resolved_ip: String::new() },
                    }
                });
            }
        }
        private_endpoints = futures::future::join_all(futs).await;
    }
    timers.insert("private".into(), t.elapsed().as_millis() as u64);
    let tcp_priv = private_endpoints.iter().filter(|p| p.tcp).count();
    println!("  {}/{} TCP in {}ms", tcp_priv, private_endpoints.len(), t.elapsed().as_millis());

    // ── Summary ─────────────────────────────────────────
    let total = start.elapsed().as_millis() as u64;
    timers.insert("TOTAL".into(), total);

    let report = Report {
        generated: Utc::now().to_rfc3339(),
        duration_ms: total,
        mesh, public_urls, private_endpoints,
        timers: timers.clone(),
    };

    std::fs::write("container_health_rust.json", serde_json::to_string_pretty(&report)?)?;

    println!("\n═══ DONE in {:.1}s ═══", total as f64 / 1000.0);
    println!("\nTimers:");
    let mut st: Vec<_> = timers.iter().collect();
    st.sort_by(|a, b| b.1.cmp(a.1));
    for (k, v) in st { println!("  {:<20} {:.1}s", k, *v as f64 / 1000.0); }

    Ok(())
}
