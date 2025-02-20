use anchor_lang::prelude::*;

#[error_code]
pub enum TitaErrorCode {
    #[msg("Insufficient funds for your ask")]
    InsufficientFunds,

    #[msg("An error occured while calculating balance")]
    CalculationError
}