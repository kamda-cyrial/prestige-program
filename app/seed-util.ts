import * as anchor from "@project-serum/anchor";
import * as constants from './const';


export class SeedUtil {

    program: anchor.Program;
    prestigeMintAuthorityPda: anchor.web3.PublicKey;
    prestigeXpMint: anchor.web3.PublicKey;
    prestigeXpMintMetadata: anchor.web3.PublicKey;

    constructor(program: anchor.Program) {
        this.program = program;
    };

    async derivePda(seeds: Buffer[]) {
        return (await anchor.web3.PublicKey.findProgramAddress(
            seeds, this.program.programId
        ))[0]
    }

    async init(
        walletPubkey: anchor.web3.PublicKey,
    ) {
        this.prestigeMintAuthorityPda = await this.derivePda([
            Buffer.from(constants.MINT_AUTHORITY_SEED),
            walletPubkey.toBuffer(), 
        ]);
        this.prestigeXpMint = await this.derivePda([
            Buffer.from(constants.XP_MINT_SEED),
        ]);
        this.prestigeXpMintMetadata = await this.derivePda([
            Buffer.from("metadata"),
            constants.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            this.prestigeXpMint.toBuffer(),
        ]);
    }

    async getXpTokenAccount(
        walletPubkey: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> {
        return await anchor.utils.token.associatedAddress({
            mint: this.prestigeXpMint, 
            owner: walletPubkey,
        });
    }

    async getUserDataPda(
        walletPubkey: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> {
        return await this.derivePda([
            Buffer.from(constants.PRESTIGE_USER_DATA_SEED),
        ]);
    }

}