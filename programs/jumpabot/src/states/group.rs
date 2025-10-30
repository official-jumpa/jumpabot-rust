use anchor_lang::prelude::*;

use crate::{constants::{MAX_MEMBERS, MAX_NAME_LEN, MAX_TRADERS, MIN_NAME_LEN}, errors::GroupError};


#[account]
#[derive(InitSpace)]
pub struct MemberProfile {
    pub group: Pubkey,
    pub pubkey: Pubkey,
    #[max_len(MAX_NAME_LEN)]
    pub name: String,
    pub last_proposal_date: Option<u64>
}

impl MemberProfile {
    pub fn is_valid_name(name: &String) -> bool {
        name.len() >= MIN_NAME_LEN && name.len() <= MAX_NAME_LEN
    }
}

#[derive(InitSpace, Clone, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum GroupState {
    Initialized,
    Trading,
    Voting,
    Ended
}

#[account]
#[derive(InitSpace)]
pub struct Group {
    pub owner: Pubkey,
    #[max_len(MAX_NAME_LEN)]
    pub name: String,
    #[max_len(MAX_TRADERS)]
    pub traders: Vec<Pubkey>,
    #[max_len(MAX_MEMBERS)]
    pub members: Vec<Pubkey>,
    pub entry_capital: u64,
    pub vote_threshhold: u8,
    pub locked: bool,
    pub bump: u8,
    pub state: GroupState,
    pub created_at: i64
}


impl Group {
    pub fn is_member(&self, pubkey: &Pubkey) -> bool {
        self.members.contains(pubkey)
    }

    pub fn is_trader(&self, pubkey: &Pubkey) -> bool {
        self.traders.contains(pubkey)
    }

    pub fn add_member(&mut self, pubkey: &Pubkey) -> Result<()> {
        if self.members.len() > MAX_MEMBERS {
            return Err(GroupError::GroupMembersComplete.into());
        }
        if self.members.contains(pubkey){
            return Err(GroupError::AlreadyJoined.into());
        }
        self.members.push(pubkey.clone());
        Ok(())
    }

    pub fn remove_member(&mut self, pubkey: &Pubkey) -> Result<()> {
        if !self.members.contains(pubkey){
            return Err(GroupError::NotAMemberError.into());
        }
        self.traders.retain(|t| t != pubkey);
        self.members.retain(|t| t != pubkey);
        Ok(())
    }

    pub fn add_trader(&mut self, pubkey: &Pubkey) -> Result<()> {
        if self.traders.len() > MAX_TRADERS {
            return Err(GroupError::GroupTradersComplete.into());
        }
        if self.traders.contains(pubkey){
            return Err(GroupError::AlreadyJoined.into());
        }
        if !self.is_member(pubkey) {
            return Err(GroupError::NotAMemberError.into());
        }
        self.traders.push(pubkey.clone());
        Ok(())
    }


    pub fn remove_trader(&mut self, pubkey: &Pubkey) -> Result<()> {
        if !self.traders.contains(pubkey){
            return Err(GroupError::NotATrader.into());
        }
        if !self.is_member(pubkey) {
            return Err(GroupError::NotAMemberError.into());
        }
        self.traders.retain(|t| t != pubkey);
        Ok(())
    }

    pub fn is_owner(&self, pubkey: &Pubkey) -> bool {
        &self.owner == pubkey
    }
}


