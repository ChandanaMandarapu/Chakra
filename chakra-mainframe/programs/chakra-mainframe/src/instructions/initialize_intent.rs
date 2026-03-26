use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

#[derive(Accounts)]
#[instruction(target_chain_id: u64)]
pub struct InitializeIntent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = EscrowState::LEN,
        seeds = [b"escrow", user.key().as_ref(), &target_chain_id.to_le_bytes()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_intent(
    ctx: Context<InitializeIntent>,
    target_chain_id: u64,
    amount: u64,
    timeout_slots: u64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;
    let clock = Clock::get()?;

    // 1. check if the timeout makes sense
    require!(timeout_slots > 10, ChakraError::TimeoutTooShort);
    require!(timeout_slots < 100_000, ChakraError::TimeoutTooLong);

    // 2. set up the initial state
    escrow.owner = ctx.accounts.user.key();
    escrow.target_chain_id = target_chain_id;
    escrow.amount = amount;
    escrow.start_slot = clock.slot;
    escrow.timeout_slot = clock.slot.checked_add(timeout_slots).ok_or(ChakraError::MathError)?;
    escrow.is_finalized = false;
    escrow.is_cancelled = false;
    escrow.bump = ctx.bumps.escrow_account;

    // 3. move the sol into our safe box (pda)
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.escrow_account.key(),
            amount,
        ),
    );
    anchor_lang::solana_program::program::invoke(
        &cpi_context.instruction,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
        ],
    )?;

    // 4. shout out to the sentinel nodes
    emit!(ControlIntent {
        user: ctx.accounts.user.key(),
        target_chain_id,
        amount,
        escrow_pda: ctx.accounts.escrow_account.key(),
        timeout_slot: escrow.timeout_slot,
    });

    Ok(())
}
