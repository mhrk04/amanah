import type { Runtime } from "@chainlink/cre-sdk";

import type { WorkflowConfig } from "../main.js";

export type ChainSnapshot = {
  universeVersion: number;
  lastReportId: bigint;
  paused: boolean[];
};

/**
 * TODO(T25): HTTP JSON-RPC getMultipleAccounts to the cluster RPC
 * (devnet public RPC, mainnet NOWNodes). Solana reads in CRE go over HTTP;
 * the Solana capability is the write path. Commitment finalized.
 */
export function chainState(_runtime: Runtime<WorkflowConfig>): ChainSnapshot {
  return { universeVersion: 0, lastReportId: 0n, paused: [] };
}
