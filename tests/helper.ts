import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

async function createWallet(amountSol: number,provider: any): Promise<Keypair>  {
    const wallet = Keypair.generate();

    const txAirdrop = await provider.connection.requestAirdrop(wallet.publicKey, LAMPORTS_PER_SOL * amountSol)
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: txAirdrop
    }, "confirmed")

    console.log("WALLET:", wallet.publicKey.toBase58());

    return wallet;
}

export { createWallet };