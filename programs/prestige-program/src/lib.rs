use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;


declare_id!("DyiVPsExWgRDUk8dTNQB9bxirCC9e6C3cyhJN1MwQ7ac");


#[program]
pub mod prestige_program {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn create_user_account(
        ctx: Context<CreateUserAccount>,
        github_user_id: String,
        first_name: String,
        last_name: String,
        school: Option<String>,
    ) -> Result<()> {
        instructions::create_user_account(
            ctx,
            github_user_id,
            first_name,
            last_name,
            school,
        )
    }

    pub fn mint_xp(
        ctx: Context<MintXp>,
        amount: u64,
    ) -> Result<()> {
        instructions::mint_xp(
            ctx,
            amount,
        )
    }
}
