use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct CancelIntent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", user.key().as_ref(), &escrow_account.target_chain_id.to_le_bytes()],
        bump = escrow_account.bump,
        has_one = owner @ ChakraError::Unauthorized,
        close = user // Cleanup and refund rent
    )]
    pub escrow_account: Account<'info, EscrowState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_cancel_intent(ctx: Context<CancelIntent>) -> Result<()> {
    let escrow = &ctx.accounts.escrow_account;
    let clock = Clock::get()?;

    // 1. check if time is actually up
    require!(
        clock.slot > escrow.timeout_slot,
        ChakraError::TimeoutNotReached
    );

    // 2. make sure it's not already finished or cancelled
    require!(!escrow.is_finalized, ChakraError::AlreadyFinalized);
    require!(!escrow.is_cancelled, ChakraError::AlreadyCancelled);

    // anchor handles the sol refund automatically via the "close = user" tag
    // in the accounts struct above. pretty clean.

    Ok(())
}
