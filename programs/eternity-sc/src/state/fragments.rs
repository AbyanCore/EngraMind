use anchor_lang::prelude::*;
use crate::utils::consta::MAX_FRAGMENT_SIZE;

#[account]
#[derive(InitSpace)]
pub struct Fragments {
    pub fragments_id: u32,
    pub owner: Pubkey,
    #[max_len(1)]
    pub fragment: Vec<[u8; MAX_FRAGMENT_SIZE]>,
    pub data_alloc: u16, // depends on fragment[]. max 1232 b
    pub next_fragments: Option<Pubkey>
}