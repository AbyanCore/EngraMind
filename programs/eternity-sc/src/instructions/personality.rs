use anchor_lang::prelude::*;
use crate::{events::{DataNotify,Operation}, state::personality::*};
use crate::utils::{errors::{PersonalityError, OtherError}, consta::PERSONALITY_SEEDS};

#[derive(Accounts)]
pub struct CreatePersonality<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Personality::INIT_SPACE,
        seeds = [PERSONALITY_SEEDS, signer.key.as_ref()],
        bump
    )]
    pub personality: Account<'info, Personality>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ManagePersonality<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [PERSONALITY_SEEDS, signer.key.as_ref()],
        bump
    )]
    pub personality: Account<'info, Personality>,
    pub system_program: Program<'info, System>
}

// PERSONALITY INSTRUCTION
pub fn create_personality_handler(ctx: Context<CreatePersonality>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
    let personality = &mut  ctx.accounts.personality;

    // Validate Data
    require!(
        Personality::validate(&name, &hobbie, &message),
        PersonalityError::ProfileInputDataNotValid
    );
    
    // Set Data
    personality.set_inner(Personality {
        owner: ctx.accounts.signer.key(),
        name: name,
        age: age,
        hobbie: hobbie,
        message: message
    });

    msg!("Personality Created: {:?}", personality.key());
    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: personality.key(),
        message: "Personality Created".to_string(),
        operation: Operation::Create
    });


    Ok(())
}
pub fn update_personality_handler(ctx: Context<ManagePersonality>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
    let personality = &mut  ctx.accounts.personality;

    // check ownership 
    require!(
        ctx.accounts.signer.key() == personality.owner,
        OtherError::UnAuthorized
    );

    // Validate Data
    require!(
        Personality::validate(&name, &hobbie, &message),
        PersonalityError::ProfileInputDataNotValid
    );
    
    // Set Data
    personality.set_inner(Personality {
        owner: ctx.accounts.signer.key(),
        name: name,
        age: age,
        hobbie: hobbie,
        message: message
    });

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: personality.key(),
        message: "Personality Updated".to_string(),
        operation: Operation::Update
    });
    
    Ok(())
}

// PERSONALITY MICRO INSTRUCTION
pub fn m_set_personality_message_handler(ctx: Context<ManagePersonality>,message: String) -> Result<()> {
    let personality = &mut  ctx.accounts.personality;

    // check ownership 
    require_keys_eq!(
        ctx.accounts.signer.key(),
        personality.owner,
        OtherError::UnAuthorized
    );

    // Validate Data
    require!(
        Personality::validate(&String::new(), &Vec::new(), &message),
        PersonalityError::ProfileInputDataNotValid
    );
    
    // Set Data
    personality.message = message;

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: personality.key(),
        message: "Personality Message Updated".to_string(),
        operation: Operation::Update
    });
    
    Ok(())
}
pub fn m_set_personality_hobbie_handler(ctx: Context<ManagePersonality>,hobbie: Vec<String>) -> Result<()> {
    let personality = &mut  ctx.accounts.personality;

    // check ownership 
    require_keys_eq!(
        ctx.accounts.signer.key(),
        personality.owner,
        OtherError::UnAuthorized
    );

    // Validate Data
    require!(
        Personality::validate(&String::new(), &hobbie, &String::new()),
        PersonalityError::ProfileInputDataNotValid
    );
    
    // Set Data
    personality.hobbie = hobbie;
    
    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: personality.key(),
        message: "Personality Hobbie Updated".to_string(),
        operation: Operation::Update
    });

    Ok(())
}
