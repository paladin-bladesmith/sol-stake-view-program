use {
    crate::processor,
    solana_program::{
        account_info::AccountInfo,
        entrypoint::{self, ProgramResult},
        msg,
        pubkey::Pubkey,
    },
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        msg!("{error}");
        return Err(error);
    }
    Ok(())
}
