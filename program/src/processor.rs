use {
    crate::instruction::SolStakeViewInstruction,
    num_traits::FromPrimitive,
    solana_program::{
        account_info::{next_account_info, AccountInfo}, clock::Clock, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey, stake, sysvar::{self, Sysvar}, program::set_return_data,
    }
};

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidArgument);
    }
    let instruction = SolStakeViewInstruction::from_u8(instruction_data[0]).ok_or(ProgramError::InvalidArgument)?;
    match instruction {
        SolStakeViewInstruction::GetStakeActivatingAndDeactivating => {
            msg!("Instruction: GetStakeActivatingAndDeactivating");
            get_stake_activating_and_deactivating(accounts)
        }
    }
}

const RETURN_DATA_SIZE: usize = 56;

fn get_stake_activating_and_deactivating(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let stake_info = next_account_info(accounts_iter)?;
    if *stake_info.owner != stake::program::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let stake_history_info = next_account_info(accounts_iter)?;
    if *stake_history_info.key != sysvar::stake_history::id() {
        return Err(ProgramError::InvalidArgument);
    }

    let mut return_data = [0u8; RETURN_DATA_SIZE];
    let stake = try_from_slice_unchecked::<stake::state::StakeStateV2>(&stake_info.data.borrow())?;

    // if not delegated, that's fine, all zeros
    if let Some(delegation) = stake.delegation() {
        let stake_history = bincode::deserialize(&stake_history_info.data.borrow()).map_err(|_| ProgramError::InvalidAccountData)?;
        let current_epoch = Clock::get()?.epoch;
        let stake_activation = delegation.stake_activating_and_deactivating(current_epoch, &stake_history, Some(0));

        if stake_activation.effective != 0 || stake_activation.activating != 0 || stake_activation.deactivating != 0 {
            return_data[0..32].copy_from_slice(delegation.voter_pubkey.as_ref());
            return_data[32..40].copy_from_slice(&stake_activation.effective.to_le_bytes());
            return_data[40..48].copy_from_slice(&stake_activation.activating.to_le_bytes());
            return_data[48..56].copy_from_slice(&stake_activation.deactivating.to_le_bytes());
        }
    }

    set_return_data(&return_data);
    Ok(())
}
