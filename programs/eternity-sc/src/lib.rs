use anchor_lang::prelude::*;

declare_id!("GWgEbF6ewumjA5ZSgxWbBjDKJhdZ6GyxKQJZmyjGFADF");

const TOKEN_LAMPORT: u64 = 10;

#[program]
pub mod eternity_sc {

    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
        let profile = &mut  ctx.accounts.profile;

        // Validate Data
        require!(
            Profile::validate(&name, &hobbie, &message),
            CustomErrorCode::DataNotValid
        );
        
        // Set Data
        profile.set_inner(Profile {
            owner: ctx.accounts.signer.key(),
            name: name,
            age: age,
            hobbie: hobbie,
            message: message
        });

        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>,name: String,age: u16, hobbie: Vec<String>, message: String) -> Result<()> {
        let profile = &mut  ctx.accounts.profile;

        // check ownership 
        require!(
            ctx.accounts.signer.key() == profile.owner,
            CustomErrorCode::UnAuthorized
        );

        // Validate Data
        require!(
            Profile::validate(&name, &hobbie, &message),
            CustomErrorCode::DataNotValid
        );
        
        // Set Data
        profile.set_inner(Profile {
            owner: ctx.accounts.signer.key(),
            name: name,
            age: age,
            hobbie: hobbie,
            message: message
        });
        
        Ok(())
    }

    pub fn create_locker(ctx: Context<CreateLocker>,locker_id: u32,name: String, description: String) -> Result<()> {
        let locker = &mut  ctx.accounts.locker;
        
        // check ownership 
        require!(
            ctx.accounts.signer.key() == locker.owner,
            CustomErrorCode::UnAuthorized
        );

        // Validate Data
        require!(
            Locker::validate(&name, &description),
            CustomErrorCode::DataNotValid
        );
        
        // Set Data
        locker.set_inner(Locker {
            owner: ctx.accounts.signer.key(),
            id: locker_id,
            name: name,
            description: description,
            data_count: 0,
            size: 0,
            storage_pointer: None,
            visibility: false
        });
        
        Ok(())
    }
    
    pub fn update_locker(ctx: Context<UpdateLocker>,_locker_id: u32,name: String, description: String, visibillity: bool) -> Result<()> {
        let locker = &mut  ctx.accounts.locker;

        // check ownership 
        require!(
            ctx.accounts.signer.key() == locker.owner,
            CustomErrorCode::UnAuthorized
        );

        // Validate Data
        require!(
            Locker::validate(&name, &description),
            CustomErrorCode::DataNotValid
        );
        
        // Set Data
        locker.name = name;
        locker.description = description;
        locker.visibility = visibillity;
        
        Ok(())
    }

    pub fn create_sp(ctx: Context<CreateSP>,locker_id: u32,sp_id: u32) -> Result<()> {
        let locker = &mut  ctx.accounts.locker;
        let sp = &mut ctx.accounts.storage_pointer;
        let account_info = &ctx.accounts.old_storage_pointer;

        // check ownership 
        require!(
            ctx.accounts.signer.key() == locker.owner,
            CustomErrorCode::UnAuthorized
        );

        sp.id = sp_id;
        sp.owner = ctx.accounts.signer.key();
        sp.locker_id = locker_id;

        if locker.storage_pointer.is_some() {
            sp.next_sp = Some(account_info.key());
        }

        locker.storage_pointer = Some(sp.key());
        
        Ok(())
    }
    
    pub fn add_sp(ctx: Context<ManageSP>,_locker_id: u32,_sp_id: u32, key: [u8; 32]) -> Result<()> {
        let storage_pointer = &mut ctx.accounts.storage_pointer;
        let locker = &mut ctx.accounts.locker;

        // check ownership 
        require!(
            ctx.accounts.signer.key() == locker.owner || ctx.accounts.signer.key() == storage_pointer.owner,
            CustomErrorCode::UnAuthorized
        );
        
        if storage_pointer.data_count >= 500 {
            return err!(CustomErrorCode::StoragePointerGroupLimitExceeded);
        }
        
        let (new_size, addtional_rent) = calculate_rent_and_size(
            storage_pointer.to_account_info().data_len(),
            8 + StoragePointer::INIT_SPACE + (storage_pointer.data_count + 1) as usize * 32
        )?;
        
        transfer_lamports(
            &ctx.accounts.signer.to_account_info(), 
            storage_pointer.as_ref(), 
            addtional_rent, 
            &ctx.accounts.system_program,
            false
        )?;
        
        storage_pointer.to_account_info().realloc(new_size, false)?;
        
        storage_pointer.data.push(key);
        storage_pointer.data_count += 1;
        locker.data_count += 1;
        
        Ok(())
    }

    pub fn update_sp(ctx: Context<ManageSP>,_locker_id: u32,_sp_id: u32, id: u16, key: [u8; 32]) -> Result<()> {
        let storage_pointer = &mut ctx.accounts.storage_pointer;

        // check ownership 
        require!(
            ctx.accounts.signer.key() == storage_pointer.owner,
            CustomErrorCode::UnAuthorized
        );
        
        if storage_pointer.data_count <= id {
            return err!(CustomErrorCode::StoragePointerGroupNotFound)
        }

        storage_pointer.data[id as usize] = key;
        
        Ok(())
    }
    
    pub fn delete_sp(ctx: Context<ManageSP>,_locker_id: u32,_sp_id: u32, id: u16) -> Result<()> {
        let storage_pointer = &mut ctx.accounts.storage_pointer;

        // check ownership 
        require!(
            ctx.accounts.signer.key() == storage_pointer.owner,
            CustomErrorCode::UnAuthorized
        );
        
        if storage_pointer.data_count <= id {
            return err!(CustomErrorCode::StoragePointerGroupNotFound)
        }

        storage_pointer.data[id as usize] = [0u8; 32];
        Ok(())
    }

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.set_inner(Vault {
            owner: ctx.accounts.signer.key(),
            token: 0
        });

        Ok(())
    }

    pub fn buy_token(ctx: Context<ManageVault>,amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let vault_lamport = &mut ctx.accounts.vault_lamport;

        // check ownership
        require!(
            ctx.accounts.signer.key() == vault.owner,
            CustomErrorCode::UnAuthorized
        );

        // check user lamport
        require!(
            ctx.accounts.signer.clone().to_account_info().lamports() > amount,
            CustomErrorCode::LamportNotEnough
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

    pub fn take_token(ctx: Context<ManageVault>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let vault_lamport = &mut ctx.accounts.vault_lamport;

        // check ownership
        require!(
            ctx.accounts.signer.key() == vault.owner,
            CustomErrorCode::UnAuthorized
        );

        // check vault lamport
        require!(
            vault_lamport.to_account_info().lamports() > amount && 
            vault.token / TOKEN_LAMPORT > amount,
            CustomErrorCode::LamportNotEnough
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
pub enum CustomErrorCode {
    #[msg("The specified profile could not be found.")]
    ProfileNotFound,
    #[msg("A profile with the same identifier already exists.")]
    ProfileAlreadyExists,
    
    #[msg("The specified locker could not be found.")]
    LockerNotFound,
    #[msg("A locker with the same identifier already exists.")]
    LockerAlreadyExists,
    #[msg("The maximum number of lockers has been exceeded.")]
    LockerLimitExceeded,
    
    #[msg("The specified storage pointer group could not be found.")]
    StoragePointerGroupNotFound,
    #[msg("A storage pointer group with the same identifier already exists.")]
    StoragePointerGroupAlreadyExists,
    #[msg("The maximum number of storage pointer groups has been exceeded.")]
    StoragePointerGroupLimitExceeded,
    
    #[msg("The provided input data is not valid.")]
    DataNotValid,

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
pub struct Profile {
    pub owner: Pubkey,
    #[max_len(100)]
    pub name: String,
    pub age: u16,
    #[max_len(5,100)]
    pub hobbie: Vec<String>,
    #[max_len(300)]
    pub message: String,
}

impl Profile {
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
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Profile::INIT_SPACE,
        seeds = [b"profile", signer.key.as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", signer.key.as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Locker {
    pub owner: Pubkey,
    pub id: u32,
    #[max_len(50)]
    pub name: String,
    #[max_len(300)]
    pub description: String,
    pub data_count: u64,
    pub size: u32,
    pub visibility: bool,
    pub storage_pointer: Option<Pubkey>
}

impl Locker {
    fn validate(name: &String, description: &String) -> bool {
        // check name
        name.len() <= 100 ||
        // check description
        description.len() <= 300
    }
}

#[derive(Accounts)]
#[instruction(locker_id: u32)]
pub struct CreateLocker<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Locker::INIT_SPACE,
        seeds = [b"locker", signer.key.as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(locker_id: u32)]
pub struct UpdateLocker<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"locker", signer.key.as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct StoragePointer {
    pub owner: Pubkey,
    pub locker_id: u32,
    pub id: u32,
    #[max_len(1)]
    pub data: Vec<[u8; 32]>,
    pub data_count: u16,
    pub next_sp: Option<Pubkey>
}

#[derive(Accounts)]
#[instruction(locker_id: u32,sp_id: u32)]
pub struct CreateSP<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + StoragePointer::INIT_SPACE,
        seeds = [b"sp", signer.key.as_ref(), locker_id.to_le_bytes().as_ref(), sp_id.to_le_bytes().as_ref()],
        bump
    )]
    pub storage_pointer: Account<'info, StoragePointer>,

    /// CHECK: Akun ini digunakan untuk menunjuk SP lama jika ada.
    #[account(mut)]
    pub old_storage_pointer: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [b"locker", signer.key.as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,
    
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(locker_id: u32,sp_id: u32)]
pub struct ManageSP<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"sp", signer.key.as_ref(), locker_id.to_le_bytes().as_ref(), sp_id.to_le_bytes().as_ref()],
        bump
    )]
    pub storage_pointer: Account<'info, StoragePointer>,
    
    #[account(
        mut,
        seeds = [b"locker", signer.key.as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,
    
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub token: u64,
}

#[account]
pub struct VaultLamport;

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

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
        space = 8,
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