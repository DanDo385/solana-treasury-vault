use anchor_lang::prelude::*;

use crate::{
    constants::{PROPOSAL_SEED, TREASURY_SEED},
    error::ErrorCode,
    state::{Treasury, WithdrawalProposal},
};

#[derive(Accounts)]
pub struct ApproveWithdrawal<'info> {
    pub approver: Signer<'info>,
    #[account(
        seeds = [TREASURY_SEED],
        bump = treasury.bump,
        has_one = approver @ ErrorCode::UnauthorizedApprover
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, treasury.key().as_ref(), &proposal.id.to_le_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, WithdrawalProposal>,
}

pub fn handle_approve_withdrawal(ctx: Context<ApproveWithdrawal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    require!(!proposal.cancelled, ErrorCode::ProposalCancelled);
    require!(!proposal.executed, ErrorCode::ProposalAlreadyExecuted);
    proposal.approved = true;
    Ok(())
}
