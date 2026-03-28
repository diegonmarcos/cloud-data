/**
 * vars-containers.ts — A2: PRIVATE_DNS, VM_CONTAINERS
 */
import { run, type VmData } from "./collectors.js";
import type { VarContext } from "./types.js";

export function varsContainers(ctx: VarContext): Record<string, string> {
  const { data, PRIVATE_DNS } = ctx;

  return {
    PRIVATE_DNS: (() => {
      const portCount = new Map<number, string[]>();
      for (const d of data.private_dns) {
        if (!portCount.has(d.port)) portCount.set(d.port, []);
        portCount.get(d.port)!.push(d.dns);
      }
      const conflictPorts = new Set([...portCount.entries()].filter(([, names]) => names.length > 1).map(([p]) => p));
      const wgDown = data.private_dns[0]?.wg_down === true;
      if (wgDown) {
        const lines: string[] = [];
        lines.push("⚠️  WireGuard DOWN — Hickory DNS (10.0.0.1) unreachable, cannot check .app names");
        lines.push("");
        // Still show the table but with ⏸️ instead of ❌
        for (const d of data.private_dns) {
          const conflict = conflictPorts.has(d.port);
          const portTag = conflict ? `⚠️${String(d.port).padEnd(5)}` : `  ${String(d.port).padEnd(5)}`;
          lines.push(`⏸️ ${d.dns.padEnd(28)} ${(d.container + ":" + d.port).padEnd(25)} ${portTag} ${d.vm}`);
        }
        // Still append port conflicts + DNS config check
        if (conflictPorts.size > 0) {
          lines.push("");
          lines.push(`  ⚠️  PORT CONFLICTS (${conflictPorts.size} duplicate ports globally):`);
          for (const [port, names] of [...portCount.entries()].filter(([, n]) => n.length > 1).sort((a, b) => a[0] - b[0])) {
            lines.push(`     :${String(port).padEnd(6)} used by: ${names.join(", ")}`);
          }
        }
        lines.push("");
        lines.push("  ─── DNS CONFIG CHECK ───");
        lines.push("  ❌ WireGuard tunnel is DOWN — bring up with: sudo wg-quick up wg0");
        lines.push("  ⏸️ All .app DNS checks skipped (not service failures)");
        return lines.join("\n");
      }
      const lines = data.private_dns.map((d: any) => {
        const conflict = conflictPorts.has(d.port);
        const icon = d.open ? "✅" : "❌";
        const portTag = conflict ? `⚠️${String(d.port).padEnd(5)}` : `  ${String(d.port).padEnd(5)}`;
        return `${icon} ${d.dns.padEnd(28)} ${(d.container + ":" + d.port).padEnd(25)} ${portTag} ${d.vm}`;
      });
      if (conflictPorts.size > 0) {
        lines.push("");
        lines.push(`  ⚠️  PORT CONFLICTS (${conflictPorts.size} duplicate ports globally):`);
        for (const [port, names] of [...portCount.entries()].filter(([, n]) => n.length > 1).sort((a, b) => a[0] - b[0])) {
          lines.push(`     :${String(port).padEnd(6)} used by: ${names.join(", ")}`);
        }
      }
      // DNS config checker
      lines.push("");
      lines.push("  ─── DNS CONFIG CHECK ───");
      const resolv = run("grep nameserver /etc/resolv.conf 2>/dev/null");
      const hasHickory = resolv.includes("10.0.0.1");
      lines.push(`  ${hasHickory ? "✅" : "❌"} /etc/resolv.conf     ${hasHickory ? "includes 10.0.0.1 (Hickory)" : "MISSING 10.0.0.1 — .app names won't resolve!"}`);
      if (resolv) { for (const ns of resolv.split("\n").filter(Boolean)) lines.push(`     ${ns.trim()}`); }
      const testName = data.private_dns[0]?.dns || "authelia.app";
      const systemResolve = run(`dig +short ${testName} 2>/dev/null`);
      const hickoryResolve = run(`dig @10.0.0.1 +short ${testName} 2>/dev/null`);
      lines.push(`  ${systemResolve ? "✅" : "❌"} dig ${testName.padEnd(20)} ${systemResolve || "NXDOMAIN"} (system DNS)`);
      lines.push(`  ${hickoryResolve ? "✅" : "❌"} dig @10.0.0.1 ${testName.padEnd(12)} ${hickoryResolve || "NXDOMAIN"} (Hickory direct)`);
      if (!systemResolve && hickoryResolve) {
        lines.push("  ⚠️  System DNS can't resolve .app names — add 10.0.0.1 to resolv.conf or WG DNS");
        lines.push("     All ❌ above are due to missing local DNS config, NOT service failures");
      }
      if (systemResolve && hickoryResolve) lines.push("  ✅ Local DNS properly configured — Hickory resolves .app names");
      if (!hickoryResolve) lines.push("  ❌ Hickory DNS (10.0.0.1) not responding — check gcp-proxy hickory-dns container");
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
        else if (c.health === "exited") { const code = c.status.match(/Exited \((\d+)\)/)?.[1] || "?"; tag = `EXITED(${code})`; }
        else if (c.status.startsWith("Up")) tag = "UP";
        const dnsEntry = PRIVATE_DNS.find(p => p.container === c.name);
        const portStr = dnsEntry ? String(dnsEntry.port).padEnd(7) : "       ";
        lines.push(`  ${c.icon} ${c.name.padEnd(25)} ${portStr} ${tag.padEnd(14)} ${c.status.substring(0, 30)}`);
      }
      lines.push("");
      return lines.join("\n");
    }).join("\n"),
  };
}
