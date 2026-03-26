use anchor_lang::prelude::*;

#[account]
pub struct EscrowState {
    /// person who started the command
    pub owner: Pubkey,
    /// where is the money going? (0 for btc, 8453 for base)
    pub target_chain_id: u64,
    /// how much sol are we locking
    pub amount: u64,
    /// when did this start
    pub start_slot: u64,
    /// after this slot, user can just take their money back
    pub timeout_slot: u64,
    /// did the zk proof arrive?
    pub is_finalized: bool,
    /// did the user cancel it?
    pub is_cancelled: bool,
    /// just a bump for the pda
    pub bump: u8,
}

impl EscrowState {
    pub const LEN: usize = 8 + // Discriminator
        32 + // owner
        8 +  // target_chain_id
        8 +  // amount
        8 +  // start_slot
        8 +  // timeout_slot
        1 +  // is_finalized
        1 +  // is_cancelled
        1;   // bump
}
