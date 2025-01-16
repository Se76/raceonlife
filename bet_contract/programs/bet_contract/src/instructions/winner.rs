use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(Accounts)]
pub struct Winner<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// Don't check
    #[account(mut)]
    pub taker: AccountInfo<'info>,
    // /// Don't check
    // #[account(mut)]
    // pub winner: AccountInfo<'info>,

    #[account(
        seeds = [b"config", initializer.key().as_ref(), b"bet_raceonlife"],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
                    b"vault", 
                    initializer.key().as_ref(), 
                    config.key().as_ref(),
                    b"bet_raceonlife"
                ],
        bump = config.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    // TODO vault, config
    pub system_program: Program<'info, System>
}