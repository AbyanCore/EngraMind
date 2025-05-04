use anchor_lang::prelude::*;
use crate::events::{AuthorityNotify, DataNotify, Operation};
use crate::state::relic::*;
use crate::utils::{errors::{OtherError,RelicError}, consta::RELIC_SEEDS};

#[derive(Accounts)]
#[instruction(relic_id: u32)]
pub struct CreateRelic<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Relic::INIT_SPACE,
        seeds = [RELIC_SEEDS, signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
        bump
    )]
    pub relic: Account<'info, Relic>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(relic_id: u32)]
pub struct ManageRelic<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [RELIC_SEEDS, signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
        bump
    )]
    pub relic: Account<'info, Relic>,
    pub system_program: Program<'info, System>
}


// RELIC INSTRUCTION
pub fn create_relic_handler(ctx: Context<CreateRelic>,relic_id: u32,name: String, description: String) -> Result<()> {
    let relic = &mut  ctx.accounts.relic;

    // Validate Data
    require!(
        Relic::validate(&name, &description),
        RelicError::RelicInputDataNotValid
    );
    
    // Set Data
    relic.set_inner(Relic {
        relic_id: relic_id,
        owner: ctx.accounts.signer.key(),
        authority: ctx.accounts.authority.key(),
        heir: None,
        name: name,
        description: description,
        data_count: 0,
        size: 0,
        fragments: None,
        visibility: false
    });

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: ctx.accounts.relic.key(),
        message: "Relic created".to_string(),
        operation: Operation::Create
    });
    
    Ok(())
}
pub fn update_relic_handler(ctx: Context<ManageRelic>,_relic_id: u32,name: String, description: String, visibillity: bool) -> Result<()> {
    let relic = &mut  ctx.accounts.relic;

    // check permission
    if ctx.accounts.signer.key() != relic.owner ||ctx.accounts.signer.key() != relic.authority {
        return err!(OtherError::UnAuthorized);
    }

    // Validate Data
    require!(
        Relic::validate(&name, &description),
        RelicError::RelicInputDataNotValid
    );
    
    // Set Data
    relic.name = name;
    relic.description = description;
    relic.visibility = visibillity;

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: ctx.accounts.relic.key(),
        message: "Relic updated".to_string(),
        operation: Operation::Update
    });
    
    Ok(())
}

// RELIC MICRO INSTRUCTION
pub fn m_set_relic_description_handler(ctx: Context<ManageRelic>, _relic_id: u32, description: String) -> Result<()> {
    let relic = &mut ctx.accounts.relic;

    // check permission
    if ctx.accounts.signer.key() != relic.owner ||ctx.accounts.signer.key() != relic.authority {
        return err!(OtherError::UnAuthorized);
    }

    require!(
        Relic::validate(&String::new(), &description),
        RelicError::RelicInputDataNotValid
    );

    relic.description = description;

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: ctx.accounts.relic.key(),
        message: "Relic description updated".to_string(),
        operation: Operation::Update
    });

    Ok(())
}
pub fn m_set_relic_heir_handler(ctx: Context<ManageRelic>, _relic_id: u32, heir: Pubkey) -> Result<()> {
    let relic = &mut ctx.accounts.relic;

    // check permission
    if ctx.accounts.signer.key() != relic.owner ||ctx.accounts.signer.key() != relic.authority {
        return err!(OtherError::UnAuthorized);
    }

    relic.heir = Some(heir);

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: ctx.accounts.relic.key(),
        message: "Relic heir updated".to_string(),
        operation: Operation::Update
    });

    Ok(())
}
pub fn m_set_relic_authority_handler(ctx: Context<ManageRelic>, _relic_id: u32, new_authority: Pubkey) -> Result<()> {
    let relic = &mut ctx.accounts.relic;

    // check permission
    if ctx.accounts.signer.key() != relic.owner ||ctx.accounts.signer.key() != relic.authority {
        return err!(OtherError::UnAuthorized);
    }

    let old_authority = relic.authority.clone();
    relic.authority = new_authority;

    emit!(AuthorityNotify {
        by: ctx.accounts.signer.key(),
        account: ctx.accounts.relic.key(),
        message: "Relic authority updated".to_string(),
        old_authority: old_authority,
        new_authority: new_authority
    });

    Ok(())
}
