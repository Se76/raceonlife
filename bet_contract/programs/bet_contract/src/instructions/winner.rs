use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Winner<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// Don't check
    #[account(mut)]
    pub taker: AccountInfo<'info>,
    /// Don't check
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    // TODO vault, config
    pub system_program: Program<'info, System>
}