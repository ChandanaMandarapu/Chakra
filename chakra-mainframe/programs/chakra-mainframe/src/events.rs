use anchor_lang::prelude::*;

#[event]
pub struct ControlIntent {
    /// person who started it
    pub user: Pubkey,
    /// btc? base? eth?
    pub target_chain_id: u64,
    /// how much sol
    pub amount: u64,
    /// where is the money locked
    pub escrow_pda: Pubkey,
    /// when can user take money back
    pub timeout_slot: u64,
}
