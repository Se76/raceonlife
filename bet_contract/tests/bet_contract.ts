import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BetContract } from "../target/types/bet_contract";
import { BN } from "bn.js";
import wallet from "../test-keypair.json";

import {
  Keypair,
  Connection,
  Commitment,
} from "@solana/web3.js";

// const INITIALIZER_KEYPAIR = anchor.web3.Keypair.fromSecretKey(wallet);
const INITIALIZER_KEYPAIR = Keypair.fromSecretKey(new Uint8Array(wallet));
// const keypair = Keypair.fromSecretKey(new Uint8Array(INITIALIZER_KEYPAIR.secretKey));

const initial_wallet = anchor.Wallet.local();

const commitment: Commitment = "confirmed";
const connection = new Connection("http://127.0.0.1:8899", commitment);

const provider = anchor.AnchorProvider.env();
// const foo = async () => {
//   console.log("airdrop");
//   // await connection.requestAirdrop(
//   //   INITIALIZER_KEYPAIR.publicKey,
//   //   100_000_000_000 // 100 SOL
//   // );
//   const signature = await connection.requestAirdrop(INITIALIZER_KEYPAIR.publicKey, 100_000_000_000);
//   console.log("airdrop done");
// };
  

//   const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

//   async function sleepyWork() {
//     console.log("I'm going to sleep for 1 second.");
//     await sleep(1000);
//     console.log("I woke up after 1 second.");
//   }

  // const requestAirdrop = async (publicKey: anchor.web3.PublicKey, lamports: number) => {
  //   console.log("Requesting airdrop...");
  //   const signature = await connection.requestAirdrop(publicKey, lamports);
  //   console.log(`Airdrop of ${lamports / 1_000_000_000} SOL complete.`);
  // };


describe("bet_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.BetContract as Program<BetContract>;

  it("Is initialized!", async () => {
    console.log(INITIALIZER_KEYPAIR.publicKey);
    
    // await connection.requestAirdrop(INITIALIZER_KEYPAIR.publicKey, 100_000_000_000);
    const tx = await program.methods.initialize(new BN(1), new BN(1)).accountsPartial(
      {
        initializer: INITIALIZER_KEYPAIR.publicKey,
      }
    ).signers([INITIALIZER_KEYPAIR])
    .rpc();
    // 
    console.log("Your transaction signature", tx);
  });
  
});
