import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EternitySc } from "../target/types/eternity_sc";
import { base64, bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { createWallet } from "./helper";

describe("Relic Test", () => {
  let program: Program<EternitySc>;
  let wallet: Keypair;
  let provider: anchor.Provider;

  async function getStoragePointer(relicId: number, fragmentsId: number) {
    const data = await program.account.fragments.all([
      {
        memcmp: {
          offset: 8, // Offset for the owner field
          bytes: wallet.publicKey.toBase58(),
        },
      },
      {
        memcmp: {
          offset: 40, // Offset for the lockerId field
          bytes: bs58.encode(Buffer.from(Uint32Array.of(relicId).buffer)),
        },
      },
      {
        memcmp: {
          offset: 44, // Offset for the spId field
          bytes: bs58.encode(Buffer.from(Uint32Array.of(fragmentsId).buffer)),
        },
      },
    ]);

    if (data.length === 0) {
      throw new Error("StoragePointer not found");
    }

    return data[0].account;
  }

  it("Init", async () => {
    anchor.setProvider({
      connection: new Connection("http://localhost:8899", "confirmed"),
    });
    program = anchor.workspace.EternitySc as Program<EternitySc>;
    provider = anchor.getProvider();

    wallet = await createWallet(10, provider);
  });

  it("Create Relic", async () => {
    const relicId = 1;
    const name = "Relic for fragment";
    const description = "This relic is used for testing fragments";

    const tx = await program.methods
      .createRelic(relicId, name, description)
      .accounts({
        signer: wallet.publicKey,
        authority: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);
  });

  it("Update Relic", async () => {
    const relicId = 1;
    const name = "Relic for fragment updated";
    const description = "This relic is used for testing fragments updated";

    const tx = await program.methods
      .updateRelic(relicId, name, description, false)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);
  })

  it("Create Fragments", async () => {
    const relicId = 1;
    const fragmentsId = 1;

    const tx = await program.methods
      .createFragments(relicId, fragmentsId)
      .accounts({
        signer: wallet.publicKey,
        oldFragments: Keypair.generate().publicKey,
      })
      .signers([wallet])
      .rpc();

      
      console.log("Transaction Signature: ", tx);
    })
    
    it("Add Fragments", async () => {
      const relicId = 1;
      const fragmentsId = 1;

      const tx1 = await program.methods.mAddFragment(relicId, fragmentsId, Array.from(Keypair.generate().publicKey.toBuffer())).accounts({
        signer: wallet.publicKey
      }).signers([wallet]).rpc()
      const tx2 = await program.methods.mAddFragment(relicId, fragmentsId, Array.from(Keypair.generate().publicKey.toBuffer())).accounts({
        signer: wallet.publicKey
      }).signers([wallet]).rpc()
      const tx3 = await program.methods.mAddFragment(relicId, fragmentsId, Array.from(Keypair.generate().publicKey.toBuffer())).accounts({
        signer: wallet.publicKey
      }).signers([wallet]).rpc()
      const tx4 = await program.methods.mAddFragment(relicId, fragmentsId, Array.from(Keypair.generate().publicKey.toBuffer())).accounts({
        signer: wallet.publicKey
      }).signers([wallet]).rpc()

    console.log("Transactions Signature: ", {
      tx1,
      tx2,
      tx3,
      tx4
    });
  })

  it("Delete Fragments", async () => {
    const relicId = 1;
    const fragmentsId = 1;

    const tx = await program.methods.mDeleteFragment(relicId, fragmentsId, 1).accounts({
      signer: wallet.publicKey
    }).signers([wallet]).rpc()

    console.log("Transactions Signature: ", tx);
  })

  it("Update Fragments", async () => {
    const relicId = 1;
    const fragmentsId = 1;

    const tx = await program.methods.mUpdateFragment(relicId, fragmentsId, 1, Array.from(Keypair.generate().publicKey.toBuffer())).accounts({
      signer: wallet.publicKey
    }).signers([wallet]).rpc()

    console.log("Transactions Signature: ", tx);
  })
});