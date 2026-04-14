import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChakraMainframe } from "../target/types/chakra_mainframe";

async function authorize() {
  const connection = new anchor.web3.Connection("https://api.devnet.solana.com", "confirmed");
  const wallet = anchor.Wallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  anchor.setProvider(provider);

  const program = anchor.workspace.ChakraMainframe as Program<ChakraMainframe>;

  // YOUR NEW SENTINEL ADDRESS
  const sentinelAddress = new anchor.web3.PublicKey("2DfLLCW9D5MWSezabF6qok5uDHsBZjkwSAVXUKyPREPy");

  console.log("Authorizing Sentinel:", sentinelAddress.toString());
  console.log("Target Program:", program.programId.toString());

  try {
    const tx = await program.methods
      .addSentinel(sentinelAddress)
      .rpc();
    
    console.log(">>> SUCCESS: Sentinel authorized on-chain.");
    console.log("Transaction Signature:", tx);
  } catch (error) {
    console.error(">>> ERROR: Failed to authorize sentinel:", error);
  }
}

authorize();
