use thiserror::Error;

#[derive(Error, Debug)]
pub enum SentinelError {
    #[error("Failed to parse key shard: {0}")]
    ShardParseError(String),

    #[error("Threshold signature generation failed: {0}")]
    TssSigningError(String),

    #[error("Network request failed: {0}")]
    NetworkError(String),

    #[error("Solana connection or transaction submission failed: {0}")]
    SolanaRpcError(String),

    #[error("Incorrect threshold configuration: expected {expected}, got {actual}")]
    InvalidThreshold { expected: usize, actual: usize },

    #[error("Decentralized key reconstruction failed: {0}")]
    ReconstructionError(String),
}
