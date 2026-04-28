use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod events;
pub mod instructions;

use crate::instructions::*;

declare_id!("2KAXwKLRTQeSTa21dsread1x7mtCVcNGwy4CUCodMxgx");

#[program]
pub mod chakra_mainframe {
    use super::*;

    pub fn initialize_tss_config(
        ctx: Context<InitializeTssConfig>,
        tss_pubkey: [u8; 64],
        threshold: u8,
        total_nodes: u8,
    ) -> Result<()> {
        handle_initialize_tss_config(ctx, tss_pubkey, threshold, total_nodes)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        treasury: Pubkey,
    ) -> Result<()> {
        handle_initialize_config(ctx, treasury)
    }

    pub fn initialize_intent(
        ctx: Context<InitializeIntent>,
        target_chain_id: u64,
        nonce: u64,
        amount: u64,
        timeout_slots: u64,
        source_chain: [u8; 32],
        target_chain: [u8; 32],
        target_address: [u8; 64],
    ) -> Result<()> {
        handle_initialize_intent(
            ctx,
            target_chain_id,
            nonce,
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

    pub fn submit_proof(
        ctx: Context<SubmitProof>,
        tx_hash: [u8; 64],
        signature_r: [u8; 32],
        signature_s: [u8; 32],
        signature_v: u8,
    ) -> Result<()> {
        handle_submit_proof(ctx, tx_hash, signature_r, signature_s, signature_v)
    }

    pub fn add_sentinel(
        ctx: Context<ManageSentinel>,
        sentinel_pubkey: Pubkey,
    ) -> Result<()> {
        handle_add_sentinel(ctx, sentinel_pubkey)
    }

    pub fn remove_sentinel(
        ctx: Context<ManageSentinel>,
        sentinel_pubkey: Pubkey,
    ) -> Result<()> {
        handle_remove_sentinel(ctx, sentinel_pubkey)
    }
}