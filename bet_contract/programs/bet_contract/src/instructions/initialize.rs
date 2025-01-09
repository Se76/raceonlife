use anchor_lang::prelude::*;
use crate::state::config::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        space = Config::INIT_SPACE + 8,
        seeds = [b"config", initializer.key().as_ref(), b"bet_raceonlife"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [
                    b"vault", 
                    initializer.key().as_ref(), 
                    config.key().as_ref(),
                    b"bet_raceonlife"
                ],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}