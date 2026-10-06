/** ScaledUiAmount extension type (design §5.6). */
export const SCALED_UI_AMOUNT_TYPE = 25;

export type ScaledUiAmount = {
  multiplier: number;
  newMultiplier: number;
  newMultiplierEffectiveTimestamp: number;
};

/**
 * TODO(T14, T17): parse Token-2022 TLV and pick the effective multiplier by timestamp.
 * m_e12 = round(m * 1e12). Reject non-finite or <= 0.
 */
export function effectiveMultiplierE12(_mintData: Uint8Array, _nowUnixSeconds: number): bigint {
  throw new Error("TODO(T14): effectiveMultiplierE12");
}
