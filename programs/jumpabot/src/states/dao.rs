use anchor_lang::prelude::*;

use crate::constants::MAX_NAME_LEN;

#[account]
#[derive(InitSpace)]
pub struct Vote {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub yes_vote: bool,
    pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct TradeProposal {
    pub group: Pubkey,
    pub proposer: Pubkey,
    #[max_len(MAX_NAME_LEN)]
    pub proposer_name: String,
    pub vote_count: u32,
    pub amount: u64,
    pub target_mint: Pubkey,
    pub buy: bool,
    pub dex: Dex,
    pub created_at: i64,
    pub deadline: i64,
    pub nounce: u64,
    pub bump: u8,
    pub executed: bool,
}

#[derive(InitSpace, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Dex {
    Raydium
}
    