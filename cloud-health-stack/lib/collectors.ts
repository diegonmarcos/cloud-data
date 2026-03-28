/**
 * collectors.ts — Shell commands, SSH, TCP checks, VM data collection
 * Supports both sync and async parallel execution with SSH mutex
 */
import { execSync, exec } from "child_process";
import { existsSync } from "fs";
import { join } from "path";
import { HOME, log, logErr } from "./config.js";

// Clear stale SSH mux sockets at startup
try {
  const socketDir = join(HOME, ".ssh", "sockets");
  if (existsSync(socketDir)) {
    execSync(`find ${socketDir} -type s -mmin +5 -delete 2>/dev/null || true`, { timeout: 3000 });
  }
} catch {}

// ── Sync execution ─────────────────────────────────────────
export function run(cmd: string, timeout = 10000): string {
  try { return execSync(cmd, { timeout, encoding: "utf-8", stdio: ["pipe", "pipe", "pipe"] }).trim(); }
  catch (e: any) {
    const stderr = e.stderr?.toString()?.trim();
    if (stderr && !stderr.includes("Connection timed out") && !stderr.includes("Connection refused")) {
      logErr(`run failed: ${cmd.substring(0, 80)}... → ${stderr.substring(0, 200)}`);
    }
    return "";
  }
}

// ── Async execution ────────────────────────────────────────
export function runAsync(cmd: string, timeout = 10000): Promise<string> {
  return new Promise((resolve) => {
    const child = exec(cmd, { timeout, encoding: "utf-8" }, (err, stdout) => {
      resolve(stdout?.trim() || "");
    });
  });
}

// ── SSH ────────────────────────────────────────────────────
export function sshCmd(vm: string, cmd: string): string {
  const b64 = Buffer.from(cmd).toString("base64");
  const result = run(`ssh -o ConnectTimeout=8 -o ControlPath=none -o BatchMode=yes ${vm} "echo ${b64} | base64 -d | sh"`, 20000);
  if (!result && cmd === "echo OK") {
    logErr(`SSH unreachable: ${vm}`);
  }
  return result;
}

// ── TCP checks (sync + async) ──────────────────────────────
export let tcpLogVerbose = true;
export function setTcpLogVerbose(v: boolean) { tcpLogVerbose = v; }

export function tcpCheck(host: string, port: number): boolean {
  const ok = run(`nc -zw3 ${host} ${port} 2>&1 && echo OK`).includes("OK");
  if (!ok && tcpLogVerbose) log(`tcp-check FAIL: ${host}:${port}`);
  return ok;
}

export function tcpCheckAsync(host: string, port: number): Promise<boolean> {
  return runAsync(`nc -zw3 ${host} ${port} 2>&1 && echo OK`, 5000).then(r => r.includes("OK"));
}

/**
 * Parallel port scan — check multiple ports on one host concurrently
 * Returns array of open port numbers
 */
export async function tcpScanParallel(host: string, ports: number[]): Promise<number[]> {
  const results = await Promise.all(
    ports.map(async (port) => ({ port, open: await tcpCheckAsync(host, port) }))
  );
  return results.filter(r => r.open).map(r => r.port);
}

/**
 * Parallel URL checks — curl multiple URLs concurrently
 * Batched to avoid overwhelming the system (max concurrency)
 */
export async function curlCheckParallel(
  urls: { url: string; upstream: string }[],
  maxConcurrency = 10
): Promise<{ url: string; upstream: string; http_code: string; up: boolean }[]> {
  const results: { url: string; upstream: string; http_code: string; up: boolean }[] = [];
  for (let i = 0; i < urls.length; i += maxConcurrency) {
    const batch = urls.slice(i, i + maxConcurrency);
    const batchResults = await Promise.all(
      batch.map(async (u) => {
        const code = await runAsync(`curl -sko /dev/null -w '%{http_code}' https://${u.url} 2>/dev/null`);
        return { ...u, http_code: code, up: code !== "" && code !== "000" && code !== "502" };
      })
    );
    results.push(...batchResults);
  }
  return results;
}

// ── Types ─────────────────────────────────────────────────────
export interface Container { name: string; status: string; health: string; icon: string; }
export interface VmData {
  alias: string; os: string; cpus: number; ram: string;
  mem_used: string; mem_total: string; mem_pct: number; swap: string;
  disk_used: string; disk_total: string; disk_pct: string;
  load: string; uptime: string;
  containers_running: number; containers_total: number;
  containers: Container[]; reachable: boolean;
}

export function collectVm(vm: { alias: string; os: string; cpus: number; ram: string }): VmData {
  const d: VmData = {
    alias: vm.alias, os: vm.os, cpus: vm.cpus, ram: vm.ram,
    mem_used: "?", mem_total: "?", mem_pct: 0, swap: "?",
    disk_used: "?", disk_total: "?", disk_pct: "?",
    load: "?", uptime: "?",
    containers_running: 0, containers_total: 0, containers: [], reachable: false,
  };

  if (sshCmd(vm.alias, "echo OK") !== "OK") return d;
  d.reachable = true;

  const mem = sshCmd(vm.alias, 'free -m | awk \'/Mem/{printf "%d %d %d", $3, $2, $3*100/$2}\'');
  if (mem) { const p = mem.split(" "); d.mem_used = p[0]+"M"; d.mem_total = p[1]+"M"; d.mem_pct = parseInt(p[2]||"0"); }

  d.swap = sshCmd(vm.alias, 'free -m | awk \'/Swap/{printf "%dM/%dM", $3, $2}\'') || "?";

  const disk = sshCmd(vm.alias, 'df -h / | awk \'NR==2{printf "%s %s %s", $3, $2, $5}\'');
  if (disk) { const p = disk.split(" "); d.disk_used = p[0]; d.disk_total = p[1]; d.disk_pct = p[2]; }

  d.load = sshCmd(vm.alias, 'cut -d" " -f1-3 /proc/loadavg') || "?";
  d.uptime = sshCmd(vm.alias, 'uptime -p 2>/dev/null || awk \'{printf "up %dd %dh", $1/86400, ($1%86400)/3600}\' /proc/uptime') || "?";

  const raw = sshCmd(vm.alias, 'docker ps -a --format "{{.Names}}|{{.Status}}" 2>/dev/null');
  for (const line of (raw || "").split("\n").filter(Boolean)) {
    const [name, status] = line.split("|");
    let health = "none", icon = "✅";
    if (status?.includes("(healthy)")) health = "healthy";
    else if (status?.includes("(unhealthy)")) { health = "unhealthy"; icon = "❌"; }
    else if (status?.includes("health: starting")) { health = "starting"; icon = "⚠️"; }
    else if (status?.startsWith("Created")) { health = "created"; icon = "❌"; }
    else if (status?.startsWith("Exited")) { health = "exited"; icon = "❌"; }
    d.containers.push({ name, status: status || "", health, icon });
    d.containers_total++;
    if (status?.startsWith("Up")) d.containers_running++;
  }
  d.containers.sort((a, b) => {
    const o: Record<string, number> = { missing: 0, created: 1, exited: 2, unhealthy: 3, starting: 4, none: 5, healthy: 6 };
    return (o[a.health] ?? 9) - (o[b.health] ?? 9);
  });
  return d;
}

// ── Timer ─────────────────────────────────────────────────────
export const timers: { name: string; ms: number }[] = [];
export function timed<T>(name: string, fn: () => T): T {
  const start = Date.now();
  const result = fn();
  timers.push({ name, ms: Date.now() - start });
  return result;
}
export async function timedAsync<T>(name: string, fn: () => Promise<T>): Promise<T> {
  const start = Date.now();
  const result = await fn();
  timers.push({ name, ms: Date.now() - start });
  return result;
}
