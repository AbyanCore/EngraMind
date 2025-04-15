use anchor_lang::prelude::*;

declare_id!("4zEC44NxzuFsrQ3ZVXqnrP1UghjL29ARnY7a6xaz2WZb");

const TOKEN_LAMPORT: u64 = 10;

#[program]
pub mod eternity_sc {
    use super::*;

    // PERSONALITY INSTRUCTION
    pub fn create_personality(ctx: Context<CreatePersonality>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
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

        Ok(())
    }
    pub fn update_personality(ctx: Context<ManagePersonality>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
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
        
        Ok(())
    }

    // PERSONALITY MICRO INSTRUCTION
    pub fn m_set_personality_message(ctx: Context<ManagePersonality>,message: String) -> Result<()> {
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
        
        Ok(())
    }
    pub fn m_set_personality_hobbie(ctx: Context<ManagePersonality>,hobbie: Vec<String>) -> Result<()> {
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
        
        Ok(())
    }

    // RELIC INSTRUCTION
    pub fn create_relic(ctx: Context<CreateRelic>,_relic_id: u32,name: String, description: String) -> Result<()> {
        let relic = &mut  ctx.accounts.relic;

        // Validate Data
        require!(
            Relic::validate(&name, &description),
            RelicError::RelicInputDataNotValid
        );
        
        // Set Data
        relic.set_inner(Relic {
            owner: ctx.accounts.signer.key(),
            authority: ctx.accounts.authority.key(),
            name: name,
            description: description,
            data_count: 0,
            size: 0,
            storage_pointer: None,
            visibility: false
        });
        
        Ok(())
    }
    pub fn update_relic(ctx: Context<ManageRelic>,_relic_id: u32,name: String, description: String, visibillity: bool) -> Result<()> {
        let relic = &mut  ctx.accounts.relic;

        // check ownership
        require_keys_eq!(
            ctx.accounts.signer.key(),
            relic.owner,
            OtherError::UnAuthorized
        );
        
        // check authority 
        require_keys_eq!(
            ctx.accounts.authority.key(),
            relic.authority,
            OtherError::UnAuthorized
        );

        // Validate Data
        require!(
            Relic::validate(&name, &description),
            RelicError::RelicInputDataNotValid
        );
        
        // Set Data
        relic.name = name;
        relic.description = description;
        relic.visibility = visibillity;
        
        Ok(())
    }
    
    // RELIC MICRO INSTRUCTION
    pub fn m_set_relic_description(ctx: Context<ManageRelic>, _locker_id: u32, description: String) -> Result<()> {
        let relic = &mut ctx.accounts.relic;

        // check ownership
        require_keys_eq!(
            ctx.accounts.signer.key(),
            relic.owner,
            OtherError::UnAuthorized
        );
        
        // check authority 
        require_keys_eq!(
            ctx.accounts.authority.key(),
            relic.authority,
            OtherError::UnAuthorized
        );

        require!(
            Relic::validate(&String::new(), &description),
            RelicError::RelicInputDataNotValid
        );

        relic.description = description;

        Ok(())
    }

    // FRAGMENTS INSTRUCTION
    pub fn create_fragments(ctx: Context<CreateFragments>,_relic_id: u32,_fragment_id: u32) -> Result<()> {
        let relic = &mut  ctx.accounts.relic;
        let fragments = &mut ctx.accounts.fragments;
        let account_info = &ctx.accounts.old_fragments;


        fragments.owner = ctx.accounts.signer.key();
        fragments.authority = ctx.accounts.authority.key();

        if relic.storage_pointer.is_some() {
            fragments.next_fragments = Some(account_info.key());
        }

        relic.storage_pointer = Some(fragments.key());
        
        Ok(())
    }

    // FRAGMENTS MICRO INSTRUCTION
    pub fn m_add_fragment(ctx: Context<ManageFragments>,_relic_id: u32,_fragment_id: u32, key: [u8; 32]) -> Result<()> {
        let fragments = &mut ctx.accounts.fragments;
        let relic = &mut ctx.accounts.relic;

        // check ownership 
        require_keys_eq!(
            ctx.accounts.signer.key(),
            fragments.owner,
            OtherError::UnAuthorized
        );

        // check authority
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fragments.authority,
            OtherError::UnAuthorized
        );

        // check data count
        if fragments.data_alloc >= 500 {
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
    pub fn m_update_fragment(ctx: Context<ManageFragments>,_relic_id: u32,_fragment_id: u32, id: u16, key: [u8; 32]) -> Result<()> {
        let fragments = &mut ctx.accounts.fragments;
        
        // check ownership 
        require_keys_eq!(
            ctx.accounts.signer.key(),
            fragments.owner,
            OtherError::UnAuthorized
        );

        // check authority
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fragments.authority,
            OtherError::UnAuthorized
        );
        
        if fragments.data_alloc <= id {
            return err!(FragmentError::FragmentDataNotFound)
        }
        
        fragments.fragment[id as usize] = key;
        
        Ok(())
    }
    pub fn m_delete_fragment(ctx: Context<ManageFragments>,_relic_id: u32,_fragment_id: u32, id: u16) -> Result<()> {
        let fragments = &mut ctx.accounts.fragments;
        let relic = &mut ctx.accounts.relic;
        
        // check ownership 
        require_keys_eq!(
            ctx.accounts.signer.key(),
            fragments.owner,
            OtherError::UnAuthorized
        );

        // check authority
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fragments.authority,
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

    // VAULT INSTRUCTION
    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let vaultlamport = &mut ctx.accounts.vault_lamport;

        vault.set_inner(Vault {
            owner: ctx.accounts.signer.key(),
            authority: ctx.accounts.authority.key(),
            token: 0
        });

        vaultlamport.set_inner(VaultLamport {});

        Ok(())
    }

    // VAULT MICRO INSTRUCTION
    pub fn m_buy_token(ctx: Context<ManageVault>,amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let vault_lamport = &mut ctx.accounts.vault_lamport;

        // check ownership
        require_keys_eq!(
            ctx.accounts.signer.key(),
            vault.owner,
            OtherError::UnAuthorized
        );

        // check authority
        require_keys_eq!(
            ctx.accounts.authority.key(),
            vault.authority,
            OtherError::UnAuthorized
        );

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
        
        Ok(())
    }
    pub fn m_take_token(ctx: Context<ManageVault>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let vault_lamport = &mut ctx.accounts.vault_lamport;

        // check ownership
        require_keys_eq!(
            ctx.accounts.signer.key(),
            vault.owner,
            OtherError::UnAuthorized
        );

        // check authority
        require_keys_eq!(
            ctx.accounts.authority.key(),
            vault.authority,
            OtherError::UnAuthorized
        );

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

        Ok(())
    }

}

// Error codes
#[error_code]
pub enum PersonalityError {
    #[msg("The provided profile data is invalid. Ensure the fields input is correct")]
    ProfileInputDataNotValid,
}

#[error_code]
pub enum RelicError {
    #[msg("The provided relic data is invalid. Ensure the fields input is correct.")]
    RelicInputDataNotValid,
}

#[error_code]
pub enum FragmentError {
    #[msg("The maximum number of fragments has been exceeded.")]
    FragmentDataLimitExceeded,
    #[msg("The specified fragment data could not be found.")]
    FragmentDataNotFound,
}

#[error_code]
pub enum OtherError {
    #[msg("Not Authorized")]
    UnAuthorized,
    #[msg("Not Enough Lamport or SOL")]
    LamportNotEnough
}

// Helper Function
fn transfer_lamports<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
    from_pda: bool
) -> Result<()> {

    let ix = &anchor_lang::solana_program::system_instruction::transfer(
        from.key,
        to.key,
        amount,
    );

    match from_pda {
        true => {
            from.sub_lamports(amount)?;
            to.add_lamports(amount)?;
        }
        false => {
            anchor_lang::solana_program::program::invoke(
                ix,
                &[
                    from.clone(),
                    to.clone(),
                    system_program.to_account_info().clone(),
                ],
            )?;
        }
    }
    Ok(())
}

fn calculate_rent_and_size(
    current_data_len: usize,
    new_data_len: usize,
) -> Result<(usize, u64)> {
    let rent = Rent::get()?;
    let additional_rent = rent.minimum_balance(new_data_len) - rent.minimum_balance(current_data_len);
    Ok((new_data_len, additional_rent))
}

// ACCOUNT DEFINITIONS
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
    fn validate(name: &String, hobbie: &Vec<String>, message: &String) -> bool {
        // check name
        name.len() <= 100 &&
        // check hobbie length and individual hobbie lengths
        hobbie.len() <= 5 && hobbie.iter().all(|h| h.len() <= 100) &&
        // check message
        message.len() <= 300
    }
}

#[derive(Accounts)]
pub struct CreatePersonality<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Personality::INIT_SPACE,
        seeds = [b"personality", signer.key.as_ref()],
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
        seeds = [b"personality", signer.key.as_ref()],
        bump
    )]
    pub personality: Account<'info, Personality>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Relic {
    pub owner: Pubkey,
    pub authority: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(300)]
    pub description: String,
    pub data_count: u64,
    pub size: u32,
    pub visibility: bool,
    pub storage_pointer: Option<Pubkey>
}

impl Relic {
    fn validate(name: &String, description: &String) -> bool {
        // check name
        name.len() <= 100 ||
        // check description
        description.len() <= 300
    }
}

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
        seeds = [b"relic", signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
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

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"relic", signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
        bump
    )]
    pub relic: Account<'info, Relic>,
    pub system_program: Program<'info, System>
}

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

#[derive(Accounts)]
#[instruction(relic_id: u32,fragments_id: u32)]
pub struct CreateFragments<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Fragments::INIT_SPACE,
        seeds = [b"sp", signer.key.as_ref(), relic_id.to_le_bytes().as_ref(), fragments_id.to_le_bytes().as_ref()],
        bump
    )]
    pub fragments: Account<'info, Fragments>,

    #[account(mut)]
    pub old_fragments: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [b"locker", signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
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
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"sp", signer.key.as_ref(), relic_id.to_le_bytes().as_ref(), fragments_id.to_le_bytes().as_ref()],
        bump
    )]
    pub fragments: Account<'info, Fragments>,
    
    #[account(
        mut,
        seeds = [b"locker", signer.key.as_ref(), relic_id.to_le_bytes().as_ref()],
        bump
    )]
    pub relic: Account<'info, Relic>,
    
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub token: u64,
}

#[account]
#[derive(InitSpace)]
pub struct VaultLamport {}

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
        seeds = [b"vault", signer.key.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = signer,
        space = 8 + VaultLamport::INIT_SPACE,
        seeds = [b"vault_lamport", signer.key.as_ref()],
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
        seeds = [b"vault", signer.key.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [b"vault_lamport", signer.key.as_ref()],
        bump
    )]
    pub vault_lamport: Account<'info, VaultLamport>,

    pub system_program: Program<'info, System>
}