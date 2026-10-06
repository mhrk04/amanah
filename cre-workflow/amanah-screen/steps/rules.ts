import type { ScreenedRow } from "./screen.js";

export type RuleEntry = {
  assetIdx: number;
  companyPassMask: number;
  flags: number;
  targetBps: number;
  purificationBps: number;
};

/**
 * TODO(T11): STRCx preferred -> wrapper_ok false, even if the company passes.
 * chain_ok = Token-2022, not paused, routable. Equal-weight target_bps over
 * AAOIFI-approved assets (sum 10000 or 0). evidence_hash = sha256 of the
 * normalised screening array. No Date.now / new Date.
 */
export function applyRules(_screened: readonly ScreenedRow[]): RuleEntry[] {
  return [];
}
