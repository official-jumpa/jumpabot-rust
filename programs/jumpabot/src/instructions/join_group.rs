use anchor_lang::prelude::*;
use crate::constants::*;
use crate::states::{Group, MemberProfile, GroupState};
use crate::errors::GroupError;
use crate::utils::transfers::transfer;

pub fn _join_group(ctx: Context<JoinGroupContext>, member_name: String) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let member_profile = &mut ctx.accounts.member_profile;
    let group = &mut ctx.accounts.group;

    if group.state == GroupState::Trading {
        return Err(GroupError::GroupStarted.into());
    }

    if group.state == GroupState::Trading {
        return Err(GroupError::GroupEnded.into());
    }

    require!(signer.lamports() >= group.entry_capital, GroupError::InsufficientFunds);
    require!(!group.is_member(&signer.key()), GroupError::AlreadyJoined);
    require!(MemberProfile::is_valid_name(&member_name), GroupError::InvalidName);

    member_profile.name = member_name.clone();
    member_profile.group = group.key().clone();
    member_profile.pubkey = signer.key().clone();
    member_profile.last_proposal_date = None;

    group.add_member(signer.key)?;

    transfer(
        ctx.accounts.system_program.to_account_info(),
        signer.to_account_info(),
        group.to_account_info(),
        group.entry_capital,
        None
    )?;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct JoinGroupContext <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub owner: SystemAccount<'info>,
    #[account(mut)]
    /// CHECK: Group PDA
    pub group: Account<'info, Group>,
    #[account(
        init,
        payer = signer,
        space = 8 + MemberProfile::INIT_SPACE,
        seeds = [MEMBER_SEED.as_bytes(), group.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub member_profile: Account<'info, MemberProfile>,
    system_program: Program<'info, System>
}
