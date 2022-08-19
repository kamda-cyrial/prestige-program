use borsh::{BorshDeserialize, BorshSerialize};
use cond_utils::{Between};

pub mod constants{
    solana_program::declare_id!("cb8zdnsdLA9dwzf65LmMZ745Z1M8DYSZZht1CADeouZ");

    pub mod authorized{
        solana_program::declare_id!("cb8zdnsdLA9dwzf65LmMZ745Z1M8DYSZZht1CADeouZ");
    }

    pub const USERREGKEY: &str = "Prestige Registration";
}

#[derive(BorshDeserialize, BorshSerialize)]
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