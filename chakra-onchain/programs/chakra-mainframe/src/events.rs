use anchor_lang::prelude::*;

#[event]
pub struct ControlIntent {
    pub owner: Pubkey,
    pub amount: u64,
    pub source_chain: [u8; 32],
    pub target_chain: [u8; 32],
    pub target_address: [u8; 64],
    pub escrow_pda: Pubkey,
    pub timeout_slot: u64,
}

#[event]
pub struct IntentFinalized {
    pub escrow_pda: Pubkey,
    pub tx_hash: [u8; 64],
}

#[event]
pub struct IntentCancelled {
    pub escrow_pda: Pubkey,
}
