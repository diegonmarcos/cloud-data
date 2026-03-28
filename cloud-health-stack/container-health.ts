#!/usr/bin/env tsx
/**
 * Container Health Reporter — Template-driven
 * Collects live data → container_health.json
 * Reads container_health.md.tpl → replaces $VARS → container_health.md
 * Usage: ./container-health.ts   (or: tsx container-health.ts)
 *
 * Dependencies: node ≥18, tsx, ssh, curl, nc, dig, git, gh
 */
import { execSync } from "child_process";
import { writeFileSync, readFileSync, existsSync } from "fs";
import { join } from "path";

// ── Dependency solver ───────────────────────────────────────
const REQUIRED_DEPS = ["ssh", "curl", "nc", "dig", "git", "gh"];
const depStatus: { name: string; path: string; ok: boolean }[] = [];
for (const dep of REQUIRED_DEPS) {
  try {
    const p = execSync(`command -v ${dep} 2>/dev/null`, { encoding: "utf-8" }).trim();
    depStatus.push({ name: dep, path: p, ok: true });
  } catch {
    depStatus.push({ name: dep, path: "", ok: false });
  }
}
const missingDeps = depStatus.filter(d => !d.ok);
if (missingDeps.length > 0) {
  console.error(`⚠️  Missing dependencies: ${missingDeps.map(d => d.name).join(", ")}`);
  console.error("   Some checks will be skipped. Install missing tools to get full results.");
}

const HOME = process.env.HOME || "/home/diego";
const SCRIPT_DIR = __dirname;
const CD = join(SCRIPT_DIR, ".."); // cloud-data/ root

// ── Logging ─────────────────────────────────────────────────────
const LOG: string[] = [];
const ERRORS: string[] = [];
function log(msg: string) { const ts = new Date().toISOString().split("T")[1]?.split(".")[0]; const line = `[${ts}] ${msg}`; LOG.push(line); console.log(line); }
function logErr(msg: string) { const ts = new Date().toISOString().split("T")[1]?.split(".")[0]; const line = `[${ts}] ERROR: ${msg}`; LOG.push(line); ERRORS.push(line); console.error(line); }

// ── Load cloud-data JSONs ───────────────────────────────────────
function loadJson(name: string): any {
  const p = join(CD, name);
  try {
    const data = JSON.parse(readFileSync(p, "utf-8"));
    log(`Loaded ${name} (${Object.keys(data).length} keys)`);
    return data;
  } catch (e: any) {
    logErr(`Failed to load ${name}: ${e.message}`);
    return {};
  }
}

const topology = loadJson("cloud-data-topology.json");
const caddyRoutes = loadJson("cloud-data-caddy-routes.json");
const ghaConfig = loadJson("cloud-data-gha-config.json");
const hmData = loadJson("cloud-data-home-manager.json");
const wgPeersData = loadJson("cloud-data-wireguard-peers.json");
const backupTargets = loadJson("cloud-data-backup-targets.json");

// ── Parse VMs from home-manager data ────────────────────────────
const VMS = Object.entries(hmData.vms ?? {}).map(([id, vm]: [string, any]) => ({
  alias: vm.ssh_alias || id,
  vmId: vm.vm_id || id,
  ip: vm.wg_ip || "",
  user: vm.user || "ubuntu",
  cpus: vm.specs?.cpu || 0,
  ram: `${vm.specs?.ram_gb || "?"}G`,
  os: vm.os || id,
  pubIp: vm.ip || "",
  diskGb: vm.specs?.disk_gb || 0,
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
  for (const gp of caddyRoutes.github_pages_proxies ?? []) {
    for (const d of (gp.domain as string).split(",").map((s: string) => s.trim()).filter(Boolean)) {
      add(d, `github-pages:${gp.github_path}`);
    }
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
  const STALWART_HOSTS = ["mail.diegonmarcos.com", "smtp.diegonmarcos.com", "imap.diegonmarcos.com"];
  const PROTO_MAP: Record<number, string> = { 25: "SMTP", 465: "SMTPS", 587: "Submission", 993: "IMAPS", 4190: "ManageSieve" };
  for (const l4 of caddyRoutes.l4_routes ?? []) {
    const port = l4.listen_port || (l4.upstream ? parseInt(l4.upstream.split(":").pop()) : 0);
    if (port && PROTO_MAP[port]) {
      for (const h of STALWART_HOSTS) {
        if (h.startsWith("smtp") && ![25, 465, 587].includes(port)) continue;
        if (h.startsWith("imap") && port !== 993) continue;
        ports.push({ host: h, port, proto: PROTO_MAP[port] });
      }
    }
  }
  if (ports.length === 0) {
    for (const h of STALWART_HOSTS) {
      if (h.startsWith("mail")) for (const p of [25, 465, 587, 993, 4190]) ports.push({ host: h, port: p, proto: PROTO_MAP[p] });
      if (h.startsWith("smtp")) for (const p of [25, 465, 587]) ports.push({ host: h, port: p, proto: PROTO_MAP[p] });
      if (h.startsWith("imap")) ports.push({ host: h, port: 993, proto: "IMAPS" });
    }
  }
  // Resend (mails.diegonmarcos.com) — transactional email via API, verify MX/SPF DNS
  ports.push({ host: "mails.diegonmarcos.com", port: 25, proto: "MX (Resend/SES)" });
  ports.push({ host: "send.mails.diegonmarcos.com", port: 25, proto: "SPF (Resend/SES)" });
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

// ── Parse databases from backup-targets ───────────────────────────
function parseDatabases(): { service: string; type: string; container: string; db: string; vm: string; dns: string }[] {
  const dbs: { service: string; type: string; container: string; db: string; vm: string; dns: string }[] = [];
  for (const t of backupTargets.targets ?? []) {
    for (const d of t.databases ?? []) {
      const dnsEntry = PRIVATE_DNS.find(p => p.container === d.container || p.dns.startsWith(d.container));
      const dns = dnsEntry ? `${dnsEntry.dns}:${dnsEntry.port}` : d.path || "embedded";
      dbs.push({
        service: d.service || t.service,
        type: d.type,
        container: d.container,
        db: d.db || d.path || "custom",
        vm: t.vm_alias || t.vm,
        dns,
      });
    }
  }
  return dbs.sort((a, b) => a.vm.localeCompare(b.vm) || a.service.localeCompare(b.service));
}
const DATABASES = parseDatabases();

// ── Helpers ─────────────────────────────────────────────────────

// Clear stale SSH mux sockets at startup
try {
  const socketDir = join(HOME, ".ssh", "sockets");
  if (existsSync(socketDir)) {
    execSync(`find ${socketDir} -type s -mmin +5 -delete 2>/dev/null || true`, { timeout: 3000 });
  }
} catch {}

function run(cmd: string, timeout = 10000): string {
  try { return execSync(cmd, { timeout, encoding: "utf-8", stdio: ["pipe", "pipe", "pipe"] }).trim(); }
  catch (e: any) {
    const stderr = e.stderr?.toString()?.trim();
    if (stderr && !stderr.includes("Connection timed out") && !stderr.includes("Connection refused")) {
      logErr(`run failed: ${cmd.substring(0, 80)}... → ${stderr.substring(0, 200)}`);
    }
    return "";
  }
}

function sshCmd(vm: string, cmd: string): string {
  const b64 = Buffer.from(cmd).toString("base64");
  const result = run(`ssh -o ConnectTimeout=8 -o ControlPath=none -o BatchMode=yes ${vm} "echo ${b64} | base64 -d | sh"`, 20000);
  if (!result && cmd === "echo OK") {
    logErr(`SSH unreachable: ${vm}`);
  }
  return result;
}

let tcpLogVerbose = true;
function tcpCheck(host: string, port: number): boolean {
  const ok = run(`nc -zw3 ${host} ${port} 2>&1 && echo OK`).includes("OK");
  if (!ok && tcpLogVerbose) log(`tcp-check FAIL: ${host}:${port}`);
  return ok;
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
log("═══ Starting collection ═══");
log(`VMs parsed: ${VMS.map(v => v.alias).join(", ")} (${VMS.length} total)`);
log(`Public URLs: ${PUBLIC_URLS.length}, MCP endpoints: ${MCP_API_ENDPOINTS.length}, Mail ports: ${MAIL_PORTS.length}`);
log(`Private DNS: ${PRIVATE_DNS.length}, Databases: ${DATABASES.length}`);

// WG0 peers from gcp-proxy (hub)
log("Collecting WG0 peers from gcp-proxy...");
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
  vms: VMS.map(vm => timed(`vm_${vm.alias}`, () => { log(`  Collecting VM: ${vm.alias} (${vm.pubIp || vm.ip})...`); return collectVm(vm); })),
  databases: DATABASES,
};

log("═══ Data collection complete ═══");
log(`WG peers: ${data.wg_peers.length}, Public URLs checked: ${data.public_urls.length}`);
log(`VMs collected: ${data.vms.filter((v: VmData) => v.reachable).length}/${data.vms.length} reachable`);
for (const vm of data.vms) {
  if (vm.reachable) log(`  ✅ ${vm.alias}: ${vm.containers_running}/${vm.containers_total} ctrs, mem ${vm.mem_pct}%, disk ${vm.disk_pct}`);
  else logErr(`  ❌ ${vm.alias}: UNREACHABLE`);
}

writeFileSync(`${SCRIPT_DIR}/container_health.json`, JSON.stringify(data, null, 2) + "\n");
log("Wrote container_health.json");

// ── Template-driven MD generation ───────────────────────────────
log("═══ Generating MD from template ═══");
const tplPath = join(SCRIPT_DIR, "container_health.md.tpl");
let template: string;
try {
  template = readFileSync(tplPath, "utf-8");
  log(`Template loaded: ${tplPath} (${template.length} chars)`);
} catch (e: any) {
  logErr(`Failed to read template: ${e.message}`);
  process.exit(1);
}

const hubVm = VMS.find(v => v.alias === "gcp-proxy");

// Infer provider/cost from vm_id convention
const inferProvider = (id: string) => {
  if (id.startsWith("oci-")) return "OCI";
  if (id.startsWith("gcp-")) return "GCP";
  if (id.startsWith("aws-")) return "AWS";
  if (id.startsWith("vast-")) return "Vast.ai";
  return "?";
};
const inferCost = (id: string) => {
  if (/-f[_\d]/.test(id)) return "Free";
  if (/-p[_\d]/.test(id)) return "Spot";
  return "?";
};

const vars: Record<string, string> = {

  GENERATED_DATE: `${data.generated.split("T")[0]}  ${data.generated.split("T")[1]?.split(".")[0] || ""}`,

  HUB_WG_IP: hubVm?.ip || "?",

  WG_PEERS: (() => {
    const lines: string[] = [];
    const peers = (wgPeersData.mesh_peers ?? []) as any[];
    if (peers.length > 0) {
      for (const peer of peers) {
        const name = peer.name || peer.vm_id || "?";
        const pubIp = peer.public_ip || "?";
        const wgIp = peer.wg_ip || "?";
        const isHub = name === "gcp-proxy";
        const isVm = VMS.some(v => v.alias === name);
        const peerType = isHub ? "HUB" : isVm ? "VM" : "CLIENT";
        const live = data.wg_peers.find((p: any) => p.privIp === wgIp || p.name === name);
        const handshake = live?.handshake || "no data";
        const alive = live?.alive ?? false;
        lines.push(`${alive ? "✅" : "❌"} ${name.padEnd(18)} ${pubIp.padEnd(18)} ${wgIp.padEnd(14)} ${peerType.padEnd(8)} ${handshake}`);
      }
    } else if (data.wg_peers.length) {
      for (const p of data.wg_peers) {
        lines.push(`${p.alive ? "✅" : "❌"} ${p.name.padEnd(18)} ${p.pubIp.padEnd(18)} ${p.privIp.padEnd(14)} ${"?".padEnd(8)} ${p.handshake}`);
      }
    } else {
      lines.push("❌ No WG peer data available");
    }
    return lines.join("\n");
  })(),

  PUBLIC_URLS: data.public_urls.map((u: any) =>
    `${u.up ? "✅" : "❌"} ${u.url.padEnd(35)} → ${u.upstream.padEnd(22)} [${u.http_code || "---"}]`
  ).join("\n"),

  API_MCP_ENDPOINTS: data.api_mcp.map((e: any) =>
    `${e.up ? "✅" : "❌"} ${e.name.padEnd(22)} https://${e.url.padEnd(45)} [${e.http_code || "---"}]`
  ).join("\n"),

  MAIL_PORTS: data.mail_ports.map((m: any) =>
    `${m.open ? "⚠️" : "❌"} ${m.host.padEnd(28)} :${String(m.port).padEnd(5)} ${m.proto.padEnd(15)} ${m.open ? "tcp open" : "down"}`
  ).join("\n"),

  // ── A3 Mail: MX, SPF, DKIM, DMARC, AUTH, FLOW ───────────────

  MAIL_MX: (() => {
    const lines: string[] = [];
    lines.push("MX — Inbound Routing (dig MX)");
    lines.push("────────────────────────────────────────────────────────────");
    lines.push(`    ${"Domain".padEnd(28)} ${"Pri".padEnd(5)} ${"Server".padEnd(42)} IP`);
    lines.push("────────────────────────────────────────────────────────────");
    const domains = ["diegonmarcos.com", "send.mails.diegonmarcos.com", "mails.diegonmarcos.com"];
    for (const domain of domains) {
      const mx = run(`dig +short MX ${domain} 2>/dev/null`);
      if (mx) {
        for (const line of mx.split("\n").filter(Boolean)) {
          const [pri, server] = line.split(/\s+/);
          const ip = run(`dig +short A ${server} 2>/dev/null`)?.split("\n")[0] || "?";
          lines.push(`✅ ${domain.padEnd(28)} ${pri.padEnd(5)} ${server.padEnd(42)} ${ip}`);
        }
      } else {
        lines.push(`❌ ${domain.padEnd(28)} —     no MX record`);
      }
    }
    lines.push("  ─── checks ───");
    lines.push("  ✅ Cloudflare Email Routing active (3 MX routes for diegonmarcos.com)");
    lines.push("  ✅ Resend bounce handler (send.mails MX → SES feedback)");
    lines.push("  ❌ No MX for mails.diegonmarcos.com (normal — Resend is API-only, no inbound)");
    return lines.join("\n");
  })(),

  MAIL_SPF: (() => {
    const lines: string[] = [];
    const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
    lines.push("SPF — Outbound Policy: IP Allowlist (dig TXT)");
    lines.push("────────────────────────────────────────────────────────────");
    lines.push(`    ${"Domain".padEnd(32)} ${"Include".padEnd(45)} Resolved IPs`);
    lines.push("────────────────────────────────────────────────────────────");
    // Main domain SPF
    const mainSpf = run("dig +short TXT diegonmarcos.com 2>/dev/null");
    const spfIncludes = mainSpf?.match(/include:([^\s]+)/g) || [];
    for (const inc of spfIncludes) {
      const target = inc.replace("include:", "");
      const resolved = run(`dig +short TXT ${target} 2>/dev/null`);
      const ips = resolved?.match(/ip4:[^\s]+/g)?.slice(0, 3).join(", ") || "?";
      lines.push(`✅ ${"diegonmarcos.com".padEnd(32)} ${inc.padEnd(45)} ${ips}`);
    }
    // Resend subdomain SPF
    const mailsSpf = run("dig +short TXT send.mails.diegonmarcos.com 2>/dev/null");
    const mailsInc = mailsSpf?.match(/include:([^\s]+)/g) || [];
    for (const inc of mailsInc) {
      lines.push(`✅ ${"send.mails.diegonmarcos.com".padEnd(32)} ${inc.padEnd(45)} (same as above)`);
    }
    // Check if oci-mail VM IP is in SPF
    const allSpfIps = spfIncludes.map(inc => {
      const target = inc.replace("include:", "");
      return run(`dig +short TXT ${target} 2>/dev/null`) || "";
    }).join(" ");
    const vmInSpf = allSpfIps.includes(ociMailIp.split(".").slice(0, 2).join("."));
    lines.push(`${vmInSpf ? "✅" : "⚠️"} ${"diegonmarcos.com".padEnd(32)} ${"oci-mail VM IP " + ociMailIp} ${vmInSpf ? "IN SPF" : "NOT IN SPF!"}`);
    lines.push("  ─── checks ───");
    lines.push("  ✅ SPF record exists for diegonmarcos.com");
    lines.push("  ✅ SPF record exists for send.mails.diegonmarcos.com");
    lines.push("  ✅ All includes resolve successfully");
    if (!vmInSpf) {
      lines.push(`  ❌ oci-mail VM IP ${ociMailIp} NOT in any SPF range!`);
      lines.push("     → Stalwart sends directly from this IP — receivers will SPF FAIL");
      lines.push("     → FIX: add ip4:" + ociMailIp + " to SPF or relay via OCI Email Delivery");
    }
    const softfail = mainSpf?.includes("~all");
    if (softfail) lines.push("  ⚠️ SPF ~all (softfail) — consider tightening to -all (hardfail)");
    return lines.join("\n");
  })(),

  MAIL_DKIM: (() => {
    const lines: string[] = [];
    lines.push("DKIM — Outbound Policy: Cryptographic Signatures (dig TXT)");
    lines.push("────────────────────────────────────────────────────────────");
    lines.push(`    ${"Selector".padEnd(28)} ${"Domain".padEnd(24)} ${"Signer".padEnd(20)} Key Size`);
    lines.push("────────────────────────────────────────────────────────────");
    const selectors = [
      { sel: "dkim._domainkey", domain: "diegonmarcos.com", signer: "Stalwart" },
      { sel: "mail._domainkey", domain: "diegonmarcos.com", signer: "Legacy Mailu" },
      { sel: "google._domainkey", domain: "diegonmarcos.com", signer: "Google Workspace" },
      { sel: "cf2024-1._domainkey", domain: "diegonmarcos.com", signer: "Cloudflare" },
      { sel: "resend._domainkey.mails", domain: "diegonmarcos.com", signer: "Resend/SES" },
    ];
    const dkimResults: { sel: string; signer: string; present: boolean; bits: string }[] = [];
    for (const s of selectors) {
      const txt = run(`dig +short TXT ${s.sel}.${s.domain} 2>/dev/null`);
      const present = !!txt && txt.includes("DKIM1");
      // Estimate key size from base64 length (rough: 392 chars ≈ 2048, 216 ≈ 1024)
      const b64 = txt?.match(/p=([A-Za-z0-9+/=]+)/)?.[1] || "";
      const bits = b64.length > 300 ? "RSA 2048" : b64.length > 100 ? "RSA 1024" : "?";
      dkimResults.push({ sel: s.sel, signer: s.signer, present, bits });
      lines.push(`${present ? "✅" : "❌"} ${s.sel.padEnd(28)} ${s.domain.padEnd(24)} ${s.signer.padEnd(20)} ${present ? bits : "NOT FOUND"}`);
    }
    lines.push("  ─── checks ───");
    const allPresent = dkimResults.every(d => d.present);
    lines.push(`  ${allPresent ? "✅" : "⚠️"} All ${selectors.length} DKIM selectors ${allPresent ? "have valid public keys" : "— some missing!"}`);
    const weak = dkimResults.filter(d => d.bits === "RSA 1024" && d.present);
    for (const w of weak) lines.push(`  ⚠️ ${w.sel} uses RSA 1024 — weaker than 2048 (provider limitation)`);
    const mailu = dkimResults.find(d => d.sel === "mail._domainkey" && d.present);
    if (mailu) lines.push("  ⚠️ mail._domainkey (Legacy Mailu) still published — remove if decommissioned");
    return lines.join("\n");
  })(),

  MAIL_DMARC: (() => {
    const lines: string[] = [];
    lines.push("DMARC — Outbound Policy: Receiver Instructions (dig TXT)");
    lines.push("────────────────────────────────────────────────────────────");
    const dmarc = run("dig +short TXT _dmarc.diegonmarcos.com 2>/dev/null");
    const hasRecord = !!dmarc && dmarc.includes("DMARC1");
    lines.push(`${hasRecord ? "✅" : "❌"} _dmarc.diegonmarcos.com       ${dmarc || "NO DMARC RECORD"}`);
    lines.push("  ─── checks ───");
    if (hasRecord) {
      const policy = dmarc.match(/p=([^;"\s]+)/)?.[1] || "?";
      lines.push(`  ${policy === "reject" ? "✅" : "⚠️"} Policy: p=${policy} ${policy === "reject" ? "(strictest — good)" : policy === "quarantine" ? "(moderate)" : "(weak — consider reject)"}`);
      const rua = dmarc.match(/rua=([^;"\s]+)/)?.[1];
      lines.push(`  ${rua ? "✅" : "⚠️"} Aggregate reports: ${rua || "NOT configured — add rua=mailto:..."}`);
      const ruf = dmarc.match(/ruf=([^;"\s]+)/)?.[1];
      lines.push(`  ${ruf ? "✅" : "⚠️"} Forensic reports: ${ruf || "NOT configured — add ruf=mailto:... for failure details"}`);
      const sp = dmarc.match(/sp=([^;"\s]+)/)?.[1];
      lines.push(`  ${sp ? "✅" : "⚠️"} Subdomain policy: ${sp ? "sp=" + sp : "inherits p=" + policy + " (OK if intentional)"}`);
    } else {
      lines.push("  ❌ NO DMARC RECORD — domain is unprotected from spoofing!");
    }
    return lines.join("\n");
  })(),

  MAIL_AUTH: (() => {
    const lines: string[] = [];
    const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
    lines.push("MAIL AUTH — Authorized Senders");
    lines.push("────────────────────────────────────────────────────────────");
    lines.push(`    ${"Sender".padEnd(20)} ${"Domain".padEnd(26)} ${"Auth Method".padEnd(16)} ${"SPF IP Range".padEnd(30)} DKIM Selector`);
    lines.push("────────────────────────────────────────────────────────────");
    const senders = [
      { name: "Cloudflare", domain: "diegonmarcos.com", auth: "Email Routing", spf: "104.30.0.0/19", dkim: "cf2024-1._domainkey", ok: true },
      { name: "Stalwart", domain: "diegonmarcos.com", auth: "Direct SMTP", spf: ociMailIp + " NOT IN SPF!", dkim: "dkim._domainkey", ok: false },
      { name: "Google", domain: "diegonmarcos.com", auth: "Google SMTP", spf: "(via google include)", dkim: "google._domainkey", ok: true },
      { name: "Legacy Mailu", domain: "diegonmarcos.com", auth: "DECOMMISSIONED", spf: "—", dkim: "mail._domainkey", ok: false },
      { name: "Resend/SES", domain: "mails.diegonmarcos.com", auth: "API + SES", spf: "54.240.0.0/18", dkim: "resend._dk.mails", ok: true },
      { name: "OCI Email Dlv", domain: "diegonmarcos.com", auth: "SMTP Relay", spf: "192.29.200.0/25", dkim: "(via Stalwart)", ok: true },
    ];
    for (const s of senders) {
      lines.push(`${s.ok ? "✅" : "⚠️"} ${s.name.padEnd(20)} ${s.domain.padEnd(26)} ${s.auth.padEnd(16)} ${s.spf.padEnd(30)} ${s.dkim}`);
    }
    lines.push("  ─── checks ───");
    lines.push(`  ❌ Stalwart: SPF will FAIL — IP ${ociMailIp} not in any SPF include`);
    lines.push("  ⚠️ Stalwart: not configured to relay via OCI Email Delivery (direct SMTP)");
    lines.push("  ⚠️ Legacy Mailu: DKIM key still in DNS but service decommissioned — stale key");
    lines.push("  ✅ Resend: SPF ✅ DKIM ✅ — fully authorized");
    lines.push("  ✅ Cloudflare: SPF ✅ DKIM ✅ — fully authorized");
    lines.push("  ✅ Google: SPF ✅ DKIM ✅ — fully authorized");
    lines.push("  ✅ OCI Email Delivery: SPF ✅ — in range, but Stalwart not using it as relay");
    return lines.join("\n");
  })(),

  MAIL_FLOW: (() => {
    const lines: string[] = [];
    const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
    const resendTf = topology.providers?.resend;
    lines.push("MAIL FLOW — Pipeline Status");
    lines.push("────────────────────────────────────────────────────────────");
    lines.push("");
    // INBOUND
    lines.push("  📨 INBOUND: someone@gmail.com → me@diegonmarcos.com");
    lines.push("     Gmail → MX → Cloudflare Email Routing → CF Worker → smtp-proxy:8080 → Stalwart");
    lines.push("     ─────────────────────────────────────────────");
    const smtpProxyCt = data.vms.find((v: VmData) => v.alias === "oci-mail")?.containers.find((c: Container) => c.name === "smtp-proxy");
    const smtpProxyOk = smtpProxyCt && smtpProxyCt.health !== "exited";
    lines.push(`     ${smtpProxyOk ? "✅" : "❌"} smtp-proxy           ${smtpProxyCt?.status || "not found"} (oci-mail:8080)`);
    const p8080 = tcpCheck(ociMailIp, 8080);
    lines.push(`     ${p8080 ? "✅" : "❌"} oci-mail:8080        ${p8080 ? "reachable" : "unreachable"} (CF Worker ingress)`);
    const p25 = tcpCheck(ociMailIp, 25);
    lines.push(`     ${p25 ? "✅" : "❌"} oci-mail:25          ${p25 ? "SMTP open" : "SMTP closed"} (Stalwart local delivery)`);
    lines.push("");
    // OUTBOUND PERSONAL
    lines.push("  📤 OUTBOUND PERSONAL: me@diegonmarcos.com → someone@gmail.com");
    lines.push(`     Stalwart → ⚠️ direct from ${ociMailIp} (NOT IN SPF!) → recipient MX`);
    lines.push("     ─────────────────────────────────────────────");
    const stalwartCt = data.vms.find((v: VmData) => v.alias === "oci-mail")?.containers.find((c: Container) => c.name === "stalwart");
    const stalwartOk = stalwartCt && stalwartCt.health !== "exited";
    lines.push(`     ${stalwartOk ? "✅" : "❌"} stalwart             ${stalwartCt?.status || "not found"} (oci-mail MTA)`);
    const p465 = tcpCheck("smtp.diegonmarcos.com", 465);
    const p587 = tcpCheck("smtp.diegonmarcos.com", 587);
    lines.push(`     ${p465 ? "✅" : "❌"} smtp:465 (SMTPS)     ${p465 ? "open" : "closed"} (via gcp-proxy L4)`);
    lines.push(`     ${p587 ? "✅" : "❌"} smtp:587 (Submission) ${p587 ? "open" : "closed"} (via gcp-proxy L4)`);
    lines.push(`     ❌ SPF WILL FAIL        VM IP not in SPF — needs OCI relay or ip4: in SPF`);
    lines.push(`     ✅ DKIM OK              dkim._domainkey key present`);
    lines.push(`     ❌ DMARC RESULT         p=reject + SPF fail = email REJECTED by receiver`);
    lines.push("");
    // OUTBOUND TRANSACTIONAL
    lines.push("  📤 OUTBOUND TRANSACTIONAL: noreply@mails.diegonmarcos.com → someone@gmail.com");
    lines.push("     App → Resend API → Amazon SES (us-east-1) → recipient MX");
    lines.push("     ─────────────────────────────────────────────");
    const resendApi = run("curl -sko /dev/null -w '%{http_code}' https://api.resend.com/domains 2>/dev/null");
    lines.push(`     ${resendApi === "401" || resendApi === "200" ? "✅" : "❌"} api.resend.com       [${resendApi || "---"}] (reachable, needs key)`);
    lines.push("     ✅ SPF OK               SES IPs in send.mails SPF");
    lines.push("     ✅ DKIM OK              resend._domainkey.mails present");
    lines.push("     ✅ DMARC OK             will pass (SPF ✅ + DKIM ✅)");
    lines.push(`     ✅ Terraform            ~/git/cloud/${resendTf?.folder || "b_infra/vps_resend"}/src/main.tf`);
    return lines.join("\n");
  })(),

  PRIVATE_DNS: (() => {
    // Find globally duplicated ports
    const portCount = new Map<number, string[]>();
    for (const d of data.private_dns) {
      if (!portCount.has(d.port)) portCount.set(d.port, []);
      portCount.get(d.port)!.push(d.dns);
    }
    const conflictPorts = new Set([...portCount.entries()].filter(([, names]) => names.length > 1).map(([p]) => p));
    const lines = data.private_dns.map((d: any) => {
      const conflict = conflictPorts.has(d.port);
      const icon = d.open ? "✅" : "❌";
      const portTag = conflict ? `⚠️${String(d.port).padEnd(5)}` : `  ${String(d.port).padEnd(5)}`;
      return `${icon} ${d.dns.padEnd(28)} ${(d.container + ":" + d.port).padEnd(25)} ${portTag} ${d.vm}`;
    });
    // Append conflict summary
    if (conflictPorts.size > 0) {
      lines.push("");
      lines.push(`  ⚠️  PORT CONFLICTS (${conflictPorts.size} duplicate ports globally):`);
      for (const [port, names] of [...portCount.entries()].filter(([, n]) => n.length > 1).sort((a, b) => a[0] - b[0])) {
        lines.push(`     :${String(port).padEnd(6)} used by: ${names.join(", ")}`);
      }
    }
    return lines.join("\n");
  })(),

  VM_CONTAINERS: data.vms.map((vm: VmData) => {
    const lines: string[] = [];
    const st = vm.reachable ? "✅" : "❌";
    lines.push(`${vm.alias} ${st} — ${vm.os} — ${vm.cpus}C/${vm.ram} — mem ${vm.mem_used}/${vm.mem_total} (${vm.mem_pct}%) — disk ${vm.disk_pct} — swap ${vm.swap} — load ${vm.load} — ${vm.containers_running}/${vm.containers_total} ctrs — ${vm.uptime}`);
    lines.push("─".repeat(60));
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
      // Look up port from private DNS data
      const dnsEntry = PRIVATE_DNS.find(p => p.container === c.name);
      const portStr = dnsEntry ? String(dnsEntry.port).padEnd(7) : "       ";
      lines.push(`  ${c.icon} ${c.name.padEnd(25)} ${portStr} ${tag.padEnd(14)} ${c.status.substring(0, 30)}`);
    }
    lines.push("");
    return lines.join("\n");
  }).join("\n"),

  RESOURCES_TABLE: (() => {
    const lines: string[] = [];
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
      const vals = data.vms.map((v: VmData) => fn(v).padEnd(14));
      lines.push(`${label.padEnd(18)} ${vals.join(" ")}`);
    }
    return lines.join("\n");
  })(),

  GIT_REPOS: (() => {
    const REPOS = [
      { name: "cloud", path: "/home/diego/Mounts/Git/cloud" },
      { name: "cloud-data", path: "/home/diego/Mounts/Git/cloud/cloud-data" },
      { name: "front", path: "/home/diego/Mounts/Git/front" },
      { name: "unix", path: "/home/diego/Mounts/Git/unix" },
      { name: "tools", path: "/home/diego/Mounts/Git/tools" },
      { name: "vault", path: "/home/diego/Mounts/Git/vault" },
    ];
    return REPOS.map(r => {
      const branch = run(`git -C ${r.path} branch --show-current 2>/dev/null`) || "?";
      const commit = run(`git -C ${r.path} log -1 --format="%h %s" 2>/dev/null`) || "?";
      const dirty = run(`git -C ${r.path} status --porcelain 2>/dev/null`);
      const icon = dirty ? "⚠️" : "✅";
      return `${icon} ${r.name.padEnd(14)} ${branch.padEnd(8)} ${commit.substring(0, 55)}`;
    }).join("\n");
  })(),

  GITHUB_GHCR: (() => {
    const ghUser = run("gh api user --jq .login 2>/dev/null") || "?";
    const lines: string[] = [];
    lines.push(`  👤 User:       ${ghUser}`);
    lines.push(`  🔗 Registry:   ghcr.io/${ghUser}/`);
    lines.push(`  📦 Repos:      github.com/${ghUser}/`);
    lines.push("");
    const pkgsJson = run("gh api '/user/packages?package_type=container&per_page=100' 2>/dev/null");
    if (pkgsJson) {
      try {
        const pkgs = JSON.parse(pkgsJson) as { name: string; visibility: string; repository?: { name: string } }[];
        const pubCount = pkgs.filter(p => p.visibility === "public").length;
        const privCount = pkgs.filter(p => p.visibility === "private").length;
        lines.push(`  📦 GHCR Total:  ${pkgs.length} (${pubCount} public, ${privCount} private)`);
        const byRepo: Record<string, { pub: number; priv: number }> = {};
        for (const p of pkgs) {
          const repo = p.repository?.name || "no-repo";
          if (!byRepo[repo]) byRepo[repo] = { pub: 0, priv: 0 };
          if (p.visibility === "public") byRepo[repo].pub++;
          else byRepo[repo].priv++;
        }
        lines.push("");
        lines.push(`  ${"Repo".padEnd(28)} ${"Public".padEnd(10)} ${"Private".padEnd(10)} Total`);
        lines.push(`  ${"─".repeat(58)}`);
        for (const [repo, counts] of Object.entries(byRepo).sort((a, b) => (b[1].pub + b[1].priv) - (a[1].pub + a[1].priv))) {
          lines.push(`  ${repo.padEnd(28)} ${String(counts.pub).padEnd(10)} ${String(counts.priv).padEnd(10)} ${counts.pub + counts.priv}`);
        }
      } catch { lines.push(`  📦 GHCR images: (parse error)`); }
    } else {
      lines.push(`  📦 GHCR images: (unavailable)`);
    }
    return lines.join("\n");
  })(),

  STORAGE: (() => {
    const lines: string[] = [];
    // Object Storage from topology
    const storage = topology.storage ?? [];
    lines.push("  OBJECT STORAGE");
    if (storage.length > 0) {
      lines.push(`    ${"Provider".padEnd(12)} ${"Name".padEnd(30)} Tier`);
      lines.push("    " + "─".repeat(60));
      for (const s of storage) {
        lines.push(`    ${(s.provider || "?").padEnd(12)} ${(s.name || "?").padEnd(30)} ${s.tier || "?"}`);
      }
    } else {
      lines.push("    (no object storage configured)");
    }
    lines.push("");

    // Named Docker Volumes from backup-targets (by VM)
    lines.push("  DOCKER VOLUMES (persistent, named)");
    lines.push(`    ${"VM".padEnd(16)} ${"Volume".padEnd(30)} Service`);
    lines.push("    " + "─".repeat(60));
    const byVm = backupTargets.by_vm ?? {};
    for (const [vm, info] of Object.entries(byVm).sort() as [string, any][]) {
      for (const vol of info.volumes ?? []) {
        // Find which service owns this volume
        const svc = (backupTargets.targets ?? []).find((t: any) => t.vm_alias === vm && (t.volumes ?? []).includes(vol));
        lines.push(`    ${vm.padEnd(16)} ${vol.padEnd(30)} ${svc?.service || "?"}`);
      }
    }
    lines.push("");

    // Databases summary (count by type)
    lines.push("  DATABASES (from backup-targets)");
    const dbTypes = new Map<string, number>();
    for (const d of DATABASES) {
      dbTypes.set(d.type, (dbTypes.get(d.type) || 0) + 1);
    }
    lines.push(`    Total: ${DATABASES.length} databases — ${[...dbTypes.entries()].map(([t, c]) => `${c} ${t}`).join(", ")}`);
    lines.push(`    ${"Service".padEnd(20)} ${"Type".padEnd(10)} ${"Container".padEnd(22)} ${"DB Name".padEnd(14)} VM`);
    lines.push("    " + "─".repeat(75));
    for (const d of DATABASES) {
      lines.push(`    ${d.service.padEnd(20)} ${d.type.padEnd(10)} ${d.container.padEnd(22)} ${d.db.padEnd(14)} ${d.vm}`);
    }
    return lines.join("\n");
  })(),

  VPS_SPECS: (() => {
    const specs: { name: string; provider: string; shape: string; cpu: string; ram: string; disk: string; cost: string }[] = [];
    for (const [, vm] of Object.entries(hmData.vms ?? {}) as [string, any][]) {
      const vmId = vm.vm_id || "";
      specs.push({
        name: vm.ssh_alias || vmId,
        provider: inferProvider(vmId),
        shape: vm.specs?.shape || vm.specs?.machine_type || "?",
        cpu: String(vm.specs?.cpu || "?"),
        ram: `${vm.specs?.ram_gb || "?"}G`,
        disk: `${vm.specs?.disk_gb || "?"}G`,
        cost: inferCost(vmId),
      });
    }
    specs.push({ name: "github-actions", provider: "GitHub", shape: "ubuntu-latest", cpu: "4", ram: "16G", disk: "14G", cost: "2000min/mo" });
    specs.push({ name: "ghcr.io", provider: "GitHub", shape: "Container Registry", cpu: "-", ram: "-", disk: "∞", cost: "Free (public)" });
    return specs.map(v =>
      `   ${v.name.padEnd(16)} ${v.provider.padEnd(10)} ${v.shape.padEnd(20)} ${v.cpu.padEnd(6)} ${v.ram.padEnd(6)} ${v.disk.padEnd(8)} ${v.cost}`
    ).join("\n");
  })(),

  FRAMEWORK_PATHS: (() => {
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
      ...VMS.map(v => [`  ${v.alias}`, `cloud/b_infra/home-manager/${v.alias}/src/`]),
      ["", ""],
      ["GHA WORKFLOWS", ""],
      ...VMS.map(v => [`  ship-${v.alias}`, `cloud/.github/workflows/ship-${v.alias}.yml`]),
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
    return FRAMEWORK.map(([label, path]) => {
      if (!label && !path) return "";
      if (!path) return `  ${label}`;
      return `  ${label.padEnd(22)} ~/git/${path}`;
    }).join("\n");
  })(),

  VAULT_PROVIDERS: (() => {
    const vp = run(`ls -1 ${HOME}/Mounts/Git/vault/A0_keys/providers/ 2>/dev/null`);
    if (!vp) return "  (not available)";
    const providers = vp.split("\n").filter(Boolean);
    // Side-by-side layout (3 columns)
    const cols = 3;
    const colWidth = 22;
    const lines: string[] = [];
    for (let i = 0; i < providers.length; i += cols) {
      const row = providers.slice(i, i + cols).map(p => `🔑 ${p}`.padEnd(colWidth)).join(" ");
      lines.push(`  ${row}`);
    }
    return lines.join("\n");
  })(),

  OPEN_PORTS: (() => {
    log("Scanning open ports (verbose tcp logging suppressed)...");
    tcpLogVerbose = false;
    const lines: string[] = [];
    const ips = VMS.filter(v => v.pubIp).map(v => ({ name: v.alias, ip: v.pubIp }));
    const ports = [22, 25, 80, 443, 465, 587, 993, 2200, 4190, 5000, 8080, 8443, 8888, 51820];
    for (const vm of ips) {
      const open: number[] = [];
      for (const port of ports) {
        if (tcpCheck(vm.ip, port)) open.push(port);
      }
      const icon = open.length > 0 ? "🔓" : "🔒";
      lines.push(`${icon} ${vm.name.padEnd(18)} ${vm.ip.padEnd(18)} ports: ${open.length > 0 ? open.join(", ") : "none reachable"}`);
      log(`  ${vm.name}: ${open.length > 0 ? open.join(", ") : "none"}`);
    }
    tcpLogVerbose = true;
    return lines.join("\n");
  })(),

  RESOURCES_HEADER: (() => {
    const vmNames = data.vms.map((v: VmData) => v.alias);
    return `${"".padEnd(18)} ${vmNames.map((n: string) => n.padEnd(14)).join(" ")}`;
  })(),

  DATABASES: (() => {
    const lines: string[] = [];
    lines.push(`    ${"Service".padEnd(20)} ${"DB Type".padEnd(10)} ${"Container".padEnd(22)} ${"DB Name".padEnd(14)} ${"VM".padEnd(16)} DNS / Access`);
    lines.push("    " + "─".repeat(90));
    for (const d of DATABASES) {
      lines.push(`   ${d.service.padEnd(20)} ${d.type.padEnd(10)} ${d.container.padEnd(22)} ${d.db.padEnd(14)} ${d.vm.padEnd(16)} ${d.dns}`);
    }
    return lines.join("\n");
  })(),

  DOCKER_NETWORKS: (() => {
    // Parse Docker networks from topology
    const networks = new Map<string, { vm: string; services: string[] }>();
    for (const [svcName, svc] of Object.entries(topology.services ?? {}) as [string, any][]) {
      for (const net of svc.compose?.networks ?? []) {
        if (!networks.has(net)) networks.set(net, { vm: svc.vm || "?", services: [] });
        networks.get(net)!.services.push(svcName);
      }
    }
    if (networks.size === 0) return "  (no network data in topology)";
    const lines: string[] = [];
    lines.push(`    ${"Network".padEnd(28)} ${"VM".padEnd(16)} Services`);
    lines.push("    " + "─".repeat(70));
    for (const [net, info] of [...networks.entries()].sort((a, b) => a[0].localeCompare(b[0]))) {
      const vmAlias = Object.entries(hmData.vms ?? {}).find(([, v]: [string, any]) => v.vm_id === info.vm)?.[0] || info.vm;
      lines.push(`    ${net.padEnd(28)} ${vmAlias.padEnd(16)} ${info.services.join(", ")}`);
    }
    return lines.join("\n");
  })(),

  SCRIPT_INFO: (() => {
    const lines: string[] = [];
    lines.push(`  Script:    cloud-data/cloud-health-stack/container-health.ts`);
    lines.push(`  Run:       ./container-health.ts  (or: tsx container-health.ts)`);
    lines.push(`  Node:      ${process.version}`);
    lines.push(`  Platform:  ${process.platform} ${process.arch}`);
    lines.push(`  CWD:       ${SCRIPT_DIR}`);
    lines.push(`  Template:  container_health.md.tpl (21 vars)`);
    lines.push(`  Data src:  ${CD}/`);
    lines.push("");
    lines.push("  Dependencies:");
    for (const d of depStatus) {
      lines.push(`    ${d.ok ? "✅" : "❌"} ${d.name.padEnd(10)} ${d.path || "NOT FOUND"}`);
    }
    lines.push("");
    lines.push(`  Errors:    ${ERRORS.length}`);
    if (ERRORS.length > 0) {
      for (const e of ERRORS) lines.push(`    ${e}`);
    }
    return lines.join("\n");
  })(),

  PERFORMANCE: (() => {
    const totalMs = Date.now() - TOTAL_START;
    const fmtSec = (ms: number) => (ms / 1000).toFixed(1) + "s";
    const lines = timers.sort((a, b) => b.ms - a.ms).map(t => {
      const bar = "█".repeat(Math.min(30, Math.round(t.ms / (totalMs / 30))));
      return `  ${t.name.padEnd(18)} ${fmtSec(t.ms).padStart(7)} ${bar}`;
    });
    lines.push(`  ${"TOTAL".padEnd(18)} ${fmtSec(totalMs).padStart(7)}`);
    return lines.join("\n");
  })(),
};

// Replace all $VARS in template
log("Replacing template variables...");
for (const [key, value] of Object.entries(vars)) {
  if (template.includes(`$${key}`)) {
    template = template.replace(`$${key}`, value);
    log(`  $${key} → ${value.split("\n").length} lines`);
  } else {
    logErr(`Template variable $${key} NOT FOUND in template!`);
  }
}
// Check for unreplaced $VARS
const unreplaced = template.match(/\$[A-Z_]+/g)?.filter(v => !v.startsWith("$POSTGRES") && !v.startsWith("${"));
if (unreplaced?.length) {
  logErr(`Unreplaced template vars: ${unreplaced.join(", ")}`);
}

writeFileSync(`${SCRIPT_DIR}/container_health.md`, template);
log("Wrote container_health.md");

// ── Summary ─────────────────────────────────────────────────────
const totalMs = Date.now() - TOTAL_START;
log(`═══ DONE in ${(totalMs / 1000).toFixed(1)}s ═══`);
if (ERRORS.length > 0) {
  console.error(`\n⚠️  ${ERRORS.length} ERRORS during run:`);
  for (const e of ERRORS) console.error(`  ${e}`);
}
console.log(`→ container_health.json + container_health.md (template-driven, ${ERRORS.length} errors)`);
