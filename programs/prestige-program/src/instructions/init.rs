use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke_signed,
    },
    anchor_spl::{
        token,
        associated_token,
    },
    mpl_token_metadata::instruction as mpl_instruction,
};

use crate::state::PrestigeMintAuthority;
use crate::state::PrestigeXpMint;

// This instruction will create:
//  - The Prestige Mint Authority PDA
//  - The XP Token Mint
//  - The XP Token Mint's Metadata
//
// Payer should be the Master Wallet

pub fn init(
    ctx: Context<Init>
) -> Result<()> {

    let prestige_mint_authority = PrestigeMintAuthority::new(
        *ctx.bumps.get(PrestigeMintAuthority::SEED_PREFIX).expect("Bump not found."),
        *ctx.bumps.get(PrestigeXpMint::SEED_PREFIX).expect("Bump not found."),
    );
    ctx.accounts.prestige_mint_authority.set_inner(prestige_mint_authority.clone());

    // Create XP Token Mint's Metadata
    //
    invoke_signed(
        &mpl_instruction::create_metadata_accounts_v2(
            ctx.accounts.mpl_token_metadata_program.key(),
            ctx.accounts.prestige_xp_mint_metadata.key(),
            ctx.accounts.prestige_xp_mint.key(),
            ctx.accounts.prestige_mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.prestige_mint_authority.key(),
            PrestigeXpMint::TITLE.to_string(),
            PrestigeXpMint::SYMBOL.to_string(),
            PrestigeXpMint::URI.to_string(),
            None,
            0,
            true,
            false,
            None,
            None,
        ),
        &[
            ctx.accounts.prestige_xp_mint_metadata.to_account_info(),
            ctx.accounts.prestige_xp_mint.to_account_info(),
            ctx.accounts.prestige_mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.prestige_mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[
            &[
                PrestigeMintAuthority::SEED_PREFIX.as_bytes().as_ref(),
                &[*ctx.bumps.get(PrestigeMintAuthority::SEED_PREFIX).expect("Bump not found.")],
            ]
        ]
    )?;
    
    Ok(())
}


#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init, 
        payer = payer,
        space = PrestigeMintAuthority::ACCOUNT_SPACE,
        seeds = [PrestigeMintAuthority::SEED_PREFIX.as_bytes().as_ref()],
        bump
    )]
    prestige_mint_authority: Account<'info, PrestigeMintAuthority>,
    #[account(
        init,
        payer = payer,
        seeds = [PrestigeXpMint::SEED_PREFIX.as_bytes().as_ref() ],
        bump,
        mint::decimals = PrestigeXpMint::DECIMALS,
        mint::authority = prestige_mint_authority.key(),
    )]
    prestige_xp_mint: Account<'info, token::Mint>,
    #[account(mut)]
    /// CHECK: Metaplex will check this
    prestige_xp_mint_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>, // Master Wallet
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    mpl_token_metadata_program: UncheckedAccount<'info>,
}