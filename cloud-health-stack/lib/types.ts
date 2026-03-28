/**
 * types.ts — Shared types for var generators
 */
import type { VmData, Container } from "./collectors.js";

export type { VmData, Container };

export interface VarContext {
  data: {
    generated: string;
    wg_peers: any[];
    api_mcp: any[];
    public_urls: any[];
    mail_ports: any[];
    private_dns: any[];
    vms: VmData[];
    databases: any[];
  };
  topology: any;
  caddyRoutes: any;
  hmData: any;
  wgPeersData: any;
  backupTargets: any;
  VMS: { alias: string; vmId: string; ip: string; user: string; cpus: number; ram: string; os: string; pubIp: string; diskGb: number; shape: string }[];
  PRIVATE_DNS: { dns: string; container: string; port: number; vm: string; hostPort: boolean }[];
  DATABASES: { service: string; type: string; container: string; db: string; vm: string; dns: string }[];
}
