import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EternitySc } from "../target/types/eternity_sc";
import { base64, bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { createWallet } from "./helper";

describe("Profile Test", () => {
  let program: anchor.Program<EternitySc>;
  let wallet: Keypair;
  let provider: anchor.Provider;

  async function getProfile() {
    const [profilePda, _] = await PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), wallet.publicKey.toBuffer()],
      program.programId
    );
    return await program.account.profile.fetch(profilePda);
  }

  it("Init", async () => {
    anchor.setProvider({
      connection: new Connection("http://localhost:8899", "confirmed"),
    });
    program = anchor.workspace.EternitySc as anchor.Program<EternitySc>;
    provider = anchor.getProvider();

    wallet = await createWallet(10, provider);
  });

  it("Create Profile", async () => {
    const name = "Abyan";
    const age = 20;
    const hobbie = ["Coding", "Gaming"];
    const message = "Hello World!";

    const tx = await program.methods
      .createProfile(name, age, hobbie, message)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const profile = await getProfile();
    assert.equal(profile.owner.toBase58(), wallet.publicKey.toBase58(), "Owner mismatch");
    assert.equal(profile.name, name, "Name mismatch");
    assert.equal(profile.age, age, "Age mismatch");
    assert.deepEqual(profile.hobbie, hobbie, "Hobbie mismatch");
    assert.equal(profile.message, message, "Message mismatch");
  });

  it("Update Profile", async () => {
    const name = "AbyanFun";
    const age = 21;
    const hobbie = ["Coding", "Gaming", "Tidur"];
    const message = "Hello World!";

    const tx = await program.methods
      .updateProfile(name, age, hobbie, message)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const profile = await getProfile();
    assert.equal(profile.owner.toBase58(), wallet.publicKey.toBase58(), "Owner mismatch");
    assert.equal(profile.name, name, "Name mismatch");
    assert.equal(profile.age, age, "Age mismatch");
    assert.deepEqual(profile.hobbie, hobbie, "Hobbie mismatch");
    assert.equal(profile.message, message, "Message mismatch");
  });

  it("Profile Validation", async () => {
    const testCases = [
      { name: "A".repeat(101), age: 20, hobbie: ["Coding"], message: "Hello", error: "Name exceeds max length" },
      { name: "Abyan", age: 65536, hobbie: ["Coding"], message: "Hello", error: "Age exceeds max value" },
      { name: "Abyan", age: 20, hobbie: Array(6).fill("Hobby"), message: "Hello", error: "Hobbie exceeds max count" },
      { name: "Abyan", age: 20, hobbie: ["Coding"], message: "C".repeat(301), error: "Message exceeds max length" },
    ];

    for (const testCase of testCases) {
      const tempWallet = await createWallet(1, provider);

      try {
        await program.methods
          .createProfile(testCase.name, testCase.age, testCase.hobbie, testCase.message)
          .accounts({
            signer: tempWallet.publicKey,
          })
          .signers([tempWallet])
          .rpc();

        console.log(false, `Validation failed: ${testCase.error}`);
      } catch (err) {
        console.log(true, `Validation passed: ${testCase.error}`);
      }
    }
  });
});