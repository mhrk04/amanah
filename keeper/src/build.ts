export type BuiltSandwich = {
  serialized: Uint8Array;
  version: 0 | 1;
  computeUnitLimit: number;
};

/**
 * TODO(T39): assemble [CU limit, CU price] + setup + pre_swap + route + cleanup + post_swap.
 *
 * `useMockRoute` (localnet and devnet): the route instruction is `mock-route`,
 * not Jupiter. xStock routes do not exist on devnet.
 * `useJupiter` (mainnet only): GET /swap/v2/build with taker = keeper and
 * destinationTokenAccount = vault ATA. v0 + address lookup tables. If the
 * serialized size is over 1232 bytes, step maxAccounts down (64, 40, 30, 20),
 * then transactionVersion=1. Simulate and set CU × 1.2, cap 1.4M.
 */
export function buildGuardedSwap(_trade: {
  outputMint: string;
  amountIn: bigint;
  useMockRoute: boolean;
}): Promise<BuiltSandwich> {
  throw new Error("TODO(T39): buildGuardedSwap");
}
