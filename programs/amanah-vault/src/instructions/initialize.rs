use anchor_lang::prelude::*;

use crate::state::{ComplianceRegistry, Config};

/// Arguments for `initialize`. Caps must be > 0 and `max_slippage_bps` <= 1000 (T05).
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeParams {
    pub keeper: Pubkey,
    pub forwarder_program: Pubkey,
    pub forwarder_state: Pubkey,
    pub allowed_transmitter: Option<Pubkey>,
    pub usdc_mint: Pubkey,
    pub max_trade_usdc: u64,
    pub max_daily_usdc: u64,
    pub max_slippage_bps: u16,
    pub max_price_age_s: u32,
    pub max_registry_age_s: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = 8 + ComplianceRegistry::INIT_SPACE,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, ComplianceRegistry>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let _ = (&ctx, &params);
    // TODO(T05): keeper != admin; caps > 0; max_slippage_bps <= 1000.
    // TODO(T05, D5): optionally require admin == program upgrade authority.
    // Write both PDAs, store bumps, emit nothing until the first config change.
    msg!("TODO(T05): initialize");
    Ok(())
}
