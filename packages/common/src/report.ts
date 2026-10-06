import { MAX_ASSETS, REPORT_VERSION } from "./constants.js";

/** Borsh report v2. About 153 bytes for 6 assets. Payload budget about 263 bytes (design §5.2). */
export type ReportEntryV2 = {
  assetIdx: number;
  companyPassMask: number;
  flags: number;
  targetBps: number;
  purificationBps: number;
  priceUsdE8: bigint;
};

export type ComplianceReportV2 = {
  version: typeof REPORT_VERSION;
  reportId: bigint;
  asOf: bigint;
  priceTs: bigint;
  universeVersion: number;
  evidenceHash: Uint8Array;
  entries: ReportEntryV2[];
};

export type ScreenedSymbol = {
  sym: string;
  mask: number;
  purifBps: number;
};

/**
 * TODO(T11): Borsh-encode ComplianceReportV2 and assert length <= 263.
 * `approved_mask` is not in the report; the program computes it.
 */
export function encodeComplianceReportV2(_report: ComplianceReportV2): Uint8Array {
  void MAX_ASSETS;
  throw new Error("TODO(T11): encodeComplianceReportV2");
}

/** TODO(T10): null verdict -> 0. Sort by symbol before consensus. */
export function normalizeScreening(_rows: readonly unknown[]): ScreenedSymbol[] {
  throw new Error("TODO(T10): normalizeScreening");
}
