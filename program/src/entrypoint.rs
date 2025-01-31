use crate::{cpi::cpi_bench, errors};

#[cfg(feature = "solana-program")]
use solana_program::{
    account_info::AccountInfo, entrypoint, log::sol_log_compute_units, msg, pubkey::Pubkey,
};

#[cfg(feature = "solana-program")]
entrypoint!(process_instruction);

#[cfg(feature = "pinocchio")]
use pinocchio::{
    account_info::AccountInfo, entrypoint, log::sol_log_compute_units, msg, pubkey::Pubkey,
    ProgramResult,
};

#[cfg(feature = "pinocchio")]
entrypoint!(process_instruction);

#[cfg(feature = "solana-program")]
type ProgramResult = std::result::Result<(), solana_program::program_error::ProgramError>;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let discriminator = InstructionType::try_from(instruction_data[0]).unwrap();
    match discriminator {
        InstructionType::BenchCopyVsTryInto8 => bench_copy_vs_try_into::<8>(&instruction_data[1..]),
        InstructionType::BenchCopyVsTryInto128 => {
            bench_copy_vs_try_into::<128>(&instruction_data[1..])
        }
        InstructionType::BenchU64FromBytes => bench_u64_from_bytes(&instruction_data[1..]),
        #[cfg(feature = "pinocchio")]
        InstructionType::ProgramAccountCheck => {
            crate::account_check::bench_program_account_check(accounts)?
        }
        InstructionType::CpiBench => cpi_bench(accounts, &instruction_data[1..]),
        _ => panic!("Invalid discriminator"),
    }?;
    Ok(())
}

#[repr(u8)]
pub enum InstructionType {
    Noop = 0,
    BenchCopyVsTryInto8 = 1,
    BenchCopyVsTryInto128 = 2,
    BenchU64FromBytes = 3,
    ProgramAccountCheck = 4,
    CpiBench = 5,
}

impl TryFrom<u8> for InstructionType {
    type Error = crate::errors::Error;

    fn try_from(discriminator: u8) -> std::result::Result<Self, Self::Error> {
        match discriminator {
            0 => Ok(InstructionType::Noop),
            1 => Ok(InstructionType::BenchCopyVsTryInto8),
            2 => Ok(InstructionType::BenchCopyVsTryInto128),
            3 => Ok(InstructionType::BenchU64FromBytes),
            4 => Ok(InstructionType::ProgramAccountCheck),
            5 => Ok(InstructionType::CpiBench),
            _ => Err(errors::Error::InvalidInstruction.into()),
        }
    }
}

// from le bytes 2 CU, from be bytes 4 CU
fn bench_u64_from_bytes(instruction_data: &[u8]) -> ProgramResult {
    msg!("bench_u64_from_bytes");
    let array = instruction_data[0..8].try_into().unwrap();
    {
        sol_log_compute_units();
        let u64_from_le_bytes = u64::from_le_bytes(array);
        sol_log_compute_units();
        if u64_from_le_bytes == 1u64 {
            msg!(format!("noop {}", u64_from_le_bytes).as_str());
        }
    }
    {
        sol_log_compute_units();
        let u64_from_be_bytes = u64::from_be_bytes(array);
        sol_log_compute_units();
        if u64_from_be_bytes == 1u64 {
            msg!(format!("noop {}", u64_from_be_bytes).as_str());
        }
    }
    Ok(())
}

// 8 bytes TryInto 3 CU, Copy is 1 CU
// 128 bytes TryInto 31 CU, Copy is 29 CU
fn bench_copy_vs_try_into<const T: usize>(instruction_data: &[u8]) -> ProgramResult {
    msg!(format!("bench_copy_vs_try_into {}", T).as_str());
    sol_log_compute_units();
    let discriminator: [u8; T] = instruction_data[0..T].try_into().unwrap();
    sol_log_compute_units();
    if discriminator == [1u8; T] {
        msg!(format!("noop {:?}", discriminator).as_str());
    }
    sol_log_compute_units();
    let mut discriminator = [0u8; T];
    discriminator.copy_from_slice(&instruction_data[0..T]);
    sol_log_compute_units();
    if discriminator == [1u8; T] {
        msg!(format!("noop {:?}", discriminator).as_str());
    }
    Ok(())
}

// noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV
pub const NOOP_PUBKEY: [u8; 32] = [
    11, 188, 15, 192, 187, 71, 202, 47, 116, 196, 17, 46, 148, 171, 19, 207, 163, 198, 52, 229,
    220, 23, 234, 203, 3, 205, 26, 35, 205, 126, 120, 124,
];
