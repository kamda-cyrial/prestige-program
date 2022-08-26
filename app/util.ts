import * as anchor from "@project-serum/anchor";
import { AnchorWallet } from "@solana/wallet-adapter-react";``
import * as constants from './const';
import { SeedUtil } from './seed-util';


/**
 * Builds the Anchor configs from the IDL
 * @param wallet 
 * @returns Provider & Program objects
 */
export async function getAnchorConfigs(
    wallet: AnchorWallet
): Promise<[anchor.AnchorProvider, anchor.Program, SeedUtil] | [null, null, null]> {

    if (!wallet) {
        return [null, null, null];
    }
    const provider = new anchor.AnchorProvider(
        new anchor.web3.Connection(constants.NETWORK, constants.PREFLIGHT_COMMITMENT), 
        wallet, 
        { "preflightCommitment": constants.PREFLIGHT_COMMITMENT }
    );
    const idl = require("./idl.json");
    const program = new anchor.Program(idl, idl.metadata.address, provider);
    let seedUtil = new SeedUtil(program);
    await seedUtil.init(wallet.publicKey);
    return [provider, program, seedUtil];
}

/**
 * Initializes the program
 * @param masterWallet 
 * @returns Provider & Transaction for use in Wallet Adapter
 */
export async function init(
    masterWallet: AnchorWallet,
): Promise<[anchor.web3.Transaction, anchor.AnchorProvider]> {
    
    const [provider, program, seedUtil] = await getAnchorConfigs(masterWallet);
    if (!provider) throw("Provider is null");
    const ix = await program.methods.init()
        .accounts({
            prestigeMintAuthority: seedUtil.prestigeMintAuthorityPda,
            prestigeXpMint: seedUtil.prestigeXpMint,
            prestigeXpMintMetadata: seedUtil.prestigeXpMintMetadata,
            payer: masterWallet.publicKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            mplTokenMetadataProgram: constants.TOKEN_METADATA_PROGRAM_ID,
        })
        .instruction();
    let tx = new anchor.web3.Transaction().add(ix);
    return [tx, provider];
};


/**
 * Creates an account for a user
 * @param wallet 
 * @param testGitHubId 
 * @param testFirstName 
 * @param testLastName 
 * @param testSchool 
 * @returns Provider & Transaction for use in Wallet Adapter
 */
export async function createUserAccount(
    wallet: AnchorWallet,
    testGitHubId: String,
    testFirstName: String,
    testLastName: String,
    testSchool: String,
): Promise<[anchor.web3.Transaction, anchor.AnchorProvider]> {

    const [provider, program, seedUtil] = await getAnchorConfigs(wallet);
    if (!provider) throw("Provider is null");
    const ix = await program.methods.createUserAccount(
        testGitHubId,
        testFirstName,
        testLastName,
        testSchool,
    )
        .accounts({
            prestigeMintAuthority: seedUtil.prestigeMintAuthorityPda,
            prestigeXpMint: seedUtil.prestigeXpMint,
            prestigeXpTokenAccount: await seedUtil.getXpTokenAccount(wallet.publicKey),
            prestigeUserData: await seedUtil.getUserDataPda(wallet.publicKey),
            payer: wallet.publicKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        })
        .instruction();
    let tx = new anchor.web3.Transaction().add(ix);
    return [tx, provider];
};

/**
 * Mints XP tokens to a user's wallet
 * @param wallet 
 * @param amount 
 * @returns Provider & Transaction for use in Wallet Adapter
 */
export async function mintXp(
    wallet: AnchorWallet,
    amount: number,
): Promise<[anchor.web3.Transaction, anchor.AnchorProvider]> {

    const [provider, program, seedUtil] = await getAnchorConfigs(wallet);
    if (!provider) throw("Provider is null");
    const ix = await program.methods.mintXp(new anchor.BN(amount))
        .accounts({
            prestigeMintAuthority: seedUtil.prestigeMintAuthorityPda,
            prestigeXpMint: seedUtil.prestigeXpMint,
            prestigeXpTokenAccount: await seedUtil.getXpTokenAccount(wallet.publicKey),
            prestigeUserData: await seedUtil.getUserDataPda(wallet.publicKey),
            payer: wallet.publicKey,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .instruction();
    let tx = new anchor.web3.Transaction().add(ix);
    return [tx, provider];
};
