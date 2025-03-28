use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("HU3pMs7p5d3mrz1MYjkiWtXEGA2PXUTWT7tkBN1rtjum");

const STORAGE_LAMPORT: u64 = 100;
const TOKEN_LAMPORT: u64 = 10;

#[program]
pub mod eternity_sc {
    use std::thread::LocalKey;

    use super::*;

    // PROFILE INSTRUCTIONS
    pub fn init_profile(ctx: Context<InitProfile>, data: ProfileData) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        profile.set_inner(Profile {
            name: data.name,
            age: data.age,
            hobbie: data.hobbie,
            message: data.message,
        });

        msg!("Profile initialized with data : \nName: {}\nAge: {}\nHobbie: {:?}\nMessage: {}", profile.name, profile.age, profile.hobbie, profile.message);
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, data: ProfileData) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        profile.set_inner(Profile {
            name: data.name,
            age: data.age,
            hobbie: data.hobbie,
            message: data.message,
        });

        msg!("Profile updated with data : \nName: {}\nAge: {}\nHobbie: {:?}\nMessage: {}", profile.name, profile.age, profile.hobbie, profile.message);
        Ok(())
    }

    // LOCKER INSTRUCTIONS
    pub fn init_locker(ctx: Context<InitLocker>,locker_id: u64, amount: u16) -> Result<()> {
        let locker = &mut ctx.accounts.locker;
        
        // check lamport and cut
        if ctx.accounts.signer.as_ref().lamports() < ((amount as u64) * STORAGE_LAMPORT){
            return Err(ProgramError::InsufficientFunds.into())
        }

        locker.set_inner(Locker {
            id: locker_id,
            storage_pointers: Vec::new(),
            current_size: 0.0,
            data_count: 0,
            max_size: amount,
            next_locker: Pubkey::default()
        });

        Ok(())
    }

    pub fn buy_more_storage(ctx: Context<BuyLocker>, _locker_id: u64, amount: u16) -> Result<()> {
        let locker = &mut ctx.accounts.locker;
        
        // check lamport and cut
        if ctx.accounts.signer.as_ref().lamports() < ((amount as u64) * STORAGE_LAMPORT){
            return Err(ProgramError::InsufficientFunds.into())
        }

        locker.max_size += amount;

        Ok(())
    }

    pub fn add_storage_pointer(ctx: Context<AddStoragePointer>, _locker_id: u64, pointer: Pubkey) -> Result<()> {
        let locker = &mut ctx.accounts.locker;

        // check if locker is full
        if locker.current_size >= locker.max_size as f32 {
            return Err(CustomErrorCode::LockerLimitExceeded.into())
        }

        if (locker.data_count < 512) {
            let rent = Rent::get()?;
            let new_size = 8 + Locker::INIT_SPACE + (locker.storage_pointers.len() + 1) * 32;
            let additional_rent = rent.minimum_balance(new_size) - rent.minimum_balance(locker.to_account_info().data_len());

            transfer_lamports(
                &ctx.accounts.signer,
                &locker.to_account_info(),
                additional_rent,
                &ctx.accounts.system_program
            )?;

            locker.to_account_info().realloc(new_size, false);
        }
        
        if (locker.data_count % 512 == 0) {
            locker.storage_pointers.clear();
        }
        
        // add storage pointer
        locker.storage_pointers.push(pointer);
        locker.data_count += 1;

        Ok(())
    }

}

// Error codes
#[error_code]
pub enum CustomErrorCode {
    #[msg("Profile Not Found")]
    ProfileNotFound,
    #[msg("Profile Not Found")]
    ProfileAlreadyExists,
    
    #[msg("Profile Not Found")]
    LockerNotFound,
    #[msg("Profile Not Found")]
    LockerAlreadyExists,
    #[msg("Profile Not Found")]
    LockerLimitExceeded,
    
    #[msg("Profile Not Found")]
    StoragePointerGroupNotFound,
    #[msg("Profile Not Found")]
    StoragePointerGroupAlreadyExists,
    #[msg("Profile Not Found")]
    StoragePointerGroupLimitExceeded,
}

// Helper Function
fn transfer_lamports<'info>(
    from: &Signer<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
) -> Result<()> {
    anchor_lang::solana_program::program::invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            from.key,
            to.key,
            amount,
        ),
        &[
            from.to_account_info(),
            to.clone(),
            system_program.to_account_info(),
        ],
    )?;
    Ok(())
}

// ACCOUNT DEFINITIONS

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Profile::INIT_SPACE,
        seeds = [b"profile", signer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", signer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize,AnchorDeserialize, Clone)]
pub struct ProfileData {
    pub name: String,
    pub age: u8,
    pub hobbie: Vec<String>,
    pub message: String,
}

#[derive(Accounts)]
#[instruction(locker_id: u64)]
pub struct InitLocker<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Locker::INIT_SPACE,
        seeds = [b"locker", signer.key().as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(locker_id: u64)]
pub struct BuyLocker<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"locker", signer.key().as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(locker_id: u64)]
pub struct AddStoragePointer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        // realloc = 8 + Locker::INIT_SPACE + ((locker.storage_pointers.len() + 1) * 32),
        // realloc::payer = signer,
        // realloc::zero = false,
        seeds = [b"locker", signer.key().as_ref(), locker_id.to_le_bytes().as_ref()],
        bump
    )]
    pub locker: Account<'info, Locker>,

    pub system_program: Program<'info, System>,
}

// DATA STRUCT ACCOUNT DEFINITIONS

#[account]
#[derive(InitSpace)]
pub struct Profile {
    #[max_len(50)]
    pub name: String,
    pub age: u8,
    #[max_len(5,50)]
    pub hobbie: Vec<String>,
    #[max_len(250)]
    pub message: String,
}

#[account]
#[derive(InitSpace)]
pub struct Locker {
    pub id: u64,
    #[max_len(1)]
    pub storage_pointers: Vec<Pubkey>,
    pub current_size: f32,
    pub max_size: u16,
    pub data_count: u16,
    pub next_locker: Pubkey
}

// #[account]
// #[derive(InitSpace)]
// pub struct StoragePointerBatch {
//     pub batch_id: u64,
//     pub locker: Pubkey,
//     pub pointers: Vec<StoragePointer>,
// }

#[derive(InitSpace)]
pub struct StoragePointer {
    pub name: [u8; 50], 
    pub file_type: u8,
    pub link: [u8; 200], 
    pub size: f32,
}
