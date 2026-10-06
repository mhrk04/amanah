use anchor_lang::prelude::*;

/// Frozen order from design §5.3. Anchor custom errors start at 6000.
/// `MintNotApproved` = 6014 (0x177e). Print the IDL value into the README in T27.
#[error_code]
pub enum VaultError {
    #[msg("Unauthorized")]
    Unauthorized, // 6000
    #[msg("Invalid parameters")]
    InvalidParams, // 6001
    #[msg("Vault buys are paused")]
    Paused, // 6002
    #[msg("Invalid mint")]
    InvalidMint, // 6003
    #[msg("Mint is not Token-2022")]
    NotToken2022, // 6004
    #[msg("A swap is already pending")]
    SwapPending, // 6005
    #[msg("No pending swap")]
    NoPendingSwap, // 6006
    #[msg("Invalid forwarder state")]
    InvalidForwarderState, // 6007
    #[msg("Invalid forwarder authority")]
    InvalidForwarderAuthority, // 6008
    #[msg("Unknown transmitter")]
    UnknownTransmitter, // 6009
    #[msg("Invalid report")]
    InvalidReport, // 6010
    #[msg("Stale report")]
    StaleReport, // 6011
    #[msg("Report timestamp is in the future")]
    FutureReport, // 6012
    #[msg("Universe version mismatch")]
    UniverseMismatch, // 6013
    #[msg("Mint is not approved under the vault methodology")]
    MintNotApproved, // 6014
    #[msg("Registry is stale")]
    RegistryStale, // 6015
    #[msg("Price missing")]
    PriceMissing, // 6016
    #[msg("Price is stale")]
    PriceStale, // 6017
    #[msg("Trade exceeds the per-trade cap")]
    TradeTooLarge, // 6018
    #[msg("Daily cap exceeded")]
    DailyCapExceeded, // 6019
    #[msg("Matching post_swap not found")]
    MissingPostSwap, // 6020
    #[msg("Route program is not allow-listed")]
    RouteNotAllowed, // 6021
    #[msg("Instruction is not top-level")]
    NotTopLevel, // 6022
    #[msg("Output is below the oracle floor")]
    SlippageExceeded, // 6023
    #[msg("Keeper retained USDC")]
    KeeperKeptFunds, // 6024
    #[msg("Token-2022 mint is paused")]
    AssetPaused, // 6025
    #[msg("Math overflow")]
    MathOverflow, // 6026
}
