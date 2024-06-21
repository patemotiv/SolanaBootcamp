import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorTest } from "../target/types/anchor_test";
import { Keypair, PublicKey, Connection, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { assert, expect } from 'chai'; // Importing the Chai assertion library
import { describe } from 'mocha'; // Importing the Mocha test framework

describe("anchor_test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorTest as Program<AnchorTest>;
  const connection = provider.connection;
  const userPubkey = Keypair.generate();
  const vaultPubkey = Keypair.generate();

  it("initialize", async () => {

    console.log("User address: ", userPubkey.publicKey.toString());
    console.log("Vault address: ", vaultPubkey.publicKey.toString());

    // Send some SOL to the user
    const airdropSignature = await connection.requestAirdrop(userPubkey.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature, "confirmed");

    // Check the balance of the user
    const balance = await connection.getBalance(userPubkey.publicKey);
    console.log("User balance: ", balance);

    // Call the initialized method
    await program.methods.initialize()
      .accounts(
        {
          vault: vaultPubkey.publicKey,
          user: userPubkey.publicKey,
          systemProgram: SystemProgram.programId,
        }
      )
      .signers([userPubkey, vaultPubkey])
      .rpc();

    // Check if the Vault is initialized with the correct balance of 100
    const vault = await program.account.vault.fetch(vaultPubkey.publicKey);
    expect(vault.balance.toNumber()).to.equal(100);
  });

  it("deposit 100", async () => {
    // Call the deposit method
    await program.methods.deposit(new anchor.BN(100))
      .accounts({
        vault: vaultPubkey.publicKey,
      })
      .signers([])
      .rpc();

    // Check if the Vault is updated with the correct balance of 200
    const vault = await program.account.vault.fetch(vaultPubkey.publicKey);
    expect(vault.balance.toNumber()).to.equal(200);
  });

  it("deposit 1000", async () => {

    try {
      // The deposit should fail because the balance is greater than 100
      await program.methods.deposit(new anchor.BN(1000))
        .accounts({
          vault: vaultPubkey.publicKey,
        })
        .signers([])
        .rpc();

      assert.fail("Expected error not thrown");
    }
    catch (error) {
      const err = anchor.AnchorError.parse(error.logs);
      expect(err.error.errorCode.code).to.equal("VaultBalanceExceeds1000");
    }
  });
});
