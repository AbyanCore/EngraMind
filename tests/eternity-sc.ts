import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { EternitySc } from "../target/types/eternity_sc";

describe("eternity-sc", () => {
  
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())
  
  const program = anchor.workspace.EternitySc as Program<EternitySc>;

  const [profilePDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), anchor.getProvider().publicKey.toBuffer()],
    program.programId
  )

  function checkBalance() {
    it("check balance", async () => {
      const balance = await anchor.getProvider().connection.getBalance(anchor.getProvider().publicKey)
      console.log("secret :",anchor.getProvider().publicKey.toBase58())
      console.log("balance :",balance)
    })
  }

  checkBalance() // 8382376960
  
  it("create profile", async () => {
    const tx = await program.methods.initProfile({
      name: "Udin Siregar",
      age: 23,
      hobbie: ["mancing","tidur","makan"],
      message: "HAIII DUNIAA!!"
    }).rpc()
    console.log("Transaction signature", tx);
  });
  
  checkBalance() // 8377367720

  it("Update profile", async () => {

    const tx = await program.methods.updateProfile({
      name: "udin sumanto",
      age: 24,
      hobbie: ["mancing","tidur","makan","minum"],
      message: "HALOO SEMUA"
    }).rpc()
    console.log("Transaction signature", tx);
  })

  checkBalance() // 8377362720
  
  it("create locker", async () => {
    const tx = await program.methods.initLocker(
      new anchor.BN(1),
      10
    ).rpc()
    console.log("Transaction signature", tx);
  })

  checkBalance() // 8357730520

  it("buy more storage", async () => {
    const tx = await program.methods.buyMoreStorage(
      new anchor.BN(1),
      10
    ).rpc()
    console.log("Transaction signature", tx);
  })

  checkBalance() // 8357725520
  
  it("add Storage Pointer", async () => {

    const tx = await program.methods.addStoragePointer(
      new anchor.BN(1),
      {
        name: "Mobil baru",
        fileType: { image:{} }, 
        link: "https://example.com/Helloworld.png",
        size: 40.0
      }
    ).rpc()
    console.log("Transaction signature", tx);
  })

  checkBalance() // 8357720520

  it(`Stress tes add Storage Pointer`, async () => {
    for (let i = 0; i < 1000; i++) {
      await program.methods.addStoragePointer(
        new anchor.BN(1),
        {
          name: `Mobil baru {i}`,
          fileType: { image: {} },
          link: "https://example.com/Helloworld.png",
          size: 40.0
        }
      ).rpc()
    }
    
    const storagePointers =await program.account.locker.all()
    storagePointers.map((sp) => {
      console.log("Count :",sp.account.storagePointers.length)
      console.log("Size Mem:" ,
        sp.account.storagePointers.reduce((acc, pointer) => acc + pointer.size, 0) / (1024)
      )
    })
    
  })
  
});
