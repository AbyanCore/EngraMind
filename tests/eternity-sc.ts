import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EternitySc } from "../target/types/eternity_sc";
import { base64, bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("General Test", () => {
  
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env())
  
  let program: Program<EternitySc>; 
  let wallet: anchor.web3.PublicKey; 
  let provider: anchor.Provider;
  
  it("Initialize Provider", () => {
      anchor.setProvider(anchor.AnchorProvider.env());
      program = anchor.workspace.EternitySc as Program<EternitySc>;
  })
  
  it("Initialize Wallet", () => {
      provider = anchor.getProvider();
      wallet = provider.publicKey
    })
  
  // it("Check All Locker", async () => {
  //   const data = await program.account.locker.all()
    
  //   console.log("Locker: ", data)
  // })

  // it("Update Locker",  async () => {
  //   const tx = await program.methods.updateLocker(
  //     1,
  //     "test locker 1",
  //     "Test Locker Creation 1"
  //   ).rpc()

  //   console.log("Transaction Signature: ", tx)
  // })


  // it("Check All Locker", async () => {
  //   const data = await program.account.locker.all([
  //     {
  //       memcmp: {
  //         offset: 8,
  //         bytes: wallet.toBase58()
  //       }
  //     }
  //   ])
    
  //   console.log("Locker: ", data)
  // })
  
  // it("create SP", async () => {
  //   const sps = (await program.account.storagePointer.all([
  //     {
  //       memcmp: {
  //         offset: 8,
  //         bytes: wallet.toBase58()
  //       }
  //     }
  //   ])).sort((a,b) => a.account.id - b.account.id)
  //   const randompubkey = PublicKey.unique()

  //   console.log("Next SP PDA: ", sps.length <= 0 ? "None" : sps[sps.length - 1].publicKey)

  //   const tx = await program.methods.createSp(
  //     1,
  //     sps.length
  //   ).accounts({
  //     oldStoragePointer: sps.length <= 0 ? randompubkey : sps[sps.length - 1].publicKey
  //   }).rpc();
    
  //   console.log("Transaction Signature: ", tx)
  // })

  // it("Check All sp", async () => {
  //   const data = (await program.account.storagePointer.all([
  //     {
  //       memcmp: {
  //         offset: 8,
  //         bytes: wallet.toBase58()
  //       }
  //     },
  //     {
  //       memcmp: {
  //         offset: 44,
  //         bytes: bs58.encode(Buffer.from(Uint32Array.of(1).buffer))
  //       }
  //     }
  //   ])).sort((a,b) => a.account.id - b.account.id)
    
  //   console.log("sp: ", data)
  // })

  // it("add SP", async () => {
  //   const data = await program.account.locker.all([
  //     {
  //       memcmp: {
  //         offset: 8,
  //         bytes: wallet.toBase58()
  //       }
  //     }
  //   ])
    
  //   const tx = await program.methods.addSp(
  //     1,
  //     0,
  //     Array.from(Keypair.generate().publicKey.toBuffer())
  //   ).rpc()
    
  //   // console.log("Inserting: ", PublicKey.unique().toBase58())
  //   console.log("Transaction Signature: ", tx)
  // })
  
  // it("delete SP", async () => {
  //   const tx = await program.methods.deleteSp(
  //     1,
  //     0,
  //     1
  //   ).rpc()
    
  //   // console.log("Inserting: ", PublicKey.unique().toBase58())
  //   console.log("Transaction Signature: ", tx)
  // })

  // it("update SP", async () => {
  //   const tx = await program.methods.updateSp(
  //     1,
  //     0,
  //     2,
  //     Array.from(Keypair.generate().publicKey.toBuffer())
  //   ).rpc()
    
  //   // console.log("Inserting: ", PublicKey.unique().toBase58())
  //   console.log("Transaction Signature: ", tx)
  // })

  // it("Check All sp", async () => {
  //   const data_sp = (await program.account.storagePointer.all([
  //     {
  //       memcmp: {
  //         offset: 8,
  //         bytes: wallet.toBase58()
  //       }
  //     },
  //     {
  //       memcmp: {
  //         offset: 40,
  //         bytes: bs58.encode(Buffer.from(Uint32Array.of(1).buffer))
  //       }
  //     },
  //     {
  //       memcmp: {
  //         offset: 44,
  //         bytes: bs58.encode(Buffer.from(Uint32Array.of(0).buffer))
  //       }
  //     }
  //   ])).sort((a,b) => a.account.id - b.account.id)
    
  //   data_sp.map(val => {
  //     console.log("sp     : ", val)
  //     console.log("data sp: ")
  //     val.account.data.map((valx,id) => {
  //       console.log("data ", id, ": ", PublicKey.decode(Buffer.from(valx)).toBase58())
  //     })
  //   })
  // })

  // it("Create Vault", async () => {
  //   const tx = await program.methods.createVault().rpc()

  //   console.log("Transaction Signature: ",tx)
  // })

  // it("Buy Token", async () => {
  //   const tx = await program.methods.buyToken(
  //     new anchor.BN(1_000_000)
  //   ).rpc()

  //   console.log("Transaction Signature: ",tx)
  // })
  
  // it("Take Token", async () => {
  //   const tx = await program.methods.takeToken(
  //     new anchor.BN(1_000_000)
  //   ).rpc()

  //   console.log("Transaction Signature: ",tx)
  // })


  // it("Check Vault", async () => {
  //   const data = await program.account.vault.all([
  //     {
  //       memcmp: {
  //         offset: 8,
  //         bytes: wallet.toBase58()
  //       }
  //     }
  //   ])

  //   console.log("vault: ",data)
  // })

});

describe("Profile Test", () => {
  let program: Program<EternitySc>; 
  let wallet: Keypair; 
  let provider: anchor.Provider;

  async function getProfile() {
    const [profilePda, _] = await PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), wallet.publicKey.toBuffer()],
      program.programId
    )
    return await program.account.profile.fetch(profilePda)
  }

  it("Init", async () => {
    wallet = Keypair.generate()
    
    anchor.setProvider({
      connection: new Connection("http://localhost:8899", "confirmed"),
      publicKey: wallet.publicKey
    });
    program = anchor.workspace.EternitySc as Program<EternitySc>;
    provider = anchor.getProvider();

    const txAirdrop = await provider.connection.requestAirdrop(wallet.publicKey, LAMPORTS_PER_SOL * 5)
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: txAirdrop
    }, "confirmed")
    
  })

  it("Create Profile", async () => {
    const tx = await program.methods.createProfile(
      "Udin Sieagar",
      23,
      ["Memancing", "Memasak"],
      "Hello World!"
    ).accounts({
      signer: wallet.publicKey
    }).signers([wallet])
    .rpc()

    console.log("Transaction Signature: ", tx)

    const data = await getProfile();
    const mock_data = {
      owner: wallet.publicKey,
      name: "Udin Sieagar",
      age: 23,
      hobbie: ["Memancing", "Memasak"],
      message: "Hello World!",
    }

    assert.deepEqual(data, mock_data, "Data Mismatch")
  })

  it("Update Profile", async () => {
    const tx = await program.methods.updateProfile(
      "Udin Sinaga",
      24,
      ["Memancing", "Memasak","Membaca"],
      "Hello Solana!.."
    ).accounts({
      signer: wallet.publicKey
    }).signers([wallet])
    .rpc()

    console.log("Transaction Signature: ", tx)

    const data = await getProfile();
    const mock_data = {
      owner: wallet.publicKey,
      name: "Udin Sinaga",
      age: 24,
      hobbie: ["Memancing", "Memasak","Membaca"],
      message: "Hello Solana!..",
    }

    assert.deepEqual(data, mock_data, "Data Mismatch")
  })

  
})

describe("Locker Test", () => {
  let program: Program<EternitySc>; 
  let wallet: anchor.web3.PublicKey; 
  let provider: anchor.Provider;
  
  it("Init", () => {
      anchor.setProvider(anchor.AnchorProvider.env());
      program = anchor.workspace.EternitySc as Program<EternitySc>;
      provider = anchor.getProvider();
      wallet = provider.publicKey
  })
  
})

describe("StoragePointer Test", () => {
  let program: Program<EternitySc>; 
  let wallet: anchor.web3.PublicKey; 
  let provider: anchor.Provider;
  
  it("Init", () => {
      anchor.setProvider(anchor.AnchorProvider.env());
      program = anchor.workspace.EternitySc as Program<EternitySc>;
      provider = anchor.getProvider();
      wallet = provider.publicKey
  })
  
})

describe("Vault Test", () => {
  let program: Program<EternitySc>; 
  let wallet: anchor.web3.PublicKey; 
  let provider: anchor.Provider;
  
  it("Init", () => {
      anchor.setProvider(anchor.AnchorProvider.env());
      program = anchor.workspace.EternitySc as Program<EternitySc>;
      provider = anchor.getProvider();
      wallet = provider.publicKey
  })
  
})