#[cfg(feature = "pinocchio")]
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Account, AccountMeta, Seed, Signer},
    log::sol_log_compute_units,
    msg, ProgramResult,
};

#[cfg(feature = "solana-program")]
use solana_program::{
    account_info::AccountInfo,
    instruction::AccountMeta,
    instruction::Instruction,
    log::sol_log_compute_units,
    msg,
    program::{invoke, invoke_signed, invoke_signed_unchecked, invoke_unchecked},
    pubkey::Pubkey,
};

use crate::entrypoint::NOOP_PUBKEY;

#[cfg(feature = "pinocchio")]
fn get_pinoccio_seeds<'a>() -> [Seed<'a>; 2] {
    [Seed::from(&[1]), Seed::from(&[255])]
}
#[cfg(feature = "solana-program")]
fn get_solana_program_seeds<'a>() -> [&'a [u8]; 2] {
    [&[1], &[255]]
}

#[cfg(feature = "pinocchio")]
pub fn cpi_bench(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    msg!("cpi_bench");

    cpi_invoke::<true>(accounts, instruction_data)?;
    cpi_invoke::<false>(accounts, instruction_data)?;
    cpi_signed::<true>(accounts, instruction_data)?;
    cpi_signed::<false>(accounts, instruction_data)?;

    Ok(())
}

#[cfg(feature = "solana-program")]
pub fn cpi_bench(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> std::result::Result<(), solana_program::program_error::ProgramError> {
    msg!("cpi_bench");
    cpi_solana_program_invoke::<true>(accounts, instruction_data);
    cpi_solana_program_invoke::<false>(accounts, instruction_data);
    cpi_solana_program_signed::<true>(accounts, instruction_data);
    cpi_solana_program_signed::<false>(accounts, instruction_data);

    Ok(())
}

#[cfg(feature = "pinocchio")]
pub fn cpi_invoke<const CHECK: bool>(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(format!("pinoccio Invoke checked: {}", CHECK).as_str());
    sol_log_compute_units();
    let instruction = pinocchio::instruction::Instruction {
        program_id: &NOOP_PUBKEY,
        accounts: &[AccountMeta::new(accounts[0].key(), true, true)],
        data: instruction_data,
    };
    if CHECK {
        pinocchio::program::invoke(&instruction, &[&accounts[0]]).unwrap();
    } else {
        unsafe {
            pinocchio::program::invoke_unchecked(&instruction, &[Account::from(&accounts[0])]);
        }
    }
    sol_log_compute_units();
    Ok(())
}

#[cfg(feature = "pinocchio")]
pub fn cpi_signed<const CHECK: bool>(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(format!("pinoccio Invoke signed checked {}", CHECK).as_str());
    sol_log_compute_units();
    let seed = get_pinoccio_seeds();
    let seeds = Signer::from(&seed[..]);

    let instruction = pinocchio::instruction::Instruction {
        program_id: &NOOP_PUBKEY,
        accounts: &[AccountMeta::new(accounts[2].key(), true, true)],
        data: instruction_data,
    };
    if CHECK {
        pinocchio::program::invoke_signed(&instruction, &[&accounts[2]], &[seeds])?;
    } else {
        unsafe {
            pinocchio::program::invoke_signed_unchecked(
                &instruction,
                &[Account::from(&accounts[2])],
                &[seeds],
            );
        }
    }
    sol_log_compute_units();
    Ok(())
}

#[cfg(feature = "solana-program")]
pub fn cpi_solana_program_invoke<const CHECK: bool>(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) {
    msg!(format!("solana program Invoke checked {}", CHECK).as_str());
    sol_log_compute_units();
    let account_info = &accounts[0];
    let instruction = solana_program::instruction::Instruction {
        program_id: Pubkey::new_from_array(NOOP_PUBKEY),
        accounts: vec![AccountMeta::new(*account_info.key, true)],
        data: instruction_data.to_vec(),
    };
    if CHECK {
        invoke(&instruction, &[accounts[0].clone()]).unwrap();
    } else {
        invoke_unchecked(&instruction, &[accounts[0].clone()]).unwrap();
    }
    sol_log_compute_units();
}

#[cfg(feature = "solana-program")]
pub fn cpi_solana_program_signed<const CHECK: bool>(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) {
    msg!(format!("solana program Invoke signed checked {}", CHECK).as_str());
    sol_log_compute_units();
    let seeds = get_solana_program_seeds();
    let account_info = &accounts[2];
    let instruction = Instruction {
        program_id: Pubkey::new_from_array(NOOP_PUBKEY),
        accounts: vec![AccountMeta::new(*account_info.key, true)],
        data: instruction_data.to_vec(),
    };
    if CHECK {
        solana_program::program::invoke_signed(&instruction, &[account_info.clone()], &[&seeds])
            .unwrap();
    } else {
        solana_program::program::invoke_signed_unchecked(
            &instruction,
            &[account_info.clone()],
            &[&seeds],
        )
        .unwrap();
    }
    sol_log_compute_units();
}
