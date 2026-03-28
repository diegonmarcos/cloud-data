#!/usr/bin/env npx tsx
/**
 * Container Health Reporter
 * Collects live data → container_health.json + container_health.md
 * Usage: npx tsx container-health.ts
 */
import { execSync } from "child_process";
import { writeFileSync, readFileSync, existsSync } from "fs";
import { join } from "path";

const HOME = process.env.HOME || "/home/diego";
const SCRIPT_DIR = __dirname;
const CD = join(SCRIPT_DIR, ".."); // cloud-data/ root

// ── Load cloud-data JSONs ───────────────────────────────────────
function loadJson(name: string): any {
  const p = join(CD, name);
  try { return JSON.parse(readFileSync(p, "utf-8")); } catch { return {}; }
}

const topology = loadJson("cloud-data-topology.json");
const caddyRoutes = loadJson("cloud-data-caddy-routes.json");
const ghaConfig = loadJson("cloud-data-gha-config.json");
const hmData = loadJson("cloud-data-home-manager.json");
const wgPeersData = loadJson("cloud-data-wireguard-peers.json");

// ── Parse VMs from home-manager data ────────────────────────────
const VMS = Object.entries(hmData.vms ?? {}).map(([id, vm]: [string, any]) => ({
  alias: vm.ssh_alias || id,
  ip: vm.wg_ip || "",
  user: vm.user || "ubuntu",
  cpus: vm.specs?.cpu || 0,
  ram: `${vm.specs?.ram_gb || "?"}G`,
  os: vm.os || id,
  pubIp: vm.ip || "",
  diskGb: vm.specs?.disk_gb || 0,
  provider: vm.provider || "?",
  shape: vm.specs?.shape || vm.specs?.machine_type || "?",
})).filter(v => v.ip); // only VMs with WG IPs

// ── Parse Public URLs from caddy routes ─────────────────────────
function parsePublicUrls(): { url: string; upstream: string }[] {
  const urls: { url: string; upstream: string }[] = [];
  const seen = new Set<string>();
  const add = (url: string, upstream: string) => { if (!seen.has(url)) { seen.add(url); urls.push({ url, upstream }); } };

  for (const r of caddyRoutes.routes ?? []) {
    if (r.domain) add(r.domain, r.upstream || "?");
  }
  for (const pr of caddyRoutes.path_routes ?? []) {
    if (pr.parent_domain) add(pr.parent_domain, "path-based");
  }
  for (const mr of caddyRoutes.mcp_routes ?? []) {
    if (mr.parent_domain) add(mr.parent_domain, "MCP hub");
  }
  const special = caddyRoutes.special;
  if (Array.isArray(special)) {
    for (const s of special) { if (s.domain) add(s.domain, s.upstream || s.comment || "special"); }
  } else if (special && typeof special === "object") {
    for (const s of Object.values(special) as any[]) { if (s?.domain) add(s.domain, s.upstream || s.comment || "special"); }
  }
  return urls;
}
const PUBLIC_URLS = parsePublicUrls();

// ── Parse MCP + API endpoints from caddy routes ─────────────────
function parseMcpApiEndpoints(): { url: string; upstream: string; name: string }[] {
  const eps: { url: string; upstream: string; name: string }[] = [];
  for (const mr of caddyRoutes.mcp_routes ?? []) {
    for (const ep of mr.endpoints ?? []) {
      eps.push({
        url: `${mr.parent_domain}${ep.base_path}/mcp`,
        upstream: ep.upstream,
        name: ep.base_path.replace(/^\//, ""),
      });
    }
  }
  // API path routes (from path_routes with parent_domain = api.*)
  for (const pr of caddyRoutes.path_routes ?? []) {
    for (const r of pr.routes ?? []) {
      if (r.path && r.upstream) {
        eps.push({ url: `${pr.parent_domain}${r.path}`, upstream: r.upstream, name: r.path.replace(/^\//, "") });
      }
    }
  }
  return eps;
}
const MCP_API_ENDPOINTS = parseMcpApiEndpoints();

// ── Parse Mail ports from L4 routes ─────────────────────────────
function parseMailPorts(): { host: string; port: number; proto: string }[] {
  const ports: { host: string; port: number; proto: string }[] = [];
  const MAIL_HOSTS = ["mail.diegonmarcos.com", "smtp.diegonmarcos.com", "imap.diegonmarcos.com"];
  const PROTO_MAP: Record<number, string> = { 25: "SMTP", 465: "SMTPS", 587: "Submission", 993: "IMAPS", 4190: "ManageSieve" };
  // From L4 routes
  for (const l4 of caddyRoutes.l4_routes ?? []) {
    const port = l4.listen_port || (l4.upstream ? parseInt(l4.upstream.split(":").pop()) : 0);
    if (port && PROTO_MAP[port]) {
      for (const h of MAIL_HOSTS) {
        // mail.* gets all, smtp.* gets 25/465/587, imap.* gets 993
        if (h.startsWith("smtp") && ![25, 465, 587].includes(port)) continue;
        if (h.startsWith("imap") && port !== 993) continue;
        ports.push({ host: h, port, proto: PROTO_MAP[port] });
      }
    }
  }
  // Fallback if L4 routes are sparse — add standard mail ports
  if (ports.length === 0) {
    for (const h of MAIL_HOSTS) {
      if (h.startsWith("mail")) for (const p of [25, 465, 587, 993, 4190]) ports.push({ host: h, port: p, proto: PROTO_MAP[p] });
      if (h.startsWith("smtp")) for (const p of [25, 465, 587]) ports.push({ host: h, port: p, proto: PROTO_MAP[p] });
      if (h.startsWith("imap")) ports.push({ host: h, port: 993, proto: "IMAPS" });
    }
  }
  return ports;
}
const MAIL_PORTS = parseMailPorts();

// ── Parse Private DNS from topology ─────────────────────────────
function parsePrivateDns(): { dns: string; container: string; port: number; vm: string }[] {
  const vmIdToAlias: Record<string, string> = {};
  for (const [id, vm] of Object.entries(hmData.vms ?? {}) as [string, any][]) {
    if (vm.ssh_alias) vmIdToAlias[id] = vm.ssh_alias;
  }
  const entries: { dns: string; container: string; port: number; vm: string }[] = [];
  for (const [, svc] of Object.entries(topology.services ?? {}) as [string, any][]) {
    const alias = vmIdToAlias[svc.vm] || svc.vm || "?";
    for (const [, ct] of Object.entries(svc.containers ?? {}) as [string, any][]) {
      if (ct.dns && ct.port) {
        entries.push({ dns: ct.dns, container: ct.container_name || "?", port: ct.port, vm: alias });
      }
    }
  }
  return entries.sort((a, b) => a.vm.localeCompare(b.vm) || a.dns.localeCompare(b.dns));
}
const PRIVATE_DNS = parsePrivateDns();

// ── Helpers ─────────────────────────────────────────────────────
function run(cmd: string, timeout = 10000): string {
  try { return execSync(cmd, { timeout, encoding: "utf-8" }).trim(); }
  catch { return ""; }
}

function sshCmd(vm: string, cmd: string): string {
  const b64 = Buffer.from(cmd).toString("base64");
  return run(`ssh -o ConnectTimeout=5 -o ControlPath=none ${vm} "echo ${b64} | base64 -d | sh"`, 15000);
}

function tcpCheck(host: string, port: number): boolean {
  return run(`nc -zw3 ${host} ${port} 2>&1 && echo OK`).includes("OK");
}

interface Container { name: string; status: string; health: string; icon: string; }
interface VmData {
  alias: string; os: string; cpus: number; ram: string;
  mem_used: string; mem_total: string; mem_pct: number; swap: string;
  disk_used: string; disk_total: string; disk_pct: string;
  load: string; uptime: string;
  containers_running: number; containers_total: number;
  containers: Container[]; reachable: boolean;
}

function collectVm(vm: typeof VMS[0]): VmData {
  const d: VmData = {
    alias: vm.alias, os: vm.os, cpus: vm.cpus, ram: vm.ram,
    mem_used: "?", mem_total: "?", mem_pct: 0, swap: "?",
    disk_used: "?", disk_total: "?", disk_pct: "?",
    load: "?", uptime: "?",
    containers_running: 0, containers_total: 0, containers: [], reachable: false,
  };

  if (sshCmd(vm.alias, "echo OK") !== "OK") return d;
  d.reachable = true;

  const mem = sshCmd(vm.alias, 'free -m | awk \'/Mem/{printf "%d %d %d", $3, $2, $3*100/$2}\'');
  if (mem) { const p = mem.split(" "); d.mem_used = p[0]+"M"; d.mem_total = p[1]+"M"; d.mem_pct = parseInt(p[2]||"0"); }

  d.swap = sshCmd(vm.alias, 'free -m | awk \'/Swap/{printf "%dM/%dM", $3, $2}\'') || "?";

  const disk = sshCmd(vm.alias, 'df -h / | awk \'NR==2{printf "%s %s %s", $3, $2, $5}\'');
  if (disk) { const p = disk.split(" "); d.disk_used = p[0]; d.disk_total = p[1]; d.disk_pct = p[2]; }

  d.load = sshCmd(vm.alias, 'cut -d" " -f1-3 /proc/loadavg') || "?";
  d.uptime = sshCmd(vm.alias, 'uptime -p 2>/dev/null || awk \'{printf "up %dd %dh", $1/86400, ($1%86400)/3600}\' /proc/uptime') || "?";

  const raw = sshCmd(vm.alias, 'docker ps -a --format "{{.Names}}|{{.Status}}" 2>/dev/null');
  for (const line of (raw || "").split("\n").filter(Boolean)) {
    const [name, status] = line.split("|");
    let health = "none", icon = "✅";
    if (status?.includes("(healthy)")) health = "healthy";
    else if (status?.includes("(unhealthy)")) { health = "unhealthy"; icon = "❌"; }
    else if (status?.includes("health: starting")) { health = "starting"; icon = "⚠️"; }
    else if (status?.startsWith("Created")) { health = "created"; icon = "❌"; }
    else if (status?.startsWith("Exited")) { health = "exited"; icon = "❌"; }
    d.containers.push({ name, status: status || "", health, icon });
    d.containers_total++;
    if (status?.startsWith("Up")) d.containers_running++;
  }
  d.containers.sort((a, b) => {
    const o: Record<string, number> = { missing: 0, created: 1, exited: 2, unhealthy: 3, starting: 4, none: 5, healthy: 6 };
    return (o[a.health] ?? 9) - (o[b.health] ?? 9);
  });
  return d;
}

// ── Collect ─────────────────────────────────────────────────────
const timers: { name: string; ms: number }[] = [];
function timed<T>(name: string, fn: () => T): T {
  const start = Date.now();
  const result = fn();
  timers.push({ name, ms: Date.now() - start });
  return result;
}
const TOTAL_START = Date.now();
console.log("Collecting...");

// WG0 peers from gcp-proxy (hub)
console.log("Collecting WG0 peers...");
const wgRaw = sshCmd("gcp-proxy", "sudo wg show wg0 2>/dev/null");
const wgPeers: { name: string; pubIp: string; privIp: string; handshake: string; transfer: string; alive: boolean }[] = [];
if (wgRaw) {
  const blocks = wgRaw.split("\npeer: ");
  for (const block of blocks.slice(1)) {
    const endpoint = block.match(/endpoint: (.+)/)?.[1] || "none";
    const pubIp = endpoint.split(":")[0] || "none";
    const handshake = block.match(/latest handshake: (.+)/)?.[1] || "never";
    const transfer = block.match(/transfer: (.+)/)?.[1] || "0 B";
    const privIp = block.match(/allowed ips: ([\d./]+)/)?.[1]?.replace("/32", "") || "?";
    const vmMatch = VMS.find(v => v.ip === privIp);
    const name = vmMatch?.alias || privIp;
    const alive = handshake !== "never" && !handshake.includes("hour") && !handshake.includes("day");
    wgPeers.push({ name, pubIp, privIp, handshake, transfer, alive });
  }
}

const data = {
  generated: new Date().toISOString(),
  wg_peers: wgPeers,
  api_mcp: timed("api_mcp", () => [...MCP_API_ENDPOINTS].map(e => {
    const code = run(`curl -sko /dev/null -w '%{http_code}' https://${e.url} 2>/dev/null`);
    return { ...e, http_code: code, up: code !== "" && code !== "000" && code !== "502" };
  })),
  public_urls: timed("public_urls", () => PUBLIC_URLS.map(u => {
    const code = run(`curl -sko /dev/null -w '%{http_code}' https://${u.url} 2>/dev/null`);
    return { ...u, http_code: code, up: code !== "" && code !== "000" && code !== "502" };
  })),
  mail_ports: timed("mail_ports", () => MAIL_PORTS.map(m => ({ ...m, open: tcpCheck(m.host, m.port) }))),
  private_dns: timed("private_dns", () => PRIVATE_DNS.map(d => {
    const vmObj = VMS.find(v => v.alias === d.vm);
    const open = vmObj ? tcpCheck(vmObj.ip, d.port) : false;
    return { ...d, open };
  })),
  vms: VMS.map(vm => timed(`vm_${vm.alias}`, () => { console.log(`  ${vm.alias}...`); return collectVm(vm); })),
};

writeFileSync(`${SCRIPT_DIR}/container_health.json`, JSON.stringify(data, null, 2) + "\n");

// ── Generate MD ─────────────────────────────────────────────────
const L: string[] = [];
const _ = (s: string) => L.push(s);

_("```");
_("");
_("  ██████╗██╗      ██████╗ ██╗   ██╗██████╗ ");
_("  ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗");
_("  ██║     ██║     ██║   ██║██║   ██║██║  ██║");
_("  ██║     ██║     ██║   ██║██║   ██║██║  ██║");
_("  ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝");
_("   ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝ ");
_(`         CONTAINER HEALTH — ${data.generated.split("T")[0]}  ${data.generated.split("T")[1]?.split(".")[0] || ""}`);
_("═".repeat(60));
_("");

_("");
_("══════════════════════════════════════════════════════════════");
_("  A) HEALTH");
_("══════════════════════════════════════════════════════════════");
_("");

_("WIREGUARD MESH (hub: gcp-proxy 10.0.0.1)");
_("─".repeat(60));
_(`${"".padEnd(3)} ${"Name".padEnd(18)} ${"Public IP".padEnd(18)} ${"WG IP".padEnd(14)} ${"Handshake"}`);
_("─".repeat(60));
if (data.wg_peers.length) {
  for (const p of data.wg_peers) {
    _(`${p.alive ? "✅" : "❌"} ${p.name.padEnd(18)} ${p.pubIp.padEnd(18)} ${p.privIp.padEnd(14)} ${p.handshake}`);
  }
} else {
  _("❌ Could not reach gcp-proxy WG");
}
_("");

_("PUBLIC URLs");
_("─".repeat(60));
for (const u of data.public_urls) {
  _(`${u.up ? "✅" : "❌"} ${u.url.padEnd(35)} → ${u.upstream.padEnd(22)} [${u.http_code || "---"}]`);
}
_("");

_("API / MCP ENDPOINTS");
_("─".repeat(60));
for (const e of data.api_mcp) {
  _(`${e.up ? "✅" : "❌"} ${e.name.padEnd(22)} https://${e.url.padEnd(45)} [${e.http_code || "---"}]`);
}
_("");

_("MAIL PORTS");
_("─".repeat(60));
for (const m of data.mail_ports) {
  const icon = m.open ? "⚠️" : "❌";
  _(`${icon} ${m.host.padEnd(28)} :${String(m.port).padEnd(5)} ${m.proto.padEnd(15)} ${m.open ? "tcp open" : "down"}`);
}
_("");

_("PRIVATE DNS (WireGuard mesh)");
_("─".repeat(60));
for (const d of data.private_dns) {
  _(`${d.open ? "✅" : "❌"} ${d.dns.padEnd(28)} ${(d.container + ":" + d.port).padEnd(25)} ${d.vm}`);
}
_("");

for (const vm of data.vms) {
  const st = vm.reachable ? "✅" : "❌";
  _(`${vm.alias} ${st} — ${vm.os} — ${vm.cpus}C/${vm.ram} — mem ${vm.mem_used}/${vm.mem_total} (${vm.mem_pct}%) — disk ${vm.disk_pct} — swap ${vm.swap} — load ${vm.load} — ${vm.containers_running}/${vm.containers_total} ctrs — ${vm.uptime}`);
  _("─".repeat(60));
  for (const c of vm.containers) {
    let tag = "";
    if (c.health === "healthy") tag = "HEALTHY";
    else if (c.health === "unhealthy") tag = "UNHEALTHY";
    else if (c.health === "starting") tag = "STARTING";
    else if (c.health === "created") tag = "CREATED";
    else if (c.health === "exited") {
      const code = c.status.match(/Exited \((\d+)\)/)?.[1] || "?";
      tag = `EXITED(${code})`;
    } else if (c.status.startsWith("Up")) tag = "UP";
    _(`  ${c.icon} ${c.name.padEnd(25)} ${tag.padEnd(14)} ${c.status.substring(0, 30)}`);
  }
  _("");
}

// Resources table
_("RESOURCES");
_("─".repeat(60));
_(`${"".padEnd(18)} ${"gcp-proxy".padEnd(14)} ${"oci-apps".padEnd(14)} ${"oci-mail".padEnd(14)} ${"oci-analytics"}`);
_("─".repeat(60));
const fields: [string, (v: VmData) => string][] = [
  ["OS", v => v.os],
  ["CPU", v => `${v.cpus} cores`],
  ["RAM", v => `${v.mem_used}/${v.mem_total}`],
  ["RAM %", v => `${v.mem_pct}%`],
  ["Swap", v => v.swap],
  ["Disk", v => `${v.disk_used}/${v.disk_total}`],
  ["Disk %", v => v.disk_pct],
  ["Load", v => v.load],
  ["Containers", v => `${v.containers_running}/${v.containers_total}`],
  ["Uptime", v => v.uptime.replace("up ", "")],
];
for (const [label, fn] of fields) {
  const vals = data.vms.map(v => fn(v).padEnd(14));
  _(`${label.padEnd(18)} ${vals.join(" ")}`);
}
_("");

// Security: open ports per public IP
_("SECURITY — Open Ports by Public IP");
_("─".repeat(60));
const PUBLIC_IPS = [
  { name: "gcp-proxy", ip: "35.226.147.64" },
  { name: "oci-mail", ip: "130.110.251.193" },
  { name: "oci-analytics", ip: "129.151.228.66" },
  { name: "oci-apps", ip: "82.70.229.129" },
];
const SCAN_PORTS = [22, 25, 80, 443, 465, 587, 993, 2200, 4190, 5000, 8080, 8443, 8888, 51820];
for (const vm of PUBLIC_IPS) {
  const openPorts: number[] = [];
  for (const port of SCAN_PORTS) {
    if (tcpCheck(vm.ip, port)) openPorts.push(port);
  }
  const icon = openPorts.length > 0 ? "🔓" : "🔒";
  _(`${icon} ${vm.name.padEnd(18)} ${vm.ip.padEnd(18)} ports: ${openPorts.length > 0 ? openPorts.join(", ") : "none reachable"}`);
}
_("");

_("");
_("══════════════════════════════════════════════════════════════");
_("  B) STACK INFO");
_("══════════════════════════════════════════════════════════════");
_("");

// Repos
_("GIT REPOSITORIES");
_("─".repeat(60));
const REPOS = [
  { name: "cloud", path: "/home/diego/Mounts/Git/cloud" },
  { name: "cloud-data", path: "/home/diego/Mounts/Git/cloud/cloud-data" },
  { name: "front", path: "/home/diego/Mounts/Git/front" },
  { name: "unix", path: "/home/diego/Mounts/Git/unix" },
  { name: "tools", path: "/home/diego/Mounts/Git/tools" },
  { name: "vault", path: "/home/diego/Mounts/Git/vault" },
];
for (const r of REPOS) {
  const branch = run(`git -C ${r.path} branch --show-current 2>/dev/null`) || "?";
  const commit = run(`git -C ${r.path} log -1 --format="%h %s" 2>/dev/null`) || "?";
  const dirty = run(`git -C ${r.path} status --porcelain 2>/dev/null`);
  const icon = dirty ? "⚠️" : "✅";
  _(`${icon} ${r.name.padEnd(14)} ${branch.padEnd(8)} ${commit.substring(0, 55)}`);
}
_("");

// VPS Resources
_("VPS / VM SPECS");
_("─".repeat(60));
_(`${"".padEnd(3)} ${"VM".padEnd(16)} ${"Provider".padEnd(10)} ${"Shape".padEnd(20)} ${"CPU".padEnd(6)} ${"RAM".padEnd(6)} ${"Disk".padEnd(8)} ${"Cost"}`);
_("─".repeat(60));
const VPS_SPECS = [
  { name: "gcp-proxy", provider: "GCP", shape: "E2 Micro", cpu: "2", ram: "2G", disk: "30G", cost: "Free" },
  { name: "gcp-t4", provider: "GCP", shape: "N1-Std-4 + T4 GPU", cpu: "4", ram: "15G", disk: "100G", cost: "Spot" },
  { name: "oci-mail", provider: "OCI", shape: "E2 Micro", cpu: "2", ram: "1G", disk: "47G", cost: "Free" },
  { name: "oci-analytics", provider: "OCI", shape: "E2 Micro", cpu: "2", ram: "1G", disk: "47G", cost: "Free" },
  { name: "oci-apps", provider: "OCI", shape: "A1 Flex (ARM)", cpu: "4", ram: "24G", disk: "100G", cost: "Free" },
  { name: "github-actions", provider: "GitHub", shape: "ubuntu-latest", cpu: "4", ram: "16G", disk: "14G", cost: "2000min/mo" },
  { name: "ghcr.io", provider: "GitHub", shape: "Container Registry", cpu: "-", ram: "-", disk: "∞", cost: "Free (public)" },
];
for (const v of VPS_SPECS) {
  _(`   ${v.name.padEnd(16)} ${v.provider.padEnd(10)} ${v.shape.padEnd(20)} ${v.cpu.padEnd(6)} ${v.ram.padEnd(6)} ${v.disk.padEnd(8)} ${v.cost}`);
}
_("");

_("FRAMEWORK — Key Paths");
_("─".repeat(60));
const FRAMEWORK = [
  ["BUILD ENGINES", ""],
  ["  Service engine", "cloud/a_solutions/_engine.sh"],
  ["  HM engine", "cloud/b_infra/home-manager/_engine.sh"],
  ["  Workflow engine", "cloud/workflows/build.sh"],
  ["  Front engine", "front/1.ops/build_main.sh"],
  ["  NixOS host", "unix/aa_nixos-surface_host/build.sh"],
  ["  HM desktop", "unix/ba_flakes_desktop/build.sh"],
  ["", ""],
  ["HOME-MANAGER FLAKES", ""],
  ["  Shared modules", "cloud/b_infra/home-manager/_shared/modules/"],
  ["  gcp-proxy", "cloud/b_infra/home-manager/gcp-proxy/src/"],
  ["  oci-apps", "cloud/b_infra/home-manager/oci-apps/src/"],
  ["  oci-mail", "cloud/b_infra/home-manager/oci-mail/src/"],
  ["  oci-analytics", "cloud/b_infra/home-manager/oci-analytics/src/"],
  ["", ""],
  ["GHA WORKFLOWS", ""],
  ["  ship-gcp-proxy", "cloud/.github/workflows/ship-gcp-proxy.yml"],
  ["  ship-oci-apps", "cloud/.github/workflows/ship-oci-apps.yml"],
  ["  ship-oci-mail", "cloud/.github/workflows/ship-oci-mail.yml"],
  ["  ship-oci-analytics", "cloud/.github/workflows/ship-oci-analytics.yml"],
  ["  ship-home-manager", "cloud/.github/workflows/ship-home-manager.yml"],
  ["  ship-ghcr", "cloud/.github/workflows/ship-ghcr.yml"],
  ["  Templates", "cloud/workflows/src/"],
  ["", ""],
  ["DAGU WORKFLOWS", ""],
  ["  DAGs source", "cloud/a_solutions/bc-obs_dagu/src/dags/"],
  ["  deploy-pull-up", "cloud/a_solutions/bc-obs_dagu/src/dags/ops_deploy-pull-up.yaml"],
  ["  cloud-data sync", "cloud/a_solutions/bc-obs_dagu/src/dags/sync_cloud-data.yaml"],
  ["", ""],
  ["DATA", ""],
  ["  cloud-data", "cloud/cloud-data/"],
  ["  Container manifests", "cloud/cloud-data/cloud-data-containers-{vm}.json"],
  ["  Topology", "cloud/cloud-data/cloud-data-topology.json"],
  ["  GHA config", "cloud/cloud-data/cloud-data-gha-config.json"],
  ["  Consolidated", "cloud/cloud-data/_cloud-data-consolidated.json"],
  ["", ""],
  ["TERRAFORM", ""],
  ["  OCI", "cloud/b_infra/vps_oci/src/main.tf"],
  ["  GCP", "cloud/b_infra/vps_gcloud/src/main.tf"],
  ["  Cloudflare", "cloud/a_solutions/ba-clo_cloudflare/src/main.tf"],
  ["", ""],
  ["SECURITY", ""],
  ["  Vault", "vault/"],
  ["  SOPS secrets", "cloud/a_solutions/*/src/secrets.yaml"],
  ["  SSH keys", "vault/A0_keys/ssh/"],
];
for (const [label, path] of FRAMEWORK) {
  if (!label && !path) { _(""); continue; }
  if (!path) { _(`  ${label}`); continue; }
  _(`  ${label.padEnd(22)} ~/git/${path}`);
}
_("");

// Vault providers
_("VAULT — CLI Access Providers");
_("─".repeat(60));
const vaultProviders = run(`ls -1 ${HOME}/Mounts/Git/vault/A0_keys/providers/ 2>/dev/null`);
if (vaultProviders) {
  for (const p of vaultProviders.split("\n").filter(Boolean)) {
    _(`  🔑 ${p}`);
  }
}
_("");

// GitHub / GHCR
_("GITHUB / GHCR");
_("─".repeat(60));
const ghcrCount = run("gh api '/user/packages?package_type=container&per_page=100' --jq 'length' 2>/dev/null");
const ghUser = run("gh api user --jq .login 2>/dev/null");
_(`  👤 User:       ${ghUser || "?"}`);
_(`  📦 GHCR images: ${ghcrCount || "?"}`);
_(`  🔗 Registry:   ghcr.io/diegonmarcos/`);
_("");

_("PERFORMANCE");
_("─".repeat(60));
const totalMs = Date.now() - TOTAL_START;
for (const t of timers.sort((a, b) => b.ms - a.ms)) {
  const bar = "█".repeat(Math.min(30, Math.round(t.ms / (totalMs / 30))));
  _(`  ${t.name.padEnd(18)} ${String(t.ms).padStart(6)}ms ${bar}`);
}
_(`  ${"TOTAL".padEnd(18)} ${String(totalMs).padStart(6)}ms`);
_("");

_("```");

writeFileSync(`${SCRIPT_DIR}/container_health.md`, L.join("\n") + "\n");
console.log("Done → container_health.json + container_health.md");
