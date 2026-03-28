/**
 * vars-security.ts — C: OPEN_PORTS, DATABASES, DOCKER_NETWORKS, VAULT_PROVIDERS
 * OPEN_PORTS uses async parallel port scanning for speed
 */
import { run, tcpCheck, tcpScanParallel, setTcpLogVerbose } from "./collectors.js";
import { HOME, log } from "./config.js";
import type { VarContext } from "./types.js";

/**
 * Async: parallel port scan across all VMs (biggest perf win)
 * Must be called separately and result passed into varsSecurity
 */
export async function scanOpenPorts(VMS: VarContext["VMS"]): Promise<string> {
  log("Scanning open ports (parallel — all VMs × 14 ports)...");
  const ips = VMS.filter(v => v.pubIp).map(v => ({ name: v.alias, ip: v.pubIp }));
  const ports = [22, 25, 80, 443, 465, 587, 993, 2200, 4190, 5000, 8080, 8443, 8888, 51820];

  // Scan ALL VMs in parallel (each VM scans all ports in parallel)
  const scanResults = await Promise.all(
    ips.map(async (vm) => {
      const open = await tcpScanParallel(vm.ip, ports);
      log(`  ${vm.name}: ${open.length > 0 ? open.join(", ") : "none"}`);
      return { name: vm.name, ip: vm.ip, open };
    })
  );

  return scanResults.map(r =>
    `${r.open.length > 0 ? "🔓" : "🔒"} ${r.name.padEnd(18)} ${r.ip.padEnd(18)} ports: ${r.open.length > 0 ? r.open.join(", ") : "none reachable"}`
  ).join("\n");
}

export function varsSecurity(ctx: VarContext, openPortsResult: string): Record<string, string> {
  const { topology, hmData, VMS, DATABASES } = ctx;

  return {
    OPEN_PORTS: openPortsResult,

    DATABASES: (() => {
      const lines: string[] = [];
      lines.push(`    ${"Service".padEnd(20)} ${"DB Type".padEnd(10)} ${"Container".padEnd(22)} ${"DB Name".padEnd(14)} ${"VM".padEnd(16)} DNS / Access`);
      lines.push("    " + "─".repeat(90));
      for (const d of DATABASES) lines.push(`   ${d.service.padEnd(20)} ${d.type.padEnd(10)} ${d.container.padEnd(22)} ${d.db.padEnd(14)} ${d.vm.padEnd(16)} ${d.dns}`);
      return lines.join("\n");
    })(),

    DOCKER_NETWORKS: (() => {
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

    VAULT_PROVIDERS: (() => {
      const vp = run(`ls -1 ${HOME}/Mounts/Git/vault/A0_keys/providers/ 2>/dev/null`);
      if (!vp) return "  (not available)";
      const providers = vp.split("\n").filter(Boolean);
      const cols = 3, colWidth = 22;
      const lines: string[] = [];
      for (let i = 0; i < providers.length; i += cols) {
        const row = providers.slice(i, i + cols).map(p => `🔑 ${p}`.padEnd(colWidth)).join(" ");
        lines.push(`  ${row}`);
      }
      return lines.join("\n");
    })(),
  };
}
