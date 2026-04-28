use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::*;
use crate::events::*;

#[derive(Accounts)]
#[instruction(target_chain_id: u64, nonce: u64)]
pub struct InitializeIntent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = EscrowState::LEN,
        seeds = [b"escrow", user.key().as_ref(), &target_chain_id.to_le_bytes(), &nonce.to_le_bytes()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_intent(
    ctx: Context<InitializeIntent>,
    target_chain_id: u64,
    nonce: u64,
    amount: u64,
    timeout_slots: u64,
    source_chain: [u8; 32],
    target_chain: [u8; 32],
    target_address: [u8; 64],
) -> Result<()> {
    let escrow_info = ctx.accounts.escrow_account.to_account_info();
    let escrow_key = ctx.accounts.escrow_account.key();
    let clock = Clock::get()?;

    let escrow = &mut ctx.accounts.escrow_account;

    require!(amount > 0, ChakraError::MathError);
    require!(timeout_slots >= 150, ChakraError::TimeoutTooShort);
    require!(timeout_slots <= 216000, ChakraError::TimeoutTooLong);

    escrow.owner = ctx.accounts.user.key();
    escrow.target_chain_id = target_chain_id;
    escrow.nonce = nonce;
    escrow.amount = amount;
    escrow.start_slot = clock.slot;
    escrow.timeout_slot = clock
        .slot
        .checked_add(timeout_slots)
        .ok_or(ChakraError::MathError)?;
    escrow.is_finalized = false;
    escrow.is_cancelled = false;
    escrow.source_chain = source_chain;
    escrow.target_chain = target_chain;
    escrow.target_address = target_address;
    escrow.bump = ctx.bumps.escrow_account;

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: escrow_info,
        },
    );

    transfer(cpi_context, amount)?;

    emit!(ControlIntent {
        owner: ctx.accounts.user.key(),
        nonce,
        amount,
        source_chain,
        target_chain,
        target_address,
        escrow_pda: escrow_key,
        timeout_slot: escrow.timeout_slot,
    });

    Ok(())
}