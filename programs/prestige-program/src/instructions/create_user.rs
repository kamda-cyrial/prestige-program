use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    associated_token,
};

use crate::state::PrestigeMintAuthority;
use crate::state::PrestigeUserData;
use crate::state::PrestigeXpMint;

// This instruction will create a user's data account

pub fn create_user_account(
    ctx: Context<CreateUserAccount>,
    github_user_id: String,
    first_name: String,
    last_name: String,
    school: Option<String>,
) -> Result<()> {

    let prestige_user_data = PrestigeUserData::new(
        ctx.accounts.payer.key(),
        github_user_id,
        first_name,
        last_name,
        school,
        *ctx.bumps.get(PrestigeUserData::SEED_PREFIX).expect("Bump not found."),
    );
    ctx.accounts.prestige_user_data.set_inner(prestige_user_data.clone());

    Ok(())
}


#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    // We need the Mint Authority PDA to derive the Mint
    #[account(
        mut, 
        seeds = [PrestigeMintAuthority::SEED_PREFIX.as_bytes().as_ref()],
        bump = prestige_mint_authority.bump,
    )]
    prestige_mint_authority: Account<'info, PrestigeMintAuthority>,
    // We need the Mint to derive the Associated Token Account
    #[account(
        mut,
        seeds = [PrestigeXpMint::SEED_PREFIX.as_bytes().as_ref()],
        bump = prestige_mint_authority.xp_mint_bump,
        mint::decimals = PrestigeXpMint::DECIMALS,
        mint::authority = prestige_mint_authority.key(),
    )]
    prestige_xp_mint: Account<'info, token::Mint>,
    // Initialize the user's XP Token Associated Token Account
    #[account(
        init,
        payer = payer,
        associated_token::mint = prestige_xp_mint,
        associated_token::authority = payer,
    )]
    prestige_xp_token_account: Account<'info, token::TokenAccount>,
    // Initializes the User Data PDA
    #[account(
        init,
        payer = payer,
        space = PrestigeUserData::ACCOUNT_SPACE,
        seeds = [
            PrestigeUserData::SEED_PREFIX.as_bytes().as_ref(),
            payer.key().as_ref(),
        ],
        bump
    )]
    prestige_user_data: Account<'info, PrestigeUserData>,
    #[account(mut)]
    payer: Signer<'info>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
