use anchor_lang::prelude::*;
use crate::constants::*;
use crate::states::{Group, MemberProfile};
use crate::errors::GroupError;

pub fn _exit_group(ctx: Context<ExitGroupContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let group = &mut ctx.accounts.group;

    require!(!group.is_member(&signer.key()), GroupError::AlreadyJoined);

    group.remove_member(signer.key)?;
    // withdraw memeber assets
    // emit event
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct ExitGroupContext <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub owner: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [GROUP_SEED.as_bytes(), name.to_uppercase().as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub group: Account<'info, Group>,
    #[account(
        mut,
        close=signer,
        seeds = [MEMBER_SEED.as_bytes(), group.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub member_profile: Account<'info, MemberProfile>,
    system_program: Program<'info, System>
}
