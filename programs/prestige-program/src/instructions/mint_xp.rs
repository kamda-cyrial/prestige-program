use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::state::PrestigeMintAuthority;
use crate::state::PrestigeUserData;
use crate::state::PrestigeXpMint;

// This instruction will mint XP tokens to a user

pub fn mint_xp(
    ctx: Context<MintXp>,
    amount: u64,
) -> Result<()> {

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.prestige_xp_mint.to_account_info(),
                to: ctx.accounts.prestige_xp_token_account.to_account_info(),
                authority: ctx.accounts.prestige_mint_authority.to_account_info(),
            },
            &[&[
                PrestigeMintAuthority::SEED_PREFIX.as_bytes().as_ref(), 
                &[ctx.accounts.prestige_mint_authority.bump],
            ]]
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintXp<'info> {
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
    // User's XP Token Associated Token Account
    #[account(
        mut,
        associated_token::mint = prestige_xp_mint,
        associated_token::authority = prestige_user_data.wallet_pubkey,
    )]
    prestige_xp_token_account: Account<'info, token::TokenAccount>,
    // We need the User Data PDA to get their wallet pubkey
    #[account(
        mut,
        seeds = [
            PrestigeUserData::SEED_PREFIX.as_bytes().as_ref(),
            prestige_user_data.wallet_pubkey.as_ref(),
        ],
        bump = prestige_user_data.bump,
    )]
    prestige_user_data: Account<'info, PrestigeUserData>,
    #[account(mut)]
    payer: Signer<'info>,
    token_program: Program<'info, token::Token>,
}