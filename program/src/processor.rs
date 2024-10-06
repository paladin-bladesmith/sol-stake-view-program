use {
    crate::{
        instruction::SolStakeViewInstruction, state::GetStakeActivatingAndDeactivatingReturnData,
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
        stake_history::StakeHistory,
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

fn get_stake_activating_and_deactivating(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let stake_info = next_account_info(accounts_iter)?;
    let stake_history_info = next_account_info(accounts_iter)?;
    if *stake_history_info.key != sysvar::stake_history::id() {
        return Err(ProgramError::InvalidArgument);
    }

    let mut stake_view = GetStakeActivatingAndDeactivatingReturnData::default();

    // if it's not a SOL stake, that's fine, all zeros
    if *stake_info.owner != stake::program::id() {
        set_return_data(bytemuck::bytes_of(&stake_view));
        return Ok(());
    }

    let stake = try_from_slice_unchecked::<stake::state::StakeStateV2>(&stake_info.data.borrow())?;

    if let Some(authorized) = stake.authorized() {
        stake_view.withdrawer = authorized.withdrawer;
    }

    // if not delegated, that's fine, all zeros
    if let Some(delegation) = stake.delegation() {
        let stake_history: StakeHistory = bincode::deserialize(&stake_history_info.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let current_epoch = Clock::get()?.epoch;
        let stake_activation =
            delegation.stake_activating_and_deactivating(current_epoch, &stake_history, Some(0));

        if stake_activation.effective != 0
            || stake_activation.activating != 0
            || stake_activation.deactivating != 0
        {
            stake_view.delegated_vote = delegation.voter_pubkey;
            stake_view.effective = stake_activation.effective;
            stake_view.activating = stake_activation.activating;
            stake_view.deactivating = stake_activation.deactivating;
        }
    }

    set_return_data(bytemuck::bytes_of(&stake_view));
    Ok(())
}
