/** Names only. This script prints which keys are missing, never their values. TODO(T03). */

const REQUIRED = [
  "CLUSTER",
  "SOLANA_RPC_URL_DEVNET",
  "SOLANA_RPC_URL_MAINNET",
  "NOWNODES_API_KEY",
  "JUPITER_API_KEY",
  "HALAL_TERMINAL_API_KEY",
  "PYTH_API_KEY",
  "PROGRAM_ID",
  "KEEPER_KEYPAIR",
  "ADMIN_KEYPAIR",
  "CRE_ETH_PRIVATE_KEY",
  "CRE_SOLANA_PRIVATE_KEY",
] as const;

export function missingEnv(env: NodeJS.ProcessEnv = process.env): string[] {
  return REQUIRED.filter((key) => !env[key]);
}

const missing = missingEnv();
if (missing.length === 0) {
  console.log("check-env: all required names are set");
} else {
  console.log("check-env: missing (names only):");
  for (const key of missing) {
    console.log(`  ${key}`);
  }
}
