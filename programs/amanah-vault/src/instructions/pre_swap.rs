use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::VaultError;
use crate::introspection::{INSTRUCTIONS_SYSVAR, TOKEN_2022};
use crate::state::{ComplianceRegistry, Config, UserVault};

/// Shared account list for `pre_swap` and `post_swap` (design §5.5).
#[derive(Accounts)]
pub struct GuardedSwap<'info> {
    pub keeper: Signer<'info>,
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = keeper)]
    pub config: Account<'info, Config>,
    #[account(seeds = [b"registry"], bump = registry.bump)]
    pub registry: Account<'info, ComplianceRegistry>,
    #[account(
        mut,
        seeds = [b"vault", user_vault.owner.as_ref()],
        bump = user_vault.bump
    )]
    pub user_vault: Account<'info, UserVault>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_vault,
        associated_token::token_program = usdc_token_program
    )]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = keeper,
        associated_token::token_program = usdc_token_program
    )]
    pub keeper_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        constraint = usdc_mint.key() == config.usdc_mint @ VaultError::InvalidMint,
        constraint = usdc_token_program.key() == *usdc_mint.to_account_info().owner @ VaultError::InvalidMint
    )]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        constraint = out_token_program.key() == *out_mint.to_account_info().owner @ VaultError::InvalidMint,
        constraint = out_token_program.key() == TOKEN_2022 @ VaultError::NotToken2022
    )]
    pub out_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = out_mint,
        associated_token::authority = user_vault,
        associated_token::token_program = out_token_program
    )]
    pub vault_out_ata: InterfaceAccount<'info, TokenAccount>,
    pub usdc_token_program: Interface<'info, TokenInterface>,
    pub out_token_program: Interface<'info, TokenInterface>,
    /// CHECK: address-constrained to the instructions sysvar (T13).
    #[account(address = INSTRUCTIONS_SYSVAR)]
    pub instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<GuardedSwap>, amount_in: u64, min_out: u64, asset_idx: u8) -> Result<()> {
    let _ = (&ctx, amount_in, min_out, asset_idx);
    // TODO(T13): design §5.5 steps 1-8.
    // Top-level, keeper, not paused, no pending.
    // approved_mask bit for user_vault.methodology, else MintNotApproved.
    // Registry and price freshness; per-trade and daily caps.
    // Forward scan: exactly one matching post_swap; only allow-listed programs in between (RouteNotAllowed).
    // On devnet and in LiteSVM the route program is mock-route (good/short/skim/foreign).
    // Jupiter v6 is mainnet-only. The allow-list must include mock-route on devnet.
    // Snapshot balances, set pending, PDA-signed transfer_checked of USDC to the keeper ATA.
    msg!("TODO(T13): pre_swap");
    Ok(())
}
