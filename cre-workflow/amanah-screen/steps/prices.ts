import type { Runtime } from "@chainlink/cre-sdk";

import type { WorkflowConfig } from "../main.js";

export type PriceRow = {
  sym: string;
  priceUsdE8: bigint;
  source: "pyth" | "xstocks" | "fixture";
};

/**
 * TODO(T25): mainnet uses Pyth Hermes (Bearer) for the six xStock feeds.
 * Devnet sets usePyth false in cluster config; return a fixture price so the
 * report shape still encodes. Zero a price when conf/price > 2%.
 * price_ts = min publish_time, from runtime.now() only as the clock source
 * when a feed time is missing. Do not call Date.now or new Date.
 */
export function prices(runtime: Runtime<WorkflowConfig>): PriceRow[] {
  void runtime.config.cluster;
  return [];
}
