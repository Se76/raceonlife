use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub pubkey_initializer: Pubkey,
    pub rating_initializer: u64,
    pub pubkey_taker: Pubkey,
    pub rating_taker: u64,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub winner: Option<Pubkey>,
}