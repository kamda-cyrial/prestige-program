use solana_program::{
    pubkey::Pubkey,
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    
};

use crate::{
    instruction::InstructionEnum,
    state::constants::*,
    utils::{assert_pubkeys_exactitude, assert_is_signer}
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(
        match InstructionEnum::decode(instruction_data){
            InstructionEnum::RegisterUser{user_name} => process_register_user(program_id, accounts, user_name)?, 
            _ => Err(ProgramError::InvalidInstructionData)?
        }
    )
}

pub fn process_register_user(program_id: &Pubkey, accounts: &[AccountInfo], user_name: Vec<u8>) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account_info = next_account_info(account_info_iter)?;
    let user_account_info = next_account_info(account_info_iter)?;
    let user_data_info = next_account_info(account_info_iter)?;

    assert_pubkeys_exactitude(payer_account_info.key, &authorized::id()).and(assert_is_signer(payer_account_info))?;

    let (expected_user_data_key, expected_user_data_bump) = Pubkey::find_program_address(&[USERREGKEY.as_ref(), user_name.as_slice()], program_id);
    assert_pubkeys_exactitude(&expected_user_data_key, user_data_info.key).unwrap();
    
    Ok(())
}