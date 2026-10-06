import type { Runtime } from "@chainlink/cre-sdk";

import type { WorkflowConfig } from "../main.js";

export type ScreenedRow = {
  sym: string;
  mask: number;
  purifBps: number;
};

/**
 * TODO(T10): one Halal Terminal portfolio scan in runInNodeMode.
 * Normalise to sorted {sym, mask, purifBps}. null -> 0. Fail closed on non-200.
 * Fixture mode reads fixtures/halal-scan.json and does not spend tokens.
 * Use runtime.now() only. Do not call Date.now or new Date.
 */
export function screen(_runtime: Runtime<WorkflowConfig>): ScreenedRow[] {
  return [];
}
