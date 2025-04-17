use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Personality {
    pub owner: Pubkey,
    #[max_len(100)]
    pub name: String,
    pub age: u16,
    #[max_len(5,100)]
    pub hobbie: Vec<String>,
    #[max_len(300)]
    pub message: String,
}

impl Personality {
    pub fn validate(name: &String, hobbie: &Vec<String>, message: &String) -> bool {
        // check name
        name.len() <= 100 &&
        // check hobbie length and individual hobbie lengths
        hobbie.len() <= 5 && hobbie.iter().all(|h| h.len() <= 100) &&
        // check message
        message.len() <= 300
    }
}