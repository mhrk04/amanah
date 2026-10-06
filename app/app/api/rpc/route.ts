import { CLUSTER_CONFIG, DEFAULT_CLUSTER, RPC_METHOD_ALLOWLIST, resolveCluster } from "@amanah/common";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const body = (await request.json().catch(() => null)) as { method?: unknown } | null;
  const method = body?.method;
  const allowList: readonly string[] = RPC_METHOD_ALLOWLIST;
  if (typeof method !== "string" || !allowList.includes(method)) {
    return NextResponse.json({ error: "method not allow-listed" }, { status: 403 });
  }

  const cluster = resolveCluster(process.env.CLUSTER ?? DEFAULT_CLUSTER);
  // TODO(T20): forward the JSON-RPC body to CLUSTER_CONFIG[cluster].rpcUrlEnv.
  // Mainnet adds the NOWNodes api-key header. Never send the key to the browser.
  void CLUSTER_CONFIG[cluster];
  return NextResponse.json({ error: "RPC proxy not implemented", cluster }, { status: 501 });
}
