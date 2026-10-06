import { NextResponse } from "next/server";

export async function POST() {
  // TODO(T41): optional LLM explanation. Must say weights are rule-based. Never changes weights.
  return NextResponse.json(
    { error: "explain not implemented", label: "AI explanation; weights are rule-based" },
    { status: 501 },
  );
}
