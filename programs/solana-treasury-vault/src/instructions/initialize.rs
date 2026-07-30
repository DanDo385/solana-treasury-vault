use anchor_lang::prelude::*;

use crate::{constants::TREASURY_SEED, error::ErrorCode, state::Treasury};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(
    ctx: Context<Initialize>,
    approver: Pubkey,
    cooldown_slots: u64,
) -> Result<()> {
    require_keys_neq!(
        ctx.accounts.owner.key(),
        approver,
        ErrorCode::OwnerCannotBeApprover
    );

    let treasury = &mut ctx.accounts.treasury;
    treasury.owner = ctx.accounts.owner.key();
    treasury.approver = approver;
    treasury.cooldown_slots = cooldown_slots;
    treasury.next_proposal_id = 0;
    treasury.bump = ctx.bumps.treasury;
    Ok(())
}
