use pinocchio::{
    account_info::AccountInfo, log::sol_log_compute_units, msg, pubkey::Pubkey, ProgramResult,
};

use crate::{errors, NOOP_PUBKEY};

// Is program account 19 CU
// If check program account 19 CU
// Match check program account 19 CU
pub fn bench_program_account_check(
    accounts: &[AccountInfo],
) -> Result<
    Result<(), pinocchio::program_error::ProgramError>,
    pinocchio::program_error::ProgramError,
> {
    msg!("ProgramAccountCheck");
    sol_log_compute_units();
    let res = is_program_account(&NOOP_PUBKEY, &accounts[1]);
    sol_log_compute_units();
    if !res {
        return Err(errors::Error::InvalidInstruction.into());
    }
    sol_log_compute_units();
    msg!("If");
    sol_log_compute_units();
    check_program_account_if(&NOOP_PUBKEY, &accounts[1])?;
    sol_log_compute_units();
    msg!("Match");
    sol_log_compute_units();
    check_program_account_match(&NOOP_PUBKEY, &accounts[1])?;
    sol_log_compute_units();
    Ok(Ok(()))
}

pub fn is_program_account(program_id: &Pubkey, account_info: &AccountInfo) -> bool {
    account_info.key() == program_id && account_info.executable()
}

fn check_program_account_if(program_id: &Pubkey, account_info: &AccountInfo) -> ProgramResult {
    if is_program_account(program_id, account_info) {
        Ok(())
    } else {
        Err(errors::Error::InvalidInstruction.into())
    }
}

fn check_program_account_match(program_id: &Pubkey, account_info: &AccountInfo) -> ProgramResult {
    match is_program_account(program_id, account_info) {
        true => Ok(()),
        false => Err(errors::Error::InvalidInstruction.into()),
    }
}
