import * as anchor from '@project-serum/anchor';
import * as constants from '../app/const';
import * as util from '../app/util';


const connection: anchor.web3.Connection = new anchor.web3.Connection(
    constants.NETWORK, 
    constants.PREFLIGHT_COMMITMENT
);

let provider: anchor.AnchorProvider;
let program: anchor.Program;
let testWallet: anchor.Wallet;

const testGitHubId = "some_github_id_00x0";
const testFirstName = "Johnny";
const testLastName = "Solana";
const testSchool = "USC";

const testXpAmount = 100;


describe("Prestige DAO", async () => {

    before("Prepare a test wallet", async () => {
        testWallet = await primeNewWallet("Test Wallet");
    });

    it("Initialize the Prestige Program", async () => {
        await anchor.web3.sendAndConfirmTransaction(
            connection,
            (await util.init(
                constants.MASTER_WALLET
            ))[0],
            [constants.MASTER_WALLET.payer]
        );
    });

    it("Create a User Account", async () => {
        await anchor.web3.sendAndConfirmTransaction(
            connection,
            (await util.createUserAccount(
                testWallet,
                testGitHubId,
                testFirstName,
                testLastName,
                testSchool,

            ))[0],
            [testWallet.payer]
        );
        await printUserData(testWallet.publicKey);
    });

    it("Mint XP to user", async () => {
        await anchor.web3.sendAndConfirmTransaction(
            connection,
            (await util.mintXp(
                testWallet,
                testXpAmount,

            ))[0],
            [testWallet.payer]
        );
        await printXpBalance(testWallet.publicKey);
    });

    async function primeNewWallet(walletName: string) {
        const keypair = anchor.web3.Keypair.generate();
        await connection.confirmTransaction(
          await connection.requestAirdrop(keypair.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
        );
        const balance = await connection.getBalance(keypair.publicKey);
        console.log(`${walletName}: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
        console.log(`Pubkey: ${keypair.publicKey}`);
        return new anchor.Wallet(keypair);
    }

    async function printUserData(address: anchor.web3.PublicKey) {
        const userData = await program.account.userData.fetch(address);
        console.log(`Profile        : ${address}`);
        console.log(`   GitHub ID   : ${userData.gitHubId}`);
        console.log(`   First Name  : ${userData.firstName}`);
        console.log(`   Last Name   : ${userData.lastName}`);
        console.log(`   School      : ${userData.school}`);
    };

    async function printXpBalance(address: anchor.web3.PublicKey) {
        // ?
    };
});