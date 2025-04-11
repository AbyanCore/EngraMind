import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EternitySc } from "../target/types/eternity_sc";
import { base64, bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { createWallet } from "./helper";

describe("StoragePointer Test", () => {
  let program: Program<EternitySc>;
  let wallet: Keypair;
  let provider: anchor.Provider;

  async function getStoragePointer(lockerId: number, spId: number) {
    const data = await program.account.storagePointer.all([
      {
        memcmp: {
          offset: 8, // Offset for the owner field
          bytes: wallet.publicKey.toBase58(),
        },
      },
      {
        memcmp: {
          offset: 40, // Offset for the lockerId field
          bytes: bs58.encode(Buffer.from(Uint32Array.of(lockerId).buffer)),
        },
      },
      {
        memcmp: {
          offset: 44, // Offset for the spId field
          bytes: bs58.encode(Buffer.from(Uint32Array.of(spId).buffer)),
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

  it("Create Locker for StoragePointer", async () => {
    const lockerId = 1;
    const name = "Locker for SP";
    const description = "This locker is used for testing StoragePointer";

    const tx = await program.methods
      .createLocker(lockerId, name, description)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);
  });

  it("Create StoragePointer", async () => {
    const lockerId = 1;
    const spId = 1;

    const existingSPs = await program.account.storagePointer.all([
      {
        memcmp: {
          offset: 8, // Offset for the owner field
          bytes: wallet.publicKey.toBase58(),
        },
      },
    ]);
    const oldStoragePointer = existingSPs.length > 0
      ? existingSPs[existingSPs.length - 1].publicKey
      : Keypair.generate().publicKey;

    const tx = await program.methods
      .createSp(lockerId, spId)
      .accounts({
        signer: wallet.publicKey,
        oldStoragePointer: oldStoragePointer,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const sp = await getStoragePointer(lockerId, spId);
    assert.equal(sp.owner.toBase58(), wallet.publicKey.toBase58(), "Owner mismatch");
    assert.equal(sp.lockerId, lockerId, "Locker ID mismatch");
    assert.equal(sp.id, spId, "StoragePointer ID mismatch");
    assert.equal(sp.data.length, 0, "Initial data length mismatch");
    assert.equal(sp.dataCount, 0, "Initial data count mismatch");
    assert.isNull(sp.nextSp, "Initial nextSp mismatch");
  });

  it("Add Data to StoragePointer", async () => {
    const lockerId = 1;
    const spId = 1;
    const key = Array.from(Keypair.generate().publicKey.toBuffer());

    const tx = await program.methods
      .addSp(lockerId, spId, key)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const sp = await getStoragePointer(lockerId, spId);
    assert.equal(sp.data.length, 1, "Data length mismatch after adding");
    assert.deepEqual(sp.data[0], key, "Data mismatch after adding");
    assert.equal(sp.dataCount, 1, "Data count mismatch after adding");
  });

  it("Update Data in StoragePointer", async () => {
    const lockerId = 1;
    const spId = 1;
    const key = Array.from(Keypair.generate().publicKey.toBuffer());
    const id = 0;

    const tx = await program.methods
      .updateSp(lockerId, spId, id, key)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const sp = await getStoragePointer(lockerId, spId);
    assert.deepEqual(sp.data[0], key, "Data mismatch after updating");
  });

  it("Delete Data from StoragePointer", async () => {
    const lockerId = 1;
    const spId = 1;
    const id = 0;

    const tx = await program.methods
      .deleteSp(lockerId, spId, id)
      .accounts({
        signer: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    console.log("Transaction Signature: ", tx);

    const sp = await getStoragePointer(lockerId, spId);
    assert.equal(sp.data.length, 0, "Data length mismatch after deleting");
    assert.equal(sp.dataCount, 0, "Data count mismatch after deleting");
  });

  it("Test Blockchain Effect (Chaining)", async () => {
    const lockerId = 1;
    const iterations = 5; // Number of StoragePointers to chain

    async function getSP(lockerId: number,spId: number ) {
      const data = await program.account.storagePointer.all([
        {
          memcmp: {
            offset: 8, // Offset for the owner field
            bytes: wallet.publicKey.toBase58(),
          },
        },
        {
          memcmp: {
            offset: 40, // Offset for the lockerId field
            bytes: bs58.encode(Buffer.from(Uint32Array.of(lockerId).buffer)),
          },
        },
        {
          memcmp: {
            offset: 44, // Offset for the spId field
            bytes: bs58.encode(Buffer.from(Uint32Array.of(spId).buffer)),
          },
        },
      ]);

      return data[0]
    }

    let previousSpId = 1;
    for (let i = previousSpId + 1; i <= previousSpId + iterations; i++) {
      const tx = await program.methods
        .createSp(lockerId, i)
        .accounts({
          signer: wallet.publicKey,
          oldStoragePointer: previousSpId === 0
            ? Keypair.generate().publicKey // Use a random key for the first SP
            : (await getSP(lockerId, previousSpId)).publicKey,
        })
        .signers([wallet])
        .rpc();

      console.log(`Transaction Signature for SP ${i}: `, tx);

      const sp = await getSP(lockerId, i);
      if (previousSpId !== 0) {
        const previousSp = await getSP(lockerId, previousSpId);
        assert.equal(
          previousSp.account.nextSp.toBase58(),
          sp.publicKey.toBase58(),
          `Chaining mismatch: SP ${previousSpId} does not point to SP ${i}`
        );
      }

      previousSpId = i;
    }
  });
});