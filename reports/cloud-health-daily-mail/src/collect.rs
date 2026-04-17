use crate::types::*;
use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

const HTTP_TIMEOUT: Duration = Duration::from_secs(5);

fn http_client() -> Client {
    Client::builder()
        .timeout(HTTP_TIMEOUT)
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

fn auth_client(token: &str) -> Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    Client::builder()
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()
        .unwrap()
}

/// Check endpoint HTTP status with latency measurement
pub async fn check_endpoint(ep: &EndpointCheck) -> EndpointResult {
    let client = http_client();
    let start = std::time::Instant::now();
    let code = match timeout(HTTP_TIMEOUT, client.get(&ep.url).send()).await {
        Ok(Ok(resp)) => resp.status().as_u16(),
        _ => 0,
    };
    let latency_ms = start.elapsed().as_millis() as u64;
    EndpointResult {
        service: ep.service.clone(),
        url: ep.url.clone(),
        status_code: code,
        latency_ms,
    }
}

/// Check TLS certificate expiry via openssl CLI
pub async fn check_cert(domain: &str) -> CertResult {
    let output = timeout(
        Duration::from_secs(8),
        tokio::process::Command::new("sh")
            .args([
                "-c",
                &format!(
                    "echo | openssl s_client -servername {} -connect {}:443 2>/dev/null | openssl x509 -noout -enddate 2>/dev/null | cut -d= -f2",
                    domain, domain
                ),
            ])
            .output(),
    )
    .await;

    let expiry_raw = output
        .ok()
        .and_then(|r| r.ok())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    if expiry_raw.is_empty() {
        return CertResult { domain: domain.into(), days_left: -1, expiry: "N/A".into() };
    }

    // Parse date
    let days_left = tokio::process::Command::new("date")
        .args(["-d", &expiry_raw, "+%s"])
        .output()
        .await
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .and_then(|s| s.parse::<i64>().ok())
        .map(|epoch| {
            let now = chrono::Utc::now().timestamp();
            (epoch - now) / 86400
        })
        .unwrap_or(-1);

    CertResult { domain: domain.into(), days_left, expiry: expiry_raw }
}

/// DNS lookups for diegonmarcos.com
pub async fn check_dns() -> Vec<DnsResult> {
    let mut rc = ResolverConfig::new();
    rc.add_name_server(NameServerConfig::new(
        "1.1.1.1:53".parse().unwrap(),
        Protocol::Udp,
    ));
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(5);
    let resolver = TokioAsyncResolver::tokio(rc, opts);

    let mut results = Vec::new();

    // MX
    let mx_val = match resolver.mx_lookup("diegonmarcos.com").await {
        Ok(mx) => mx.iter().next().map(|m| format!("{} {}", m.preference(), m.exchange())).unwrap_or_default(),
        Err(_) => String::new(),
    };
    results.push(DnsResult { record_type: "MX".into(), value: mx_val });

    // SPF
    let spf_val = match resolver.txt_lookup("diegonmarcos.com").await {
        Ok(txt) => txt.iter().find(|t| t.to_string().contains("v=spf1")).map(|t| t.to_string()).unwrap_or_default(),
        Err(_) => String::new(),
    };
    results.push(DnsResult { record_type: "SPF".into(), value: spf_val });

    // DKIM
    let dkim_val = match resolver.txt_lookup("default._domainkey.diegonmarcos.com").await {
        Ok(txt) => txt.iter().next().map(|t| t.to_string()).unwrap_or_default(),
        Err(_) => String::new(),
    };
    results.push(DnsResult { record_type: "DKIM".into(), value: dkim_val });

    // DMARC
    let dmarc_val = match resolver.txt_lookup("_dmarc.diegonmarcos.com").await {
        Ok(txt) => txt.iter().next().map(|t| t.to_string()).unwrap_or_default(),
        Err(_) => String::new(),
    };
    results.push(DnsResult { record_type: "DMARC".into(), value: dmarc_val });

    results
}

/// Fetch GHA workflow runs
pub async fn fetch_gha(github_token: &str) -> Vec<GhaRun> {
    let client = auth_client(github_token);
    let resp = timeout(
        Duration::from_secs(10),
        client
            .get("https://api.github.com/repos/diegonmarcos/cloud/actions/runs?per_page=10&status=completed")
            .header("User-Agent", "cloud-health-daily-mail")
            .send(),
    )
    .await;

    let json: serde_json::Value = match resp {
        Ok(Ok(r)) => r.json().await.unwrap_or_default(),
        _ => return vec![],
    };

    json["workflow_runs"]
        .as_array()
        .map(|runs| {
            runs.iter()
                .take(10)
                .map(|r| GhaRun {
                    name: r["name"].as_str().unwrap_or("?").to_string(),
                    conclusion: r["conclusion"].as_str().unwrap_or("?").to_string(),
                    created_at: r["created_at"].as_str().unwrap_or("").to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Fetch GHCR packages
pub async fn fetch_ghcr(github_token: &str) -> (Vec<GhcrPackage>, usize) {
    let client = auth_client(github_token);
    let resp = timeout(
        Duration::from_secs(10),
        client
            .get("https://api.github.com/user/packages?package_type=container&per_page=100")
            .header("User-Agent", "cloud-health-daily-mail")
            .send(),
    )
    .await;

    let json: serde_json::Value = match resp {
        Ok(Ok(r)) => r.json().await.unwrap_or_default(),
        _ => return (vec![], 0),
    };

    let arr = json.as_array().cloned().unwrap_or_default();
    let total = arr.len();

    let mut pkgs: Vec<GhcrPackage> = arr
        .iter()
        .map(|p| GhcrPackage {
            name: p["name"].as_str().unwrap_or("?").to_string(),
            updated_at: p["updated_at"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    pkgs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    pkgs.truncate(10);

    (pkgs, total)
}

/// Fetch Dagu DAG status via v2 API at WG IP 10.0.0.4:8070
pub async fn fetch_dags() -> Vec<DagStatus> {
    let client = http_client();
    // Try WG mesh first (production), then localhost (when running inside Dagu container)
    let urls = [
        "http://10.0.0.4:8070/api/v2/dags",
        "http://localhost:8070/api/v2/dags",
        "http://localhost:8070/api/v1/dags",
    ];

    for url in &urls {
        let resp = timeout(Duration::from_secs(5), client.get(*url).send()).await;
        let json: serde_json::Value = match resp {
            Ok(Ok(r)) if r.status().is_success() => r.json().await.unwrap_or_default(),
            _ => continue,
        };

        // v2 format: { "dags": [{ "dag": { "name": ... }, "latestDAGRun": { "statusLabel": ... } }] }
        if let Some(dags) = json["dags"].as_array() {
            return dags.iter().map(|d| {
                DagStatus {
                    name: d["dag"]["name"].as_str()
                        .or(d["fileName"].as_str())
                        .unwrap_or("?").to_string(),
                    status: d["latestDAGRun"]["statusLabel"].as_str()
                        .unwrap_or("not_started").to_string(),
                    started_at: d["latestDAGRun"]["startedAt"].as_str()
                        .unwrap_or("").to_string(),
                }
            }).collect();
        }

        // v1 format: { "DAGs": [{ "DAG": { "Name": ... }, "Status": { "Status": ... } }] }
        if let Some(dags) = json["DAGs"].as_array() {
            return dags.iter().map(|d| {
                DagStatus {
                    name: d["DAG"]["Name"].as_str()
                        .or(d["File"].as_str())
                        .unwrap_or("?").to_string(),
                    status: d["Status"]["Status"].as_str()
                        .unwrap_or("none").to_string(),
                    started_at: d["Status"]["StartedAt"].as_str()
                        .unwrap_or("").to_string(),
                }
            }).collect();
        }
    }

    vec![]
}

/// Fetch OCI bucket sizes via `oci os bucket get` (parallel)
pub async fn fetch_bucket_sizes(buckets: &[CloudBucket]) -> Vec<CloudBucket> {
    let futs: Vec<_> = buckets.iter().map(|bucket| {
        let b = bucket.clone();
        async move {
            let mut result = b.clone();
            if b.provider == "oci" {
                let output = timeout(
                    Duration::from_secs(10),
                    tokio::process::Command::new("oci")
                        .args([
                            "os", "bucket", "get",
                            "--bucket-name", &b.name,
                            "--fields", "approximateSize",
                            "--query", "data.\"approximate-size\"",
                            "--raw-output",
                        ])
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped())
                        .output(),
                )
                .await;

                result.size_bytes = output
                    .ok()
                    .and_then(|r| r.ok())
                    .filter(|o| o.status.success())
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
            }
            result
        }
    }).collect();
    futures::future::join_all(futs).await
}

/// Fetch GitHub repos via `gh repo list`
pub async fn fetch_repos() -> Vec<GithubRepo> {
    let output = timeout(
        Duration::from_secs(15),
        tokio::process::Command::new("gh")
            .args([
                "repo", "list",
                "--limit", "50",
                "--json", "name,visibility,updatedAt,primaryLanguage,diskUsage",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output(),
    )
    .await;

    let json: serde_json::Value = match output {
        Ok(Ok(o)) if o.status.success() => {
            serde_json::from_slice(&o.stdout).unwrap_or_default()
        }
        _ => return vec![],
    };

    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|r| GithubRepo {
                    name: r["name"].as_str().unwrap_or("?").to_string(),
                    visibility: r["visibility"].as_str().unwrap_or("?").to_string(),
                    updated_at: r["updatedAt"].as_str().unwrap_or("").to_string(),
                    language: r["primaryLanguage"]["name"].as_str().unwrap_or("—").to_string(),
                    disk_kb: r["diskUsage"].as_u64().unwrap_or(0),
                })
                .collect()
        })
        .unwrap_or_default()
}
