use {
    base64::{prelude::BASE64_STANDARD, Engine},
    clap::{crate_description, crate_name, crate_version, Arg, Command},
    paladin_sol_stake_view_program_client::{
        instructions::GetStakeActivatingAndDeactivating,
        GetStakeActivatingAndDeactivatingReturnData,
    },
    solana_clap_v3_utils::{
        input_parsers::{
            parse_url_or_moniker,
            signer::{SignerSource, SignerSourceParserBuilder},
        },
        input_validators::normalize_to_url_if_moniker,
        keypair::signer_from_path,
    },
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer, sysvar,
        transaction::Transaction,
    },
    std::rc::Rc,
};

struct Config {
    commitment_config: CommitmentConfig,
    default_signer: Box<dyn Signer>,
    json_rpc_url: String,
    verbose: bool,
    websocket_url: String,
}

async fn process_get(
    rpc_client: &RpcClient,
    signer: &dyn Signer,
    stake: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = GetStakeActivatingAndDeactivating {
        stake: *stake,
        stake_history: sysvar::stake_history::id(),
    }
    .instruction();

    let blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(|err| format!("error: unable to get latest blockhash: {err}"))?;

    let transaction =
        Transaction::new_signed_with_payer(&[ix], Some(&signer.pubkey()), &[signer], blockhash);
    let simulation_results = rpc_client.simulate_transaction(&transaction).await.unwrap();
    println!("{simulation_results:?}");
    let return_data = BASE64_STANDARD
        .decode(simulation_results.value.return_data.unwrap().data.0)
        .unwrap();
    let amounts =
        bytemuck::try_from_bytes::<GetStakeActivatingAndDeactivatingReturnData>(&return_data)
            .unwrap();
    println!("{amounts:?}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg({
            let arg = Arg::new("config_file")
                .short('C')
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::new("keypair")
                .value_parser(SignerSourceParserBuilder::default().allow_all().build())
                .long("keypair")
                .value_name("KEYPAIR")
                .takes_value(true)
                .global(true)
                .help("Filepath or URL to a keypair [default: client keypair]"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::new("json_rpc_url")
                .short('u')
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .value_parser(parse_url_or_moniker)
                .help("JSON RPC URL for the cluster [default: value from configuration file]"),
        )
        .subcommand(
            Command::new("get")
                .about("Get staked amount for SOL stake account")
                .arg(
                    Arg::new("address")
                        .value_parser(SignerSourceParserBuilder::default().allow_all().build())
                        .value_name("ADDRESS")
                        .takes_value(true)
                        .required(true)
                        .index(1)
                        .help("Stake account address to get the staked amount of"),
                ),
        )
        .get_matches();

    let (command, matches) = app_matches.subcommand().unwrap();
    let mut wallet_manager: Option<Rc<RemoteWalletManager>> = None;

    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_signer = if let Ok(Some((signer, _))) =
            SignerSource::try_get_signer(matches, "keypair", &mut wallet_manager)
        {
            Box::new(signer)
        } else {
            signer_from_path(
                matches,
                &cli_config.keypair_path,
                "keypair",
                &mut wallet_manager,
            )?
        };

        let json_rpc_url = normalize_to_url_if_moniker(
            matches
                .get_one::<String>("json_rpc_url")
                .unwrap_or(&cli_config.json_rpc_url),
        );

        let websocket_url = solana_cli_config::Config::compute_websocket_url(&json_rpc_url);
        Config {
            commitment_config: CommitmentConfig::confirmed(),
            default_signer,
            json_rpc_url,
            verbose: matches.is_present("verbose"),
            websocket_url,
        }
    };
    solana_logger::setup_with_default("solana=info");

    if config.verbose {
        println!("JSON RPC URL: {}", config.json_rpc_url);
        println!("Websocket URL: {}", config.websocket_url);
    }
    let rpc_client =
        RpcClient::new_with_commitment(config.json_rpc_url.clone(), config.commitment_config);

    match (command, matches) {
        ("get", arg_matches) => {
            let address =
                SignerSource::try_get_pubkey(arg_matches, "address", &mut wallet_manager)?.unwrap();
            process_get(&rpc_client, config.default_signer.as_ref(), &address)
                .await
                .unwrap();
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use {super::*, solana_test_validator::*};

    #[tokio::test]
    async fn test_get() {
        let (test_validator, payer) = TestValidatorGenesis::default().start_async().await;
        let rpc_client = test_validator.get_async_rpc_client();

        assert!(matches!(
            process_get(&rpc_client, &payer, &Pubkey::new_unique()).await,
            Ok(_)
        ));
    }
}
