pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri");

#[program]
pub mod solana_treasury_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        approver: Pubkey,
        cooldown_slots: u64,
    ) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx, approver, cooldown_slots)
    }

    pub fn fund_treasury(ctx: Context<FundTreasury>, amount: u64) -> Result<()> {
        crate::instructions::fund_treasury::handle_fund_treasury(ctx, amount)
    }

    pub fn propose_withdrawal(
        ctx: Context<ProposeWithdrawal>,
        recipient: Pubkey,
        amount: u64,
    ) -> Result<()> {
        crate::instructions::propose_withdrawal::handle_propose_withdrawal(ctx, recipient, amount)
    }

    pub fn approve_withdrawal(ctx: Context<ApproveWithdrawal>) -> Result<()> {
        crate::instructions::approve_withdrawal::handle_approve_withdrawal(ctx)
    }

    pub fn execute_withdrawal(ctx: Context<ExecuteWithdrawal>) -> Result<()> {
        crate::instructions::execute_withdrawal::handle_execute_withdrawal(ctx)
    }

    pub fn cancel_withdrawal(ctx: Context<CancelWithdrawal>) -> Result<()> {
        crate::instructions::cancel_withdrawal::handle_cancel_withdrawal(ctx)
    }
}
