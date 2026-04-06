use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod events;
pub mod instructions;

use crate::instructions::*;

declare_id!("8C4teHPBFRMrpx4J1LNTHPj8jex6RPrQCHdZswJLbPPp");

#[program]
pub mod chakra_mainframe {
    use super::*;

    pub fn initialize_intent(
        ctx: Context<InitializeIntent>,
        target_chain_id: u64,
        amount: u64,
        timeout_slots: u64,
        source_chain: String,
        target_chain: String,
        target_address: String,
    ) -> Result<()> {
        handle_initialize_intent(
            ctx,
            target_chain_id,
            amount,
            timeout_slots,
            source_chain,
            target_chain,
            target_address,
        )
    }

    pub fn cancel_intent(ctx: Context<CancelIntent>) -> Result<()> {
        handle_cancel_intent(ctx)
    }

    /// Called by Sentinel Nodes after successful execution on target chain.
    /// Accepts the TSS signature as proof and finalizes the escrow.
    pub fn submit_proof(
        ctx: Context<SubmitProof>,
        tx_hash: String,
        signature_r: String,
        signature_s: String,
        signature_v: u8,
    ) -> Result<()> {
        handle_submit_proof(ctx, tx_hash, signature_r, signature_s, signature_v)
    }
}