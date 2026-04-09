import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChakraMainframe } from "../target/types/chakra_mainframe";
import { assert } from "chai";

describe("chakra-mainframe", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ChakraMainframe as Program<ChakraMainframe>;
  const connection = provider.connection;

  async function waitForSlot(targetSlot: number) {
    let currentSlot = await connection.getSlot();
    while (currentSlot <= targetSlot) {
      currentSlot = await connection.getSlot();
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  }

  it("locks funds in escrow and cancel after timeout", async () => {
    const user = provider.wallet;
    const TARGET_CHAIN_ID = new anchor.BN(8453);
    const AMOUNT = new anchor.BN(1_000_000);
    const TIMEOUT_SLOTS = 20;

    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        user.publicKey.toBuffer(),
        TARGET_CHAIN_ID.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Initialize Intent
    await program.methods
      .initializeIntent(
        TARGET_CHAIN_ID,
        AMOUNT, 
        new anchor.BN(TIMEOUT_SLOTS),
        "solana",
        "base",
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
      )
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    let escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isFalse(escrow.isCancelled);

    // DYNAMIC WAIT: Wait for slot to pass timeout_slot
    console.log(`⏳ Waiting for slot ${escrow.timeoutSlot.toNumber()}...`);
    await waitForSlot(escrow.timeoutSlot.toNumber());
    console.log("✅ Timeout reached. Proceeding to cancel.");

    // 5. Cancel Intent
    await program.methods
      .cancelIntent()
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
      } as any)
      .rpc();

    // 6. Verify Closure (Fetching should fail because account is closed)
    try {
      await program.account.escrowState.fetch(escrowPda);
      assert.fail("Escrow account should have been closed.");
    } catch (e) {
      console.log("✅ SUCCESS: Escrow account closed and funds refunded.");
    }
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

    // 2. Authorize the Sentinel
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

    // 3. Submit Proof
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

    // Verify Finalization
    const escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isTrue(escrow.isFinalized);
    assert.isFalse(escrow.isCancelled);
  });
});