use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::errors::VaultError;
use crate::state::{Config, UserVault};

#[derive(Accounts)]
pub struct OpenVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = owner,
        space = 8 + UserVault::INIT_SPACE,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub user_vault: Account<'info, UserVault>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_vault,
        associated_token::token_program = token_program
    )]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(constraint = usdc_mint.key() == config.usdc_mint @ VaultError::InvalidMint)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetProfile<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = user_vault.bump,
        has_one = owner
    )]
    pub user_vault: Account<'info, UserVault>,
}

pub fn open_vault(ctx: Context<OpenVault>, risk_profile: u8, methodology: u8) -> Result<()> {
    let _ = (&ctx, risk_profile, methodology);
    // TODO(T06): risk_profile <= 2, methodology <= 4, else InvalidParams.
    // Write owner, profile, methodology, bump. Emit VaultOpened.
    msg!("TODO(T06): open_vault");
    Ok(())
}

pub fn set_profile(ctx: Context<SetProfile>, risk_profile: u8, methodology: u8) -> Result<()> {
    let _ = (&ctx, risk_profile, methodology);
    // TODO(T06): reject when pending.is_some() (SwapPending). Same bounds as open_vault.
    msg!("TODO(T06): set_profile");
    Ok(())
}
