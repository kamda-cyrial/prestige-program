use anchor_lang::prelude::*;


// The Metadata for our XP token.

pub struct PrestigeXpMint {}

impl PrestigeXpMint {
    pub const SEED_PREFIX: &'static str = "prestige_xp_mint";
    pub const DECIMALS: u8 = 9;

    pub const TITLE: &'static str = "Prestige DAO XP Token";
    pub const SYMBOL: &'static str = "XP";
    pub const URI: &'static str = "https://raw.githubusercontent.com/PrestigeDAO/prestige-program/assets/xp.json";
}


// Our Mint Authority will be a PDA.
// This way, we don't need any kind of Master Wallet to
//      sign off on minting

#[account]
pub struct PrestigeMintAuthority {
    pub bump: u8,
    pub xp_mint_bump: u8,
}

impl PrestigeMintAuthority {
    pub const ACCOUNT_SPACE: usize = 8 + 8;
    pub const SEED_PREFIX: &'static str = "prestige_mint_authority";

    pub fn new(
        bump: u8,
        xp_mint_bump: u8,
    ) -> Self {
        return PrestigeMintAuthority { 
            bump,
            xp_mint_bump,
        }
    }
}