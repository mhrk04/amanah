use anchor_lang::prelude::*;

use crate::state::{ComplianceRegistry, Config};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SetParamsArgs {
    pub max_trade_usdc: u64,
    pub max_daily_usdc: u64,
    pub max_slippage_bps: u16,
    pub max_price_age_s: u32,
    pub max_registry_age_s: u32,
}

#[derive(Accounts)]
pub struct Admin<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = admin)]
    pub config: Account<'info, Config>,
}

/// `set_assets` reads mints from `remaining_accounts` (design §5.3, T05).
#[derive(Accounts)]
pub struct SetAssets<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = admin)]
    pub config: Account<'info, Config>,
    #[account(mut, seeds = [b"registry"], bump = registry.bump)]
    pub registry: Account<'info, ComplianceRegistry>,
}

pub fn set_assets(ctx: Context<SetAssets>) -> Result<()> {
    let _ = &ctx;
    // TODO(T05): <= 8 unique mints, each owned by Token-2022 with ScaledUiAmount.
    // Bump universe_version, clear registry entries, emit ConfigChanged.
    msg!("TODO(T05): set_assets");
    Ok(())
}

pub fn set_keeper(ctx: Context<Admin>, keeper: Pubkey) -> Result<()> {
    let _ = (&ctx, keeper);
    // TODO(T05): write config.keeper and emit ConfigChanged { field, old, new }.
    msg!("TODO(T05): set_keeper");
    Ok(())
}

pub fn set_forwarder(ctx: Context<Admin>, forwarder_program: Pubkey, forwarder_state: Pubkey) -> Result<()> {
    let _ = (&ctx, forwarder_program, forwarder_state);
    // TODO(T05): update forwarder program + state. Keystone switch is this plus set_transmitter(None).
    msg!("TODO(T05): set_forwarder");
    Ok(())
}

pub fn set_transmitter(ctx: Context<Admin>, allowed_transmitter: Option<Pubkey>) -> Result<()> {
    let _ = (&ctx, allowed_transmitter);
    // TODO(T05): Some(transmitter) while on the mock forwarder; None on Keystone.
    msg!("TODO(T05): set_transmitter");
    Ok(())
}

pub fn set_params(ctx: Context<Admin>, params: SetParamsArgs) -> Result<()> {
    let _ = (&ctx, &params);
    // TODO(T05): caps > 0, max_slippage_bps <= 1000, emit ConfigChanged.
    msg!("TODO(T05): set_params");
    Ok(())
}

pub fn set_paused(ctx: Context<Admin>, paused: bool) -> Result<()> {
    let _ = (&ctx, paused);
    // TODO(T05): pause blocks deposit and pre_swap. Withdraw stays open (T06).
    msg!("TODO(T05): set_paused");
    Ok(())
}
