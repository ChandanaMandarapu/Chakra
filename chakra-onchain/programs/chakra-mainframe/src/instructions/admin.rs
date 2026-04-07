use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(sentinel_pubkey: Pubkey)]
pub struct ManageSentinel<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        space = SentinelAccount::LEN,
        seeds = [b"sentinel", sentinel_pubkey.as_ref()],
        bump
    )]
    pub sentinel_account: Account<'info, SentinelAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handle_add_sentinel(
    ctx: Context<ManageSentinel>,
    sentinel_pubkey: Pubkey,
) -> Result<()> {
    let sentinel = &mut ctx.accounts.sentinel_account;
    sentinel.sentinel_pubkey = sentinel_pubkey;
    sentinel.is_active = true;
    sentinel.bump = ctx.bumps.sentinel_account;
    
    msg!("Sentinel authorized: {:?}", sentinel_pubkey);
    Ok(())
}

pub fn handle_remove_sentinel(
    ctx: Context<ManageSentinel>,
    _sentinel_pubkey: Pubkey,
) -> Result<()> {
    let sentinel = &mut ctx.accounts.sentinel_account;
    sentinel.is_active = false;
    
    msg!("Sentinel de-authorized: {:?}", sentinel.sentinel_pubkey);
    Ok(())
}
