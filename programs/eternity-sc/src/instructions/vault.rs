use anchor_lang::prelude::*;

use crate::events::{DataNotify, Operation, TokenNotify};
use crate::state::vault::*;
use crate::utils::{errors::*,helper::*,consta::{VAULT_SEEDS, VAULT_LAMPORT_SEEDS}};

const TOKEN_LAMPORT: u64 = 10;

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Vault::INIT_SPACE,
        seeds = [VAULT_SEEDS, signer.key.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = signer,
        space = 8 + VaultLamport::INIT_SPACE,
        seeds = [VAULT_LAMPORT_SEEDS, signer.key.as_ref()],
        bump
    )]
    pub vault_lamport: Account<'info, VaultLamport>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ManageVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEEDS, signer.key.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [VAULT_LAMPORT_SEEDS, signer.key.as_ref()],
        bump
    )]
    pub vault_lamport: Account<'info, VaultLamport>,

    pub system_program: Program<'info, System>
}

// VAULT INSTRUCTION
pub fn create_vault_handler(ctx: Context<CreateVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vaultlamport = &mut ctx.accounts.vault_lamport;

    vault.set_inner(Vault {
        owner: ctx.accounts.signer.key(),
        authority: ctx.accounts.authority.key(),
        token: 0
    });

    vaultlamport.set_inner(VaultLamport {});

    emit!(DataNotify {
        by: ctx.accounts.signer.key(),
        account: vault.key(),
        message: "Vault Created".to_string(),
        operation: Operation::Create
    });

    Ok(())
}

// VAULT MICRO INSTRUCTION
pub fn m_buy_token_handler(ctx: Context<ManageVault>,amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_lamport = &mut ctx.accounts.vault_lamport;

    // check permission
    if ctx.accounts.signer.key() != vault.owner || ctx.accounts.authority.key() != vault.authority {
        return err!(OtherError::UnAuthorized);
    }

    // check user lamport
    require!(
        ctx.accounts.signer.clone().to_account_info().lamports() > amount,
        OtherError::LamportNotEnough
    );

    vault.token += amount * TOKEN_LAMPORT;

    transfer_lamports(
        &ctx.accounts.signer.to_account_info(), 
        &vault_lamport.to_account_info(), 
        amount,
        &ctx.accounts.system_program,
        false
    )?;

    emit!(TokenNotify {
        by: ctx.accounts.signer.key(),
        account: vault.key(),
        message: "Vault Token Bought".to_string(),
        amount: amount
    });
    
    Ok(())
}
pub fn m_take_token_handler(ctx: Context<ManageVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_lamport = &mut ctx.accounts.vault_lamport;

    // check permission
    if ctx.accounts.signer.key() != vault.owner || ctx.accounts.authority.key() != vault.authority {
        return err!(OtherError::UnAuthorized);
    }

    // check vault lamport
    require!(
        vault_lamport.to_account_info().lamports() > amount && 
        vault.token / TOKEN_LAMPORT > amount,
        OtherError::LamportNotEnough
    );

    vault.token -= amount * TOKEN_LAMPORT;

    transfer_lamports(
        &vault_lamport.to_account_info(), 
        &ctx.accounts.signer.to_account_info(), 
        amount,
        &ctx.accounts.system_program,
        true
    )?;

    emit!(TokenNotify {
        by: ctx.accounts.signer.key(),
        account: vault.key(),
        message: "Vault Token Taken".to_string(),
        amount: amount
    });

    Ok(())
}
    