use anchor_lang::prelude::*;

declare_id!("3zxFMG7wjw3RNe4fgrZ258ppxWUt4S5b6g2pugkd3jhd");

const STORAGE_LAMPORT: u64 = 100;
const TOKEN_LAMPORT: u64 = 10;

#[program]
pub mod eternity_sc {
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
            storage_pointers: None,
            current_size: 0.0,
            max_size: amount,
            data_count: 0,
            next_locker: None,
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

    pub fn add_storage_pointer(ctx: Context<AddStoragePointer>, _locker_id: u64, data: StoragePointer) -> Result<()> {
        let locker = &mut ctx.accounts.locker;

        // check if locker is full
        if locker.current_size >= locker.max_size as f32 {
            return Err(CustomErrorCode::LockerLimitExceeded.into())
        }

        // add storage pointer
        if let Some(storage_pointers) = locker.storage_pointers.as_mut() {
            storage_pointers.push(data);
        } else {
            locker.storage_pointers = Some(vec![data]);
        }

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

// PROFILE DEFINITIONS
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

// LOCKER DEFINITIONS
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

#[account]
#[derive(InitSpace)]
pub struct Locker {
    pub id: u64,
    #[max_len(100)]
    pub storage_pointers: Option<Vec<StoragePointer>>,
    pub current_size: f32,
    pub max_size: u16,
    pub data_count: u16,
    pub next_locker: Option<Pubkey>
}

// STORAGE POINTER DEFINITIONS
#[derive(Accounts)]
#[instruction(locker_id: u64)]
pub struct AddStoragePointer<'info> {
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct StoragePointer {
    #[max_len(50)]
    pub name: String, 
    pub file_type: FileType,
    #[max_len(200)]
    pub link: String, 
    pub size: f32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum FileType {
    Document,
    Text,
    Image,
    Video,
    Audio,
    Other,
}