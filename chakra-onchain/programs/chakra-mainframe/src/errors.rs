use anchor_lang::prelude::*;

#[error_code]
pub enum ChakraError {
    #[msg("The requested timeout is too short for cross-chain finality.")]
    TimeoutTooShort,
    #[msg("The requested timeout is unreasonably long.")]
    TimeoutTooLong,
    #[msg("The cross-chain intent has not timed out yet.")]
    TimeoutNotReached,
    #[msg("This intent has already been finalized by a ZK-Proof.")]
    AlreadyFinalized,
    #[msg("This intent has already been cancelled.")]
    AlreadyCancelled,
    #[msg("User is not authorized to manage this intent.")]
    Unauthorized,
    #[msg("The signer is not an authorized Sentinel node.")]
    UnauthorizedSentinel,
    #[msg("Mathematical overflow or underflow occurred.")]
    MathError,
}
