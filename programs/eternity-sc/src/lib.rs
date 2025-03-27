use anchor_lang::prelude::*;

declare_id!("9msZFGH5gNoBcKU8bi2BxNDok2RVNnJjZXxukDRgAgFS");

#[program]
pub mod eternity_sc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
