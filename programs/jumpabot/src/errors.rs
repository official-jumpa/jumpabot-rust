use anchor_lang::prelude::*;
use crate::constants::*;

#[error_code]
pub enum GroupError {
    #[msg("Name length should be between {MIN_NAME_LEN} to {MAX_NAME_LEN} characters long")]
    InvalidName,
    #[msg("Capital cannot be zero")]
    InvalidCapital,
    #[msg("Vote threshold cannot be less than 10% and greater than 100%")]
    InvalidThreshold,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Group is currently locked")]
    GroupLockedError,
    #[msg("Cannot join, group members completed")]
    GroupMembersComplete,
    #[msg("Cannot add more traders")]
    GroupTradersComplete,
    #[msg("Already joined")]
    AlreadyJoined,
    #[msg("Only admin can perform this action")]
    NotAdminError,
    #[msg("Must be a member first")]
    NotAMemberError,
    #[msg("Member is not a trader")]
    NotATrader,
    #[msg("Group has alreaded ended")]
    GroupEnded,
    #[msg("Group has already started trading")]
    GroupStarted
}

#[error_code]
pub enum DaoError {
    #[msg("Proposal has already been executed")]
    ProposalAlreadyExecuted,
    #[msg("Proposal has expired")]
    ProposalExpired,
    #[msg("Proposal does not belong to this group")]
    GroupMismatch
}
