use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub owner: Pubkey,
    pub approver: Pubkey,
    pub cooldown_slots: u64,
    pub next_proposal_id: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct WithdrawalProposal {
    pub treasury: Pubkey,
    pub id: u64,
    pub proposer: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub proposed_slot: u64,
    pub approved: bool,
    pub executed: bool,
    pub cancelled: bool,
    pub bump: u8,
}
