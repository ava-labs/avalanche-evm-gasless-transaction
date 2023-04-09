mod gasless_counter_increment;
mod gasless_faucet_withdraw;

use std::{
    io::{self, Error, ErrorKind},
    str::FromStr,
};

use clap::{crate_version, Command};
use primitive_types::H160;

const NAME: &str = "avalanche-evm-gasless-transaction";

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("{} version: {}", NAME, crate_version!());

    let matches = Command::new(NAME)
        .version(crate_version!())
        .about("avalanche-evm-gasless-transaction")
        .subcommands(vec![
            gasless_counter_increment::command(),
            gasless_faucet_withdraw::command(),
        ])
        .get_matches();

    match matches.subcommand() {
        Some((gasless_counter_increment::NAME, sub_matches)) => {
            let s = sub_matches
                .get_one::<String>("TRUSTED_FORWARDER_CONTRACT_ADDRESS")
                .unwrap_or(&String::new())
                .clone();
            let trusted_forwarder_contract_address = H160::from_str(&s.trim_start_matches("0x"))
                .map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "failed to load H160 from trusted-forwarder-contract-address {}",
                            e
                        ),
                    )
                })?;

            let s = sub_matches
                .get_one::<String>("RECIPIENT_CONTRACT_ADDRESS")
                .unwrap_or(&String::new())
                .clone();
            let recipient_contract_address =
                H160::from_str(&s.trim_start_matches("0x")).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "failed to load H160 from trusted-forwarder-contract-address {}",
                            e
                        ),
                    )
                })?;

            gasless_counter_increment::execute(
                &sub_matches
                    .get_one::<String>("LOG_LEVEL")
                    .unwrap_or(&String::from("info"))
                    .clone(),
                &sub_matches
                    .get_one::<String>("KEY")
                    .unwrap_or(&String::new())
                    .clone(),
                &sub_matches
                    .get_one::<String>("GAS_RELAYER_SERVER_RPC_URL")
                    .unwrap()
                    .clone(),
                &sub_matches
                    .get_one::<String>("CHAIN_RPC_URL")
                    .unwrap()
                    .clone(),
                trusted_forwarder_contract_address,
                recipient_contract_address,
                &sub_matches
                    .get_one::<String>("DOMAIN_NAME")
                    .unwrap()
                    .clone(),
                &sub_matches
                    .get_one::<String>("DOMAIN_VERSION")
                    .unwrap()
                    .clone(),
                &sub_matches.get_one::<String>("TYPE_NAME").unwrap().clone(),
                &sub_matches
                    .get_one::<String>("TYPE_SUFFIX_DATA")
                    .unwrap()
                    .clone(),
                sub_matches.get_flag("SKIP_PROMPT"),
            )
            .await?;
        }

        Some((gasless_faucet_withdraw::NAME, sub_matches)) => {
            let s = sub_matches
                .get_one::<String>("TRUSTED_FORWARDER_CONTRACT_ADDRESS")
                .unwrap_or(&String::new())
                .clone();
            let trusted_forwarder_contract_address = H160::from_str(&s.trim_start_matches("0x"))
                .map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "failed to load H160 from trusted-forwarder-contract-address {}",
                            e
                        ),
                    )
                })?;

            let s = sub_matches
                .get_one::<String>("RECIPIENT_CONTRACT_ADDRESS")
                .unwrap_or(&String::new())
                .clone();
            let recipient_contract_address =
                H160::from_str(&s.trim_start_matches("0x")).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "failed to load H160 from trusted-forwarder-contract-address {}",
                            e
                        ),
                    )
                })?;

            gasless_faucet_withdraw::execute(
                &sub_matches
                    .get_one::<String>("LOG_LEVEL")
                    .unwrap_or(&String::from("info"))
                    .clone(),
                &sub_matches
                    .get_one::<String>("KEY")
                    .unwrap_or(&String::new())
                    .clone(),
                &sub_matches
                    .get_one::<String>("GAS_RELAYER_SERVER_RPC_URL")
                    .unwrap()
                    .clone(),
                &sub_matches
                    .get_one::<String>("CHAIN_RPC_URL")
                    .unwrap()
                    .clone(),
                trusted_forwarder_contract_address,
                recipient_contract_address,
                &sub_matches
                    .get_one::<String>("DOMAIN_NAME")
                    .unwrap()
                    .clone(),
                &sub_matches
                    .get_one::<String>("DOMAIN_VERSION")
                    .unwrap()
                    .clone(),
                &sub_matches.get_one::<String>("TYPE_NAME").unwrap().clone(),
                &sub_matches
                    .get_one::<String>("TYPE_SUFFIX_DATA")
                    .unwrap()
                    .clone(),
                &sub_matches
                    .get_one::<String>("WITHDRAW_AMOUNT_IN_HEX")
                    .unwrap_or(&String::from("0x123456789"))
                    .clone(),
                sub_matches.get_flag("SKIP_PROMPT"),
            )
            .await?;
        }

        _ => unreachable!("unknown sub-command"),
    }

    Ok(())
}
