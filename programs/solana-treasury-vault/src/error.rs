use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The treasury owner and approver must be different accounts")]
    OwnerCannotBeApprover,
    #[msg("Only the treasury owner may perform this action")]
    UnauthorizedOwner,
    #[msg("Only the configured approver may approve a withdrawal")]
    UnauthorizedApprover,
    #[msg("Withdrawal amount must be greater than zero")]
    InvalidWithdrawalAmount,
    #[msg("The withdrawal proposal is not approved")]
    ProposalNotApproved,
    #[msg("The withdrawal proposal has already been executed")]
    ProposalAlreadyExecuted,
    #[msg("The withdrawal proposal has been cancelled")]
    ProposalCancelled,
    #[msg("The treasury cooldown has not elapsed")]
    CooldownNotElapsed,
    #[msg("The treasury does not have enough available lamports")]
    InsufficientTreasuryFunds,
    #[msg("Proposal identifier overflowed")]
    ProposalIdOverflow,
    #[msg("Slot arithmetic overflowed")]
    SlotOverflow,
    #[msg("Lamport arithmetic overflowed")]
    LamportOverflow,
}
