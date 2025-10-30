use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;
use crate::constants::*;
use crate::states::{Group, MemberProfile, Vote};
use crate::errors::{DaoError, GroupError};
use crate::states::TradeProposal;

pub fn _vote(ctx: Context<VoteContext>, yes_vote: bool) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let owner = &ctx.accounts.owner;
    let group = &ctx.accounts.group;
    let proposal = &mut ctx.accounts.proposal;
    
    let clock = Clock::get()?;

    require!(group.is_trader(&signer.key()), GroupError::NotAdminError);
    require!(proposal.group == group.key(), DaoError::GroupMismatch);
    require!(proposal.executed == false, DaoError::ProposalAlreadyExecuted);
    require!(proposal.deadline > clock.unix_timestamp, DaoError::ProposalExpired);

    proposal.vote_count += 1;

    // check for consensus
    let total_traders = group.members.len();
    let vote_pct = (proposal.vote_count * 100) / total_traders as u32;

    if vote_pct >= group.vote_threshhold as u32 {
        proposal.executed = true;
        // trade tokens
        msg!("Trading executed")
    }

    // emit vote event
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, nonce: u64)]
pub struct VoteContext <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub owner: SystemAccount<'info>,
    #[account(
        seeds = [GROUP_SEED.as_bytes(), name.to_uppercase().as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub group: Account<'info, Group>,
    #[account(
        seeds = [MEMBER_SEED.as_bytes(), group.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub member_profile: Account<'info, MemberProfile>,
    #[account (
        seeds = [PROPOSAL_SEED.as_bytes(), signer.key().as_ref(), group.key().as_ref(), &nonce.to_be_bytes()],
        bump
    )]
    pub proposal: Account<'info, TradeProposal>,
    #[account(
        init,
        payer = signer,
        seeds = [VOTE_SEED.as_bytes(), proposal.key().as_ref(), signer.key().as_ref()],
        space = 8 + Vote::INIT_SPACE,
        bump
    )]
    pub vote: Account<'info, Vote>,
    pub system_program: Program<'info, System>
}
