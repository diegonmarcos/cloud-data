//! Data-driven Caddy target loader — shared across all report crates.
//!
//! SOURCE OF TRUTH: `build-caddy.json` (output of `build-caddy-routes.ts` derive).
//! Never hard-code URLs/hosts/upstreams here.
//!
//! Exposes:
//!   - `load_public_targets()`  — every public URL Caddy serves (edge: *.diegonmarcos.com).
//!   - `load_private_app_targets()` — `*.app` canonical private URLs (Hickory + Caddy wildcard).
//!   - `load_private_db_targets()` — `*.db` catalog entries.
//!   - `load_private_all_targets()` — app + db (merged).
//!
//! Schema inside build-caddy.json (array elements in categorised keys):
//!   { host, path?, upstream, kind, tls, zone, service?, notes? }
//! except `.routes[]` which is the legacy simplified aggregate:
//!   { domain, upstream, comment, auth? }

use crate::context::find_cloud_data_file;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct CaddyTarget {
    /// Hostname (e.g. `ide.diegonmarcos.com`, `code-server-https-8443.app`).
    pub host: String,
    /// Optional path (e.g. `/crawlee`).
    pub path: Option<String>,
    /// Backend upstream `ip:port` (may be `embedded` for sqlite DB catalog).
    pub upstream: String,
    /// `reverse_proxy` | `catalog` | `redirect` | `canonical` | ...
    pub kind: String,
    /// `public` | `on_demand` | `internal` | `none`.
    pub tls: String,
    /// `com` (public zone) | `app` | `db`.
    pub zone: String,
    /// Service name declared in the source build.json (may be null for shared routes).
    pub service: Option<String>,
    /// Which build-caddy.json key this target came from.
    pub category: String,
    /// Optional auth hint (from legacy `.routes[].auth`).
    pub auth: Option<String>,
    /// Composed full URL (`https://host[/path]`). Used by public probes.
    pub url: String,
}

/// Categories in build-caddy.json that hold public endpoints.
const PUBLIC_CATEGORIES: &[&str] = &[
    "public_A_mcp",
    "public_B_apis",
    "public_C_app_paths",
    "public_D_others",
];

/// Categories in build-caddy.json that hold private `.app` canonical hosts.
const PRIVATE_APP_CATEGORIES: &[&str] = &[
    "private_A0_app_short",
    "private_A1_app_canonical",
    "private_A2_app_portless",
];

/// Load raw `build-caddy.json` value.
pub fn load_build_caddy() -> Option<Value> {
    find_cloud_data_file("build-caddy.json")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
}

fn compose_url(host: &str, path: Option<&str>, tls: &str) -> String {
    let scheme = if tls == "none" { "http" } else { "https" };
    match path {
        Some(p) if !p.is_empty() && p != "null" => format!("{}://{}{}", scheme, host, p),
        _ => format!("{}://{}/", scheme, host),
    }
}

fn parse_entry(v: &Value, category: &str) -> Option<CaddyTarget> {
    let host = v["host"].as_str()?;
    if host.is_empty() || host.starts_with('<') {
        return None; // `<global>`, `<catch-all>` etc.
    }
    let path = v["path"].as_str().map(|s| s.to_string());
    let upstream = v["upstream"].as_str().unwrap_or("").to_string();
    let kind = v["kind"].as_str().unwrap_or("reverse_proxy").to_string();
    let tls = v["tls"].as_str().unwrap_or("public").to_string();
    let zone = v["zone"].as_str().unwrap_or("com").to_string();
    let service = v["service"].as_str().map(|s| s.to_string());
    let url = compose_url(host, path.as_deref(), &tls);
    Some(CaddyTarget {
        host: host.to_string(),
        path,
        upstream,
        kind,
        tls,
        zone,
        service,
        category: category.to_string(),
        auth: None,
        url,
    })
}

fn parse_legacy_route(v: &Value) -> Option<CaddyTarget> {
    let domain = v["domain"].as_str()?;
    if domain.is_empty() || !domain.contains('.') {
        return None;
    }
    let upstream = v["upstream"].as_str().unwrap_or("").to_string();
    let auth = v["auth"].as_str().map(|s| s.to_string());
    let url = format!("https://{}/", domain);
    Some(CaddyTarget {
        host: domain.to_string(),
        path: None,
        upstream,
        kind: "reverse_proxy".to_string(),
        tls: "public".to_string(),
        zone: "com".to_string(),
        service: None,
        category: "routes".to_string(),
        auth,
        url,
    })
}

/// Load all public endpoints (edge-served domains). Deduplicated by (host, path).
pub fn load_public_targets() -> Vec<CaddyTarget> {
    let Some(json) = load_build_caddy() else {
        return Vec::new();
    };

    let mut out: Vec<CaddyTarget> = Vec::new();

    // Legacy simplified `.routes[]` (domain + upstream).
    if let Some(arr) = json["routes"].as_array() {
        out.extend(arr.iter().filter_map(parse_legacy_route));
    }

    // Categorised entries.
    for cat in PUBLIC_CATEGORIES {
        if let Some(arr) = json[cat].as_array() {
            out.extend(arr.iter().filter_map(|v| parse_entry(v, cat)));
        }
    }

    dedup_targets(&mut out);
    out
}

/// Load private `.app` canonical targets (`private_A1_app_canonical` etc.).
pub fn load_private_app_targets() -> Vec<CaddyTarget> {
    let Some(json) = load_build_caddy() else {
        return Vec::new();
    };
    let mut out: Vec<CaddyTarget> = Vec::new();
    for cat in PRIVATE_APP_CATEGORIES {
        if let Some(arr) = json[cat].as_array() {
            out.extend(arr.iter().filter_map(|v| parse_entry(v, cat)));
        }
    }
    dedup_targets(&mut out);
    out
}

/// Load `.db` catalog targets.
pub fn load_private_db_targets() -> Vec<CaddyTarget> {
    let Some(json) = load_build_caddy() else {
        return Vec::new();
    };
    let mut out: Vec<CaddyTarget> = json["private_B0_db"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| parse_entry(v, "private_B0_db")).collect())
        .unwrap_or_default();
    dedup_targets(&mut out);
    out
}

/// Load all private targets (app + db). Deduplicated by (host, path).
pub fn load_private_all_targets() -> Vec<CaddyTarget> {
    let mut out = load_private_app_targets();
    out.extend(load_private_db_targets());
    dedup_targets(&mut out);
    out
}

fn dedup_targets(out: &mut Vec<CaddyTarget>) {
    out.sort_by(|a, b| {
        a.host
            .cmp(&b.host)
            .then(a.path.cmp(&b.path))
            .then(a.upstream.cmp(&b.upstream))
    });
    out.dedup_by(|a, b| a.host == b.host && a.path == b.path);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fire Rule #4: a task is not done until it has a tester.
    ///
    /// Asserts that build-caddy.json is wired in and yields the expected
    /// minimum target counts. These are lower bounds that hold as long as
    /// the core services exist.
    #[test]
    fn loaders_return_non_trivial_sets() {
        // Gate on file presence so the test doesn't fail in sandbox builds
        // that lack cloud-data access.
        if load_build_caddy().is_none() {
            eprintln!("build-caddy.json not found — skipping");
            return;
        }

        let pubs = load_public_targets();
        let apps = load_private_app_targets();
        let dbs = load_private_db_targets();

        assert!(pubs.len() >= 10, "expected ≥10 public targets, got {}", pubs.len());
        assert!(apps.len() >= 50, "expected ≥50 private .app targets, got {}", apps.len());
        assert!(dbs.len() >= 20, "expected ≥20 private .db targets, got {}", dbs.len());

        // Every target has a non-empty URL.
        for t in pubs.iter().chain(apps.iter()).chain(dbs.iter()) {
            assert!(!t.host.is_empty());
            assert!(!t.url.is_empty());
        }

        // At least one .app host uses the .app zone suffix.
        assert!(apps.iter().any(|t| t.host.ends_with(".app")),
                "expected ≥1 .app host in private_app targets");

        // At least one .db host uses the .db zone suffix.
        assert!(dbs.iter().any(|t| t.host.ends_with(".db")),
                "expected ≥1 .db host in private_db targets");
    }

    #[test]
    fn compose_url_joins_host_and_path() {
        assert_eq!(
            compose_url("ide.diegonmarcos.com", None, "public"),
            "https://ide.diegonmarcos.com/"
        );
        assert_eq!(
            compose_url("api.diegonmarcos.com", Some("/crawlee"), "public"),
            "https://api.diegonmarcos.com/crawlee"
        );
        assert_eq!(
            compose_url("x.app", None, "on_demand"),
            "https://x.app/"
        );
        assert_eq!(
            compose_url("x.local", None, "none"),
            "http://x.local/"
        );
    }
}
