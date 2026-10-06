import type { Runtime, SolanaAccountMeta } from "@chainlink/cre-sdk";

/**
 * TODO(T09): replace with `cre generate-bindings solana` output from the IDL.
 * The generated class exposes writeReportFromComplianceReportV2.
 */
export class AmanahVaultBindings {
  constructor(
    private readonly client: unknown,
    private readonly programId: string,
  ) {}

  writeReportFromComplianceReportV2(
    _runtime: Runtime<unknown>,
    _report: unknown,
    _remainingAccounts: readonly SolanaAccountMeta[],
    _compute?: { computeLimit: number },
  ): { txStatus: number } {
    void this.client;
    void this.programId;
    throw new Error("TODO(T09): writeReportFromComplianceReportV2");
  }
}
