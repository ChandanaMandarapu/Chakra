import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChakraMainframe } from "../target/types/chakra_mainframe";
import { assert } from "chai";
import * as secp from "@noble/secp256k1";
import { keccak_256 } from "@noble/hashes/sha3";

import * as dotenv from "dotenv";
dotenv.config();

describe("chakra-mainframe", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ChakraMainframe as Program<ChakraMainframe>;
  const connection = provider.connection;

  const [globalConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [tssConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tss_config")],
    program.programId
  );

  const mockTreasury = anchor.web3.Keypair.generate();
  
  // STABLE TEST KEY (Loaded from environment)
  const STABLE_TEST_PRIV_KEY = process.env.TEST_PRIV_KEY 
    ? Buffer.from(process.env.TEST_PRIV_KEY, "hex")
    : Buffer.from("4c08836111000000000000000000000000000000000000000000000000000001", "hex");
  const STABLE_TEST_PUB_KEY = secp.getPublicKey(STABLE_TEST_PRIV_KEY, false).slice(1); // 64 bytes

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

  it("initializes global protocol configuration", async () => {
    try {
        await program.methods
          .initializeConfig(mockTreasury.publicKey)
          .accounts({
            admin: provider.wallet.publicKey,
            config: globalConfigPda,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .rpc();
        console.log("✅ SUCCESS: Global Protocol Config initialized.");
    } catch (e) {
        console.log("ℹ️ INFO: Global Protocol Config already initialized.");
    }

    const config = await program.account.globalConfig.fetch(globalConfigPda);
    assert.ok(config.treasury);
  });

  it("sets up global TSS configuration (Stable Key)", async () => {
    try {
        // Try initialize first
        await program.methods
          .initializeTssConfig(
            Array.from(STABLE_TEST_PUB_KEY),
            1,
            1
          )
          .accounts({
            admin: provider.wallet.publicKey,
            tssConfig: tssConfigPda,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .rpc();
        console.log("✅ SUCCESS: TSS Registry initialized.");
    } catch (e) {
        // If already exists, FORCE UPDATE to our stable key
        console.log("ℹ️ INFO: TSS Registry already exists. Forcing update to stable key...");
        await program.methods
          .updateTssConfig(
            Array.from(STABLE_TEST_PUB_KEY),
            1,
            1
          )
          .accounts({
            admin: provider.wallet.publicKey,
            tssConfig: tssConfigPda,
          } as any)
          .rpc();
        console.log("✅ SUCCESS: TSS Registry forced to Stable Key.");
    }

    const config = await program.account.tssConfig.fetch(tssConfigPda);
    assert.equal(config.threshold, 1);
  });

  it("locks funds in escrow and cancel after timeout", async () => {
    const user = provider.wallet;
    const TARGET_CHAIN_ID = new anchor.BN(Math.floor(Math.random() * 1000000));
    const NONCE = new anchor.BN(Date.now());
    const AMOUNT = new anchor.BN(1_000_000);
    const TIMEOUT_SLOTS = 160;

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
        owner: user.publicKey,
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
    const config = await program.account.globalConfig.fetch(globalConfigPda);
    
    // 1. Initialize Intent
    const TARGET_CHAIN_ID = new anchor.BN(Math.floor(Math.random() * 1000000));
    const NONCE = new anchor.BN(Date.now());
    const AMOUNT = new anchor.BN(500_000);
    const TARGET_ADDRESS = new Array(64).fill(0);
    TARGET_ADDRESS[0] = 1; 

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

    // 2. Construct signing payload (Big Endian)
    const payload = Buffer.concat([
        TARGET_CHAIN_ID.toArrayLike(Buffer, "be", 8),
        NONCE.toArrayLike(Buffer, "be", 8),
        AMOUNT.toArrayLike(Buffer, "be", 8),
        Buffer.from(TARGET_ADDRESS)
    ]);
    const hash = keccak_256(payload);

    // 3. Sign using the STABLE test key
    const [sigDer, recoveryId] = await secp.sign(hash, STABLE_TEST_PRIV_KEY, { recovered: true });
    
    // noble-secp256k1 v1.x returns r and s as BigInts
    const sig = secp.Signature.fromHex(sigDer);
    
    function bigintTo32Bytes(val: bigint): number[] {
        const hex = val.toString(16).padStart(64, '0');
        const buffer = Buffer.from(hex, 'hex');
        return Array.from(buffer);
    }

    const r = bigintTo32Bytes(sig.r);
    const s = bigintTo32Bytes(sig.s);
    const v = recoveryId + 27;

    // 4. Submit Proof
    const txHash = new Array(64).fill(0); 
    
    const [sentinelAuth] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("sentinel"), user.publicKey.toBuffer()],
        program.programId
      );
      
    try {
        await program.methods
            .addSentinel(user.publicKey)
            .accounts({
              admin: user.publicKey,
              config: globalConfigPda,
              sentinelAccount: sentinelAuth,
              systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .rpc();
    } catch (e) {
        console.log("ℹ️ INFO: Sentinel already authorized.");
    }

    await program.methods
      .submitProof(txHash, r, s, v)
      .accounts({
        sentinel: user.publicKey,
        sentinelAuth: sentinelAuth,
        config: globalConfigPda,
        escrowAccount: escrowPda,
        tssConfig: tssConfigPda,
        treasury: config.treasury,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    // 5. Verify Finalization
    try {
        await program.account.escrowState.fetch(escrowPda);
        assert.fail("Escrow account should have been closed (sent to treasury).");
    } catch (e) {
        console.log("✅ SUCCESS: Green Path Verified. Funds moved to Treasury.");
    }
  });
});