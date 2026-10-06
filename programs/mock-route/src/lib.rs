use anchor_lang::prelude::*;

declare_id!("GsMGHSe7XUcnw4Xe7bQvpvavdkae3nTorgwPY2RPNvyX");

/// Stand-in for a Jupiter route in LiteSVM and on devnet (T15).
/// Jupiter routes for xStocks exist only on mainnet, so the devnet swap guard
/// sandwiches this program instead. Modes: 0 good, 1 short (output below the floor),
/// 2 skim (keeper keeps USDC), 3 foreign-program (this id is not on the allow-list).
#[program]
pub mod mock_route {
    use super::*;

    pub fn execute(ctx: Context<Execute>, mode: u8) -> Result<()> {
        let _ = (&ctx, mode);
        // TODO(T15): good / short / skim / foreign-program.
        msg!("TODO(T15): mock-route execute");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    pub keeper: Signer<'info>,
}
