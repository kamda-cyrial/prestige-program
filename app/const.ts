import * as anchor from "@project-serum/anchor";


export const MASTER_WALLET = new anchor.Wallet(
    anchor.web3.Keypair.fromSecretKey(
        Buffer.from(
            JSON.parse(require('fs').readFileSync(
                __dirname + '/../wallet/master.json', 
                "utf-8"
)))));

export const NETWORK = "https://api.devnet.solana.com";
// export const NETWORK = "http://localhost:8899";
export const PREFLIGHT_COMMITMENT = "confirmed";
export const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const MINT_AUTHORITY_SEED = "prestige_mint_authority";
export const XP_MINT_SEED = "prestige_xp_mint";
export const PRESTIGE_USER_DATA_SEED = "prestige_user_data";