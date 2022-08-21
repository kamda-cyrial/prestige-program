use borsh::BorshSerialize;
use solana_program::{
    pubkey::Pubkey,
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError, system_instruction, rent::Rent, sysvar::Sysvar, program::{invoke_signed, invoke}, clock::Clock,
    
};

use spl_associated_token_account::{get_associated_token_address, *};

use crate::{
    instruction::InstructionEnum,
    state::{constants::*, UserData, Rank},
    utils::{assert_pubkeys_exactitude, assert_is_signer}
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(
        match InstructionEnum::decode(instruction_data){
            InstructionEnum::InitProgram => process_init_configuration(program_id, accounts)?,
            InstructionEnum::RegisterUser{user_name} => process_register_user(program_id, accounts, user_name)?,
            InstructionEnum::RewardXP { total_xp, user_name } => process_reward_xp(program_id, accounts, total_xp, user_name)?, 
            
            _ => Err(ProgramError::InvalidInstructionData)?
        }
    )
}

pub fn process_reward_xp(program_id: &Pubkey, accounts: &[AccountInfo], total_xp: u64, user_name: Vec<u8>) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account_info = next_account_info(account_info_iter)?;
    let user_account_info = next_account_info(account_info_iter)?;
    let user_data_info = next_account_info(account_info_iter)?;
    let user_token_account_info = next_account_info(account_info_iter)?;
    let mint_account_info = next_account_info(account_info_iter)?;
    // let system_program_account_info = next_account_info(account_info_iter)?;
    let spl_token_program_account_info = next_account_info(account_info_iter)?;
    let general_authority = next_account_info(account_info_iter)?;


    assert_pubkeys_exactitude(payer_account_info.key, &authorized::id()).and(assert_is_signer(payer_account_info))?;

    let (xp_mint_address, _xp_mint_bump) = Pubkey::find_program_address(&[XP_MINT_KEY.as_ref()], program_id);
    assert_pubkeys_exactitude(&xp_mint_address, mint_account_info.key).unwrap();

    let (expected_user_data_key, _expected_user_data_bump) = Pubkey::find_program_address(&[USERREGKEY.as_ref(), user_name.as_slice()], program_id);
    assert_pubkeys_exactitude(&expected_user_data_key, user_data_info.key).unwrap();

    let expected_token_account_info = get_associated_token_address(user_account_info.key, &xp_mint_address);
    assert_pubkeys_exactitude(&expected_token_account_info, user_token_account_info.key).unwrap();
    let (expected_general_authority_key, general_authority_bump) = Pubkey::find_program_address(&[GENERAL_AUTHORITY_KEY.as_ref()], program_id);
    assert_pubkeys_exactitude(&expected_general_authority_key, general_authority.key).unwrap();

    invoke_signed(
        &spl_token::instruction::thaw_account(spl_token_program_account_info.key, user_token_account_info.key, mint_account_info.key, general_authority.key, &[])?,
        &[user_token_account_info.clone(), mint_account_info.clone(), general_authority.clone()],
        &[&[GENERAL_AUTHORITY_KEY.as_ref(), &[general_authority_bump]]]
    )?;

    invoke_signed(
        &spl_token::instruction::mint_to(&spl_token::id(), mint_account_info.key, user_token_account_info.key, general_authority.key, &[], total_xp as u64)?,
        &[mint_account_info.clone(), user_token_account_info.clone(), general_authority.clone()],
        &[&[GENERAL_AUTHORITY_KEY.as_ref(), &[general_authority_bump]]]
    )?;

    invoke_signed(
        &spl_token::instruction::freeze_account(spl_token_program_account_info.key, user_token_account_info.key, mint_account_info.key, general_authority.key, &[])?,
        &[user_token_account_info.clone(), mint_account_info.clone(), general_authority.clone()],
        &[&[GENERAL_AUTHORITY_KEY.as_ref(), &[general_authority_bump]]]
    )?;

    let mut user_data = UserData::decode(user_data_info)?;

    assert_pubkeys_exactitude(&Pubkey::from(user_data.user_address), user_account_info.key ).unwrap();
    user_data.total_xp = user_data.total_xp.checked_add(total_xp).unwrap();
    if user_data.rank != Rank::xp_to_rank(user_data.total_xp){
        user_data.rank = Rank::xp_to_rank(user_data.total_xp);
        //do more here to mint soulbound badge
    }

    user_data.serialize(&mut &mut user_data_info.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_init_configuration(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account_info = next_account_info(account_info_iter)?;
    let mint_account_info = next_account_info(account_info_iter)?;
    // let system_program_account_info = next_account_info(account_info_iter)?;
    let spl_token_program_account_info = next_account_info(account_info_iter)?;
    let general_authority = next_account_info(account_info_iter)?;
    let sysvar_rent_account_info = next_account_info(account_info_iter)?;

    let (xp_mint_address, xp_mint_bump) = Pubkey::find_program_address(&[XP_MINT_KEY.as_ref()], program_id);
    assert_pubkeys_exactitude(&xp_mint_address, mint_account_info.key).unwrap();

    let (expected_general_authority_key, _general_authority_bump) = Pubkey::find_program_address(&[GENERAL_AUTHORITY_KEY.as_ref()], program_id);
    assert_pubkeys_exactitude(&expected_general_authority_key, general_authority.key).unwrap();

    let space = 82;
    let rent_lamports = Rent::get()?.minimum_balance(space);

    // msg!("Create mint account");
    invoke_signed(
        &system_instruction::create_account(
            payer_account_info.key,
            mint_account_info.key,
            rent_lamports,
            space as u64,
            spl_token_program_account_info.key,
        ),
        &[payer_account_info.clone(), mint_account_info.clone()],
        &[&[XP_MINT_KEY.as_ref(), &[xp_mint_bump]]],
    )?;
    
    invoke(
        &spl_token::instruction::initialize_mint(
            &spl_token::id(),
            mint_account_info.key,
            &general_authority.key,
            Some(&general_authority.key),
            0,
        )?,
        &[mint_account_info.clone(), sysvar_rent_account_info.clone()],
    )?;

    Ok(())
}

pub fn process_register_user(program_id: &Pubkey, accounts: &[AccountInfo], user_name: Vec<u8>) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account_info = next_account_info(account_info_iter)?;
    let user_account_info = next_account_info(account_info_iter)?;
    let user_data_info = next_account_info(account_info_iter)?;
    let user_token_account_info = next_account_info(account_info_iter)?;
    let mint_account_info = next_account_info(account_info_iter)?;
    let system_program_account_info = next_account_info(account_info_iter)?;
    let spl_token_program_account_info = next_account_info(account_info_iter)?;
    let freeze_authority = next_account_info(account_info_iter)?;


    assert_pubkeys_exactitude(payer_account_info.key, &authorized::id()).and(assert_is_signer(payer_account_info))?;

    let (expected_user_data_key, expected_user_data_bump) = Pubkey::find_program_address(&[USERREGKEY.as_ref(), user_name.as_slice()], program_id);
    assert_pubkeys_exactitude(&expected_user_data_key, user_data_info.key).unwrap();

    let (xp_mint_address, _xp_mint_bump) = Pubkey::find_program_address(&[XP_MINT_KEY.as_ref()], program_id);
    assert_pubkeys_exactitude(&xp_mint_address, mint_account_info.key).unwrap();

    let expected_token_account_info = get_associated_token_address(user_account_info.key, &xp_mint_address);
    assert_pubkeys_exactitude(&expected_token_account_info, user_token_account_info.key).unwrap();
    let space = 50;

    let (expected_freeze_authority_key, freeze_authority_bump) = Pubkey::find_program_address(&[GENERAL_AUTHORITY_KEY.as_ref()], program_id);
    assert_pubkeys_exactitude(&expected_freeze_authority_key, freeze_authority.key).unwrap();

    invoke_signed(
        &system_instruction::create_account(payer_account_info.key, user_data_info.key,  Rent::get()?.minimum_balance(space), space as u64, program_id),
        &[payer_account_info.clone(), user_data_info.clone()],
        &[&[USERREGKEY.as_ref(), user_name.as_slice(), &[expected_user_data_bump]]]
    )?;
    let user_data = UserData{
        total_xp: 0,
        user_address: user_account_info.key.to_bytes(),
        rank: Rank::None,
        registration_date: Clock::get()?.unix_timestamp as u32,
        struct_key: USERDATA_STRUCT_KEY,
    };
    user_data.serialize(&mut &mut user_data_info.data.borrow_mut()[..])?;
    
    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer_account_info.key,
            user_account_info.key,
            mint_account_info.key,
            spl_token_program_account_info.key,
        ),
        &[
            payer_account_info.clone(),
            user_token_account_info.clone(),
            user_account_info.clone(),
            mint_account_info.clone(),
            system_program_account_info.clone(),
            spl_token_program_account_info.clone(),
        ]
    )?;

    invoke_signed(
        &spl_token::instruction::freeze_account(spl_token_program_account_info.key, user_token_account_info.key, mint_account_info.key, freeze_authority.key, &[])?,
        &[user_token_account_info.clone(), mint_account_info.clone(), freeze_authority.clone()],
        &[&[GENERAL_AUTHORITY_KEY.as_ref(), &[freeze_authority_bump]]]
    )?;


    Ok(())
}

