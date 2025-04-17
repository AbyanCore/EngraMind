use anchor_lang::prelude::*;

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