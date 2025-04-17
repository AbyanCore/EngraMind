use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub token: u64,
}

#[account]
#[derive(InitSpace)]
pub struct VaultLamport {}