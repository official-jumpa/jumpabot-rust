use anchor_lang::prelude::*;
use crate::constants::*;
use crate::states::{Group, GroupState};
use crate::errors::GroupError;

pub fn _remove_trader(ctx: Context<RemoveTraderContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let group = &mut ctx.accounts.group;
    let trader = &ctx.accounts.trader;

    require!(group.is_owner(&signer.key), GroupError::NotAdminError);
    require!(group.state != GroupState::Ended, GroupError::GroupEnded);
    require!(group.is_trader(&trader.key()), GroupError::NotATrader);
    
    group.remove_trader(trader.key)?;
    // emit event
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveTraderContext <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub group: Account<'info, Group>,
    pub trader: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}
