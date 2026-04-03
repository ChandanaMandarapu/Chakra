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
            target_address
        )
    }

    pub fn cancel_intent(ctx: Context<CancelIntent>) -> Result<()> {
        handle_cancel_intent(ctx)
    }
}
