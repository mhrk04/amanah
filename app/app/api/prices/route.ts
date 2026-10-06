import { CLUSTER_CONFIG, DEFAULT_CLUSTER, resolveCluster } from "@amanah/common";
import { NextResponse } from "next/server";

export async function GET() {
  const cluster = resolveCluster(process.env.CLUSTER ?? DEFAULT_CLUSTER);
  // TODO(T22): mainnet prices from Pyth or the xStocks quote. Devnet returns fixtures.
  if (!CLUSTER_CONFIG[cluster].usePyth) {
    return NextResponse.json({ cluster, prices: [], note: "TODO: devnet fixture prices" }, { status: 501 });
  }
  return NextResponse.json({ cluster, prices: [], note: "TODO(T22): Pyth or xStocks quote" }, { status: 501 });
}
