use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct CancelIntent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", owner.key().as_ref(), &escrow_account.target_chain_id.to_le_bytes()],
        bump = escrow_account.bump,
        has_one = owner @ ChakraError::Unauthorized,
        close = owner
    )]
    pub escrow_account: Account<'info, EscrowState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_cancel_intent(ctx: Context<CancelIntent>) -> Result<()> {
    let escrow = &ctx.accounts.escrow_account;
    let clock = Clock::get()?;

    require!(
        clock.slot > escrow.timeout_slot,
        ChakraError::TimeoutNotReached
    );

    require!(!escrow.is_finalized, ChakraError::AlreadyFinalized);
    require!(!escrow.is_cancelled, ChakraError::AlreadyCancelled);

    Ok(())
}
