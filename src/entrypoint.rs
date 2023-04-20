/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-04-19 11:34:31
 */
use crate::{error::TokenError, state::State};
use solana_program::{
    account_info::AccountInfo,
    program_error::PrintProgramError, pubkey::Pubkey,
};
use solana_program::{entrypoint,entrypoint::ProgramResult};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = State::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}
