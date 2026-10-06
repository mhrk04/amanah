import { solanaAccountMeta, type Runtime, type SolanaAccountMeta } from "@chainlink/cre-sdk";

import type { WorkflowConfig } from "../main.js";
import type { RuleEntry } from "./rules.js";

/**
 * TODO(T11, T12): Borsh-encode ComplianceReportV2 (<= 263 bytes) and, on a
 * real run, call the generated writeReportFromComplianceReportV2.
 * remainingAccounts: state, authority, registry (writable), config, instructions sysvar.
 * computeLimit 300_000. Throw unless txStatus is SUCCESS (2).
 * Devnet write first. Mainnet --broadcast is a human, once, for the proof tx.
 */
export function buildRemainingAccounts(runtime: Runtime<WorkflowConfig>): SolanaAccountMeta[] {
  return [solanaAccountMeta(runtime.config.forwarderState, false)];
}

export function submitReport(
  _runtime: Runtime<WorkflowConfig>,
  _entries: readonly RuleEntry[],
): { written: false } {
  return { written: false };
}
