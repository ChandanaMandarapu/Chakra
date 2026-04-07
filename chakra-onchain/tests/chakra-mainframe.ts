import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChakraMainframe } from "../target/types/chakra_mainframe";
import { assert } from "chai";

describe("chakra-mainframe", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .ChakraMainframe as Program<ChakraMainframe>;

  it("locks funds in escrow and cancel after timeout", async () => {
    const user = provider.wallet;
    const TARGET_CHAIN_ID = new anchor.BN(8453);
    const AMOUNT = new anchor.BN(1_000_000);

    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        user.publicKey.toBuffer(),
        TARGET_CHAIN_ID.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Initialize
    await program.methods
      .initializeIntent(
        TARGET_CHAIN_ID,
        AMOUNT,
        new anchor.BN(5),
        Buffer.from("solana"),
        Buffer.from("base"),
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
      )
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    let escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isFalse(escrow.isCancelled);

    // Wait (increase buffer slightly)
    await new Promise((resolve) => setTimeout(resolve, 6000));

    // Cancel
    await program.methods
      .cancelIntent()
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
      })
      .signers([])
      .rpc();

    escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isTrue(escrow.isCancelled);
  });

  it("finalizes escrow after sentinel proof submission", async () => {
    const user = provider.wallet;
    const TARGET_CHAIN_ID = new anchor.BN(137);
    const AMOUNT = new anchor.BN(500_000);

    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        user.publicKey.toBuffer(),
        TARGET_CHAIN_ID.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // 1. Initialize
    await program.methods
      .initializeIntent(
        TARGET_CHAIN_ID,
        AMOUNT, 
        new anchor.BN(100),
        "solana",
        "polygon",
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
      )
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // 2. Authorize the Sentinel (Required for Phase 2)
    const [sentinelAuth] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("sentinel"), user.publicKey.toBuffer()],
      program.programId
    );
    
    await program.methods
      .addSentinel(user.publicKey)
      .accounts({
        admin: user.publicKey,
        sentinelAccount: sentinelAuth,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // 3. Submit Proof (Successfully authorized)
    console.log("🛡️ Submitting Sentinel Proof...");
    await program.methods
      .submitProof(
        "0xabc123...",
        "0x...",
        "0x...",
        27
      )
      .accounts({
        sentinel: user.publicKey,
        sentinelAuth: sentinelAuth,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // Verify
    const escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isTrue(escrow.isFinalized);
    assert.isFalse(escrow.isCancelled);

    console.log("🎉 SUCCESS: Intent Finalized and Escrow Closed.");
  });
});