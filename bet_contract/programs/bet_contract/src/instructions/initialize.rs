use anchor_lang::prelude::*;
use crate::state::config::Config;

const LAMPORTS_PER_SOL: u64 = 1000000000;

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

impl<'info> Initialize<'info> {
    pub fn initialize_bet_with_initializer(&mut self, bumps: &InitializeBumps, amount_of_bet_in_sol: u64, rating: u64) -> Result<()> {
        self.config.set_inner(Config {
            pubkey_initializer: self.initializer.to_account_info(),
            rating_initializer: rating,
            
        });


        

        Ok(())
    }
}