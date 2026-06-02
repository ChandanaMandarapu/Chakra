use anchor_lang::prelude::*;

#[account]
/// Configuration storing the multi-node Threshold Signature Scheme settings.
pub struct TssConfig {
    /// Uncompressed 64-byte secp256k1 public key of the Sentinel network.
    pub tss_pubkey: [u8; 64],
    /// Minimum number of nodes required to produce a signature (usually 2).
    pub threshold: u8,
    /// Total number of nodes participating in the scheme (usually 3).
    pub total_nodes: u8,
    /// Admin address authorized to update the TSS settings.
    pub admin: Pubkey,
    /// PDA bump seed.
    pub bump: u8,
}

impl TssConfig {
    pub const LEN: usize = 8 + 64 + 1 + 1 + 32 + 1;
}

#[account]
/// State tracking each individual cross-chain execution intent and locked escrow vault.
pub struct EscrowState {
    /// Owner of the funds locked in the escrow.
    pub owner: Pubkey,
    /// Destination EVM or non-Solana target chain identifier.
    pub target_chain_id: u64,
    /// Unique sequence counter preventing replay attacks.
    pub nonce: u64,
    /// Amount of lamports or native tokens locked in the escrow.
    pub amount: u64,
    /// The Solana block slot when the intent was initialized.
    pub start_slot: u64,
    /// Slot expiration threshold. After this, owner can trigger cancel_intent and reclaim funds.
    pub timeout_slot: u64,
    /// Flag indicating if execution proof has been validated and funds released to the treasury.
    pub is_finalized: bool,
    /// Flag indicating if the user has cancelled the intent due to timeout.
    pub is_cancelled: bool,
    /// PDA bump seed.
    pub bump: u8,
    /// UTF-8 encoded name of the source chain (padded to 32 bytes).
    pub source_chain: [u8; 32],
    /// UTF-8 encoded name of the target chain (padded to 32 bytes).
    pub target_chain: [u8; 32],
    /// The target wallet address on the destination chain (padded to 64 bytes).
    pub target_address: [u8; 64],
}

impl EscrowState {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + 32 + 32 + 64;
}

#[account]
/// Global admin and treasury configuration for the CHAKRA program.
pub struct GlobalConfig {
    /// Authorized administrator who can register sentinel nodes.
    pub admin: Pubkey,
    /// Destination wallet where finalized escrow funds are released.
    pub treasury: Pubkey,
    /// Flag indicating if the global program config is initialized.
    pub is_initialized: bool,
    /// PDA bump seed.
    pub bump: u8,
}

impl GlobalConfig {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1;
}

#[account]
/// Authorized sentinel account configuration.
pub struct SentinelAccount {
    /// Solana pubkey of the Sentinel validator node.
    pub sentinel_pubkey: Pubkey,
    /// Active status of this validator node.
    pub is_active: bool,
    /// PDA bump seed.
    pub bump: u8,
}

impl SentinelAccount {
    pub const LEN: usize = 8 + 32 + 1 + 1;
}
