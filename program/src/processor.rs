use {
    crate::{
        instruction::SolStakeViewInstruction, state::{GetStakeActivatingAndDeactivatingReturnData, Wrapper},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh1::try_from_slice_unchecked,
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program::set_return_data,
        program_error::ProgramError,
        pubkey::Pubkey,
        stake,
        sysvar::{self, Sysvar},
    },
};

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidArgument);
    }
    let instruction = SolStakeViewInstruction::try_from(instruction_data[0])
        .map_err(|_| ProgramError::InvalidArgument)?;
    match instruction {
        SolStakeViewInstruction::GetStakeActivatingAndDeactivating => {
            msg!("Instruction: GetStakeActivatingAndDeactivating");
            get_stake_activating_and_deactivating(accounts)
        }
    }
}

fn get_return_data(data: &mut [u8]) -> GetStakeActivatingAndDeactivatingReturnData {
    let val = bytemuck::try_from_bytes_mut::<Wrapper>(data).unwrap();
    val.data
}

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

    let mut return_data = [0u8; std::mem::size_of::<GetStakeActivatingAndDeactivatingReturnData>()];
    let stake = try_from_slice_unchecked::<stake::state::StakeStateV2>(&stake_info.data.borrow())?;

    // if not delegated, that's fine, all zeros
    if let Some(delegation) = stake.delegation() {
        // safe to unwrap since we created this ourselves
        let mut stake_amount = get_return_data(&mut return_data);
        let stake_history = bincode::deserialize(&stake_history_info.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let current_epoch = Clock::get()?.epoch;
        let stake_activation =
            delegation.stake_activating_and_deactivating(current_epoch, &stake_history, Some(0));

        if stake_activation.effective != 0
            || stake_activation.activating != 0
            || stake_activation.deactivating != 0
        {
            stake_amount.delegated_vote = delegation.voter_pubkey;
            stake_amount.effective = stake_activation.effective;
            stake_amount.activating = stake_activation.activating;
            stake_amount.deactivating = stake_activation.deactivating;
        }
    }

    set_return_data(&return_data);
    Ok(())
}
