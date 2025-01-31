use pinocchio::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Error {
    #[error("InvalidInstruction")]
    InvalidInstruction,
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
