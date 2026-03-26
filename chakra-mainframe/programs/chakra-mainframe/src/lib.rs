use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod events;

use instructions::*;

declare_id!("CHAKRA11111111111111111111111111111111111111");

#[program]
pub mod chakra_mainframe {
    use super::*;

    /// this is where we lock the money and start the cross-chain stuff
    pub fn initialize_intent(
        ctx: Context<InitializeIntent>,
        target_chain_id: u64,
        amount: u64,
        timeout_slots: u64,
    ) -> Result<()> {
        instructions::handle_initialize_intent(ctx, target_chain_id, amount, timeout_slots)
    }

    /// if the nodes fail or take too long, user can get their money back here
    pub fn cancel_intent(ctx: Context<CancelIntent>) -> Result<()> {
        instructions::handle_cancel_intent(ctx)
    }
}
