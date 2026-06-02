use anchor_lang::prelude::*;

#[error_code]
/// Custom error codes returned by the CHAKRA mainframe smart contract.
pub enum ChakraError {
    /// E.g., less than 150 slots which is insecure for coordinator observation.
    #[msg("The requested timeout is too short for cross-chain finality.")]
    TimeoutTooShort,
    /// E.g., greater than 216,000 slots (approx 24 hours on Solana).
    #[msg("The requested timeout is unreasonably long.")]
    TimeoutTooLong,
    /// Thrown if a user attempts to cancel an active intent before the timeout_slot is reached.
    #[msg("The cross-chain intent has not timed out yet.")]
    TimeoutNotReached,
    /// Thrown if proof submission is attempted after the timeout_slot has passed.
    #[msg("The cross-chain intent has timed out and can no longer be finalized.")]
    TimeoutReached,
    /// The escrow has already been unlocked and funds claimed by the treasury.
    #[msg("This intent has already been finalized by a ZK-Proof.")]
    AlreadyFinalized,
    /// The user has already cancelled this intent and claimed a refund.
    #[msg("This intent has already been cancelled.")]
    AlreadyCancelled,
    /// General signature or authority verification failure for admin tasks.
    #[msg("User is not authorized to manage this intent.")]
    Unauthorized,
    /// Thrown if the submit_proof signer is not in the registered Sentinel registry.
    #[msg("The signer is not an authorized Sentinel node.")]
    UnauthorizedSentinel,
    /// System math overflow/underflow.
    #[msg("Mathematical overflow or underflow occurred.")]
    MathError,
    /// The recovered key from secp256k1_recover did not match the registered TSS public key.
    #[msg("The provided cryptographic proof is invalid or improperly signed.")]
    InvalidProof,
}
