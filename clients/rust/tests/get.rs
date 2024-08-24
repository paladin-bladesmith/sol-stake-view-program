use {
    paladin_sol_stake_view_program_client::{
        instructions::GetStakeActivatingAndDeactivating,
        GetStakeActivatingAndDeactivatingReturnData,
    },
    solana_program_test::{tokio, ProgramTest, ProgramTestContext},
    solana_sdk::{
        instruction::InstructionError,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        stake, system_instruction, system_program, sysvar,
        transaction::{Transaction, TransactionError},
    },
    solana_vote_program::{
        vote_instruction,
        vote_state::{VoteInit, VoteState},
    },
    spl_pod::option::PodOption,
};

async fn create_vote(
    context: &mut ProgramTestContext,
    validator: &Keypair,
    voter: &Pubkey,
    withdrawer: &Pubkey,
    vote_account: &Keypair,
) {
    let rent = context.banks_client.get_rent().await.unwrap();
    let rent_voter = rent.minimum_balance(VoteState::size_of());

    let mut instructions = vec![system_instruction::create_account(
        &context.payer.pubkey(),
        &validator.pubkey(),
        rent.minimum_balance(0),
        0,
        &system_program::id(),
    )];
    instructions.append(&mut vote_instruction::create_account_with_config(
        &context.payer.pubkey(),
        &vote_account.pubkey(),
        &VoteInit {
            node_pubkey: validator.pubkey(),
            authorized_voter: *voter,
            authorized_withdrawer: *withdrawer,
            ..VoteInit::default()
        },
        rent_voter,
        vote_instruction::CreateVoteAccountConfig {
            space: VoteState::size_of() as u64,
            ..Default::default()
        },
    ));

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&context.payer.pubkey()),
        &[validator, vote_account, &context.payer],
        context.last_blockhash,
    );

    // ignore errors for idempotency
    let _ = context.banks_client.process_transaction(transaction).await;
}

async fn get_stake_account_rent(context: &mut ProgramTestContext) -> u64 {
    let rent = context.banks_client.get_rent().await.unwrap();
    rent.minimum_balance(std::mem::size_of::<stake::state::StakeStateV2>())
}

async fn create_stake_account(
    context: &mut ProgramTestContext,
    stake: &Keypair,
    authorized: &stake::state::Authorized,
    lockup: &stake::state::Lockup,
    stake_amount: u64,
) -> u64 {
    let lamports = get_stake_account_rent(context).await + stake_amount;
    let transaction = Transaction::new_signed_with_payer(
        &stake::instruction::create_account(
            &context.payer.pubkey(),
            &stake.pubkey(),
            authorized,
            lockup,
            lamports,
        ),
        Some(&context.payer.pubkey()),
        &[&context.payer, stake],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    lamports
}

async fn delegate_stake_account(
    context: &mut ProgramTestContext,
    stake_address: &Pubkey,
    vote: &Pubkey,
) {
    let transaction = Transaction::new_signed_with_payer(
        &[stake::instruction::delegate_stake(
            stake_address,
            &context.payer.pubkey(),
            vote,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

async fn deactivate_stake_account(context: &mut ProgramTestContext, stake_address: &Pubkey) {
    let transaction = Transaction::new_signed_with_payer(
        &[stake::instruction::deactivate_stake(
            stake_address,
            &context.payer.pubkey(),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

async fn setup(context: &mut ProgramTestContext, stake_amount: u64) -> (Pubkey, Pubkey) {
    let vote = Keypair::new();
    create_vote(
        context,
        &Keypair::new(),
        &Pubkey::new_unique(),
        &Pubkey::new_unique(),
        &vote,
    )
    .await;
    let vote = vote.pubkey();

    let stake_account = Keypair::new();
    let _ = create_stake_account(
        context,
        &stake_account,
        &stake::state::Authorized::auto(&context.payer.pubkey()),
        &stake::state::Lockup::default(),
        stake_amount,
    )
    .await;
    let stake_account = stake_account.pubkey();

    delegate_stake_account(context, &stake_account, &vote).await;

    (vote, stake_account)
}

#[tokio::test]
async fn success_undelegated() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given an undelegated stake account

    let stake_amount = 1_000_000_000;
    let stake_account = Keypair::new();
    let payer = context.payer.pubkey();
    let _ = create_stake_account(
        &mut context,
        &stake_account,
        &stake::state::Authorized::auto(&payer),
        &stake::state::Lockup::default(),
        stake_amount,
    )
    .await;
    let stake_account = stake_account.pubkey();

    let ix = GetStakeActivatingAndDeactivating {
        stake: stake_account,
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then we get no data

    let return_data = simulation_results
        .simulation_details
        .unwrap()
        .return_data
        .unwrap();
    assert_eq!(
        return_data.program_id,
        paladin_sol_stake_view_program_client::ID
    );
    let expected_staker: PodOption<Pubkey> = Some(context.payer.pubkey()).try_into().unwrap();
    let expected = GetStakeActivatingAndDeactivatingReturnData {
        staker: expected_staker,
        withdrawer: expected_staker,
        ..Default::default()
    };
    let returned =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data.data)
            .unwrap();
    assert_eq!(&expected, returned);
}

#[tokio::test]
async fn success_activating() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given a stake account

    let stake_amount = 1_000_000_000;
    let (vote, stake_account) = setup(&mut context, stake_amount).await;

    let ix = GetStakeActivatingAndDeactivating {
        stake: stake_account,
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then we get zero effective, all activating, zero deactivating

    let return_data = simulation_results
        .simulation_details
        .unwrap()
        .return_data
        .unwrap();
    assert_eq!(
        return_data.program_id,
        paladin_sol_stake_view_program_client::ID
    );
    let expected_staker: PodOption<Pubkey> = Some(context.payer.pubkey()).try_into().unwrap();
    let expected = GetStakeActivatingAndDeactivatingReturnData {
        staker: expected_staker,
        withdrawer: expected_staker,
        delegated_vote: Some(vote).try_into().unwrap(),
        activating: stake_amount.into(),
        ..Default::default()
    };
    let returned =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data.data)
            .unwrap();
    assert_eq!(&expected, returned);
}

#[tokio::test]
async fn success_effective() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given an effective stake account

    let stake_amount = 1_000_000_000;
    let (vote, stake_account) = setup(&mut context, stake_amount).await;

    let slot = context.genesis_config().epoch_schedule.first_normal_slot + 1;
    context.warp_to_slot(slot).unwrap();

    let ix = GetStakeActivatingAndDeactivating {
        stake: stake_account,
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then we get all effective, zero activating, zero deactivating

    let return_data = simulation_results
        .simulation_details
        .unwrap()
        .return_data
        .unwrap();
    assert_eq!(
        return_data.program_id,
        paladin_sol_stake_view_program_client::ID
    );
    let expected_staker: PodOption<Pubkey> = Some(context.payer.pubkey()).try_into().unwrap();
    let expected = GetStakeActivatingAndDeactivatingReturnData {
        staker: expected_staker,
        withdrawer: expected_staker,
        delegated_vote: Some(vote).try_into().unwrap(),
        effective: stake_amount.into(),
        ..Default::default()
    };
    let returned =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data.data)
            .unwrap();
    assert_eq!(&expected, returned);
}

#[tokio::test]
async fn success_deactivating() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given a deactivating stake account

    let stake_amount = 1_000_000_000;
    let (vote, stake_account) = setup(&mut context, stake_amount).await;

    let slot = context.genesis_config().epoch_schedule.first_normal_slot + 1;
    context.warp_to_slot(slot).unwrap();

    deactivate_stake_account(&mut context, &stake_account).await;

    let ix = GetStakeActivatingAndDeactivating {
        stake: stake_account,
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then we get all effective, zero activating, all deactivating

    let return_data = simulation_results
        .simulation_details
        .unwrap()
        .return_data
        .unwrap();
    assert_eq!(
        return_data.program_id,
        paladin_sol_stake_view_program_client::ID
    );
    let expected_staker: PodOption<Pubkey> = Some(context.payer.pubkey()).try_into().unwrap();
    let expected = GetStakeActivatingAndDeactivatingReturnData {
        staker: expected_staker,
        withdrawer: expected_staker,
        delegated_vote: Some(vote).try_into().unwrap(),
        effective: stake_amount.into(),
        deactivating: stake_amount.into(),
        ..Default::default()
    };
    let returned =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data.data)
            .unwrap();
    assert_eq!(&expected, returned);
}

#[tokio::test]
async fn success_inactive() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given an inactive stake account

    let stake_amount = 1_000_000_000;
    let (_, stake_account) = setup(&mut context, stake_amount).await;

    let slot = context.genesis_config().epoch_schedule.first_normal_slot + 1;
    context.warp_to_slot(slot).unwrap();

    deactivate_stake_account(&mut context, &stake_account).await;

    let slot = slot + context.genesis_config().epoch_schedule.slots_per_epoch;
    context.warp_to_slot(slot).unwrap();

    let ix = GetStakeActivatingAndDeactivating {
        stake: stake_account,
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then we get all zeroes

    let return_data = simulation_results
        .simulation_details
        .unwrap()
        .return_data
        .unwrap();
    assert_eq!(
        return_data.program_id,
        paladin_sol_stake_view_program_client::ID
    );
    let expected = GetStakeActivatingAndDeactivatingReturnData {
        staker: Some(context.payer.pubkey()).try_into().unwrap(),
        withdrawer: Some(context.payer.pubkey()).try_into().unwrap(),
        ..Default::default()
    };
    let returned =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data.data)
            .unwrap();
    assert_eq!(&expected, returned);
}

#[tokio::test]
async fn fail_not_stake_history() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given a stake account, but not the stake history

    let stake_amount = 1_000_000_000;
    let (_, stake_account) = setup(&mut context, stake_amount).await;

    let ix = GetStakeActivatingAndDeactivating {
        stake: stake_account,
        stake_history: Pubkey::new_unique(), // not the stake history
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then it fails.

    assert_eq!(
        simulation_results.result.unwrap().unwrap_err(),
        TransactionError::InstructionError(0, InstructionError::InvalidArgument)
    );
}

#[tokio::test]
async fn success_not_stake() {
    let mut context = ProgramTest::new(
        "paladin_sol_stake_view_program",
        paladin_sol_stake_view_program_client::ID,
        None,
    )
    .start_with_context()
    .await;

    // Given an invalid stake account

    let ix = GetStakeActivatingAndDeactivating {
        stake: Pubkey::new_unique(),
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    // When we get the stake amounts.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let simulation_results = context.banks_client.simulate_transaction(tx).await.unwrap();

    // Then we get all zeroes

    let return_data = simulation_results
        .simulation_details
        .unwrap()
        .return_data
        .unwrap();
    assert_eq!(
        return_data.program_id,
        paladin_sol_stake_view_program_client::ID
    );
    let expected = GetStakeActivatingAndDeactivatingReturnData::default();
    let returned =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data.data)
            .unwrap();
    assert_eq!(&expected, returned);
}
