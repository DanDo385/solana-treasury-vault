use anchor_lang::prelude::*;

use crate::{
    constants::{PROPOSAL_SEED, TREASURY_SEED},
    error::ErrorCode,
    state::{Treasury, WithdrawalProposal},
};

#[derive(Accounts)]
pub struct ProposeWithdrawal<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury.bump,
        has_one = owner @ ErrorCode::UnauthorizedOwner
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = owner,
        space = 8 + WithdrawalProposal::INIT_SPACE,
        seeds = [
            PROPOSAL_SEED,
            treasury.key().as_ref(),
            &treasury.next_proposal_id.to_le_bytes()
        ],
        bump
    )]
    pub proposal: Account<'info, WithdrawalProposal>,
    pub system_program: Program<'info, System>,
}

pub fn handle_propose_withdrawal(
    ctx: Context<ProposeWithdrawal>,
    recipient: Pubkey,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidWithdrawalAmount);

    let id = ctx.accounts.treasury.next_proposal_id;
    let proposal = &mut ctx.accounts.proposal;
    proposal.treasury = ctx.accounts.treasury.key();
    proposal.id = id;
    proposal.proposer = ctx.accounts.owner.key();
    proposal.recipient = recipient;
    proposal.amount = amount;
    proposal.proposed_slot = Clock::get()?.slot;
    proposal.approved = false;
    proposal.executed = false;
    proposal.cancelled = false;
    proposal.bump = ctx.bumps.proposal;

    ctx.accounts.treasury.next_proposal_id =
        id.checked_add(1).ok_or(ErrorCode::ProposalIdOverflow)?;
    Ok(())
}
