use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum Operation {
    Create,
    Update,
    Delete,
}

#[event]
pub struct DataNotify {
    pub by: Pubkey,
    pub account: Pubkey,
    pub message: String,
    pub operation: Operation,
}

#[event]
pub struct TokenNotify {
    pub by: Pubkey,
    pub account: Pubkey,
    pub message: String,

    pub amount: u64
}