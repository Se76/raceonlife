use anchor_lang::prelude::*;
mod instructions;
mod state;

declare_id!("FBzKRcpiLyb6zMM8XVU8mBvJdHxyknGBioihUGFxcuh4");

#[program]
pub mod bet_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
