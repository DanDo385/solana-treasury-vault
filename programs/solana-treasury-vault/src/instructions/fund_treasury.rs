use anchor_lang::{prelude::*, system_program};

use crate::{constants::TREASURY_SEED, error::ErrorCode, state::Treasury};

#[derive(Accounts)]
pub struct FundTreasury<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut, seeds = [TREASURY_SEED], bump = treasury.bump)]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handle_fund_treasury(ctx: Context<FundTreasury>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidWithdrawalAmount);

    let transfer_accounts = system_program::Transfer {
        from: ctx.accounts.depositor.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
    };
    system_program::transfer(
        CpiContext::new(ctx.accounts.system_program.key(), transfer_accounts),
        amount,
    )
}
