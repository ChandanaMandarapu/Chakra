use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

#[derive(Accounts)]
pub struct SubmitProof<'info> {
    #[account(mut)]
    pub sentinel: Signer<'info>,

    #[account(
        seeds = [b"sentinel", sentinel.key().as_ref()],
        bump = sentinel_auth.bump,
        constraint = sentinel_auth.is_active @ ChakraError::UnauthorizedSentinel
    )]
    pub sentinel_auth: Account<'info, SentinelAccount>,

    #[account(
        mut,
        seeds = [b"escrow", escrow_account.owner.as_ref(), 
                 &escrow_account.target_chain_id.to_le_bytes()],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Account<'info, EscrowState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_submit_proof(
    ctx: Context<SubmitProof>,
    tx_hash: String,
    signature_r: String,
    signature_s: String,
    signature_v: u8,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;
    let clock = Clock::get()?;

    require!(!escrow.is_finalized, ChakraError::AlreadyFinalized);
    require!(!escrow.is_cancelled, ChakraError::AlreadyCancelled);
    require!(
        clock.slot <= escrow.timeout_slot,
        ChakraError::TimeoutNotReached
    );

    escrow.is_finalized = true;

    emit!(IntentFinalized {
        escrow_pda: ctx.accounts.escrow_account.key(),
        tx_hash,
    });

    Ok(())
}