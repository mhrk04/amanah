use anchor_lang::prelude::*;

use crate::introspection::INSTRUCTIONS_SYSVAR;
use crate::state::{ComplianceRegistry, Config};

#[derive(Accounts)]
pub struct OnReport<'info> {
    /// CHECK: key and owner checked against config in the handler (T07).
    pub state: UncheckedAccount<'info>,
    pub forwarder_authority: Signer<'info>,
    #[account(mut, seeds = [b"registry"], bump = registry.bump)]
    pub registry: Account<'info, ComplianceRegistry>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    /// CHECK: address-constrained to the instructions sysvar (T07).
    #[account(address = INSTRUCTIONS_SYSVAR)]
    pub instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<OnReport>, metadata: Vec<u8>, report: Vec<u8>) -> Result<()> {
    let _ = (&ctx, &metadata, &report);
    // TODO(T07): design §5.4 steps 1-7.
    // 1. state key == config.forwarder_state and state.owner == config.forwarder_program.
    // 2. forwarder_authority is the canonical PDA [b"forwarder", state, program_id] of the forwarder program, and is a signer.
    // 3. allowed transmitter is a signer meta of the top-level forwarder instruction (position-agnostic).
    // 4. Borsh-decode ComplianceReportV2, version == 2, report.len() <= 300.
    // 5. report_id > last; as_of and price_ts windows.
    // 6. universe_version match; sorted unique asset_idx; reserved bits zero; sum target_bps in {0, 10000}.
    // 7. approved_mask = company_pass_mask if wrapper_ok && chain_ok else 0. Write registry. Emit RegistryUpdated.
    msg!("TODO(T07): on_report");
    Ok(())
}
