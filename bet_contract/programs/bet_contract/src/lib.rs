use anchor_lang::prelude::*;
use instructions::*;
mod instructions;
mod state;
mod errors;

declare_id!("FBzKRcpiLyb6zMM8XVU8mBvJdHxyknGBioihUGFxcuh4");

#[program]
pub mod bet_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount_of_bet_in_sol: u64, rating: u64) -> Result<()> {
        ctx.accounts.initialize_bet_with_initializer(&ctx.bumps, amount_of_bet_in_sol, rating)
    }
}
