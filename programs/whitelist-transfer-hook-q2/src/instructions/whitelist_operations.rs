use anchor_lang::prelude::*;

use crate::state::WhitelistEntry;

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: the address to whitelist
    pub user: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        space = 8,
        seeds = [b"whitelist_entry", user.key().as_ref()],
        bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: the address to remove from whitelist
    pub user: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"whitelist_entry", user.key().as_ref()],
        bump,
        close = admin,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    pub system_program: Program<'info, System>,
}
