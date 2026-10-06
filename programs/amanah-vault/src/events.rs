use anchor_lang::prelude::*;

use crate::state::{ComplianceReportV2, RegistryEntry};

/// Emitted by `on_report` (T07). `entries` are mints resolved from `Config.assets`.
#[event]
pub struct RegistryUpdated {
    pub report_id: u64,
    pub as_of: i64,
    pub price_ts: i64,
    pub universe_version: u16,
    pub evidence_hash: [u8; 32],
    pub entries: Vec<RegistryEntry>,
    pub metadata_len: u32,
    pub transmitter: Pubkey,
}

/// Not emitted on the report path. Present so Anchor keeps `ComplianceReportV2`
/// in the IDL for `writeReportFromComplianceReportV2` (design §5.2, T04, T09).
#[event]
pub struct ComplianceReportV2Schema {
    pub report: ComplianceReportV2,
}

/// Emitted by `post_swap` (T14).
#[event]
pub struct SwapGuarded {
    pub vault: Pubkey,
    pub out_mint: Pubkey,
    pub amount_in: u64,
    pub out: u64,
    pub floor: u64,
    pub price_usd_e8: u64,
    pub multiplier_e12: u128,
    pub methodology: u8,
}

#[event]
pub struct VaultOpened {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub risk_profile: u8,
    pub methodology: u8,
}

#[event]
pub struct Deposited {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Withdrawn {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

/// `field` is a short name (`keeper`, `paused`, `assets`, ...). `old` and `new`
/// are the previous and next values, encoded by the setter (T05).
#[event]
pub struct ConfigChanged {
    pub field: String,
    pub old: Vec<u8>,
    pub new: Vec<u8>,
}
