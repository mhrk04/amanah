import { CLUSTER_CONFIG, UNIVERSE, mintFor } from "@amanah/common";

/**
 * Devnet bootstrap. Do not point this at mainnet.
 *
 * TODO: create Token-2022 test mints with the ScaledUiAmount extension where
 * the cluster supports it, standing in for AAPLx, NVDAx, MSTRx, and STRCx
 * (plus GOOGLx and MSFTx so the universe stays six assets).
 * TODO: fund a test vault with devnet USDC (or a 6-decimal test mint).
 * TODO: write registry entries for those mints so the swap guard can run.
 * The route in that guard is `programs/mock-route`, not Jupiter.
 * Replace the TODO_DEVNET_* placeholders in packages/common afterwards.
 */
export async function devnetSetup(): Promise<void> {
  const devnet = CLUSTER_CONFIG.devnet;
  if (devnet.useJupiter || !devnet.useMockRoute) {
    throw new Error("devnet-setup: devnet must use mock-route, not Jupiter");
  }
  const mints = UNIVERSE.map((asset) => ({
    symbol: asset.symbol,
    mint: mintFor("devnet", asset),
  }));
  void mints;
  throw new Error("TODO: devnet-setup");
}
