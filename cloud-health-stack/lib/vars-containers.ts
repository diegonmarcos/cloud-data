/**
 * vars-containers.ts — A2: PRIVATE_HEALTH, A3: VM_CONTAINERS
 * A2: actual health checks to .app DNS names (curl/nc, not just dig)
 * A3: docker ps state per VM (HEALTHY / UNHEALTHY / DOWN)
 */
import { run, tcpCheckAsync, type VmData } from "./collectors.js";
import { log } from "./config.js";
import type { VarContext } from "./types.js";

/**
 * Async: parallel health checks on all private .app endpoints
 * Uses the DNS name directly (not IP) — tests real connectivity
 */
export async function checkPrivateHealth(
  privateDns: { dns: string; container: string; port: number; vm: string }[]
): Promise<{ dns: string; container: string; port: number; vm: string; healthy: boolean; method: string; wg_down: boolean }[]> {
  // First check if WG/Hickory is reachable at all
  const hickoryUp = await tcpCheckAsync("10.0.0.1", 53);
  if (!hickoryUp) {
    log("  ⚠️ WireGuard/Hickory DOWN — skipping private health checks");
    return privateDns.map(d => ({ ...d, healthy: false, method: "skipped", wg_down: true }));
  }

  log(`  Checking ${privateDns.length} private endpoints in parallel...`);
  // Check all endpoints in parallel (nc to dns_name:port)
  const results = await Promise.all(
    privateDns.map(async (d) => {
      const ok = await tcpCheckAsync(d.dns, d.port);
      return { ...d, healthy: ok, method: ok ? "tcp" : "unreachable", wg_down: false };
    })
  );
  const up = results.filter(r => r.healthy).length;
  log(`  Private health: ${up}/${results.length} reachable`);
  return results;
}

export function varsContainers(ctx: VarContext, privateHealthResults?: Awaited<ReturnType<typeof checkPrivateHealth>>): Record<string, string> {
  const { data, PRIVATE_DNS } = ctx;

  return {
    PRIVATE_HEALTH: (() => {
      const results = privateHealthResults || [];
      const lines: string[] = [];
      const wgDown = results.length > 0 && results[0]?.wg_down === true;

      if (wgDown) {
        lines.push("⚠️  WireGuard DOWN — cannot reach private .app endpoints");
        lines.push("    Run: sudo wg-quick up wg0");
        lines.push("");
      }

      // Port conflict detection
      const portCount = new Map<number, string[]>();
      for (const d of PRIVATE_DNS) {
        if (!portCount.has(d.port)) portCount.set(d.port, []);
        portCount.get(d.port)!.push(d.dns);
      }
      const conflictPorts = new Set([...portCount.entries()].filter(([, n]) => n.length > 1).map(([p]) => p));

      // Table header
      lines.push(`    ${"DNS Name".padEnd(28)} ${"Container:Port".padEnd(25)} ${"Port".padEnd(7)} ${"VM".padEnd(16)} Health`);
      lines.push("    " + "─".repeat(85));

      for (const r of results) {
        const conflict = conflictPorts.has(r.port);
        const portTag = conflict ? `⚠️${String(r.port).padEnd(4)}` : `  ${String(r.port).padEnd(4)}`;
        let icon: string;
        if (wgDown) icon = "⏸️";
        else if (r.healthy) icon = "✅";
        else icon = "❌";
        lines.push(`${icon} ${r.dns.padEnd(28)} ${(r.container + ":" + r.port).padEnd(25)} ${portTag} ${r.vm.padEnd(16)} ${r.method}`);
      }

      // Port conflict summary
      if (conflictPorts.size > 0) {
        lines.push("");
        lines.push(`  ⚠️  PORT CONFLICTS (${conflictPorts.size} duplicate ports globally):`);
        for (const [port, names] of [...portCount.entries()].filter(([, n]) => n.length > 1).sort((a, b) => a[0] - b[0])) {
          lines.push(`     :${String(port).padEnd(6)} used by: ${names.join(", ")}`);
        }
      }

      // Summary
      if (!wgDown) {
        const up = results.filter(r => r.healthy).length;
        const down = results.filter(r => !r.healthy).length;
        lines.push("");
        lines.push(`  ${up > 0 ? "✅" : "❌"} ${up}/${results.length} private endpoints reachable, ${down} unreachable`);
      }

      return lines.join("\n");
    })(),

    VM_CONTAINERS: data.vms.map((vm: VmData) => {
      const lines: string[] = [];
      const st = vm.reachable ? "✅" : "❌";
      lines.push(`${vm.alias} ${st} — ${vm.os} — ${vm.cpus}C/${vm.ram} — mem ${vm.mem_used}/${vm.mem_total} (${vm.mem_pct}%) — disk ${vm.disk_pct} — swap ${vm.swap} — load ${vm.load} — ${vm.containers_running}/${vm.containers_total} ctrs — ${vm.uptime}`);
      lines.push("─".repeat(60));
      for (const c of vm.containers) {
        // Normalize to 3 states: HEALTHY / UNHEALTHY / DOWN
        let state: string;
        let icon: string;
        if (c.health === "healthy") { state = "HEALTHY"; icon = "✅"; }
        else if (c.health === "unhealthy") { state = "UNHEALTHY"; icon = "❌"; }
        else if (c.health === "exited" || c.health === "created") {
          const code = c.status.match(/Exited \((\d+)\)/)?.[1] || "?";
          state = `DOWN(${code})`;
          icon = "❌";
        } else if (c.status.startsWith("Up") && c.health === "none") {
          // UP but no healthcheck = UNHEALTHY (no healthcheck configured)
          state = "UP (no hc)";
          icon = "⚠️";
        } else if (c.health === "starting") {
          state = "STARTING";
          icon = "⚠️";
        } else {
          state = c.health.toUpperCase();
          icon = "⚠️";
        }
        const dnsEntry = PRIVATE_DNS.find(p => p.container === c.name);
        const portStr = dnsEntry ? String(dnsEntry.port).padEnd(7) : "       ";
        lines.push(`  ${icon} ${c.name.padEnd(25)} ${portStr} ${state.padEnd(14)} ${c.status.substring(0, 30)}`);
      }
      lines.push("");
      return lines.join("\n");
    }).join("\n"),
  };
}
