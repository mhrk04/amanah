pub mod errors;
pub mod events;
pub mod instructions;
pub mod introspection;
pub mod state;
pub mod token2022;

use anchor_lang::prelude::*;

pub use errors::*;
pub use events::*;
pub use instructions::admin::{Admin, SetAssets, SetParamsArgs};
pub use instructions::deposit::Deposit;
pub use instructions::initialize::{Initialize, InitializeParams};
pub use instructions::on_report::OnReport;
pub use instructions::open_vault::{OpenVault, SetProfile};
pub use instructions::pre_swap::GuardedSwap;
pub use instructions::withdraw::Withdraw;
pub use state::*;

// The program macro looks these modules up at the crate root. The Accounts
// derive emits them next to each struct, so re-export them here.
// `__cpi_client_accounts_*` is only named when the `cpi` feature is on.
#[allow(unused_imports)]
mod generated_accounts {
    pub(crate) use super::instructions::admin::{
        __client_accounts_admin, __client_accounts_set_assets, __cpi_client_accounts_admin,
        __cpi_client_accounts_set_assets,
    };
    pub(crate) use super::instructions::deposit::{
        __client_accounts_deposit, __cpi_client_accounts_deposit,
    };
    pub(crate) use super::instructions::initialize::{
        __client_accounts_initialize, __cpi_client_accounts_initialize,
    };
    pub(crate) use super::instructions::on_report::{
        __client_accounts_on_report, __cpi_client_accounts_on_report,
    };
    pub(crate) use super::instructions::open_vault::{
        __client_accounts_open_vault, __client_accounts_set_profile, __cpi_client_accounts_open_vault,
        __cpi_client_accounts_set_profile,
    };
    pub(crate) use super::instructions::pre_swap::{
        __client_accounts_guarded_swap, __cpi_client_accounts_guarded_swap,
    };
    pub(crate) use super::instructions::withdraw::{
        __client_accounts_withdraw, __cpi_client_accounts_withdraw,
    };
}
#[allow(unused_imports)]
pub(crate) use generated_accounts::*;

declare_id!("AP9K6Z7bpxhKTPf733sDtvMJyXDfK6ThWwC1xjekEseF");

#[program]
pub mod amanah_vault {
    use super::*;

    /// TODO(T05): create Config + ComplianceRegistry. FR-01.
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    /// TODO(T05): set the asset universe and clear approvals. FR-02.
    pub fn set_assets(ctx: Context<SetAssets>) -> Result<()> {
        instructions::admin::set_assets(ctx)
    }

    /// TODO(T05): admin setter. FR-03.
    pub fn set_keeper(ctx: Context<Admin>, keeper: Pubkey) -> Result<()> {
        instructions::admin::set_keeper(ctx, keeper)
    }

    /// TODO(T05): admin setter. FR-03.
    pub fn set_forwarder(
        ctx: Context<Admin>,
        forwarder_program: Pubkey,
        forwarder_state: Pubkey,
    ) -> Result<()> {
        instructions::admin::set_forwarder(ctx, forwarder_program, forwarder_state)
    }

    /// TODO(T05): `None` switches the transmitter check off for Keystone. FR-03.
    pub fn set_transmitter(ctx: Context<Admin>, allowed_transmitter: Option<Pubkey>) -> Result<()> {
        instructions::admin::set_transmitter(ctx, allowed_transmitter)
    }

    /// TODO(T05): caps, slippage, and max ages. FR-03.
    pub fn set_params(ctx: Context<Admin>, params: SetParamsArgs) -> Result<()> {
        instructions::admin::set_params(ctx, params)
    }

    /// TODO(T05): pause switch. Buys stop; withdraw stays open. FR-03.
    pub fn set_paused(ctx: Context<Admin>, paused: bool) -> Result<()> {
        instructions::admin::set_paused(ctx, paused)
    }

    /// TODO(T06): create UserVault + pinned USDC ATA. FR-04.
    pub fn open_vault(ctx: Context<OpenVault>, risk_profile: u8, methodology: u8) -> Result<()> {
        instructions::open_vault::open_vault(ctx, risk_profile, methodology)
    }

    /// TODO(T06): owner changes risk profile and methodology when no swap is pending. FR-04.
    pub fn set_profile(ctx: Context<SetProfile>, risk_profile: u8, methodology: u8) -> Result<()> {
        instructions::open_vault::set_profile(ctx, risk_profile, methodology)
    }

    /// TODO(T06): USDC `transfer_checked` into the vault. Blocked while paused. FR-05.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// TODO(T06): in-kind withdraw. Allowed while paused. Blocked while a swap is pending. FR-05.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    /// TODO(T07): hardened CRE receiver. FR-06, FR-07.
    pub fn on_report(ctx: Context<OnReport>, metadata: Vec<u8>, report: Vec<u8>) -> Result<()> {
        instructions::on_report::handler(ctx, metadata, report)
    }

    /// TODO(T13): top-level guard before the route. Devnet/LiteSVM route is mock-route; mainnet is Jupiter. FR-08.
    pub fn pre_swap(
        ctx: Context<GuardedSwap>,
        amount_in: u64,
        min_out: u64,
        asset_idx: u8,
    ) -> Result<()> {
        instructions::pre_swap::handler(ctx, amount_in, min_out, asset_idx)
    }

    /// TODO(T14): floor check after the route. FR-09.
    pub fn post_swap(ctx: Context<GuardedSwap>) -> Result<()> {
        instructions::post_swap::handler(ctx)
    }
}
