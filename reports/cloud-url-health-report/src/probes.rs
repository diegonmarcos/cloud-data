//! Async probe implementations for public and private URLs

use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;
use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProbeStatus {
    Ok,
    Fail,
    Skip,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProbeResult {
    pub domain: String,
    pub url: String,
    pub dns: ProbeStatus,
    pub tcp: ProbeStatus,
    pub tls: ProbeStatus,
    pub http: ProbeStatus,
    pub http_code: Option<u16>,
    pub latency_ms: u64,
    pub resolved_ips: Vec<String>,
    pub errors: Vec<String>,
}

impl ProbeResult {
    pub fn is_healthy(&self) -> bool {
        self.dns != ProbeStatus::Fail
            && self.tcp != ProbeStatus::Fail
            && self.tls != ProbeStatus::Fail
            && self.http != ProbeStatus::Fail
    }
}

// ────────────────────────────────────────────────────────────────
// Public URL probe: DNS -> TCP -> TLS -> HTTP (with bearer)
// ────────────────────────────────────────────────────────────────

pub async fn probe_public(domain: &str, bearer: Option<&str>) -> ProbeResult {
    let start = Instant::now();
    let url = format!("https://{}", domain);
    let mut r = ProbeResult {
        domain: domain.to_string(),
        url: url.clone(),
        dns: ProbeStatus::Fail,
        tcp: ProbeStatus::Skip,
        tls: ProbeStatus::Skip,
        http: ProbeStatus::Skip,
        http_code: None,
        latency_ms: 0,
        resolved_ips: vec![],
        errors: vec![],
    };

    // L1: DNS
    let cf = TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default());
    let sys = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let ips = match resolve(&cf, domain).await {
        Ok(ips) if !ips.is_empty() => ips,
        _ => match resolve(&sys, domain).await {
            Ok(ips) if !ips.is_empty() => ips,
            Ok(_) => {
                r.errors.push("DNS: no records".into());
                r.latency_ms = start.elapsed().as_millis() as u64;
                return r;
            }
            Err(e) => {
                r.errors.push(format!("DNS: {}", e));
                r.latency_ms = start.elapsed().as_millis() as u64;
                return r;
            }
        },
    };
    r.dns = ProbeStatus::Ok;
    r.resolved_ips = ips.iter().map(|ip| ip.to_string()).collect();

    // L2: TCP :443
    let addr = SocketAddr::new(ips[0], 443);
    match tokio::time::timeout(Duration::from_secs(5), tokio::net::TcpStream::connect(addr)).await
    {
        Ok(Ok(_)) => r.tcp = ProbeStatus::Ok,
        Ok(Err(e)) => {
            r.tcp = ProbeStatus::Fail;
            r.errors.push(format!("TCP: {}", e));
            r.latency_ms = start.elapsed().as_millis() as u64;
            return r;
        }
        Err(_) => {
            r.tcp = ProbeStatus::Fail;
            r.errors.push("TCP: timeout 5s".into());
            r.latency_ms = start.elapsed().as_millis() as u64;
            return r;
        }
    }

    // L3+L4: TLS + HTTP
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(false)
        .build()
        .unwrap();

    let mut req = client.get(&url);
    if let Some(token) = bearer {
        req = req.header("Authorization", format!("Bearer {}", token));
    }

    match req.send().await {
        Ok(resp) => {
            r.tls = ProbeStatus::Ok;
            let code = resp.status().as_u16();
            r.http_code = Some(code);
            r.http = if code < 500 {
                ProbeStatus::Ok
            } else {
                r.errors.push(format!("HTTP {}", code));
                ProbeStatus::Fail
            };
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            if e.is_connect() {
                if msg.contains("certificate") || msg.contains("tls") || msg.contains("ssl") {
                    r.tls = ProbeStatus::Fail;
                    r.errors.push(format!("TLS: {}", e));
                } else {
                    r.tls = ProbeStatus::Fail;
                    r.errors.push(format!("CONNECT: {}", e));
                }
            } else if e.is_timeout() {
                r.tls = ProbeStatus::Ok;
                r.http = ProbeStatus::Fail;
                r.errors.push("HTTP: timeout 10s".into());
            } else {
                r.tls = ProbeStatus::Fail;
                r.errors.push(format!("REQ: {}", e));
            }
        }
    }

    r.latency_ms = start.elapsed().as_millis() as u64;
    r
}

// ────────────────────────────────────────────────────────────────
// Private URL probe: TCP -> HTTP (plain, no TLS, no auth)
// ────────────────────────────────────────────────────────────────

pub async fn probe_private(name: &str, upstream: &str, vm: &str) -> ProbeResult {
    let start = Instant::now();
    let url = format!("http://{}", upstream);
    let mut r = ProbeResult {
        domain: format!("{} ({})", name, vm),
        url: upstream.to_string(),
        dns: ProbeStatus::Skip,
        tcp: ProbeStatus::Skip,
        tls: ProbeStatus::Skip,
        http: ProbeStatus::Skip,
        http_code: None,
        latency_ms: 0,
        resolved_ips: vec![],
        errors: vec![],
    };

    // Parse host:port
    let parts: Vec<&str> = upstream.splitn(2, ':').collect();
    if parts.len() != 2 {
        r.tcp = ProbeStatus::Fail;
        r.errors.push(format!("bad upstream: {}", upstream));
        r.latency_ms = start.elapsed().as_millis() as u64;
        return r;
    }
    let ip: IpAddr = match parts[0].parse() {
        Ok(ip) => ip,
        Err(_) => {
            r.tcp = ProbeStatus::Fail;
            r.errors.push(format!("bad IP: {}", parts[0]));
            r.latency_ms = start.elapsed().as_millis() as u64;
            return r;
        }
    };
    let port: u16 = match parts[1].parse() {
        Ok(p) => p,
        Err(_) => {
            r.tcp = ProbeStatus::Fail;
            r.errors.push(format!("bad port: {}", parts[1]));
            r.latency_ms = start.elapsed().as_millis() as u64;
            return r;
        }
    };

    // L1: TCP
    let addr = SocketAddr::new(ip, port);
    match tokio::time::timeout(Duration::from_secs(5), tokio::net::TcpStream::connect(addr)).await
    {
        Ok(Ok(_)) => r.tcp = ProbeStatus::Ok,
        Ok(Err(e)) => {
            r.tcp = ProbeStatus::Fail;
            r.errors.push(format!("TCP: {}", e));
            r.latency_ms = start.elapsed().as_millis() as u64;
            return r;
        }
        Err(_) => {
            r.tcp = ProbeStatus::Fail;
            r.errors.push("TCP: timeout 5s".into());
            r.latency_ms = start.elapsed().as_millis() as u64;
            return r;
        }
    }

    // L2: HTTP (plain)
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    match client.get(&url).send().await {
        Ok(resp) => {
            let code = resp.status().as_u16();
            r.http_code = Some(code);
            r.http = if code < 500 {
                ProbeStatus::Ok
            } else {
                r.errors.push(format!("HTTP {}", code));
                ProbeStatus::Fail
            };
        }
        Err(e) => {
            r.http = ProbeStatus::Fail;
            r.errors.push(format!("HTTP: {}", e));
        }
    }

    r.latency_ms = start.elapsed().as_millis() as u64;
    r
}

// ────────────────────────────────────────────────────────────────
// DNS helper
// ────────────────────────────────────────────────────────────────

async fn resolve(resolver: &TokioAsyncResolver, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
    let response = resolver.lookup_ip(domain).await?;
    Ok(response.iter().collect())
}
