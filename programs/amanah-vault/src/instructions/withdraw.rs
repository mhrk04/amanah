use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::VaultError;
use crate::state::UserVault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"vault", owner.key().as_ref()],
        bump = user_vault.bump,
        has_one = owner
    )]
    pub user_vault: Account<'info, UserVault>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user_vault,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(constraint = token_program.key() == *mint.to_account_info().owner @ VaultError::InvalidMint)]
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let _ = (&ctx, amount);
    // TODO(T06): allowed while paused. Reject when pending.is_some() (SwapPending).
    // PDA-signed transfer_checked. Map Token-2022 Pausable to AssetPaused.
    // Emit Withdrawn.
    msg!("TODO(T06): withdraw");
    Ok(())
}
