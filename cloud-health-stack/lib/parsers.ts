/**
 * parsers.ts — Parse ALL data from _cloud-data-consolidated.json
 * Single source of truth — one file, zero redundancy
 */

export interface VmInfo {
  alias: string; vmId: string; ip: string; user: string;
  cpus: number; ram: string; os: string; pubIp: string;
  diskGb: number; shape: string;
}

export interface ParsedData {
  vms: VmInfo[];
  publicUrls: { url: string; upstream: string }[];
  mcpEndpoints: { url: string; upstream: string; name: string }[];
  mailPorts: { host: string; port: number; proto: string }[];
  privateDns: { dns: string; container: string; port: number; vm: string }[];
  databases: { service: string; type: string; container: string; db: string; vm: string; dns: string }[];
  wgPeers: { vm_id: string; name: string; wg_ip: string; public_ip: string; user: string; role?: string }[];
  backupTargets: any;
  caddyRoutes: any;
  hmData: any;
  topology: any;
  consolidated: any;
}

/**
 * Parse everything from one consolidated JSON
 */
export function parseConsolidated(c: any): ParsedData {
  const vmIdToAlias: Record<string, string> = {};

  // ── VMs ────────────────────────────────────────────────
  const vms: VmInfo[] = Object.entries(c.vms ?? {}).map(([id, vm]: [string, any]) => {
    vmIdToAlias[id] = vm.ssh_alias || id;
    return {
      alias: vm.ssh_alias || id,
      vmId: id,
      ip: vm.wg_ip || "",
      user: vm.user || "ubuntu",
      cpus: vm.specs?.cpu || 0,
      ram: `${vm.specs?.ram_gb || "?"}G`,
      os: vm.os || vm.ssh_alias || id,
      pubIp: vm.ip || "",
      diskGb: vm.specs?.disk_gb || 0,
      shape: vm.specs?.shape || vm.specs?.machine_type || "?",
    };
  }).filter(v => v.ip);

  // ── Services → URLs, DNS, MCP ──────────────────────────
  const publicUrls: { url: string; upstream: string }[] = [];
  const privateDns: { dns: string; container: string; port: number; vm: string }[] = [];
  const mcpEndpoints: { url: string; upstream: string; name: string }[] = [];
  const seenUrls = new Set<string>();
  const addUrl = (url: string, upstream: string) => { if (!seenUrls.has(url)) { seenUrls.add(url); publicUrls.push({ url, upstream }); } };

  for (const [, svc] of Object.entries(c.services ?? {}) as [string, any][]) {
    const vmAlias = vmIdToAlias[svc.vm] || svc.vm || "?";
    if (svc.domain) addUrl(svc.domain, svc.upstream || `${svc.dns}:${svc.port}` || "?");
    for (const [, ct] of Object.entries(svc.containers ?? {}) as [string, any][]) {
      if (ct.dns && ct.port) privateDns.push({ dns: ct.dns, container: ct.container_name || "?", port: ct.port, vm: vmAlias });
    }
  }

  // ── Caddy config (routes, L4, GH pages, MCP, special) ──
  const caddy = c.configs?.caddy ?? {};
  for (const r of caddy.routes ?? []) { if (r.domain) addUrl(r.domain, r.upstream || "?"); }
  for (const pr of caddy.path_routes ?? []) {
    if (pr.parent_domain) addUrl(pr.parent_domain, "path-based");
    for (const r of pr.routes ?? []) {
      if (r.path && r.upstream) mcpEndpoints.push({ url: `${pr.parent_domain}${r.path}`, upstream: r.upstream, name: r.path.replace(/^\//, "") });
    }
  }
  for (const mr of caddy.mcp_routes ?? []) {
    if (mr.parent_domain) addUrl(mr.parent_domain, "MCP hub");
    for (const ep of mr.endpoints ?? []) {
      mcpEndpoints.push({ url: `${mr.parent_domain}${ep.base_path}/mcp`, upstream: ep.upstream, name: ep.base_path.replace(/^\//, "") });
    }
  }
  const special = caddy.special;
  if (special && typeof special === "object") {
    for (const s of (Array.isArray(special) ? special : Object.values(special)) as any[]) {
      if (s?.domain) addUrl(s.domain, s.upstream || s.comment || "special");
    }
  }
  for (const gp of caddy.github_pages_proxies ?? []) {
    for (const d of (gp.domain as string || "").split(",").map((s: string) => s.trim()).filter(Boolean)) {
      addUrl(d, `github-pages:${gp.github_path}`);
    }
  }

  privateDns.sort((a, b) => a.vm.localeCompare(b.vm) || a.dns.localeCompare(b.dns));

  // ── Mail ports from L4 ────────────────────────────────
  const mailPorts: { host: string; port: number; proto: string }[] = [];
  const HOSTS = ["mail.diegonmarcos.com", "smtp.diegonmarcos.com", "imap.diegonmarcos.com"];
  const PROTO: Record<number, string> = { 25: "SMTP", 465: "SMTPS", 587: "Submission", 993: "IMAPS", 4190: "ManageSieve" };
  for (const l4 of caddy.l4_routes ?? []) {
    const port = l4.listen_port || l4.port || (l4.upstream ? parseInt(l4.upstream.split(":").pop()) : 0);
    if (port && PROTO[port]) {
      for (const h of HOSTS) {
        if (h.startsWith("smtp") && ![25, 465, 587].includes(port)) continue;
        if (h.startsWith("imap") && port !== 993) continue;
        mailPorts.push({ host: h, port, proto: PROTO[port] });
      }
    }
  }
  if (mailPorts.length === 0) {
    for (const h of HOSTS) {
      if (h.startsWith("mail")) for (const p of [25, 465, 587, 993, 4190]) mailPorts.push({ host: h, port: p, proto: PROTO[p] });
      if (h.startsWith("smtp")) for (const p of [25, 465, 587]) mailPorts.push({ host: h, port: p, proto: PROTO[p] });
      if (h.startsWith("imap")) mailPorts.push({ host: h, port: 993, proto: "IMAPS" });
    }
  }
  mailPorts.push({ host: "mails.diegonmarcos.com", port: 25, proto: "MX (Resend/SES)" });
  mailPorts.push({ host: "send.mails.diegonmarcos.com", port: 25, proto: "SPF (Resend/SES)" });

  // ── Databases from containers with db_user/db_name + image detection ──
  const databases: { service: string; type: string; container: string; db: string; vm: string; dns: string }[] = [];
  const backupByVm: Record<string, { volumes: string[] }> = {};
  for (const [svcName, svc] of Object.entries(c.services ?? {}) as [string, any][]) {
    const vmAlias = vmIdToAlias[svc.vm] || svc.vm || "?";
    // Extract DB containers
    for (const [, ct] of Object.entries(svc.containers ?? {}) as [string, any][]) {
      if (ct.db_user || ct.db_name || ct.db_path) {
        // Detect type from image name
        const img = (ct.image || "").toLowerCase();
        let type = "?";
        if (img.includes("postgres")) type = "postgres";
        else if (img.includes("mariadb") || img.includes("mysql")) type = "mariadb";
        else if (img.includes("redis")) type = "redis";
        else if (img.includes("surrealdb")) type = "surrealdb";
        else if (ct.db_path?.includes("sqlite")) type = "sqlite";
        else if (img.includes("sqlite")) type = "sqlite";
        const dnsEntry = privateDns.find(p => p.container === ct.container_name);
        databases.push({
          service: svcName, type, container: ct.container_name || "?",
          db: ct.db_name || ct.db_path || "custom", vm: vmAlias,
          dns: dnsEntry ? `${dnsEntry.dns}:${dnsEntry.port}` : ct.db_path || "embedded",
        });
      }
    }
    // Extract backup volumes
    const bk = svc.backup;
    if (bk?.volumes?.length) {
      if (!backupByVm[vmAlias]) backupByVm[vmAlias] = { volumes: [] };
      for (const vol of bk.volumes) backupByVm[vmAlias].volumes.push(vol);
    }
  }
  databases.sort((a, b) => a.vm.localeCompare(b.vm) || a.service.localeCompare(b.service));

  // ── WG peers ──────────────────────────────────────────
  const wg = c.native?.wireguard ?? {};
  const wgPeers: any[] = [];
  for (const peer of wg.peers ?? []) {
    const vm = Object.entries(c.vms ?? {}).find(([, v]: [string, any]) => v.ssh_alias === peer.name);
    wgPeers.push({ vm_id: vm?.[0] || "", name: peer.name, wg_ip: peer.wg_ip, public_ip: peer.endpoint?.replace(/:.*$/, "") ?? "", user: vm ? (vm[1] as any).user : "" });
  }
  for (const [name, client] of Object.entries(wg.clients ?? {}) as [string, any][]) {
    wgPeers.push({ vm_id: "", name, wg_ip: client.wg_ip, public_ip: "dynamic", user: "", role: "client" });
  }

  return {
    vms, publicUrls, mcpEndpoints, mailPorts, privateDns, databases, wgPeers,
    backupTargets: { by_vm: backupByVm },
    caddyRoutes: caddy,
    hmData: { vms: c.vms, owner: c.owner, home_manager: c.home_manager },
    topology: { services: c.services, providers: c.providers, storage: c.storage, firewalls: c.firewalls },
    consolidated: c,
  };
}
