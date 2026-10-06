/**
 * Shared constants. Mainnet mints and Pyth ids are from design §4.
 * Devnet mints are created by `scripts/devnet-setup.ts` (Token-2022 stand-ins).
 */

export const PROGRAM_ID = "AP9K6Z7bpxhKTPf733sDtvMJyXDfK6ThWwC1xjekEseF";
export const MOCK_ROUTE_PROGRAM_ID = "GsMGHSe7XUcnw4Xe7bQvpvavdkae3nTorgwPY2RPNvyX";

export const USDC_MINT_MAINNET = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
/** TODO(devnet-setup): devnet USDC or a test SPL mint with 6 decimals. */
export const USDC_MINT_DEVNET = "TODO_DEVNET_USDC_MINT";

export const USDC_DECIMALS = 6;
export const XSTOCK_DECIMALS = 8;

export const SPL_TOKEN_PROGRAM = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
export const TOKEN_2022_PROGRAM = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
export const ASSOCIATED_TOKEN_PROGRAM = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
export const JUPITER_V6_PROGRAM = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";

/** Methodology bit order, frozen with report v2. A null verdict is 0 (fail closed). */
export const METHODOLOGIES = [
  { id: "aaoifi", bit: 0, label: "AAOIFI SS21" },
  { id: "djim", bit: 1, label: "DJIM" },
  { id: "ftse", bit: 2, label: "FTSE" },
  { id: "msci", bit: 3, label: "MSCI" },
  { id: "sp", bit: 4, label: "S&P" },
] as const;

export type MethodologyId = (typeof METHODOLOGIES)[number]["id"];

export const RISK_PROFILES = ["conservative", "balanced", "growth"] as const;
export type RiskProfile = (typeof RISK_PROFILES)[number];

/** USDC buffer in bps by risk profile (design §7.1). */
export const USDC_BUFFER_BPS: Record<RiskProfile, number> = {
  conservative: 4000,
  balanced: 2000,
  growth: 500,
};

export const MAX_ASSETS = 8;
export const REPORT_VERSION = 2;
export const BPS_DENOMINATOR = 10_000;

/**
 * Design §5.3 frozen Anchor error order. Confirm from the IDL after T04
 * and print into the README in T27. MintNotApproved = 6014.
 */
export const VAULT_ERRORS = {
  Unauthorized: 6000,
  InvalidParams: 6001,
  Paused: 6002,
  InvalidMint: 6003,
  NotToken2022: 6004,
  SwapPending: 6005,
  NoPendingSwap: 6006,
  InvalidForwarderState: 6007,
  InvalidForwarderAuthority: 6008,
  UnknownTransmitter: 6009,
  InvalidReport: 6010,
  StaleReport: 6011,
  FutureReport: 6012,
  UniverseMismatch: 6013,
  MintNotApproved: 6014,
  RegistryStale: 6015,
  PriceMissing: 6016,
  PriceStale: 6017,
  TradeTooLarge: 6018,
  DailyCapExceeded: 6019,
  MissingPostSwap: 6020,
  RouteNotAllowed: 6021,
  NotTopLevel: 6022,
  SlippageExceeded: 6023,
  KeeperKeptFunds: 6024,
  AssetPaused: 6025,
  MathOverflow: 6026,
} as const;

export const CHAIN_SELECTORS = {
  "solana-mainnet": "124615329519749607",
  "solana-devnet": "16423721717087811551",
} as const;

export const FORWARDERS = {
  mock: {
    programId: "7kuEAA3mSC1Tz8gQjnvH7bKFda9xSPRRin9SZbH49cNK",
    mainnetState: "jhCjuD4Z3V7HeSUChMRpkRwpw6B9yC63mxDMv8SdLNX",
    devnetState: "5Tipz3yhTBdVsDbaBxZkrp7Gjf3brGq5SKkxReefPMP7",
  },
  keystone: {
    mainnetProgramId: "GFrSSvQXaVGkc6Nrr8y2msie6pivJkR1s2EnDk4et294",
    mainnetState: "9FgdPyU28bGMCuJyD34pzw9W7Ys36ziLbT9cbZtsaraV",
    devnetProgramId: "CXsKEJcs25TQEYU2e5jZ8QTPE3ffMLZhH6BWHrdcCCB5",
    devnetState: "8QoomCQyPSkJ8WopJbX9B4HyvrFzziwvJdU8hZE6DCr9",
  },
} as const;

export type XStockSymbol = "NVDAx" | "AAPLx" | "GOOGLx" | "MSFTx" | "MSTRx" | "STRCx";

export type Cluster = "localnet" | "devnet" | "mainnet";

export type UniverseAsset = {
  symbol: XStockSymbol;
  company: string;
  /** `preferred` fails the wrapper layer for every methodology (STRCx). */
  wrapper: "common" | "preferred";
  /** Mainnet Pyth 24/7 feed. Unused on devnet (`usePyth` is false). */
  pythFeedId: string;
  /** Real xStock mint. Design §4. */
  mainnetMint: string;
  /**
   * Devnet/localnet stand-in. Token-2022 mint with ScaledUiAmount, created by
   * `scripts/devnet-setup.ts`. Not an xStock address.
   */
  devnetMint: string;
};

/**
 * Universe order is the on-chain `asset_idx` order (design §4).
 * Devnet values are placeholders until devnet-setup writes the real test mints.
 * AAPLx, NVDAx, MSTRx, and STRCx are the demo stand-ins; GOOGLx and MSFTx stay
 * in the universe so the six-asset report shape matches the design.
 */
export const UNIVERSE: readonly UniverseAsset[] = [
  {
    symbol: "NVDAx",
    company: "NVDA",
    wrapper: "common",
    pythFeedId: "4244d07890e4610f46bbde67de8f43a4bf8b569eebe904f136b469f148503b7f",
    mainnetMint: "Xsc9qvGR1efVDFGLrVsmkzv3qi45LTBjeUKSPmx9qEh",
    devnetMint: "TODO_DEVNET_NVDAX_MINT",
  },
  {
    symbol: "AAPLx",
    company: "AAPL",
    wrapper: "common",
    pythFeedId: "978e6cc68a119ce066aa830017318563a9ed04ec3a0a6439010fc11296a58675",
    mainnetMint: "XsbEhLAtcf6HdfpFZ5xEMdqW8nfAvcsP5bdudRLJzJp",
    devnetMint: "TODO_DEVNET_AAPLX_MINT",
  },
  {
    symbol: "GOOGLx",
    company: "GOOGL",
    wrapper: "common",
    pythFeedId: "b911b0329028cd0283e4259c33809d62942bd2716a58084e5f31d64c00b5424e",
    mainnetMint: "XsCPL9dNWBMvFtTmwcCA5v3xWPSMEBCszbQdiLLq6aN",
    devnetMint: "TODO_DEVNET_GOOGLX_MINT",
  },
  {
    symbol: "MSFTx",
    company: "MSFT",
    wrapper: "common",
    pythFeedId: "bb723a70af731ab56b9a650eb7e8ac22b7bc07ea77f8670bd1fa9a37bf6df3f5",
    mainnetMint: "XspzcW1PRtgf6Wj92HCiZdjzKCyFekVD8P5Ueh3dRMX",
    devnetMint: "TODO_DEVNET_MSFTX_MINT",
  },
  {
    symbol: "MSTRx",
    company: "MSTR",
    wrapper: "common",
    pythFeedId: "53f95ba4e23ed15ea56083e2ee9a5eec48055d6f59033d4bb95f1ca2a2349c28",
    mainnetMint: "XsP7xzNPvEHS1m6qfanPUGjNmdnmsLKEoNAnHjdxxyZ",
    devnetMint: "TODO_DEVNET_MSTRX_MINT",
  },
  {
    symbol: "STRCx",
    company: "MSTR",
    wrapper: "preferred",
    pythFeedId: "5f5193ed44dd1f4b0732dbffa2e27caf953e16a2d911854c8c3766d8bc609a47",
    mainnetMint: "Xs78JED6PFZxWc2wCEPspZW9kL3Se5J7L5TChKgsidH",
    devnetMint: "TODO_DEVNET_STRCX_MINT",
  },
];

export type ClusterConfig = {
  cluster: Cluster;
  /** Env var that holds the RPC URL. Values stay out of git. */
  rpcUrlEnv: string;
  /**
   * Mainnet uses NOWNodes `https://sol.nownodes.io` plus the `api-key` header.
   * Devnet has no NOWNodes host (design §0); use the public devnet RPC.
   * Localnet is `http://127.0.0.1:8899` or LiteSVM (no RPC).
   */
  rpcNote: string;
  usdcMint: string;
  /** Jupiter `/swap/v2/build` exists for these xStocks only on mainnet. */
  useJupiter: boolean;
  /** Pyth xStock feeds are mainnet. Devnet floor tests use fixtures. */
  usePyth: boolean;
  /** Sandwich `programs/mock-route` instead of Jupiter. */
  useMockRoute: boolean;
};

export const CLUSTER_CONFIG: Record<Cluster, ClusterConfig> = {
  localnet: {
    cluster: "localnet",
    rpcUrlEnv: "SOLANA_RPC_URL_LOCALNET",
    rpcNote: "LiteSVM in tests, or a local validator. No NOWNodes.",
    usdcMint: USDC_MINT_DEVNET,
    useJupiter: false,
    usePyth: false,
    useMockRoute: true,
  },
  devnet: {
    cluster: "devnet",
    rpcUrlEnv: "SOLANA_RPC_URL_DEVNET",
    rpcNote: "Public devnet RPC. NOWNodes has no devnet host.",
    usdcMint: USDC_MINT_DEVNET,
    useJupiter: false,
    usePyth: false,
    useMockRoute: true,
  },
  mainnet: {
    cluster: "mainnet",
    rpcUrlEnv: "SOLANA_RPC_URL_MAINNET",
    rpcNote: "NOWNodes https://sol.nownodes.io with the api-key header (NOWNODES_API_KEY).",
    usdcMint: USDC_MINT_MAINNET,
    useJupiter: true,
    usePyth: true,
    useMockRoute: false,
  },
};

/** Development default. The single mainnet deploy is for the demo and proof txs. */
export const DEFAULT_CLUSTER: Cluster = "devnet";

export function resolveCluster(value: string | undefined): Cluster {
  if (value === "mainnet" || value === "devnet" || value === "localnet") {
    return value;
  }
  return DEFAULT_CLUSTER;
}

export function mintFor(cluster: Cluster, asset: UniverseAsset): string {
  return cluster === "mainnet" ? asset.mainnetMint : asset.devnetMint;
}

/** @deprecated Prefer USDC_MINT_MAINNET or clusterConfig.usdcMint. Kept so older imports still parse. */
export const USDC_MINT = USDC_MINT_MAINNET;

/** App `/api/rpc` allow-list (design §7.4, T20). */
export const RPC_METHOD_ALLOWLIST = [
  "getAccountInfo",
  "getMultipleAccounts",
  "getTokenAccountsByOwner",
  "getLatestBlockhash",
  "sendTransaction",
  "getSignatureStatuses",
  "getTransaction",
  "simulateTransaction",
] as const;

export type RpcMethod = (typeof RPC_METHOD_ALLOWLIST)[number];

export const CONFIG_SEED = "config";
export const REGISTRY_SEED = "registry";
export const VAULT_SEED = "vault";
export const FORWARDER_AUTHORITY_SEED = "forwarder";

export const HALAL_TERMINAL_SCAN_URL = "https://api.halalterminal.com/api/portfolio/scan";
export const XSTOCKS_API_BASE = "https://api.xstocks.fi/api/v2/public";
export const JUPITER_BUILD_URL = "https://api.jup.ag/swap/v2/build";
export const NOWNODES_SOLANA_URL = "https://sol.nownodes.io";
export const NOWNODES_ETH_URL = "https://eth.nownodes.io";
export const PUBLIC_DEVNET_RPC_URL = "https://api.devnet.solana.com";
