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
    pub source_chain: String,
    pub target_chain: String,
    pub target_address: String,
}

impl EscrowState {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + (4 + 32) + (4 + 32) + (4 + 64);
}
