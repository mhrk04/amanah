export {
  ASSOCIATED_TOKEN_PROGRAM,
  BPS_DENOMINATOR,
  CHAIN_SELECTORS,
  CLUSTER_CONFIG,
  CONFIG_SEED,
  DEFAULT_CLUSTER,
  FORWARDER_AUTHORITY_SEED,
  FORWARDERS,
  HALAL_TERMINAL_SCAN_URL,
  JUPITER_BUILD_URL,
  JUPITER_V6_PROGRAM,
  MAX_ASSETS,
  METHODOLOGIES,
  MOCK_ROUTE_PROGRAM_ID,
  NOWNODES_ETH_URL,
  NOWNODES_SOLANA_URL,
  PROGRAM_ID,
  PUBLIC_DEVNET_RPC_URL,
  REGISTRY_SEED,
  REPORT_VERSION,
  RISK_PROFILES,
  RPC_METHOD_ALLOWLIST,
  SPL_TOKEN_PROGRAM,
  TOKEN_2022_PROGRAM,
  UNIVERSE,
  USDC_BUFFER_BPS,
  USDC_DECIMALS,
  USDC_MINT,
  USDC_MINT_DEVNET,
  USDC_MINT_MAINNET,
  VAULT_ERRORS,
  VAULT_SEED,
  XSTOCK_DECIMALS,
  XSTOCKS_API_BASE,
  mintFor,
  resolveCluster,
} from "./constants.js";
export type {
  Cluster,
  ClusterConfig,
  MethodologyId,
  RiskProfile,
  RpcMethod,
  UniverseAsset,
  XStockSymbol,
} from "./constants.js";

export { floorRawOut } from "./floor.js";
export { SCALED_UI_AMOUNT_TYPE, effectiveMultiplierE12 } from "./multiplier.js";
export type { ScaledUiAmount } from "./multiplier.js";
export { portfolioWeights, profileFromQuizScore } from "./portfolio.js";
export type { PortfolioEntry, PortfolioPreview, PortfolioWeight } from "./portfolio.js";
export { encodeComplianceReportV2, normalizeScreening } from "./report.js";
export type { ComplianceReportV2, ReportEntryV2, ScreenedSymbol } from "./report.js";

import idl from "./idl/amanah_vault.json" with { type: "json" };

/** Placeholder until `anchor build` writes the IDL (T04). */
export const amanahVaultIdl = idl;
