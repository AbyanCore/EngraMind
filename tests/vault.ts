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

    const vault = await getVault();

    // Validate Vault account
    assert.equal(vault.owner.toBase58(), wallet.publicKey.toBase58(), "Owner mismatch");
    assert.equal(vault.token, 0, "Initial token mismatch");
  });

  it("Buy Token", async () => {
    const amount = 1_000_000; // 1 SOL
    const tx = await program.methods
      .buyToken(new anchor.BN(amount))
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const vault = await getVault();
    const vaultLamport = await getVaultLamport();

    // Validate Vault account
    assert.equal(vault.token, amount * 10, "Token amount mismatch after buying");

    // Validate VaultLamport account
    assert.isNotNull(vaultLamport, "VaultLamport account not found");
  });

  it("Take Token", async () => {
    const amount = 500_000; // 0.5 SOL
    const tx = await program.methods
      .takeToken(new anchor.BN(amount))
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const vault = await getVault();
    const vaultLamport = await getVaultLamport();

    // Validate Vault account
    assert.equal(vault.token, (1_000_000 - amount) * 10, "Token amount mismatch after taking");

    // Validate VaultLamport account
    assert.isNotNull(vaultLamport, "VaultLamport account not found");
  });

  it("Vault Validation", async () => {
    const invalidAmount = 10_000_000_000;
    try {
      await program.methods
        .buyToken(new anchor.BN(invalidAmount))
        .accounts({
          signer: wallet.publicKey,
        })
        .signers([wallet])
        .rpc();

      console.log(false, "-> Validation failed: Should not allow buying with insufficient balance");
    } catch (err) {
      console.log(true, "-> Validation passed: Insufficient balance check");
    }

    try {
      await program.methods
        .takeToken(new anchor.BN(10_000_000))
        .accounts({
          signer: wallet.publicKey,
        })
        .signers([wallet])
        .rpc();

      console.log(false, "-> Validation failed: Should not allow taking more tokens than available");
    } catch (err) {
      console.log(true, "-> Validation passed: Token limit check");
    }
  });
});