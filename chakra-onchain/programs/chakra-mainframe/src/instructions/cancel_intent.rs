use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
/// Context required to cancel an expired intent.
pub struct CancelIntent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        // 1. Derivation seeds check: ensures the PDA is derived exactly using the signer's pubkey,
        // preventing malicious actors from trying to close someone else's escrow.
        seeds = [b"escrow", owner.key().as_ref(), 
                 &escrow_account.target_chain_id.to_le_bytes(),
                 &escrow_account.nonce.to_le_bytes()],
        bump = escrow_account.bump,
        // 2. has_one constraint: enforces that the escrow owner matches the signer's public key.
        has_one = owner @ ChakraError::Unauthorized,
        // 3. close constraint: when the transaction executes successfully, the escrow account 
        // is closed and its rent lamports are returned to the owner.
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

    msg!("Cancelling intent. Refunding {} lamports to user: {:?}", escrow.amount, escrow.owner);

    Ok(())
}
