use anchor_lang::prelude::*;
use crate::constants::*;
use crate::states::{Group, GroupState, MemberProfile};
use crate::errors::GroupError;
use crate::states::TradeProposal;

pub fn _propose_trade(ctx: Context<ProposeTradeContext>, nounce: u64, amount: u64, buy: bool) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let group = &mut ctx.accounts.group;
    let proposal = &mut ctx.accounts.proposal;
    let token_account = &ctx.accounts.token_account;
    let mint_account = &ctx.accounts.mint_account;
    let member_profile = &ctx.accounts.member_profile;
    
    require!(group.is_trader(&signer.key()), GroupError::NotAdminError);

    // validate balances
    if buy && group.get_lamports() < amount {
        return Err(GroupError::InsufficientFunds.into());
    }

    let clock = Clock::get()?;
    let one_day: i64 = 24 * 60 * 60; 

    group.state = GroupState::Voting;
    proposal.group = group.key().clone();
    proposal.proposer = signer.key().clone();
    proposal.proposer_name = member_profile.name.clone();
    proposal.vote_count = 0;
    proposal.executed = false;
    proposal.created_at = clock.unix_timestamp;
    proposal.deadline = clock.unix_timestamp + one_day;
    proposal.nounce = nounce;
    proposal.bump = ctx.bumps.proposal;
    proposal.amount = amount;
    proposal.buy = buy;
    proposal.target_mint = mint_account.key();

    // emit proposal event

    Ok(())
}

#[derive(Accounts)]
#[instruction(nounce: u64)]
pub struct ProposeTradeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub owner: SystemAccount<'info>,
    /// CHECK: this is a custom account
    pub group: Account<'info, Group>,
    #[account(
        init,
        payer = signer,
        space = 8 + TradeProposal::INIT_SPACE,
        seeds = [
            PROPOSAL_SEED.as_bytes(),
            signer.key().as_ref(),
            group.key().as_ref(),
            &nounce.to_be_bytes()
        ],
        bump
    )]
    pub proposal: Account<'info, TradeProposal>,
    /// CHECK: 
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: 
    pub mint_account: UncheckedAccount<'info>,
    #[account(
        seeds = [MEMBER_SEED.as_bytes(), group.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub member_profile: Account<'info, MemberProfile>,
    pub system_program: Program<'info, System>,
}