use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeTssConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = TssConfig::LEN,
        seeds = [b"tss_config"],
        bump
    )]
    pub tss_config: Account<'info, TssConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = GlobalConfig::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, GlobalConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sentinel_pubkey: Pubkey)]
pub struct ManageSentinel<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.admin == admin.key()
    )]
    pub config: Account<'info, GlobalConfig>,

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

pub fn handle_initialize_tss_config(
    ctx: Context<InitializeTssConfig>,
    tss_pubkey: [u8; 64],
    threshold: u8,
    total_nodes: u8,
) -> Result<()> {
    let tss_config = &mut ctx.accounts.tss_config;
    tss_config.tss_pubkey = tss_pubkey;
    tss_config.threshold = threshold;
    tss_config.total_nodes = total_nodes;
    tss_config.admin = ctx.accounts.admin.key();
    tss_config.bump = ctx.bumps.tss_config;
    Ok(())
}

pub fn handle_initialize_config(
    ctx: Context<InitializeConfig>,
    treasury: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.treasury = treasury;
    config.is_initialized = true;
    config.bump = ctx.bumps.config;
    Ok(())
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