import { address } from "@solana/kit";

import { buildGuardedSwap } from "./build.js";
import { planNextTrade } from "./plan.js";
import { sendTransaction } from "./send.js";

/**
 * TODO(T17, T39, T18, T19): `keeper plan <vault>`, `--target <SYM>`, `--skip-preflight`.
 * The keeper key has no withdraw rights. Do not commit the keypair.
 */
export async function main(argv: readonly string[]): Promise<void> {
  void planNextTrade;
  void buildGuardedSwap;
  void sendTransaction;
  const vault = argv.find((arg) => !arg.startsWith("-"));
  if (vault) {
    address(vault);
  }
  console.log("amanah-keeper skeleton. Trade planning is TODO(T17). Args:", argv.join(" "));
}

const isDirect = process.argv[1]?.includes("cli.ts");
if (isDirect) {
  main(process.argv.slice(2)).catch((err: unknown) => {
    console.error(err);
    process.exitCode = 1;
  });
}
