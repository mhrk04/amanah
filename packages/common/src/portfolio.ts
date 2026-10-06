import { BPS_DENOMINATOR, USDC_BUFFER_BPS, type MethodologyId, type RiskProfile } from "./constants.js";

export type PortfolioEntry = {
  mint: string;
  approvedMask: number;
  priceUsdE8: bigint;
  targetBps?: number;
};

export type PortfolioWeight = {
  mint: string;
  weightBps: number;
};

export type PortfolioPreview = {
  profile: RiskProfile;
  methodology: MethodologyId;
  bufferBps: number;
  weights: PortfolioWeight[];
};

/**
 * TODO(T17, T21): rules-based weights (design §7.1).
 * profile = quiz score <= 4 conservative, <= 8 balanced, else growth.
 * buffer = USDC_BUFFER_BPS[profile].
 * eligible = entries approved for the methodology bit with price > 0.
 * weight_i = floor((10000 - buffer) / eligible.length); remainder stays in the USDC buffer.
 * An LLM must not change these weights (T41 is explanation only).
 */
export function portfolioWeights(
  _profile: RiskProfile,
  _methodology: MethodologyId,
  _entries: readonly PortfolioEntry[],
): PortfolioPreview {
  void USDC_BUFFER_BPS;
  void BPS_DENOMINATOR;
  throw new Error("TODO(T17): portfolioWeights");
}

/** TODO(T21): 5 questions scored 0-2. */
export function profileFromQuizScore(_score: number): RiskProfile {
  throw new Error("TODO(T21): profileFromQuizScore");
}
