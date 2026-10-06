use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::VaultError;
use crate::state::{Config, UserVault};

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub owner: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"vault", owner.key().as_ref()],
        bump = user_vault.bump,
        has_one = owner
    )]
    pub user_vault: Account<'info, UserVault>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_vault,
        associated_token::token_program = token_program
    )]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        constraint = usdc_mint.key() == config.usdc_mint @ VaultError::InvalidMint,
        constraint = token_program.key() == *usdc_mint.to_account_info().owner @ VaultError::InvalidMint
    )]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let _ = (&ctx, amount);
    // TODO(T06): reject when config.paused (Paused) and when amount == 0 (InvalidParams).
    // transfer_checked from owner_usdc_ata to vault_usdc_ata. Emit Deposited.
    msg!("TODO(T06): deposit");
    Ok(())
}
