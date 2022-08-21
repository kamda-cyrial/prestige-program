use borsh::{BorshDeserialize, BorshSerialize};
use cond_utils::{Between};
use solana_program::{program_error::ProgramError, account_info::AccountInfo, borsh::try_from_slice_unchecked};

use crate::error::PrestigeError;

use self::constants::USERDATA_STRUCT_KEY;

pub mod constants{
    solana_program::declare_id!("cb8zdnsdLA9dwzf65LmMZ745Z1M8DYSZZht1CADeouZ");

    pub mod authorized{
        solana_program::declare_id!("cb8zdnsdLA9dwzf65LmMZ745Z1M8DYSZZht1CADeouZ");
    }

    pub const USERREGKEY: &str = "Prestige Registration";
    pub const XP_MINT_KEY: &str = "Prestige Mint Key";
    pub const GENERAL_AUTHORITY_KEY: &str = "freeze_and_mint_authority Key";

    pub const USERDATA_STRUCT_KEY: u32 = 138_734_492;
}

#[derive(BorshDeserialize, PartialEq, Eq, BorshSerialize)]
pub enum Rank{
    None,
    Novice,
    Scholar,
    Developer,
    Mentor
}

impl Rank{
    pub fn get_min_xp(self) -> u64 {
        10_000 * 2u64.pow(self.try_to_vec().unwrap()[0] as u32)
    }
    pub fn xp_to_rank(xp: u64) -> Self{
        if xp.leftween(0, Self::Novice.get_min_xp()){
            Self::None
        }
        else if xp.leftween(Self::Novice.get_min_xp(), Self::Scholar.get_min_xp()){
            Self::Novice
        }
        else if xp.leftween(Self::Scholar.get_min_xp(), Self::Developer.get_min_xp()){
            Self::Scholar
        }
        else if xp.leftween(Self::Developer.get_min_xp(), Self::Mentor.get_min_xp()){
            Self::Developer
        }
        else{
            Self::Mentor
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Issue{
    pub issued_xp: u64,
    pub issue_id: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserData{
    pub struct_key: u32,
    pub total_xp: u64,
    pub rank: Rank,
    pub registration_date: u32,
    // pub all_issues: Vec<Issue>, //account Space Too Expensive to have. Will need to rediscuss in the meeting
}

impl UserData {
    pub fn authenticate(self) -> Result<Self, ProgramError> {
        if self.struct_key != USERDATA_STRUCT_KEY {
            Err(PrestigeError::WrongStructKey.log(Some("User Data")))?
        }
        Ok(self)
    }
    pub fn decode(account: &AccountInfo) -> Result<Self, ProgramError> {
        let a: Self = try_from_slice_unchecked(&account.data.borrow()).unwrap();
        a.authenticate()
    }
}