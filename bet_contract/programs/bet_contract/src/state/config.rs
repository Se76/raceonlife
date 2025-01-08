use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub pubkey_initializer: Pubkey,
    pub pubkey_taker: Pubkey,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub winner: Option<Pubkey>,
}