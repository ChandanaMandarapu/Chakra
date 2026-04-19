use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use solana_program::{
    keccak::hash as keccak256,
    secp256k1_recover::secp256k1_recover,
};

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
                 &escrow_account.target_chain_id.to_le_bytes(),
                 &escrow_account.nonce.to_le_bytes()],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Box<Account<'info, EscrowState>>,

    #[account(
        seeds = [b"tss_config"],
        bump = tss_config.bump,
    )]
    pub tss_config: Account<'info, TssConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handle_submit_proof(
    ctx: Context<SubmitProof>,
    tx_hash: [u8; 64],
    signature_r: [u8; 32],
    signature_s: [u8; 32],
    signature_v: u8,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;
    let tss_config = &ctx.accounts.tss_config;
    let clock = Clock::get()?;

    require!(!escrow.is_finalized, ChakraError::AlreadyFinalized);
    require!(!escrow.is_cancelled, ChakraError::AlreadyCancelled);
    require!(
        clock.slot <= escrow.timeout_slot,
        ChakraError::TimeoutReached
    );

    // --- MATHEMATICAL VERIFICATION ---
    // Reconstruct the message hash for the cross-chain intent
    // Format: [target_chain_id (8), nonce (8), amount (8), target_address (64)]
    let mut msg_data = Vec::with_capacity(8 + 8 + 8 + 64);
    msg_data.extend_from_slice(&escrow.target_chain_id.to_be_bytes());
    msg_data.extend_from_slice(&escrow.nonce.to_be_bytes());
    msg_data.extend_from_slice(&escrow.amount.to_be_bytes());
    msg_data.extend_from_slice(&escrow.target_address);

    let msg_hash = keccak256(&msg_data).to_bytes();

    // The signature recovery expects [r (32), s (32)]
    let mut sig_bytes = [0u8; 64];
    sig_bytes[0..32].copy_from_slice(&signature_r);
    sig_bytes[32..64].copy_from_slice(&signature_s);

    // secp256k1_recover uses 0 or 1 for the recovery ID (v - 27)
    let recovery_id = signature_v.checked_sub(27).ok_or(ChakraError::InvalidProof)?;
    
    let recovered_pubkey = secp256k1_recover(&msg_hash, recovery_id, &sig_bytes)
        .map_err(|_| ChakraError::InvalidProof)?;

    // Verify the recovered public key matches the group's registered TSS key
    require!(
        recovered_pubkey.to_bytes() == tss_config.tss_pubkey,
        ChakraError::InvalidProof
    );

    escrow.is_finalized = true;

    emit!(IntentFinalized {
        escrow_pda: ctx.accounts.escrow_account.key(),
        tx_hash,
    });

    Ok(())
}