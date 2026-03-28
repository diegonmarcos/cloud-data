/**
 * vars-health.ts — A1: WG_PEERS, PUBLIC_URLS, API_MCP_ENDPOINTS, REPOS_REGISTRIES
 */
import { run, tcpCheck, type VmData } from "./collectors.js";
import type { VarContext } from "./types.js";

export function varsHealth(ctx: VarContext): Record<string, string> {
  const { data, wgPeersData, VMS } = ctx;

  return {
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
          const isClient = !isVm && !isHub;
          const peerType = isHub ? "HUB" : isVm ? "VM" : "CLIENT";
          const live = data.wg_peers.find((p: any) => p.privIp === wgIp || p.name === name);
          const handshake = live?.handshake || "no data";
          const alive = live?.alive ?? false;

          let vpsOk = false, pubOk = false, wgOk = false;
          if (isClient) {
            vpsOk = true; pubOk = true; wgOk = alive;
          } else {
            const vmData = data.vms.find((v: VmData) => v.alias === name);
            vpsOk = vmData?.reachable ?? false;
            pubOk = pubIp !== "?" && pubIp !== "dynamic" && run(`nc -zw3 ${pubIp} 22 2>&1 && echo OK`).includes("OK");
            wgOk = alive;
          }
          const allOk = vpsOk && pubOk && wgOk;
          const overallIcon = allOk ? "✅" : (vpsOk || pubOk || wgOk) ? "⚠️" : "❌";
          lines.push(`${overallIcon} ${name.padEnd(18)} ${vpsOk ? "✅" : "❌"}  ${pubOk ? "✅" : "❌"}  ${wgOk ? "✅" : "❌"}  ${pubIp.padEnd(18)} ${wgIp.padEnd(14)} ${peerType.padEnd(8)} ${handshake}`);
        }
      } else if (data.wg_peers.length) {
        for (const p of data.wg_peers) {
          lines.push(`${p.alive ? "✅" : "❌"} ${p.name.padEnd(18)} ?    ?    ${p.alive ? "✅" : "❌"}  ${p.pubIp.padEnd(18)} ${p.privIp.padEnd(14)} ${"?".padEnd(8)} ${p.handshake}`);
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

    REPOS_REGISTRIES: (() => {
      const ghUser = run("gh api user --jq .login 2>/dev/null") || "?";
      const lines: string[] = [];
      lines.push("  GIT REPOS (github.com)");
      lines.push(`    ${"Repo".padEnd(14)} ${"URL".padEnd(48)} Status`);
      lines.push("    " + "─".repeat(70));
      const REPOS = [
        { name: "cloud", path: "/home/diego/Mounts/Git/cloud" },
        { name: "cloud-data", path: "/home/diego/Mounts/Git/cloud/cloud-data" },
        { name: "front", path: "/home/diego/Mounts/Git/front" },
        { name: "unix", path: "/home/diego/Mounts/Git/unix" },
        { name: "tools", path: "/home/diego/Mounts/Git/tools" },
        { name: "vault", path: "/home/diego/Mounts/Git/vault" },
      ];
      for (const r of REPOS) {
        const url = `github.com/${ghUser}/${r.name}`;
        const code = run(`curl -sko /dev/null -w '%{http_code}' https://${url} 2>/dev/null`);
        const ok = code === "200" || code === "301" || code === "302";
        const branch = run(`git -C ${r.path} branch --show-current 2>/dev/null`) || "?";
        const dirty = run(`git -C ${r.path} status --porcelain 2>/dev/null`);
        const dirtyTag = dirty ? " ⚠️dirty" : "";
        lines.push(`    ${ok ? "✅" : "❌"} ${r.name.padEnd(12)} ${url.padEnd(46)} [${code}] ${branch}${dirtyTag}`);
      }
      lines.push("");
      lines.push("  CONTAINER REGISTRY (ghcr.io)");
      lines.push(`    ${"Image".padEnd(40)} Status`);
      lines.push("    " + "─".repeat(50));
      const pkgsJson = run("gh api '/user/packages?package_type=container&per_page=100' 2>/dev/null");
      let ghcrTotal = 0;
      if (pkgsJson) {
        try {
          const pkgs = JSON.parse(pkgsJson) as { name: string; visibility: string; repository?: { name: string } }[];
          ghcrTotal = pkgs.length;
          const byRepo: Record<string, number> = {};
          for (const p of pkgs) { const repo = p.repository?.name || "no-repo"; byRepo[repo] = (byRepo[repo] || 0) + 1; }
          for (const [repo, count] of Object.entries(byRepo).sort((a, b) => b[1] - a[1])) {
            const samplePkg = pkgs.find(p => p.repository?.name === repo);
            lines.push(`    ✅ ${(`ghcr.io/${ghUser}/${samplePkg?.name || "?"}`).padEnd(38)} ${count} images (${repo})`);
          }
        } catch { lines.push("    ❌ (parse error)"); }
      } else { lines.push("    ❌ (gh api unavailable)"); }
      lines.push(`    📦 Total: ${ghcrTotal} container images`);
      return lines.join("\n");
    })(),
  };
}
