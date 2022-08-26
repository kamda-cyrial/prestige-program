use anchor_lang::prelude::*;


// Our User Data PDA.

#[account]
pub struct PrestigeUserData {
    pub wallet_pubkey: Pubkey,
    pub github_user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub school: Option<String>,
    pub bump: u8,
}

impl PrestigeUserData {
    pub const ACCOUNT_SPACE: usize = 8 + 32 + 40 + 40 + 40 + 40;
    pub const SEED_PREFIX: &'static str = "prestige_user_data";
    
    pub fn new(
        wallet_pubkey: Pubkey,
        github_user_id: String,
        first_name: String,
        last_name: String,
        school: Option<String>,
        bump: u8,
    ) -> Self {
        return PrestigeUserData { 
            wallet_pubkey,
            github_user_id,
            first_name,
            last_name,
            school,
            bump,
        }
    }
}
