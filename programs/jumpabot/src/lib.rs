use anchor_lang::prelude::*;

mod instructions;
mod states;
mod errors;
mod constants;
mod utils;

use instructions::*;

declare_id!("E4bVkwfeUuh1eWLuz9pjaAkuZYUGY8nWiybCBG3cpHpr");

#[program]
pub mod jumpabot {

    use super::*;

    pub fn initialize_group(
        ctx: Context<InitializeGroupContext>,
        group_name: String,
        admin_name: String,
        entry_capital: u64,
        vote_threshhold: u8,
    ) -> Result<()> {
        _initialize_group(
            ctx,
            group_name,
            admin_name,
            entry_capital,
            vote_threshhold,
        )
    }

    pub fn join_group(
        ctx: Context<JoinGroupContext>,
        member_name: String
    ) -> Result<()> {
        _join_group(ctx, member_name)
    }

    pub fn exit_group(
        ctx: Context<ExitGroupContext>
    ) -> Result<()> {
        _exit_group(ctx, )
    }

    pub fn propose_trade(
        ctx: Context<ProposeTradeContext>, 
        name: String, 
        nounce: u64,
        amount: u64,
        buy: bool,
    ) -> Result<()> {
        _propose_trade(ctx, nounce, amount, buy)
    }

    pub fn add_trader(ctx: Context<AddTraderContext>) -> Result<()> {
        _add_trader(ctx)
    }

    pub fn remove_trader(ctx: Context<RemoveTraderContext>) -> Result<()> {
        _remove_trader(ctx)
    }
}