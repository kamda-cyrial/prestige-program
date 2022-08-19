use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::borsh::try_from_slice_unchecked;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionEnum {
    RegisterUser{user_name: Vec<u8>},
    RewardXP{total_xp: u32},
}

impl InstructionEnum {
    pub fn decode(data: &[u8]) -> Self {
        try_from_slice_unchecked(data).unwrap()
    }
}

