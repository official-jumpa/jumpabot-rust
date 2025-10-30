use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::constants::*;
use crate::states::{Group, MemberProfile, GroupState};
use crate::errors::GroupError;

pub fn _initialize_group(ctx: Context<InitializeGroupContext>, 
    group_name: String,
    admin_name: String,
    entry_capital: u64,
    vote_threshhold: u8,
) -> Result<()> {
    let clock = Clock::get()?;
    let signer = &ctx.accounts.signer;
    let group = &mut ctx.accounts.group;
    let member_profile = &mut ctx.accounts.member_profile;

    require!(group_name.len() >= MIN_NAME_LEN && group_name.len() <= MAX_NAME_LEN, GroupError::InvalidName);
    require!(admin_name.len() >= MIN_NAME_LEN && admin_name.len() <= MAX_NAME_LEN, GroupError::InvalidName);
    require!(entry_capital > 0, GroupError::InvalidCapital);
    require!(vote_threshhold >= 10 && vote_threshhold <= 100, GroupError::InvalidThreshold);
    require!(signer.lamports() > entry_capital, GroupError::InsufficientFunds);

    member_profile.group = group.key().clone();
    member_profile.pubkey = signer.key().clone();
    member_profile.name = admin_name.clone();
    member_profile.last_proposal_date = None;

    group.owner = signer.key.to_owned();
    group.name = group_name;
    group.traders.push(signer.key().clone());
    group.members.push(signer.key().clone());
    group.entry_capital = entry_capital;
    group.vote_threshhold = vote_threshhold;
    group.locked = false;
    group.state = GroupState::Initialized;
    group.created_at = clock.unix_timestamp;

    let from_pubkey = ctx.accounts.signer.to_account_info();
    let to_pubkey = ctx.accounts.group.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();


    let cpi_context = CpiContext::new(program_id, Transfer{
        from: from_pubkey,
        to: to_pubkey
    });

    transfer(cpi_context, entry_capital)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(group_name: String)]
pub struct InitializeGroupContext <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Group::INIT_SPACE,
        seeds = [GROUP_SEED.as_bytes(), group_name.to_uppercase().as_bytes(), signer.key().as_ref()],
        bump
    )]
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
