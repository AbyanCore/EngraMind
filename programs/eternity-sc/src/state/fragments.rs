use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Fragments {
    pub owner: Pubkey,
    pub authority: Pubkey,
    #[max_len(1)]
    pub fragment: Vec<[u8; 32]>,
    pub data_alloc: u16, // depends on fragment[]. max 1232 b
    pub next_fragments: Option<Pubkey>
}