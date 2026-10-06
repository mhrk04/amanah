use anchor_lang::prelude::*;

/// `Config` PDA seed: `[b"config"]`. Design §5.1. INIT_SPACE 528; on-chain size 536.
pub const CONFIG_SEED: &[u8] = b"config";
/// `ComplianceRegistry` PDA seed: `[b"registry"]`. INIT_SPACE 444; on-chain size 452.
pub const REGISTRY_SEED: &[u8] = b"registry";
/// `UserVault` PDA seeds: `[b"vault", owner]`. INIT_SPACE 118; on-chain size 126.
pub const VAULT_SEED: &[u8] = b"vault";

pub const MAX_ASSETS: usize = 8;

/// Methodology bit order, frozen with report v2 (design §4).
/// bit0 AAOIFI · bit1 DJIM · bit2 FTSE · bit3 MSCI · bit4 S&P.
pub const METHODOLOGY_AAOIFI: u8 = 0;
pub const METHODOLOGY_DJIM: u8 = 1;
pub const METHODOLOGY_FTSE: u8 = 2;
pub const METHODOLOGY_MSCI: u8 = 3;
pub const METHODOLOGY_SP: u8 = 4;

pub const FLAG_WRAPPER_OK: u8 = 1 << 0;
pub const FLAG_CHAIN_OK: u8 = 1 << 1;

/// Admin-owned global config. Design §5.1.
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub keeper: Pubkey,
    pub forwarder_program: Pubkey,
    pub forwarder_state: Pubkey,
    pub allowed_transmitter: Option<Pubkey>,
    pub usdc_mint: Pubkey,
    pub assets: [Pubkey; MAX_ASSETS],
    pub asset_count: u8,
    pub universe_version: u16,
    pub max_trade_usdc: u64,
    pub max_daily_usdc: u64,
    pub daily_spent_usdc: u64,
    pub day_index: i64,
    pub max_slippage_bps: u16,
    pub max_price_age_s: u32,
    pub max_registry_age_s: u32,
    pub paused: bool,
    pub bump: u8,
    pub reserved: [u8; 32],
}

/// Latest CRE report, written only by `on_report`. Design §5.1.
#[account]
#[derive(InitSpace)]
pub struct ComplianceRegistry {
    pub report_id: u64,
    pub as_of: i64,
    pub price_ts: i64,
    pub updated_slot: u64,
    pub universe_version: u16,
    pub evidence_hash: [u8; 32],
    pub entry_count: u8,
    pub entries: [RegistryEntry; MAX_ASSETS],
    pub bump: u8,
}

/// One universe slot. 47 bytes. `approved_mask` is computed on-chain (T07).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Default)]
pub struct RegistryEntry {
    pub mint: Pubkey,
    pub company_pass_mask: u8,
    pub flags: u8,
    pub approved_mask: u8,
    pub target_bps: u16,
    pub purification_bps: u16,
    pub price_usd_e8: u64,
}

/// Per-user vault. Design §5.1.
#[account]
#[derive(InitSpace)]
pub struct UserVault {
    pub owner: Pubkey,
    pub risk_profile: u8,
    pub methodology: u8,
    pub pending: Option<PendingSwap>,
    pub bump: u8,
    pub reserved: [u8; 16],
}

/// In-transaction snapshot taken by `pre_swap` and cleared by `post_swap`. 66 bytes.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct PendingSwap {
    pub out_mint: Pubkey,
    pub amount_in: u64,
    pub min_out: u64,
    pub out_before: u64,
    pub keeper_usdc_before: u64,
    pub asset_idx: u8,
    pub pre_ix_index: u8,
}

/// Compact report v2. Borsh, about 153 bytes for 6 assets. Design §5.2.
/// Referenced by [`crate::events::ComplianceReportV2Schema`] so it lands in the IDL (T04, T09).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ComplianceReportV2 {
    pub version: u8,
    pub report_id: u64,
    pub as_of: i64,
    pub price_ts: i64,
    pub universe_version: u16,
    pub evidence_hash: [u8; 32],
    #[max_len(MAX_ASSETS)]
    pub entries: Vec<ReportEntryV2>,
}

/// One report row. 15 bytes. `asset_idx` resolves the mint from `Config.assets`.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ReportEntryV2 {
    pub asset_idx: u8,
    pub company_pass_mask: u8,
    pub flags: u8,
    pub target_bps: u16,
    pub purification_bps: u16,
    pub price_usd_e8: u64,
}

pub const REPORT_VERSION: u8 = 2;

// Design §5.1 sizes include the 8-byte discriminator. INIT_SPACE does not.
const _: () = assert!(Config::INIT_SPACE == 528);
const _: () = assert!(ComplianceRegistry::INIT_SPACE == 444);
const _: () = assert!(RegistryEntry::INIT_SPACE == 47);
const _: () = assert!(UserVault::INIT_SPACE == 118);
const _: () = assert!(PendingSwap::INIT_SPACE == 66);
