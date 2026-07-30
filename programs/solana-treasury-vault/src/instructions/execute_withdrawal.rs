use anchor_lang::prelude::*;

use crate::{
    constants::{PROPOSAL_SEED, TREASURY_SEED},
    error::ErrorCode,
    state::{Treasury, WithdrawalProposal},
};

#[derive(Accounts)]
pub struct ExecuteWithdrawal<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury.bump,
        has_one = owner @ ErrorCode::UnauthorizedOwner
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, treasury.key().as_ref(), &proposal.id.to_le_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, WithdrawalProposal>,
    #[account(mut, address = proposal.recipient)]
    pub recipient: SystemAccount<'info>,
}

pub fn handle_execute_withdrawal(ctx: Context<ExecuteWithdrawal>) -> Result<()> {
    let treasury = &ctx.accounts.treasury;
    let proposal = &mut ctx.accounts.proposal;

    require!(!proposal.cancelled, ErrorCode::ProposalCancelled);
    require!(!proposal.executed, ErrorCode::ProposalAlreadyExecuted);
    require!(proposal.approved, ErrorCode::ProposalNotApproved);

    let earliest_slot = proposal
        .proposed_slot
        .checked_add(treasury.cooldown_slots)
        .ok_or(ErrorCode::SlotOverflow)?;
    require!(
        Clock::get()?.slot >= earliest_slot,
        ErrorCode::CooldownNotElapsed
    );

    let rent_reserve = Rent::get()?.minimum_balance(8 + Treasury::INIT_SPACE);
    let required_lamports = rent_reserve
        .checked_add(proposal.amount)
        .ok_or(ErrorCode::LamportOverflow)?;
    require!(
        treasury.to_account_info().lamports() >= required_lamports,
        ErrorCode::InsufficientTreasuryFunds
    );

    let treasury_info = treasury.to_account_info();
    let recipient_info = ctx.accounts.recipient.to_account_info();
    **treasury_info.try_borrow_mut_lamports()? = treasury_info
        .lamports()
        .checked_sub(proposal.amount)
        .ok_or(ErrorCode::InsufficientTreasuryFunds)?;
    **recipient_info.try_borrow_mut_lamports()? = recipient_info
        .lamports()
        .checked_add(proposal.amount)
        .ok_or(ErrorCode::LamportOverflow)?;

    proposal.executed = true;
    Ok(())
}
