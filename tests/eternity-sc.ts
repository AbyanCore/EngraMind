import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EternitySc } from "../target/types/eternity_sc";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("eternity-sc", () => {
  
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env())
  
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.EternitySc as Program<EternitySc>;

  const provider = anchor.getProvider();
  const wallet = provider.publicKey

  // it("Create Profile", async () => {
  //   const tx = await program.methods.createProfile({
  //     name: "Udin Sieagar",
  //     age: 10,
  //     hobbie: ["Memasak","Memancing"],
  //     message:"Aku ingin jadi pemancing handal di masa yang akan datang"
  //   }).rpc()

  //   console.log("Transaction Signature: ", tx)
  // })

  // it("Check Profile", async () => {
  //   const data = await program.account.profile.all()

  //   console.log("Profile: ", data)
  // })
  
  // it("Update Profile", async () => {
  //   const tx = await program.methods.updateProfile({
  //     name: "Udin Priaty",
  //     age: 10,
  //     hobbie: ["Memasak","Memancing","Ngoding","Tidur","Makan"],
  //     message:"Aku ingin jadi pemancing handal di masa yang akan datang"
  //   }).rpc()

  //   console.log("Transaction Signature: ", tx)
  // })
  
  // it("Check Profile", async () => {
  //   const data = await program.account.profile.all()

  //   console.log("Profile: ", data)
  // })

  // it("Create Locker",  async () => {
  //   const tx = await program.methods.createLocker(
  //     1,
  //     "test locker 1",
  //     "Test Locker Creation 1"
  //   ).rpc()

  //   console.log("Transaction Signature: ", tx)
  // })
  
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
  
  it("Take Token", async () => {
    const tx = await program.methods.takeToken(
      new anchor.BN(1_000_000)
    ).rpc()

    console.log("Transaction Signature: ",tx)
  })


  it("Check Vault", async () => {
    const data = await program.account.vault.all([
      {
        memcmp: {
          offset: 8,
          bytes: wallet.toBase58()
        }
      }
    ])

    console.log("vault: ",data)
  })

});
