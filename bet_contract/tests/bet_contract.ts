import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BetContract } from "../target/types/bet_contract";
import { BN } from "bn.js";
import { Keypair, Connection, Commitment } from "@solana/web3.js";

const INITIALIZER_KEYPAIR = anchor.web3.Keypair.generate();
// const keypair = Keypair.fromSecretKey(new Uint8Array(INITIALIZER_KEYPAIR.secretKey));

const initial_wallet = anchor.Wallet.local();

const commitment: Commitment = "confirmed";
const connection = new Connection("http://127.0.0.1:8899", commitment);

connection.requestAirdrop(INITIALIZER_KEYPAIR.publicKey, 10000000000000);

describe("bet_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BetContract as Program<BetContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new BN(0.01), new BN(1)).accountsPartial(
      {
        initializer: initial_wallet.publicKey,
      }
    ).signers([initial_wallet.payer])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
