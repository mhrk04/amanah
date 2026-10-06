use anchor_lang::prelude::*;

pub use crate::instructions::pre_swap::GuardedSwap;

pub fn handler(ctx: Context<GuardedSwap>) -> Result<()> {
    let _ = &ctx;
    // TODO(T14): design §5.5 post_swap.
    // Top-level; pending present; same pinned accounts.
    // reload() both token accounts.
    // effective_multiplier_e12 from the mint TLV (token2022.rs); floor in u128, rounds down.
    // delta >= max(min_out, floor), else SlippageExceeded. Missing or stale price rejects.
    // keeper USDC <= snapshot, else KeeperKeptFunds.
    // Clear pending. Emit SwapGuarded.
    msg!("TODO(T14): post_swap");
    Ok(())
}
