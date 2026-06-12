import { ethers } from "ethers";

/**
 * Verify a combined secp256k1 TSS signature.
 * 
 * @param messageHashHex The Keccak256 hash of the intent payload.
 * @param signatureR The R component of the signature.
 * @param signatureS The S component of the signature.
 * @param signatureV The recovery ID component (27 or 28).
 * @param expectedPublicKey The uncompressed 64-byte protocol public key.
 */
export function verifyTssSignature(
  messageHashHex: string,
  signatureR: string,
  signatureS: string,
  signatureV: number,
  expectedPublicKey: string
): boolean {
  try {
    const r = "0x" + signatureR;
    const s = "0x" + signatureS;
    const sig = ethers.Signature.from({ r, s, v: signatureV });
    const recoveredAddress = ethers.recoverAddress(messageHashHex, sig);
    
    // Convert uncompressed public key to expected address
    const expectedAddress = ethers.computeAddress("0x" + expectedPublicKey);
    return recoveredAddress.toLowerCase() === expectedAddress.toLowerCase();
  } catch (error) {
    console.error("Signature verification failed:", error);
    return false;
  }
}
