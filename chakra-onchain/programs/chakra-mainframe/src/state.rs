use anchor_lang::prelude::*;

#[account]
pub struct TssConfig {
    pub tss_pubkey: [u8; 64],
    pub threshold: u8,
    pub total_nodes: u8,
    pub admin: Pubkey,
    pub bump: u8,
}

impl TssConfig {
    pub const LEN: usize = 8 + 64 + 1 + 1 + 32 + 1;
}

#[account]
pub struct EscrowState {
    pub owner: Pubkey,
    pub target_chain_id: u64,
    pub nonce: u64,
    pub amount: u64,
    pub start_slot: u64,
    pub timeout_slot: u64,
    pub is_finalized: bool,
    pub is_cancelled: bool,
    pub bump: u8,
    pub source_chain: [u8; 32],
    pub target_chain: [u8; 32],
    pub target_address: [u8; 64],
}

impl EscrowState {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + 32 + 32 + 64;
}

#[account]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub is_initialized: bool,
    pub bump: u8,
}

impl GlobalConfig {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1;
}

#[account]
pub struct SentinelAccount {
    pub sentinel_pubkey: Pubkey,
    pub is_active: bool,
    pub bump: u8,
}

impl SentinelAccount {
    pub const LEN: usize = 8 + 32 + 1 + 1;
}
