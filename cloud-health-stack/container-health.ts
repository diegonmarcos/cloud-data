#!/usr/bin/env tsx
/**
 * Container Health Reporter — Template-driven
 * Collects live data → container_health.json
 * Reads container_health.md.tpl → replaces $VARS → container_health.md
 * Usage: ./container-health.ts   (or: tsx container-health.ts)
 *
 * Dependencies: node ≥18, tsx, ssh, curl, nc, dig, git, gh
 */
import { writeFileSync, readFileSync } from "fs";
import { join } from "path";

import { SCRIPT_DIR, CD, log, logErr, ERRORS, loadJson } from "./lib/config.js";
import { run, sshCmd, tcpCheck, timed, timedAsync, collectVm, type VmData } from "./lib/collectors.js";
import { parseVms, parsePublicUrls, parseMcpApiEndpoints, parseMailPorts, parsePrivateDns, parseDatabases } from "./lib/parsers.js";
import type { VarContext } from "./lib/types.js";
import { varsHealth, checkPublicUrls } from "./lib/vars-health.js";
import { varsContainers, checkPrivateHealth } from "./lib/vars-containers.js";
import { varsMail } from "./lib/vars-mail.js";
import { varsInfra } from "./lib/vars-infra.js";
import { varsSecurity, scanOpenPorts } from "./lib/vars-security.js";
import { varsStack } from "./lib/vars-stack.js";
import { varsAppendix, buildIssuesSummary } from "./lib/vars-appendix.js";

// ── Load cloud-data ──────────────────────────────────────────
const topology = loadJson("cloud-data-topology.json");
const caddyRoutes = loadJson("cloud-data-caddy-routes.json");
const ghaConfig = loadJson("cloud-data-gha-config.json");
const hmData = loadJson("cloud-data-home-manager.json");
const wgPeersData = loadJson("cloud-data-wireguard-peers.json");
const backupTargets = loadJson("cloud-data-backup-targets.json");

// ── Parse ────────────────────────────────────────────────────
const VMS = parseVms(hmData);
const PUBLIC_URLS = parsePublicUrls(caddyRoutes);
const MCP_API_ENDPOINTS = parseMcpApiEndpoints(caddyRoutes);
const MAIL_PORTS = parseMailPorts(caddyRoutes);
const PRIVATE_DNS = parsePrivateDns(topology, hmData);
const DATABASES = parseDatabases(backupTargets, PRIVATE_DNS);

// ── Collect live data ────────────────────────────────────────
const TOTAL_START = Date.now();
log("═══ Starting collection ═══");
log(`VMs: ${VMS.map(v => v.alias).join(", ")} (${VMS.length})`);
log(`URLs: ${PUBLIC_URLS.length}, MCP: ${MCP_API_ENDPOINTS.length}, Mail: ${MAIL_PORTS.length}, DNS: ${PRIVATE_DNS.length}, DBs: ${DATABASES.length}`);

// WG0 peers
log("Collecting WG0 peers from gcp-proxy...");
const wgRaw = sshCmd("gcp-proxy", "sudo wg show wg0 2>/dev/null");
const wgPeers: any[] = [];
if (wgRaw) {
  for (const block of wgRaw.split("\npeer: ").slice(1)) {
    const endpoint = block.match(/endpoint: (.+)/)?.[1] || "none";
    const pubIp = endpoint.split(":")[0] || "none";
    const handshake = block.match(/latest handshake: (.+)/)?.[1] || "never";
    const transfer = block.match(/transfer: (.+)/)?.[1] || "0 B";
    const privIp = block.match(/allowed ips: ([\d./]+)/)?.[1]?.replace("/32", "") || "?";
    const name = VMS.find(v => v.ip === privIp)?.alias || privIp;
    const alive = handshake !== "never" && !handshake.includes("hour") && !handshake.includes("day");
    wgPeers.push({ name, pubIp, privIp, handshake, transfer, alive });
  }
}

const data = {
  generated: new Date().toISOString(),
  wg_peers: wgPeers,
  api_mcp: timed("api_mcp", () => MCP_API_ENDPOINTS.map(e => {
    const code = run(`curl -sko /dev/null -w '%{http_code}' https://${e.url} 2>/dev/null`);
    return { ...e, http_code: code, up: code !== "" && code !== "000" && code !== "502" };
  })),
  public_urls: timed("public_urls", () => PUBLIC_URLS.map(u => {
    const code = run(`curl -sko /dev/null -w '%{http_code}' https://${u.url} 2>/dev/null`);
    return { ...u, http_code: code, up: code !== "" && code !== "000" && code !== "502" };
  })),
  mail_ports: timed("mail_ports", () => MAIL_PORTS.map(m => ({ ...m, open: tcpCheck(m.host, m.port) }))),
  private_dns: PRIVATE_DNS, // raw parsed data — health checks done async in A2
  vms: VMS.map(vm => timed(`vm_${vm.alias}`, () => { log(`  Collecting VM: ${vm.alias} (${vm.pubIp || vm.ip})...`); return collectVm(vm); })),
  databases: DATABASES,
};

log("═══ Data collection complete ═══");
log(`WG peers: ${data.wg_peers.length}, URLs: ${data.public_urls.length}`);
log(`VMs: ${data.vms.filter((v: VmData) => v.reachable).length}/${data.vms.length} reachable`);
for (const vm of data.vms) {
  if (vm.reachable) log(`  ✅ ${vm.alias}: ${vm.containers_running}/${vm.containers_total} ctrs, mem ${vm.mem_pct}%, disk ${vm.disk_pct}`);
  else logErr(`  ❌ ${vm.alias}: UNREACHABLE`);
}

writeFileSync(`${SCRIPT_DIR}/container_health.json`, JSON.stringify(data, null, 2) + "\n");
log("Wrote container_health.json");

// ── Template-driven MD generation ────────────────────────────
log("═══ Generating MD from template ═══");
const tplPath = join(SCRIPT_DIR, "container_health.md.tpl");
let template: string;
try { template = readFileSync(tplPath, "utf-8"); log(`Template loaded (${template.length} chars)`); }
catch (e: any) { logErr(`Failed to read template: ${e.message}`); process.exit(1); }

const hubVm = VMS.find(v => v.alias === "gcp-proxy");
const ctx: VarContext = { data, topology, caddyRoutes, hmData, wgPeersData, backupTargets, VMS, PRIVATE_DNS, DATABASES };

// Build vars — parallel async tasks first, then sync vars
(async () => {
  // Launch ALL parallel checks at once
  log("Launching parallel checks (public URLs + private health + port scan)...");
  const portScanPromise = timedAsync("open_ports", () => scanOpenPorts(VMS));
  const privateHealthPromise = timedAsync("private_health", () => checkPrivateHealth(PRIVATE_DNS));
  const publicUrlPromise = timedAsync("public_urls_multi", () => checkPublicUrls(
    data.public_urls.map((u: any) => ({ url: u.url, upstream: u.upstream }))
  ));

  // Build sync vars while parallel tasks run
  const vars: Record<string, string> = {
    GENERATED_DATE: `${data.generated.split("T")[0]}  ${data.generated.split("T")[1]?.split(".")[0] || ""}`,
    HUB_WG_IP: hubVm?.ip || "?",
    ...varsMail(ctx),
    ...varsInfra(ctx),
    ...varsStack(ctx),
    ...varsAppendix(ctx, TOTAL_START),
  };

  // Wait for ALL parallel tasks
  const [openPortsResult, privateHealthResults, publicUrlResults] = await Promise.all([
    portScanPromise, privateHealthPromise, publicUrlPromise
  ]);
  Object.assign(vars, varsHealth(ctx, publicUrlResults));
  Object.assign(vars, varsContainers(ctx, privateHealthResults));
  Object.assign(vars, varsSecurity(ctx, openPortsResult));

  // Issues summary (must be last — scans all collected data)
  vars.ISSUES_SUMMARY = buildIssuesSummary(ctx);

  // Replace all $VARS in template (longest key first to avoid partial matches)
  log("Replacing template variables...");
  const sortedKeys = Object.keys(vars).sort((a, b) => b.length - a.length);
  for (const key of sortedKeys) {
    const value = vars[key];
    if (template.includes(`$${key}`)) {
      template = template.replace(`$${key}`, value);
      log(`  $${key} → ${value.split("\n").length} lines`);
    } else {
      logErr(`Template variable $${key} NOT FOUND in template!`);
    }
  }
  const unreplaced = template.match(/\$[A-Z_]+/g)?.filter(v => !v.startsWith("$POSTGRES") && !v.startsWith("${"));
  if (unreplaced?.length) logErr(`Unreplaced template vars: ${unreplaced.join(", ")}`);

  writeFileSync(`${SCRIPT_DIR}/container_health.md`, template);
  log("Wrote container_health.md");

  // ── Summary ──────────────────────────────────────────────────
  const totalMs = Date.now() - TOTAL_START;
  log(`═══ DONE in ${(totalMs / 1000).toFixed(1)}s ═══`);
  if (ERRORS.length > 0) {
    console.error(`\n⚠️  ${ERRORS.length} ERRORS during run:`);
    for (const e of ERRORS) console.error(`  ${e}`);
  }
  console.log(`→ container_health.json + container_health.md (template-driven, ${ERRORS.length} errors)`);
})();
