/**
 * config.ts — Constants, paths, logging, JSON loading, dependency checker
 */
import { execSync } from "child_process";
import { readFileSync } from "fs";
import { join } from "path";

export const HOME = process.env.HOME || "/home/diego";
export const SCRIPT_DIR = __dirname.replace(/\/lib$/, "");
export const CD = join(SCRIPT_DIR, ".."); // cloud-data/ root

// ── Logging ─────────────────────────────────────────────────────
export const LOG: string[] = [];
export const ERRORS: string[] = [];
export function log(msg: string) {
  const ts = new Date().toISOString().split("T")[1]?.split(".")[0];
  const line = `[${ts}] ${msg}`;
  LOG.push(line);
  console.log(line);
}
export function logErr(msg: string) {
  const ts = new Date().toISOString().split("T")[1]?.split(".")[0];
  const line = `[${ts}] ERROR: ${msg}`;
  LOG.push(line);
  ERRORS.push(line);
  console.error(line);
}

// ── Load cloud-data JSONs ───────────────────────────────────────
export function loadJson(name: string): any {
  const p = join(CD, name);
  try {
    const data = JSON.parse(readFileSync(p, "utf-8"));
    log(`Loaded ${name} (${Object.keys(data).length} keys)`);
    return data;
  } catch (e: any) {
    logErr(`Failed to load ${name}: ${e.message}`);
    return {};
  }
}

// ── Dependency solver ───────────────────────────────────────
const REQUIRED_DEPS = ["ssh", "curl", "nc", "dig", "git", "gh"];
export const depStatus: { name: string; path: string; ok: boolean }[] = [];
for (const dep of REQUIRED_DEPS) {
  try {
    const p = execSync(`command -v ${dep} 2>/dev/null`, { encoding: "utf-8" }).trim();
    depStatus.push({ name: dep, path: p, ok: true });
  } catch {
    depStatus.push({ name: dep, path: "", ok: false });
  }
}
const missingDeps = depStatus.filter(d => !d.ok);
if (missingDeps.length > 0) {
  console.error(`⚠️  Missing dependencies: ${missingDeps.map(d => d.name).join(", ")}`);
  console.error("   Some checks will be skipped. Install missing tools to get full results.");
}

// ── Helpers ─────────────────────────────────────────────────────
export const inferProvider = (id: string) => {
  if (id.startsWith("oci-")) return "OCI";
  if (id.startsWith("gcp-")) return "GCP";
  if (id.startsWith("aws-")) return "AWS";
  if (id.startsWith("vast-")) return "Vast.ai";
  return "?";
};
export const inferCost = (id: string) => {
  if (/-f[_\d]/.test(id)) return "Free";
  if (/-p[_\d]/.test(id)) return "Spot";
  return "?";
};
