use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Relic {
    pub relic_id: u32,
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub heir: Option<Pubkey>,
    #[max_len(50)]
    pub name: String,
    #[max_len(300)]
    pub description: String,
    pub data_count: u64,
    pub size: u32,
    pub visibility: bool,
    pub fragments: Option<Pubkey>
}

impl Relic {
    pub fn validate(name: &String, description: &String) -> bool {
        // check name
        name.len() <= 100 ||
        // check description
        description.len() <= 300
    }
}