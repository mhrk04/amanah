/**
 * Oracle floor shared by `post_swap` and the keeper (design §5.5 step 4, §7.5).
 * USDC 6 dp to USD e8 is ×10^2. xStock raw amount is 8 dp.
 * raw_expected = amount_in × 10^2 × 10^8 × 10^12 / (price_usd_e8 × multiplier_e12)
 * floor = raw_expected × (10000 − slippage_bps) / 10000, rounding down.
 *
 * TODO(T01, T14): confirm Pyth prices one UI token. If it prices the raw token, drop multiplier_e12.
 */
export function floorRawOut(_params: {
  amountIn: bigint;
  priceUsdE8: bigint;
  multiplierE12: bigint;
  maxSlippageBps: number;
}): bigint {
  throw new Error("TODO(T14): floorRawOut");
}
