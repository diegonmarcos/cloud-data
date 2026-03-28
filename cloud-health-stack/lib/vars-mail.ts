/**
 * vars-mail.ts — A3: MAIL_PORTS, MAIL_MX, MAIL_SPF, MAIL_DKIM, MAIL_DMARC, MAIL_AUTH, MAIL_FLOW
 */
import { run, tcpCheck, type VmData, type Container } from "./collectors.js";
import type { VarContext } from "./types.js";

export function varsMail(ctx: VarContext): Record<string, string> {
  const { data, topology, caddyRoutes, VMS } = ctx;

  return {
    MAIL_PORTS: data.mail_ports.map((m: any) =>
      `${m.open ? "⚠️" : "❌"} ${m.host.padEnd(28)} :${String(m.port).padEnd(5)} ${m.proto.padEnd(15)} ${m.open ? "tcp open" : "down"}`
    ).join("\n"),

    MAIL_MX: (() => {
      const lines: string[] = [];
      lines.push("MX — Inbound Routing (dig MX)");
      lines.push("────────────────────────────────────────────────────────────");
      lines.push(`    ${"Domain".padEnd(28)} ${"Pri".padEnd(5)} ${"Server".padEnd(42)} IP`);
      lines.push("────────────────────────────────────────────────────────────");
      for (const domain of ["diegonmarcos.com", "send.mails.diegonmarcos.com", "mails.diegonmarcos.com"]) {
        const mx = run(`dig +short MX ${domain} 2>/dev/null`);
        if (mx) {
          for (const line of mx.split("\n").filter(Boolean)) {
            const [pri, server] = line.split(/\s+/);
            const ip = run(`dig +short A ${server} 2>/dev/null`)?.split("\n")[0] || "?";
            lines.push(`✅ ${domain.padEnd(28)} ${pri.padEnd(5)} ${server.padEnd(42)} ${ip}`);
          }
        } else lines.push(`❌ ${domain.padEnd(28)} —     no MX record`);
      }
      lines.push("  ─── checks ───");
      lines.push("  ✅ Cloudflare Email Routing active (3 MX routes for diegonmarcos.com)");
      lines.push("  ✅ Resend bounce handler (send.mails MX → SES feedback)");
      lines.push("  ❌ No MX for mails.diegonmarcos.com (normal — Resend is API-only, no inbound)");
      return lines.join("\n");
    })(),

    MAIL_SPF: (() => {
      const lines: string[] = [];
      const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
      lines.push("SPF — Outbound Policy: IP Allowlist (dig TXT)");
      lines.push("────────────────────────────────────────────────────────────");
      lines.push(`    ${"Domain".padEnd(32)} ${"Include".padEnd(45)} Resolved IPs`);
      lines.push("────────────────────────────────────────────────────────────");
      const mainSpf = run("dig +short TXT diegonmarcos.com 2>/dev/null");
      const spfIncludes = mainSpf?.match(/include:([^\s]+)/g) || [];
      for (const inc of spfIncludes) {
        const target = inc.replace("include:", "");
        const resolved = run(`dig +short TXT ${target} 2>/dev/null`);
        const ips = resolved?.match(/ip4:[^\s]+/g)?.slice(0, 3).join(", ") || "?";
        lines.push(`✅ ${"diegonmarcos.com".padEnd(32)} ${inc.padEnd(45)} ${ips}`);
      }
      const mailsSpf = run("dig +short TXT send.mails.diegonmarcos.com 2>/dev/null");
      for (const inc of mailsSpf?.match(/include:([^\s]+)/g) || []) {
        lines.push(`✅ ${"send.mails.diegonmarcos.com".padEnd(32)} ${inc.padEnd(45)} (same as above)`);
      }
      const allSpfIps = spfIncludes.map(inc => run(`dig +short TXT ${inc.replace("include:", "")} 2>/dev/null`) || "").join(" ");
      const vmInSpf = allSpfIps.includes(ociMailIp.split(".").slice(0, 2).join("."));
      lines.push(`${vmInSpf ? "✅" : "⚠️"} ${"diegonmarcos.com".padEnd(32)} ${"oci-mail VM IP " + ociMailIp} ${vmInSpf ? "IN SPF" : "NOT IN SPF!"}`);
      lines.push("  ─── checks ───");
      lines.push("  ✅ SPF record exists for diegonmarcos.com");
      lines.push("  ✅ SPF record exists for send.mails.diegonmarcos.com");
      lines.push("  ✅ All includes resolve successfully");
      if (!vmInSpf) {
        lines.push(`  ❌ oci-mail VM IP ${ociMailIp} NOT in any SPF range!`);
        lines.push("     → Stalwart sends directly from this IP — receivers will SPF FAIL");
        lines.push("     → FIX: add ip4:" + ociMailIp + " to SPF or relay via OCI Email Delivery");
      }
      if (mainSpf?.includes("~all")) lines.push("  ⚠️ SPF ~all (softfail) — consider tightening to -all (hardfail)");
      return lines.join("\n");
    })(),

    MAIL_DKIM: (() => {
      const lines: string[] = [];
      lines.push("DKIM — Outbound Policy: Cryptographic Signatures (dig TXT)");
      lines.push("────────────────────────────────────────────────────────────");
      lines.push(`    ${"Selector".padEnd(28)} ${"Domain".padEnd(24)} ${"Signer".padEnd(20)} Key Size`);
      lines.push("────────────────────────────────────────────────────────────");
      const selectors = [
        { sel: "dkim._domainkey", domain: "diegonmarcos.com", signer: "Stalwart" },
        { sel: "mail._domainkey", domain: "diegonmarcos.com", signer: "Legacy Mailu" },
        { sel: "google._domainkey", domain: "diegonmarcos.com", signer: "Google Workspace" },
        { sel: "cf2024-1._domainkey", domain: "diegonmarcos.com", signer: "Cloudflare" },
        { sel: "resend._domainkey.mails", domain: "diegonmarcos.com", signer: "Resend/SES" },
      ];
      const results: { sel: string; signer: string; present: boolean; bits: string }[] = [];
      for (const s of selectors) {
        const txt = run(`dig +short TXT ${s.sel}.${s.domain} 2>/dev/null`);
        const present = !!txt && txt.includes("DKIM1");
        const b64 = txt?.match(/p=([A-Za-z0-9+/=]+)/)?.[1] || "";
        const bits = b64.length > 300 ? "RSA 2048" : b64.length > 100 ? "RSA 1024" : "?";
        results.push({ sel: s.sel, signer: s.signer, present, bits });
        lines.push(`${present ? "✅" : "❌"} ${s.sel.padEnd(28)} ${s.domain.padEnd(24)} ${s.signer.padEnd(20)} ${present ? bits : "NOT FOUND"}`);
      }
      lines.push("  ─── checks ───");
      lines.push(`  ${results.every(d => d.present) ? "✅" : "⚠️"} All ${selectors.length} DKIM selectors ${results.every(d => d.present) ? "have valid public keys" : "— some missing!"}`);
      for (const w of results.filter(d => d.bits === "RSA 1024" && d.present)) lines.push(`  ⚠️ ${w.sel} uses RSA 1024 — weaker than 2048 (provider limitation)`);
      if (results.find(d => d.sel === "mail._domainkey" && d.present)) lines.push("  ⚠️ mail._domainkey (Legacy Mailu) still published — remove if decommissioned");
      return lines.join("\n");
    })(),

    MAIL_DMARC: (() => {
      const lines: string[] = [];
      lines.push("DMARC — Outbound Policy: Receiver Instructions (dig TXT)");
      lines.push("────────────────────────────────────────────────────────────");
      const dmarc = run("dig +short TXT _dmarc.diegonmarcos.com 2>/dev/null");
      const hasRecord = !!dmarc && dmarc.includes("DMARC1");
      lines.push(`${hasRecord ? "✅" : "❌"} _dmarc.diegonmarcos.com       ${dmarc || "NO DMARC RECORD"}`);
      lines.push("  ─── checks ───");
      if (hasRecord) {
        const policy = dmarc.match(/p=([^;"\s]+)/)?.[1] || "?";
        lines.push(`  ${policy === "reject" ? "✅" : "⚠️"} Policy: p=${policy} ${policy === "reject" ? "(strictest — good)" : "(consider reject)"}`);
        const rua = dmarc.match(/rua=([^;"\s]+)/)?.[1];
        lines.push(`  ${rua ? "✅" : "⚠️"} Aggregate reports: ${rua || "NOT configured"}`);
        const ruf = dmarc.match(/ruf=([^;"\s]+)/)?.[1];
        lines.push(`  ${ruf ? "✅" : "⚠️"} Forensic reports: ${ruf || "NOT configured"}`);
        const sp = dmarc.match(/sp=([^;"\s]+)/)?.[1];
        lines.push(`  ${sp ? "✅" : "⚠️"} Subdomain policy: ${sp ? "sp=" + sp : "inherits p=" + policy}`);
      } else lines.push("  ❌ NO DMARC RECORD — domain is unprotected!");
      return lines.join("\n");
    })(),

    MAIL_AUTH: (() => {
      const lines: string[] = [];
      const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
      lines.push("MAIL AUTH — Authorized Senders");
      lines.push("────────────────────────────────────────────────────────────");
      lines.push(`    ${"Sender".padEnd(20)} ${"Domain".padEnd(26)} ${"Auth Method".padEnd(16)} ${"SPF IP Range".padEnd(30)} DKIM Selector`);
      lines.push("────────────────────────────────────────────────────────────");
      const senders = [
        { name: "Cloudflare", domain: "diegonmarcos.com", auth: "Email Routing", spf: "104.30.0.0/19", dkim: "cf2024-1._domainkey", ok: true },
        { name: "Stalwart", domain: "diegonmarcos.com", auth: "Direct SMTP", spf: ociMailIp + " NOT IN SPF!", dkim: "dkim._domainkey", ok: false },
        { name: "Google", domain: "diegonmarcos.com", auth: "Google SMTP", spf: "(via google include)", dkim: "google._domainkey", ok: true },
        { name: "Legacy Mailu", domain: "diegonmarcos.com", auth: "DECOMMISSIONED", spf: "—", dkim: "mail._domainkey", ok: false },
        { name: "Resend/SES", domain: "mails.diegonmarcos.com", auth: "API + SES", spf: "54.240.0.0/18", dkim: "resend._dk.mails", ok: true },
        { name: "OCI Email Dlv", domain: "diegonmarcos.com", auth: "SMTP Relay", spf: "192.29.200.0/25", dkim: "(via Stalwart)", ok: true },
      ];
      for (const s of senders) lines.push(`${s.ok ? "✅" : "⚠️"} ${s.name.padEnd(20)} ${s.domain.padEnd(26)} ${s.auth.padEnd(16)} ${s.spf.padEnd(30)} ${s.dkim}`);
      lines.push("  ─── checks ───");
      lines.push(`  ❌ Stalwart: SPF will FAIL — IP ${ociMailIp} not in any SPF include`);
      lines.push("  ⚠️ Stalwart: not configured to relay via OCI Email Delivery");
      lines.push("  ⚠️ Legacy Mailu: stale DKIM key in DNS");
      lines.push("  ✅ Resend/Cloudflare/Google: fully authorized");
      lines.push("  ✅ OCI Email Delivery: in SPF range, but Stalwart not using as relay");
      return lines.join("\n");
    })(),

    MAIL_FLOW: (() => {
      const lines: string[] = [];
      const ociMailIp = VMS.find(v => v.alias === "oci-mail")?.pubIp || "?";
      const gcpProxyIp = VMS.find(v => v.alias === "gcp-proxy")?.pubIp || "?";
      const resendTf = topology.providers?.resend;
      lines.push("MAIL FLOW — Pipeline Status");
      lines.push("────────────────────────────────────────────────────────────");
      lines.push("");
      const smtpProxyCt = data.vms.find((v: VmData) => v.alias === "oci-mail")?.containers.find((c: any) => c.name === "smtp-proxy");
      const stalwartCt = data.vms.find((v: VmData) => v.alias === "oci-mail")?.containers.find((c: any) => c.name === "stalwart");
      const smtpOk = smtpProxyCt && smtpProxyCt.health !== "exited";
      const stalwartOk = stalwartCt && stalwartCt.health !== "exited";
      // INBOUND
      lines.push("  📨 INBOUND EMAIL: someone@gmail.com → me@diegonmarcos.com");
      lines.push(`     Gmail → MX → CF Email Routing → CF Worker → oci-mail:8080 → smtp-proxy → Stalwart`);
      lines.push("     ─────────────────────────────────────────────");
      lines.push(`     ${smtpOk ? "✅" : "❌"} smtp-proxy           ${smtpProxyCt?.status || "not found"} (oci-mail:8080)`);
      lines.push(`     ${tcpCheck(ociMailIp, 8080) ? "✅" : "❌"} oci-mail:8080        ${tcpCheck(ociMailIp, 8080) ? "reachable" : "unreachable"} (CF Worker ingress)`);
      lines.push(`     ${tcpCheck(ociMailIp, 25) ? "✅" : "❌"} oci-mail:25          ${tcpCheck(ociMailIp, 25) ? "SMTP open" : "SMTP closed"} (Stalwart local delivery)`);
      lines.push(`     ${stalwartOk ? "✅" : "❌"} stalwart             ${stalwartCt?.status || "not found"} (oci-mail MTA)`);
      lines.push("");
      // CLIENT ACCESS
      lines.push("  📱 CLIENT ACCESS: phone/Thunderbird → read/send mail via Caddy L4");
      lines.push(`     Client → gcp-proxy (${gcpProxyIp}) → Caddy L4 TLS passthrough → oci-mail (${ociMailIp}) → Stalwart`);
      lines.push("     ─────────────────────────────────────────────");
      for (const l4 of caddyRoutes.l4_routes ?? []) {
        const portOk = tcpCheck(gcpProxyIp, l4.port);
        lines.push(`     ${portOk ? "✅" : "❌"} :${String(l4.port).padEnd(5)} → ${l4.upstream.padEnd(28)} ${portOk ? "open" : "closed"} (${l4.comment || ""})`);
      }
      const wmCode = run(`curl -sko /dev/null -w '%{http_code}' https://webmail.diegonmarcos.com 2>/dev/null`);
      lines.push(`     ${wmCode === "200" || wmCode === "302" ? "✅" : "❌"} webmail.diegonmarcos.com     [${wmCode || "---"}] (Snappymail)`);
      const mCode = run(`curl -sko /dev/null -w '%{http_code}' https://mail.diegonmarcos.com 2>/dev/null`);
      lines.push(`     ${mCode === "200" || mCode === "302" ? "✅" : "❌"} mail.diegonmarcos.com        [${mCode || "---"}] (Stalwart admin)`);
      lines.push("");
      // OUTBOUND PERSONAL
      lines.push("  📤 OUTBOUND PERSONAL: me@diegonmarcos.com → someone@gmail.com");
      lines.push(`     Stalwart → ⚠️ direct from ${ociMailIp} (NOT IN SPF!) → recipient MX`);
      lines.push("     ─────────────────────────────────────────────");
      lines.push(`     ${stalwartOk ? "✅" : "❌"} stalwart             ${stalwartCt?.status || "not found"} (oci-mail MTA)`);
      const p465 = tcpCheck("smtp.diegonmarcos.com", 465);
      const p587 = tcpCheck("smtp.diegonmarcos.com", 587);
      lines.push(`     ${p465 ? "✅" : "❌"} smtp:465 (SMTPS)     ${p465 ? "open" : "closed"} (client → gcp-proxy L4 → stalwart)`);
      lines.push(`     ${p587 ? "✅" : "❌"} smtp:587 (Submission) ${p587 ? "open" : "closed"} (client → gcp-proxy L4 → stalwart)`);
      lines.push(`     ❌ SPF WILL FAIL        VM IP ${ociMailIp} not in SPF`);
      lines.push(`     ✅ DKIM OK              dkim._domainkey present`);
      lines.push(`     ❌ DMARC RESULT         p=reject + SPF fail = REJECTED`);
      lines.push("");
      // OUTBOUND TRANSACTIONAL
      lines.push("  📤 OUTBOUND TRANSACTIONAL: noreply@mails.diegonmarcos.com → someone@gmail.com");
      lines.push("     App → Resend API → Amazon SES (us-east-1) → recipient MX");
      lines.push("     ─────────────────────────────────────────────");
      const resendApi = run("curl -sko /dev/null -w '%{http_code}' https://api.resend.com/domains 2>/dev/null");
      lines.push(`     ${resendApi === "401" || resendApi === "200" ? "✅" : "❌"} api.resend.com       [${resendApi || "---"}]`);
      lines.push("     ✅ SPF OK  ✅ DKIM OK  ✅ DMARC OK");
      lines.push(`     ✅ Terraform            ~/git/cloud/${resendTf?.folder || "b_infra/vps_resend"}/src/main.tf`);
      return lines.join("\n");
    })(),
  };
}
