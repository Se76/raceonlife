use anchor_lang::prelude::*;

use crate::state::Config;

#[derive(Accounts)]
pub struct Take<'info> {
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
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
    pub system_program: Program<'info, System>,
}   

impl<'info> Take<'info> {
    pub fn take_bet(&mut self, rating: u64) -> Result<()> {
        self.config.has_been_taken = true;
        self.config.pubkey_taker = Some(self.taker.key());
        self.config.rating_taker = Some(rating);
        

        Ok(())
    }
}