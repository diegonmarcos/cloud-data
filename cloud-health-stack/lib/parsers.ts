/**
 * parsers.ts — Parse cloud-data JSONs into structured data
 */

export function parseVms(hmData: any) {
  return Object.entries(hmData.vms ?? {}).map(([id, vm]: [string, any]) => ({
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
  })).filter(v => v.ip);
}

export function parsePublicUrls(caddyRoutes: any): { url: string; upstream: string }[] {
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

export function parseMcpApiEndpoints(caddyRoutes: any): { url: string; upstream: string; name: string }[] {
  const eps: { url: string; upstream: string; name: string }[] = [];
  for (const mr of caddyRoutes.mcp_routes ?? []) {
    for (const ep of mr.endpoints ?? []) {
      eps.push({ url: `${mr.parent_domain}${ep.base_path}/mcp`, upstream: ep.upstream, name: ep.base_path.replace(/^\//, "") });
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

export function parseMailPorts(caddyRoutes: any): { host: string; port: number; proto: string }[] {
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
  ports.push({ host: "mails.diegonmarcos.com", port: 25, proto: "MX (Resend/SES)" });
  ports.push({ host: "send.mails.diegonmarcos.com", port: 25, proto: "SPF (Resend/SES)" });
  return ports;
}

export function parsePrivateDns(topology: any, hmData: any): { dns: string; container: string; port: number; vm: string }[] {
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

export function parseDatabases(
  backupTargets: any,
  privateDns: { dns: string; container: string; port: number; vm: string }[]
): { service: string; type: string; container: string; db: string; vm: string; dns: string }[] {
  const dbs: { service: string; type: string; container: string; db: string; vm: string; dns: string }[] = [];
  for (const t of backupTargets.targets ?? []) {
    for (const d of t.databases ?? []) {
      const dnsEntry = privateDns.find(p => p.container === d.container || p.dns.startsWith(d.container));
      const dns = dnsEntry ? `${dnsEntry.dns}:${dnsEntry.port}` : d.path || "embedded";
      dbs.push({ service: d.service || t.service, type: d.type, container: d.container, db: d.db || d.path || "custom", vm: t.vm_alias || t.vm, dns });
    }
  }
  return dbs.sort((a, b) => a.vm.localeCompare(b.vm) || a.service.localeCompare(b.service));
}
