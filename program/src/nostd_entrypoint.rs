#[cfg(feature = "nostd")]
use solana_nostd_entrypoint::{entrypoint_nostd, NoStdAccountInfo};

use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

#[cfg(feature = "nostd")]
entrypoint_nostd!(process_instruction, 32);

#[cfg(feature = "nostd")]
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[NoStdAccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
