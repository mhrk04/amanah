import { NextResponse } from "next/server";

export async function GET() {
  // TODO(T40): reuse scripts/purification-check.ts. Mainnet multiplier history only.
  return NextResponse.json(
    { error: "purification not implemented", note: "estimate; confirm with your scholar" },
    { status: 501 },
  );
}
