use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not rent exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Expected amount mismatch
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,
    /// Amount overflow
    #[error("Amount overflow. Get more Lamports")]
    AmountOverflow
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}