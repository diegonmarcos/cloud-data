// VM identifiers
#[allow(dead_code)]
pub const MAIL_VM: &str = "oci-E2-f_0";
pub const MAIL_ALIAS: &str = "oci-mail";
pub const MAIL_WG_IP: &str = "10.0.0.3";

#[allow(dead_code)]
pub const C3_VM: &str = "oci-A1-f_0";
pub const APPS_ALIAS: &str = "oci-apps";
pub const APPS_WG_IP: &str = "10.0.0.6";

#[allow(dead_code)]
pub const PROXY_VM: &str = "gcp-E2-f_0";
pub const PROXY_ALIAS: &str = "gcp-proxy";
pub const PROXY_WG_IP: &str = "10.0.0.1";

// Mail
pub const MAIL_DOMAIN: &str = "mail.diegonmarcos.com";
pub const WEBMAIL_DOMAIN: &str = "webmail.diegonmarcos.com";
pub const AUTH_DOMAIN: &str = "auth.diegonmarcos.com";
pub const MCP_DOMAIN: &str = "mcp.diegonmarcos.com";
pub const BASE_DOMAIN: &str = "diegonmarcos.com";
pub const MAIL_CONTAINERS: &[&str] = &["stalwart"];
pub const EXTRA_CONTAINERS: &[&str] = &["smtp-proxy", "snappymail"];
pub const TEST_FROM: &str = "health@mails.diegonmarcos.com";
pub const TEST_TO: &str = "me@diegonmarcos.com";

// Ports to verify bound on oci-mail
pub const EXPECTED_PORTS: &[u16] = &[25, 443, 465, 587, 993, 4190, 8888];

// Bearer token path (relative to $HOME)
pub const BEARER_TOKEN_PATH: &str =
    "Mounts/Git/vault/A0_keys/providers/authelia/signed-bearer_jwt/tokens/cloud-admin.json";

// SMTP relay hosts
pub const OCI_RELAY_HOST: &str = "smtp.email.eu-marseille-1.oci.oraclecloud.com";
pub const OCI_RELAY_PORT: u16 = 587;
pub const AWS_RELAY_HOST: &str = "email-smtp.us-east-1.amazonaws.com";
pub const AWS_RELAY_PORT: u16 = 587;
pub const SMTP_PROXY_DOMAIN: &str = "smtp.diegonmarcos.com";
pub const SMTP_PROXY_PORT: u16 = 8080;

// Cloudflare Worker
pub const CF_WORKER_URL: &str = "https://email-forwarder.diegonm-workers.workers.dev";

// Google OAuth token endpoint (for Gmail API health)
pub const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

// Stalwart Admin API (via WG direct)
pub const STALWART_ADMIN_PORT: u16 = 443;
