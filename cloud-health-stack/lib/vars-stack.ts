/**
 * vars-stack.ts — D: FRAMEWORK_PATHS
 */
import type { VarContext } from "./types.js";

export function varsStack(ctx: VarContext): Record<string, string> {
  const { VMS } = ctx;

  return {
    FRAMEWORK_PATHS: (() => {
      const FRAMEWORK = [
        ["BUILD ENGINES", ""],
        ["  Service engine", "cloud/a_solutions/_engine.sh"],
        ["  HM engine", "cloud/b_infra/home-manager/_engine.sh"],
        ["  Workflow engine", "cloud/workflows/build.sh"],
        ["  Front engine", "front/1.ops/build_main.sh"],
        ["  NixOS host", "unix/aa_nixos-surface_host/build.sh"],
        ["  HM desktop", "unix/ba_flakes_desktop/build.sh"],
        ["", ""],
        ["HOME-MANAGER FLAKES", ""],
        ["  Shared modules", "cloud/b_infra/home-manager/_shared/modules/"],
        ...VMS.map(v => [`  ${v.alias}`, `cloud/b_infra/home-manager/${v.alias}/src/`]),
        ["", ""],
        ["GHA WORKFLOWS", ""],
        ...VMS.map(v => [`  ship-${v.alias}`, `cloud/.github/workflows/ship-${v.alias}.yml`]),
        ["  ship-home-manager", "cloud/.github/workflows/ship-home-manager.yml"],
        ["  ship-ghcr", "cloud/.github/workflows/ship-ghcr.yml"],
        ["  Templates", "cloud/workflows/src/"],
        ["", ""],
        ["DAGU WORKFLOWS", ""],
        ["  DAGs source", "cloud/a_solutions/bc-obs_dagu/src/dags/"],
        ["  deploy-pull-up", "cloud/a_solutions/bc-obs_dagu/src/dags/ops_deploy-pull-up.yaml"],
        ["  cloud-data sync", "cloud/a_solutions/bc-obs_dagu/src/dags/sync_cloud-data.yaml"],
        ["", ""],
        ["DATA", ""],
        ["  cloud-data", "cloud/cloud-data/"],
        ["  Container manifests", "cloud/cloud-data/cloud-data-containers-{vm}.json"],
        ["  Topology", "cloud/cloud-data/cloud-data-topology.json"],
        ["  GHA config", "cloud/cloud-data/cloud-data-gha-config.json"],
        ["  Consolidated", "cloud/cloud-data/_cloud-data-consolidated.json"],
        ["", ""],
        ["TERRAFORM", ""],
        ["  OCI", "cloud/b_infra/vps_oci/src/main.tf"],
        ["  GCP", "cloud/b_infra/vps_gcloud/src/main.tf"],
        ["  Cloudflare", "cloud/a_solutions/ba-clo_cloudflare/src/main.tf"],
        ["", ""],
        ["SECURITY", ""],
        ["  Vault", "vault/"],
        ["  SOPS secrets", "cloud/a_solutions/*/src/secrets.yaml"],
        ["  SSH keys", "vault/A0_keys/ssh/"],
      ];
      return FRAMEWORK.map(([label, path]) => {
        if (!label && !path) return "";
        if (!path) return `  ${label}`;
        return `  ${label.padEnd(22)} ~/git/${path}`;
      }).join("\n");
    })(),
  };
}
