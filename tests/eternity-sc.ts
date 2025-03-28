import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { EternitySc } from "../target/types/eternity_sc";
const fs = require('fs');

describe("eternity-sc", () => {
  
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.EternitySc as Program<EternitySc>;
  let wallet: Keypair

  wallet = Keypair.generate()

  it("airdrop req", async () => {
    // await anchor.getProvider().connection.requestAirdrop(wallet.publicKey, 2 * LAMPORTS_PER_SOL * 5);
    const blockhash = await anchor.getProvider().connection.getLatestBlockhash();
    const signature = await anchor.getProvider().connection.requestAirdrop(wallet.publicKey, LAMPORTS_PER_SOL * 2);
    await anchor.getProvider().connection.confirmTransaction({
      signature,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });
  })
  
  const provider = new anchor.AnchorProvider(
    anchor.getProvider().connection,
    new anchor.Wallet(wallet),
    anchor.AnchorProvider.defaultOptions()
  );
  
  anchor.setProvider(provider);

  console.log("Wallet :", anchor.getProvider().publicKey.toBase58())

  async function checkBalance(print = true) {
      const balance = await anchor.getProvider().connection.getBalance(anchor.getProvider().publicKey)
      if (print) {
        console.log("Balance :",balance,"Pubkey :", anchor.getProvider().publicKey.toBase58())
      }
      return balance
  }

  checkBalance() // 8382376960
  
  // it("create profile", async () => {
  //   const tx = await program.methods.initProfile({
  //     name: "Udin Siregar",
  //     age: 23,
  //     hobbie: ["mancing","tidur","makan"],
  //     message: "HAIII DUNIAA!!"
  //   }).rpc()
  //   console.log("Transaction signature", tx);
  // });
  
  // checkBalance() // 8377367720

  // it("Update profile", async () => {

  //   const tx = await program.methods.updateProfile({
  //     name: "udin sumanto",
  //     age: 24,
  //     hobbie: ["mancing","tidur","makan","minum"],
  //     message: "HALOO SEMUA"
  //   }).rpc()
  //   console.log("Transaction signature", tx);
  // })

  // checkBalance() // 8377362720
  
  it("create locker", async () => {
    const tx = await program.methods.initLocker(
      new anchor.BN(1),
      10
    ).accounts({
      signer: wallet.publicKey
    }).signers([
      wallet
    ]).rpc()
    console.log("Transaction signature", tx);
  })

  checkBalance() // 8357730520

  // it("buy more storage", async () => {
  //   const tx = await program.methods.buyMoreStorage(
  //     new anchor.BN(1),
  //     10
  //   ).rpc()
  //   console.log("Transaction signature", tx);
  // })

  // checkBalance() // 8357725520
  
  // it("add Storage Pointer", async () => {
  //   const tx = await program.methods.addStoragePointer(
  //     new anchor.BN(4),
  //     Keypair.generate().publicKey
  //   ).rpc()
  //   console.log("Transaction signature", tx);
  // })
  
  // checkBalance() // 8357720520
  
  it(`Stress test add Storage Pointer concurrently`, async () => {
    const promises = Array.from({ length: 1000 }, async (_,i) => {
      const tx = await program.methods.addStoragePointer(
        new anchor.BN(1),
        Keypair.generate().publicKey
      ).accounts({
        signer: wallet.publicKey
      }).signers([
        wallet
      ]).rpc();
      
      console.log("ID",i,"TX: ", tx);
      const balance = await checkBalance(false);
      const logMessage = `${tx},${balance}\n`;
      fs.appendFileSync('transaction_log.csv', logMessage);
    });

    await Promise.all(promises);
  });

  it("check any", async () => {
    const data = await program.account.locker.all()
    console.log(data[0].account.dataCount)
  })
});
