/**
 * vars-infra.ts — B: VPS_SPECS, RESOURCES_HEADER, RESOURCES_TABLE, STORAGE
 */
import { run, type VmData } from "./collectors.js";
import { HOME, inferProvider, inferCost } from "./config.js";
import type { VarContext } from "./types.js";

export function varsInfra(ctx: VarContext): Record<string, string> {
  const { data, topology, hmData, backupTargets, VMS, DATABASES } = ctx;

  return {
    VPS_SPECS: (() => {
      const specs: { name: string; provider: string; shape: string; cpu: string; ram: string; disk: string; cost: string }[] = [];
      for (const [, vm] of Object.entries(hmData.vms ?? {}) as [string, any][]) {
        const vmId = vm.vm_id || "";
        specs.push({ name: vm.ssh_alias || vmId, provider: inferProvider(vmId), shape: vm.specs?.shape || vm.specs?.machine_type || "?", cpu: String(vm.specs?.cpu || "?"), ram: `${vm.specs?.ram_gb || "?"}G`, disk: `${vm.specs?.disk_gb || "?"}G`, cost: inferCost(vmId) });
      }
      const ghaRepos = ["cloud", "cloud-data", "front", "unix", "tools"];
      for (const repo of ghaRepos) {
        specs.push({ name: `gha-${repo}`, provider: "GitHub", shape: "ubuntu-latest (x86)", cpu: "4", ram: "16G", disk: "14G", cost: "2000min/mo" });
      }
      return specs.map(v => `   ${v.name.padEnd(16)} ${v.provider.padEnd(10)} ${v.shape.padEnd(20)} ${v.cpu.padEnd(6)} ${v.ram.padEnd(6)} ${v.disk.padEnd(8)} ${v.cost}`).join("\n");
    })(),

    RESOURCES_HEADER: (() => {
      const vmNames = data.vms.map((v: VmData) => v.alias);
      return `${"".padEnd(18)} ${vmNames.map((n: string) => n.padEnd(14)).join(" ")}`;
    })(),

    RESOURCES_TABLE: (() => {
      const lines: string[] = [];
      const fields: [string, (v: VmData) => string][] = [
        ["OS", v => v.os], ["CPU", v => `${v.cpus} cores`], ["RAM", v => `${v.mem_used}/${v.mem_total}`],
        ["RAM %", v => `${v.mem_pct}%`], ["Swap", v => v.swap], ["Disk", v => `${v.disk_used}/${v.disk_total}`],
        ["Disk %", v => v.disk_pct], ["Load", v => v.load], ["Containers", v => `${v.containers_running}/${v.containers_total}`],
        ["Uptime", v => v.uptime.replace("up ", "")],
      ];
      for (const [label, fn] of fields) {
        const vals = data.vms.map((v: VmData) => fn(v).padEnd(14));
        lines.push(`${label.padEnd(18)} ${vals.join(" ")}`);
      }
      return lines.join("\n");
    })(),

    STORAGE: (() => {
      const ghUser = run("gh api user --jq .login 2>/dev/null") || "diegonmarcos";
      const lines: string[] = [];
      lines.push("  OBJECT STORAGE");
      lines.push(`    ${"Provider".padEnd(14)} ${"Type".padEnd(20)} Details`);
      lines.push("    " + "─".repeat(60));
      const storage = topology.storage ?? [];
      for (const s of storage) lines.push(`    ${(s.provider || "OCI").padEnd(14)} ${"Object Storage".padEnd(20)} ${s.name || "bucket"} (${s.tier || "standard"})`);
      if (storage.length === 0) lines.push(`    ${"OCI".padEnd(14)} ${"Object Storage".padEnd(20)} (configured in terraform)`);
      const ghcrCount = run("gh api '/user/packages?package_type=container&per_page=100' --jq 'length' 2>/dev/null") || "?";
      lines.push(`    ${"GitHub".padEnd(14)} ${"Container Registry".padEnd(20)} ghcr.io/${ghUser}/ (${ghcrCount} images)`);
      lines.push("");
      lines.push("  DATA / FILES (git repositories)");
      lines.push(`    ${"Repo".padEnd(14)} ${"Path".padEnd(40)} Purpose`);
      lines.push("    " + "─".repeat(65));
      for (const r of [
        { name: "cloud", path: "~/git/cloud", purpose: "Services, infra, HM, workflows" },
        { name: "cloud-data", path: "~/git/cloud/cloud-data", purpose: "Generated config, topology, manifests" },
        { name: "front", path: "~/git/front", purpose: "32 front-end projects" },
        { name: "unix", path: "~/git/unix", purpose: "NixOS host, HM desktop/termux" },
        { name: "tools", path: "~/git/tools", purpose: "CLI tools, scripts" },
        { name: "vault", path: "~/git/vault", purpose: "Credentials, keys, 2FA, IDs" },
      ]) lines.push(`    ${r.name.padEnd(14)} ${r.path.padEnd(40)} ${r.purpose}`);
      lines.push("");
      lines.push("  DOCKER VOLUMES (persistent, named)");
      lines.push(`    ${"VM".padEnd(16)} ${"Volume".padEnd(30)} Service`);
      lines.push("    " + "─".repeat(60));
      const byVm = backupTargets.by_vm ?? {};
      for (const [vm, info] of Object.entries(byVm).sort() as [string, any][]) {
        for (const vol of info.volumes ?? []) {
          const svc = (backupTargets.targets ?? []).find((t: any) => t.vm_alias === vm && (t.volumes ?? []).includes(vol));
          lines.push(`    ${vm.padEnd(16)} ${vol.padEnd(30)} ${svc?.service || "?"}`);
        }
      }
      lines.push("");
      lines.push("  DATABASES");
      const dbTypes = new Map<string, number>();
      for (const d of DATABASES) dbTypes.set(d.type, (dbTypes.get(d.type) || 0) + 1);
      lines.push(`    Total: ${DATABASES.length} — ${[...dbTypes.entries()].map(([t, c]) => `${c} ${t}`).join(", ")}`);
      lines.push(`    ${"Service".padEnd(20)} ${"Type".padEnd(10)} ${"Container".padEnd(22)} ${"DB Name".padEnd(14)} VM`);
      lines.push("    " + "─".repeat(75));
      for (const d of DATABASES) lines.push(`    ${d.service.padEnd(20)} ${d.type.padEnd(10)} ${d.container.padEnd(22)} ${d.db.padEnd(14)} ${d.vm}`);
      return lines.join("\n");
    })(),
  };
}
