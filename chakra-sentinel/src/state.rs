use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub is_initialized: bool,
    pub bump: u8,
}
