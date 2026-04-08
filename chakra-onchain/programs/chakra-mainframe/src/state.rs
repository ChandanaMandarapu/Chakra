use anchor_lang::prelude::*;

#[account]
pub struct EscrowState {
    pub owner: Pubkey,
    pub target_chain_id: u64,
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
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + 32 + 32 + 64;
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
