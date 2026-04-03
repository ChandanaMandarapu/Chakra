use anchor_lang::prelude::*;

#[event]
pub struct ControlIntent {
    pub owner: Pubkey,
    pub amount: u64,
    pub source_chain: String,
    pub target_chain: String,
    pub target_address: String,
    pub escrow_pda: Pubkey,
    pub timeout_slot: u64,
}

#[event]
pub struct IntentFinalized {
    pub escrow_pda: Pubkey,
    pub tx_hash: String,
}

#[event]
pub struct IntentCancelled {
    pub escrow_pda: Pubkey,
}
