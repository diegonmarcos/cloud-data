/**
 * vars-containers.ts — A2: PRIVATE_HEALTH, A3: VM_CONTAINERS
 * A2: multi-protocol health checks to .app DNS names (TCP + HTTP)
 * A3: docker ps state per VM (HEALTHY / UNHEALTHY / DOWN)
 */
import { run, tcpCheckAsync, privateEndpointCheck, type VmData } from "./collectors.js";
import { log } from "./config.js";
import type { VarContext } from "./types.js";

export interface PrivateHealthResult {
  dns: string; container: string; port: number; vm: string;
  tcp: boolean; http: boolean; code: string;
  wg_down: boolean;
}

/**
 * Async: parallel multi-protocol health checks on all private .app endpoints
 * Uses DNS NAME directly (not IP) — tests if local system can resolve + reach
 */
export async function checkPrivateHealth(
  privateDns: { dns: string; container: string; port: number; vm: string }[]
): Promise<PrivateHealthResult[]> {
  // First check if Hickory is reachable (dig probe — more reliable than nc under load)
  const hickoryProbe = await runAsync("dig @10.0.0.1 +short +time=3 +tries=2 authelia.app 2>/dev/null", 8000);
  const hickoryUp = !!hickoryProbe && hickoryProbe.length > 0;
  if (!hickoryUp) {
    log("  ⚠️ WireGuard/Hickory DOWN — skipping private health checks");
    return privateDns.map(d => ({ ...d, tcp: false, http: false, code: "---", wg_down: true }));
  }

  log(`  Checking ${privateDns.length} private endpoints (TCP+HTTP, parallel)...`);
  const results = await Promise.all(
    privateDns.map(async (d) => {
      const check = await privateEndpointCheck(d.dns, d.port);
      return { ...d, ...check, wg_down: false };
    })
  );
  const tcpUp = results.filter(r => r.tcp).length;
  const httpUp = results.filter(r => r.http).length;
  log(`  Private health: ${tcpUp}/${results.length} TCP, ${httpUp}/${results.length} HTTP`);
  return results;
}

export function varsContainers(ctx: VarContext, privateHealthResults?: PrivateHealthResult[]): Record<string, string> {
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

      // Table
      lines.push(`    ${"DNS Name".padEnd(28)} 📡TCP 🌐HTTP ${"Port".padEnd(7)} ${"VM".padEnd(16)} ${"Container".padEnd(22)} Code`);
      lines.push("    " + "─".repeat(95));

      for (const r of results) {
        const conflict = conflictPorts.has(r.port);
        const portTag = conflict ? `⚠️${String(r.port).padEnd(4)}` : `  ${String(r.port).padEnd(4)}`;
        const allOk = r.tcp && r.http;
        let icon: string;
        if (wgDown) icon = "⏸️";
        else if (allOk) icon = "✅";
        else if (r.tcp) icon = "⚠️";
        else icon = "❌";
        const tcpIcon = wgDown ? "⏸️" : r.tcp ? "✅" : "❌";
        const httpIcon = wgDown ? "⏸️" : r.http ? "✅" : "❌";
        lines.push(`${icon} ${r.dns.padEnd(28)} ${tcpIcon}   ${httpIcon}   ${portTag} ${r.vm.padEnd(16)} ${r.container.padEnd(22)} [${r.code}]`);
      }

      // Port conflict summary
      if (conflictPorts.size > 0) {
        lines.push("");
        lines.push(`  ⚠️  PORT CONFLICTS (${conflictPorts.size} duplicate ports globally):`);
        for (const [port, names] of [...portCount.entries()].filter(([, n]) => n.length > 1).sort((a, b) => a[0] - b[0])) {
          lines.push(`     :${String(port).padEnd(6)} used by: ${names.join(", ")}`);
        }
      }

      // DNS config checker at bottom
      lines.push("");
      lines.push("  ─── DNS CONFIG CHECK ───");
      const resolv = run("grep nameserver /etc/resolv.conf 2>/dev/null");
      const hasHickory = resolv.includes("10.0.0.1");
      lines.push(`  ${hasHickory ? "✅" : "❌"} /etc/resolv.conf     ${hasHickory ? "includes 10.0.0.1 (Hickory)" : "MISSING 10.0.0.1 — .app names won't resolve!"}`);
      if (resolv) { for (const ns of resolv.split("\n").filter(Boolean)) lines.push(`     ${ns.trim()}`); }
      const testName = PRIVATE_DNS[0]?.dns || "authelia.app";
      const systemResolve = run(`dig +short ${testName} 2>/dev/null`);
      const hickoryResolve = run(`dig @10.0.0.1 +short +time=2 +tries=1 ${testName} 2>/dev/null`);
      lines.push(`  ${systemResolve ? "✅" : "❌"} dig ${testName.padEnd(20)} ${systemResolve || "NXDOMAIN"} (system DNS)`);
      lines.push(`  ${hickoryResolve ? "✅" : "❌"} dig @10.0.0.1 ${testName.padEnd(12)} ${hickoryResolve || "NXDOMAIN"} (Hickory direct)`);
      if (!systemResolve && hickoryResolve) {
        lines.push("  ⚠️  System DNS can't resolve .app — add 10.0.0.1 to resolv.conf");
        lines.push("     All checks above test via system DNS — if Hickory not configured, all fail");
      }
      if (systemResolve && hickoryResolve) lines.push("  ✅ Local DNS properly configured");
      if (!hickoryResolve) lines.push("  ❌ Hickory DNS (10.0.0.1) not responding — WG down or hickory-dns container down");

      // Summary
      if (!wgDown) {
        const tcpUp = results.filter(r => r.tcp).length;
        const httpUp = results.filter(r => r.http).length;
        lines.push("");
        lines.push(`  📡 TCP: ${tcpUp}/${results.length}  🌐 HTTP: ${httpUp}/${results.length}`);
      }

      return lines.join("\n");
    })(),

    VM_CONTAINERS: data.vms.map((vm: VmData) => {
      const lines: string[] = [];
      const st = vm.reachable ? "✅" : "❌";
      lines.push(`${vm.alias} ${st} — ${vm.os} — ${vm.cpus}C/${vm.ram} — mem ${vm.mem_used}/${vm.mem_total} (${vm.mem_pct}%) — disk ${vm.disk_pct} — swap ${vm.swap} — load ${vm.load} — ${vm.containers_running}/${vm.containers_total} ctrs — ${vm.uptime}`);
      lines.push("─".repeat(60));
      for (const c of vm.containers) {
        let state: string;
        let icon: string;
        if (c.health === "healthy") { state = "HEALTHY"; icon = "✅"; }
        else if (c.health === "unhealthy") { state = "UNHEALTHY"; icon = "❌"; }
        else if (c.health === "exited" || c.health === "created") {
          const code = c.status.match(/Exited \((\d+)\)/)?.[1] || "?";
          state = `DOWN(${code})`;
          icon = "❌";
        } else if (c.status.startsWith("Up") && c.health === "none") {
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
