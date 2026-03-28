/**
 * vars-appendix.ts — Z: PERFORMANCE, SCRIPT_INFO + ISSUES_SUMMARY (post-build)
 */
import { timers, type VmData } from "./collectors.js";
import { SCRIPT_DIR, CD, ERRORS, depStatus } from "./config.js";
import type { VarContext } from "./types.js";

export function varsAppendix(ctx: VarContext, TOTAL_START: number): Record<string, string> {
  return {
    SCRIPT_INFO: (() => {
      const lines: string[] = [];
      lines.push(`  Script:    cloud-data/cloud-health-stack/container-health.ts`);
      lines.push(`  Run:       ./container-health.ts  (or: tsx container-health.ts)`);
      lines.push(`  Node:      ${process.version}`);
      lines.push(`  Platform:  ${process.platform} ${process.arch}`);
      lines.push(`  CWD:       ${SCRIPT_DIR}`);
      lines.push(`  Template:  container_health.md.tpl`);
      lines.push(`  Data src:  ${CD}/`);
      lines.push("");
      lines.push("  Dependencies:");
      for (const d of depStatus) lines.push(`    ${d.ok ? "✅" : "❌"} ${d.name.padEnd(10)} ${d.path || "NOT FOUND"}`);
      lines.push("");
      lines.push(`  Errors:    ${ERRORS.length}`);
      if (ERRORS.length > 0) { for (const e of ERRORS) lines.push(`    ${e}`); }
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
}

export function buildIssuesSummary(ctx: VarContext): string {
  const { data, VMS } = ctx;
  const issues: { severity: string; section: string; msg: string }[] = [];

  for (const vm of data.vms) {
    if (!vm.reachable) issues.push({ severity: "❌", section: "A2", msg: `VM ${vm.alias} — UNREACHABLE` });
  }
  for (const vm of data.vms) {
    for (const c of vm.containers) {
      if (c.health === "exited" || c.health === "unhealthy") {
        const code = c.status.match(/Exited \((\d+)\)/)?.[1];
        issues.push({ severity: c.health === "unhealthy" ? "⚠️" : "❌", section: "A2", msg: `${vm.alias}/${c.name} — ${c.health}${code ? `(${code})` : ""}` });
      }
    }
  }
  for (const u of data.public_urls) { if (!u.up) issues.push({ severity: "❌", section: "A1", msg: `${u.url} — [${u.http_code || "---"}]` }); }
  for (const e of data.api_mcp) { if (!e.up) issues.push({ severity: "❌", section: "A1", msg: `MCP ${e.name} — [${e.http_code || "---"}]` }); }
  for (const m of data.mail_ports) { if (!m.open) issues.push({ severity: "⚠️", section: "A3", msg: `${m.host}:${m.port} ${m.proto} — down` }); }
  const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
  issues.push({ severity: "❌", section: "A3", msg: `Stalwart SPF FAIL — VM IP ${ociMailIp} not in SPF (outbound emails rejected)` });
  for (const e of ERRORS) { issues.push({ severity: "❌", section: "SYS", msg: e.replace(/^\[.*?\] ERROR: /, "") }); }

  if (issues.length === 0) return "✅ No issues found — all systems healthy";

  const lines: string[] = [];
  const critCount = issues.filter(i => i.severity === "❌").length;
  const warnCount = issues.filter(i => i.severity === "⚠️").length;
  lines.push(`${critCount} critical, ${warnCount} warnings — ${issues.length} total`);
  lines.push("");
  lines.push(`    ${"".padEnd(3)} ${"Section".padEnd(8)} Issue`);
  lines.push("    " + "─".repeat(70));
  const sorted = [...issues].sort((a, b) => (a.severity === "❌" ? 0 : 1) - (b.severity === "❌" ? 0 : 1));
  for (const i of sorted) lines.push(`    ${i.severity} ${i.section.padEnd(8)} ${i.msg}`);
  return lines.join("\n");
}
