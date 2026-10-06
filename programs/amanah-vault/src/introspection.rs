use anchor_lang::prelude::*;

/// Top-level programs allowed between `pre_swap` and `post_swap` (design §5.5, T13).
/// Jupiter v6, SPL Token, Token-2022, and the associated-token program.
pub const JUPITER_V6: Pubkey = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
pub const SPL_TOKEN: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const TOKEN_2022: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub const ASSOCIATED_TOKEN: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// LiteSVM and devnet use this program in place of Jupiter. xStock routes exist only on mainnet.
/// The stub allow-list includes this id so those environments can sandwich a route.
/// TODO(T13): drop it from the mainnet allow-list (Jupiter v6, SPL Token, Token-2022, ATA only).
pub const MOCK_ROUTE: Pubkey = pubkey!("GsMGHSe7XUcnw4Xe7bQvpvavdkae3nTorgwPY2RPNvyX");

/// Instructions sysvar address. `anchor_lang` 1.2.1 does not re-export
/// `sysvar::instructions::ID` (design §5.6 / §13 item 9). T04/T13 should switch
/// this to the crate helper, and add `solana-instructions-sysvar` on the ^3 line
/// if `load_instruction_at_checked` is still missing. Check `cargo tree -d`.
pub const INSTRUCTIONS_SYSVAR: Pubkey = pubkey!("Sysvar1nstructions1111111111111111111111111");

pub fn is_allow_listed(program_id: &Pubkey) -> bool {
    *program_id == JUPITER_V6
        || *program_id == SPL_TOKEN
        || *program_id == TOKEN_2022
        || *program_id == ASSOCIATED_TOKEN
        || *program_id == MOCK_ROUTE
}

/// TODO(T13): `get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT`, else `NotTopLevel`.
/// `anchor_lang::solana_program::instruction::get_stack_height` exists in 1.2.1.
pub fn assert_top_level() -> Result<()> {
    Ok(())
}

/// TODO(T13): forward-scan from the current instruction index.
/// Exactly one matching `post_swap` (program id + discriminator + user_vault + vault_out_ata).
/// Every instruction in between must be allow-listed, else `RouteNotAllowed`.
/// A second `pre_swap` for the same vault before `post_swap` is `SwapPending`.
/// None is `MissingPostSwap`.
pub fn assert_forward_route(_ix_sysvar: &AccountInfo) -> Result<()> {
    Ok(())
}

/// TODO(T07): if `config.allowed_transmitter` is `Some`, the top-level instruction
/// must be the forwarder program and must include that pubkey as a signer meta.
/// Position-agnostic (mock index 1, Keystone index 2). Else `UnknownTransmitter`.
pub fn assert_allowed_transmitter(_ix_sysvar: &AccountInfo, _transmitter: Option<Pubkey>) -> Result<()> {
    Ok(())
}
