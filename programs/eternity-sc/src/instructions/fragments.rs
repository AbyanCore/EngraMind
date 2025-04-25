use anchor_lang::prelude::*;

use crate::state::{fragments::*, relic::*};
use crate::utils::{errors::*, helper::*};

#[derive(Accounts)]
#[instruction(relic_id: u32,fragments_id: u32)]
pub struct CreateFragments<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Fragments::INIT_SPACE,
        seeds = [b"fragments", signer.key.as_ref(), relic_id.to_le_bytes().as_ref(), fragments_id.to_le_bytes().as_ref()],
        bump
    )]
    pub fragments: Account<'info, Fragments>,

    /// CHECK: This account is used as a reference to link to the previous fragments. No data is read or written.
    #[account(mut)]
    pub old_fragments: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [b"relic", signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
        bump
    )]
    pub relic: Account<'info, Relic>,
    
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(relic_id: u32,fragments_id: u32)]
pub struct ManageFragments<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"fragments", signer.key.as_ref(), relic_id.to_le_bytes().as_ref(), fragments_id.to_le_bytes().as_ref()],
        bump
    )]
    pub fragments: Account<'info, Fragments>,
    
    #[account(
        mut,
        seeds = [b"relic", signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
        bump
    )]
    pub relic: Account<'info, Relic>,
    
    pub system_program: Program<'info, System>
}

// FRAGMENTS INSTRUCTION
pub fn create_fragments_handler(ctx: Context<CreateFragments>,_relic_id: u32,_fragment_id: u32) -> Result<()> {
    let relic = &mut  ctx.accounts.relic;
    let fragments = &mut ctx.accounts.fragments;
    let account_info = &ctx.accounts.old_fragments;

    fragments.owner = ctx.accounts.signer.key();

    if relic.fragments.is_some() {
        fragments.next_fragments = Some(account_info.key());
    }

    relic.fragments = Some(fragments.key());
    
    Ok(())
}

// FRAGMENTS MICRO INSTRUCTION
pub fn m_add_fragment_handler(ctx: Context<ManageFragments>,_relic_id: u32,_fragment_id: u32, key: [u8; 32]) -> Result<()> {
    let fragments = &mut ctx.accounts.fragments;
    let relic = &mut ctx.accounts.relic;

    // check ownership 
    require_keys_eq!(
        ctx.accounts.signer.key(),
        fragments.owner,
        OtherError::UnAuthorized
    );

    // check data count
    if fragments.data_alloc + 1 >= 500 {
        return err!(FragmentError::FragmentDataLimitExceeded);
    }
    
    let (new_size, addtional_rent) = calculate_rent_and_size(
        fragments.to_account_info().data_len(),
        8 + Fragments::INIT_SPACE + (fragments.data_alloc + 1) as usize * 32
    )?;
    
    transfer_lamports(
        &ctx.accounts.signer.to_account_info(), 
        fragments.as_ref(), 
        addtional_rent, 
        &ctx.accounts.system_program,
        false
    )?;
    
    fragments.to_account_info().realloc(new_size, false)?;
    
    fragments.fragment.push(key);
    fragments.data_alloc += 1;
    relic.data_count += 1;
    
    Ok(())
}
pub fn m_update_fragment_handler(ctx: Context<ManageFragments>,_relic_id: u32,_fragment_id: u32, id: u16, key: [u8; 32]) -> Result<()> {
    let fragments = &mut ctx.accounts.fragments;
    
    // check ownership 
    require_keys_eq!(
        ctx.accounts.signer.key(),
        fragments.owner,
        OtherError::UnAuthorized
    );
    
    if fragments.data_alloc <= id {
        return err!(FragmentError::FragmentDataNotFound)
    }
    
    fragments.fragment[id as usize] = key;
    
    Ok(())
}
pub fn m_delete_fragment_handler(ctx: Context<ManageFragments>,_relic_id: u32,_fragment_id: u32, id: u16) -> Result<()> {
    let fragments = &mut ctx.accounts.fragments;
    let relic = &mut ctx.accounts.relic;
    
    // check ownership 
    require_keys_eq!(
        ctx.accounts.signer.key(),
        fragments.owner,
        OtherError::UnAuthorized
    );

    // check count in locker and sp
    require!(
        relic.data_count > 0 && fragments.data_alloc > 0,
        FragmentError::FragmentDataNotFound
    );

    
    if fragments.data_alloc <= id {
        return err!(FragmentError::FragmentDataNotFound)
    }

    fragments.fragment.remove(id as usize);
    relic.data_count -= 1;

    Ok(())
}
