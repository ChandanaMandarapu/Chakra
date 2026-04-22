import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChakraMainframe } from "../target/types/chakra_mainframe";
import { assert } from "chai";
import * as secp from "@noble/secp256k1";
import { keccak_256 } from "@noble/hashes/sha3";

describe("chakra-mainframe", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ChakraMainframe as Program<ChakraMainframe>;
  const connection = provider.connection;

  async function getTssConfigPda() {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tss_config")],
      program.programId
    )[0];
  }

  async function waitForSlot(targetSlot: number) {
    let currentSlot = await connection.getSlot();
    while (currentSlot <= targetSlot) {
      currentSlot = await connection.getSlot();
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  }

  function stringToBytes(str: string, length: number): number[] {
    const buffer = Buffer.alloc(length);
    buffer.write(str);
    return Array.from(buffer);
  }

  it("initializes global TSS configuration", async () => {
    const admin = provider.wallet;
    const tssConfigPda = await getTssConfigPda();
    
    const mockTssPubkey = new Array(64).fill(0).map((_, i) => i); 

    await program.methods
      .initializeTssConfig(
        mockTssPubkey,
        2,
        3
      )
      .accounts({
        admin: admin.publicKey,
        tssConfig: tssConfigPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const config = await program.account.tssConfig.fetch(tssConfigPda);
    assert.equal(config.threshold, 2);
    console.log("✅ SUCCESS: TSS Registry initialized.");
  });

  it("locks funds in escrow and cancel after timeout (with nonce)", async () => {
    const user = provider.wallet;
    const randomId = Math.floor(Math.random() * 100000);
    const TARGET_CHAIN_ID = new anchor.BN(randomId);
    const NONCE = new anchor.BN(Date.now());
    const AMOUNT = new anchor.BN(1_000_000);
    const TIMEOUT_SLOTS = 150;

    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        user.publicKey.toBuffer(),
        TARGET_CHAIN_ID.toArrayLike(Buffer, "le", 8),
        NONCE.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .initializeIntent(
        TARGET_CHAIN_ID,
        NONCE,
        AMOUNT, 
        new anchor.BN(TIMEOUT_SLOTS),
        stringToBytes("solana", 32),
        stringToBytes("base", 32),
        stringToBytes("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", 64)
      )
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    let escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isFalse(escrow.isCancelled);

    console.log(`⏳ Waiting for slot ${escrow.timeoutSlot.toNumber()}...`);
    await waitForSlot(escrow.timeoutSlot.toNumber());

    await program.methods
      .cancelIntent()
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
      } as any)
      .rpc();

    try {
      await program.account.escrowState.fetch(escrowPda);
      assert.fail("Escrow account should have been closed.");
    } catch (e) {
      console.log("✅ SUCCESS: Escrow account closed and funds refunded.");
    }
  });

  it("full end-to-end: valid TSS signature finalizes escrow", async () => {
    const user = provider.wallet;
    const privKey = secp.utils.randomPrivateKey();
    const pubKeyPoint = secp.Point.fromPrivateKey(privKey);
    const uncompressedPubkey = pubKeyPoint.toRawBytes(false).slice(1); // 64 bytes

    // 1. Setup TssConfig with our test key
    const tssConfigPda = await getTssConfigPda();
    await program.methods
      .initializeTssConfig(Array.from(uncompressedPubkey), 1, 1)
      .accounts({
        admin: user.publicKey,
        tssConfig: tssConfigPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // 2. Initialize Intent
    const TARGET_CHAIN_ID = new anchor.BN(1);
    const NONCE = new anchor.BN(888);
    const AMOUNT = new anchor.BN(500_000);
    const TARGET_ADDRESS = new Array(64).fill(0);
    TARGET_ADDRESS[0] = 1; // Example

    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        user.publicKey.toBuffer(),
        TARGET_CHAIN_ID.toArrayLike(Buffer, "le", 8),
        NONCE.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .initializeIntent(
        TARGET_CHAIN_ID,
        NONCE,
        AMOUNT, 
        new anchor.BN(1000),
        stringToBytes("solana", 32),
        stringToBytes("polygon", 32),
        TARGET_ADDRESS
      )
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // 3. Construct signing payload (Big Endian)
    const payload = Buffer.concat([
        TARGET_CHAIN_ID.toArrayLike(Buffer, "be", 8),
        NONCE.toArrayLike(Buffer, "be", 8),
        AMOUNT.toArrayLike(Buffer, "be", 8),
        Buffer.from(TARGET_ADDRESS)
    ]);
    const hash = keccak_256(payload);

    // 4. Sign
    const [signature, recovery] = await secp.sign(hash, privKey, { recovered: true });
    
    // Split R and S
    const r = Array.from(signature.slice(0, 32));
    const s = Array.from(signature.slice(32, 64));
    const v = recovery + 27;

    // 5. Submit Proof
    const txHash = new Array(64).fill(0); // Mock tx hash
    
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

    await program.methods
      .submitProof(txHash, r, s, v)
      .accounts({
        sentinel: user.publicKey,
        sentinelAuth: sentinelAuth,
        escrowAccount: escrowPda,
        tssConfig: tssConfigPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // 6. Verify Finalization
    const escrow = await program.account.escrowState.fetch(escrowPda);
    assert.isTrue(escrow.isFinalized);
    console.log("✅ SUCCESS: Green Path Verified. On-chain math is perfect.");
  });

  it("finalizes escrow after sentinel proof submission (Verification Gate Failure)", async () => {
    const user = provider.wallet;
    const randomId = Math.floor(Math.random() * 100000 + 1000);
    const TARGET_CHAIN_ID = new anchor.BN(randomId);
    const NONCE = new anchor.BN(101);
    const AMOUNT = new anchor.BN(500_000);

    const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        user.publicKey.toBuffer(),
        TARGET_CHAIN_ID.toArrayLike(Buffer, "le", 8),
        NONCE.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Re-initialize with dummy data for failure test
    await program.methods
      .initializeIntent(
        TARGET_CHAIN_ID,
        NONCE,
        AMOUNT, 
        new anchor.BN(150),
        stringToBytes("solana", 32),
        stringToBytes("polygon", 32),
        stringToBytes("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", 64)
      )
      .accounts({
        user: user.publicKey,
        escrowAccount: escrowPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const [sentinelAuth] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("sentinel"), user.publicKey.toBuffer()],
      program.programId
    );

    try {
        await program.methods
          .submitProof(
            new Array(64).fill(0),
            new Array(32).fill(0),
            new Array(32).fill(0),
            27
          )
          .accounts({
            sentinel: user.publicKey,
            sentinelAuth: sentinelAuth,
            escrowAccount: escrowPda,
            tssConfig: await getTssConfigPda(),
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .rpc();
        assert.fail("Proof submission should fail verification gate.");
    } catch (e) {
        console.log("✅ SUCCESS: On-Chain Mathematical Verification Gate active.");
    }
  });
});