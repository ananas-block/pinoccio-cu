pub mod account_check;
mod cpi;
pub mod entrypoint;
pub mod errors;
#[cfg(feature = "nostd")]
pub mod nostd_entrypoint;

// noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV
pub const NOOP_PUBKEY: [u8; 32] = [
    11, 188, 15, 192, 187, 71, 202, 47, 116, 196, 17, 46, 148, 171, 19, 207, 163, 198, 52, 229,
    220, 23, 234, 203, 3, 205, 26, 35, 205, 126, 120, 124,
];

#[repr(u8)]
pub enum InstructionType {
    Noop = 0,
    BenchCopyVsTryInto8 = 1,
    BenchCopyVsTryInto128 = 2,
    BenchU64FromBytes = 3,
    ProgramAccountCheck = 4,
    CpiBench = 5,
    Entrypoint = 6,
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
            6 => Ok(InstructionType::Entrypoint),
            _ => Err(errors::Error::InvalidInstruction.into()),
        }
    }
}
