import { portfolioWeights, type MethodologyId, type RiskProfile } from "@amanah/common";

export type PlannedTrade = {
  vault: string;
  outputMint: string;
  symbol: string;
  amountIn: bigint;
  minOut: bigint;
};

/**
 * TODO(T17): getMultipleAccounts via NOWNodes, then targets from portfolioWeights
 * for the vault methodology. Pick the largest underweight asset.
 * amount_in = min(gap, max_trade_usdc). min_out uses the same floor as on-chain.
 */
export function planNextTrade(_vault: string): PlannedTrade {
  void portfolioWeights;
  throw new Error("TODO(T17): planNextTrade");
}

export type VaultPlanInput = {
  methodology: MethodologyId;
  profile: RiskProfile;
};
