use anchor_lang::prelude::*;

pub fn transfer_lamports<'info>(
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

pub fn calculate_rent_and_size(
    current_data_len: usize,
    new_data_len: usize,
) -> Result<(usize, u64)> {
    let rent = Rent::get()?;
    let additional_rent = rent.minimum_balance(new_data_len) - rent.minimum_balance(current_data_len);
    Ok((new_data_len, additional_rent))
}
