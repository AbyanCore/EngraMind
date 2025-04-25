import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EternitySc } from "../target/types/eternity_sc";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { createWallet } from "./helper";

describe("Vault Test", () => {
  let program: Program<EternitySc>;
  let wallet: Keypair;
  let provider: anchor.Provider;

  async function getVault() {
    const [vaultPda, _] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), wallet.publicKey.toBuffer()],
      program.programId
    );
    return await program.account.vault.fetch(vaultPda);
  }

  async function getVaultLamport() {
    const [vaultLamportPda, _] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault_lamport"), wallet.publicKey.toBuffer()],
      program.programId
    );
    return await provider.connection.getAccountInfo(vaultLamportPda);
  }

  it("Init", async () => {
    anchor.setProvider({
      connection: new Connection("http://localhost:8899", "confirmed"),
    });
    program = anchor.workspace.EternitySc as Program<EternitySc>;
    provider = anchor.getProvider();

    wallet = await createWallet(10, provider);
  });

  it("Create Vault", async () => {
    const tx = await program.methods
      .createVault()
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);
  });

  it("Buy Token", async () => {
    const tx = await program.methods.mBuyToken(
        new anchor.BN(100)
      ).accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
    
    console.log("Transaction Signature: ", tx);
  })

  it("Take Token", async () => {
    const tx = await program.methods.mTakeToken(
        new anchor.BN(99)
      ).accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
    
    console.log("Transaction Signature: ", tx);
  })
});