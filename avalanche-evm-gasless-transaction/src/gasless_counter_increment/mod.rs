#![allow(deprecated)]

use std::{
    io::{self, stdout},
    sync::Arc,
};

use avalanche_types::{
    evm::{abi, eip712::gsn::Tx},
    jsonrpc::client::evm as json_client_evm,
    key::secp256k1::private_key::Key,
    wallet::evm as wallet_evm,
};
use clap::{Arg, Command};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use dialoguer::{theme::ColorfulTheme, Select};
use ethers::prelude::Eip1559TransactionRequest;
use ethers_core::{
    abi::{Function, Param, ParamType, StateMutability, Token},
    types::transaction::eip2718::TypedTransaction,
    types::{H160, U256},
};
use ethers_providers::Middleware;
use tokio::time::Duration;

pub const NAME: &str = "gasless-counter-increment";

pub fn command() -> Command {
    Command::new(NAME)
        .about("Increments the counter")
        .arg(
            Arg::new("LOG_LEVEL")
                .long("log-level")
                .short('l')
                .help("Sets the log level")
                .required(false)
                .num_args(1)
                .value_parser(["debug", "info"])
                .default_value("info"),
        )
        .arg(
            Arg::new("KEY")
                .long("key")
                .help("Hex-formatted key for signing")
                .required(false)
                .num_args(1),
        )
        .arg(
            Arg::new("GAS_RELAYER_SERVER_RPC_URL")
                .long("gas-relayer-server-rpc-url")
                .help("Gas relayer server RPC URL")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("CHAIN_RPC_URL")
                .long("chain-rpc-url")
                .help("Chain RPC URL")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("TRUSTED_FORWARDER_CONTRACT_ADDRESS")
                .long("trusted-forwarder-contract-address")
                .help("Sets the trusted forwarder contract address")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("RECIPIENT_CONTRACT_ADDRESS")
                .long("recipient-contract-address")
                .help("Sets the recipient contract address")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("DOMAIN_NAME")
                .long("domain-name")
                .help("Sets the domain name (must be registered before)")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("DOMAIN_VERSION")
                .long("domain-version")
                .help("Sets the domain version (must be registered before)")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("TYPE_NAME")
                .long("type-name")
                .help("Sets the type name (must be registered before)")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("TYPE_SUFFIX_DATA")
                .long("type-suffix-data")
                .help("Sets the type suffix data (must be registered before)")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("SKIP_PROMPT")
                .long("skip-prompt")
                .short('s')
                .help("Skips prompt mode")
                .required(false)
                .num_args(0),
        )
}

pub async fn execute(
    log_level: &str,
    key: &str,
    gas_relayer_server_rpc_url: &str,
    chain_rpc_url: &str,
    trusted_forwarder_contract_address: H160,
    recipient_contract_address: H160,
    domain_name: &str,
    domain_version: &str,
    type_name: &str,
    type_suffix_data: &str,
    skip_prompt: bool,
) -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, log_level),
    );

    let no_gas_key = if key.is_empty() {
        Key::generate()?
    } else {
        Key::from_hex(key)?
    };
    let no_gas_key_signer: ethers_signers::LocalWallet =
        no_gas_key.to_ethers_core_signing_key().into();

    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print(format!(
            "\nLoaded keys: '{}'\n",
            no_gas_key.to_public_key().to_eth_address()
        )),
        ResetColor
    )?;

    let chain_id = json_client_evm::chain_id(chain_rpc_url).await.unwrap();
    log::info!(
        "running against {chain_rpc_url}, {chain_id} for forwarder contract {trusted_forwarder_contract_address}, recipient contract {recipient_contract_address}"
    );

    println!();
    if !skip_prompt {
        let options = &[
            format!("No, I am not ready to increment with the recipient contract {recipient_contract_address}."),
            format!("Yes, let's increment with the recipient contract {recipient_contract_address}."),
        ];
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your 'gasless-counter-increment' option")
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap();
        if selected == 0 {
            return Ok(());
        }
    } else {
        log::info!("skipping prompt...")
    }

    log::info!("incrementing");

    let relay_server_provider = wallet_evm::new_provider(
        gas_relayer_server_rpc_url,
        Duration::from_secs(15),
        Duration::from_secs(30),
        10,
        Duration::from_secs(3),
    )
    .unwrap();
    log::info!("created gas relay server provider for {gas_relayer_server_rpc_url}");

    let chain_rpc_provider = wallet_evm::new_provider(
        chain_rpc_url,
        Duration::from_secs(15),
        Duration::from_secs(30),
        10,
        Duration::from_secs(3),
    )
    .unwrap();
    log::info!("created chain rpc server provider for {chain_rpc_url}");

    let tx = Eip1559TransactionRequest::new()
        .chain_id(chain_id.as_u64())
        .to(ethers::prelude::H160::from(
            trusted_forwarder_contract_address.as_fixed_bytes(),
        ))
        .data(get_nonce_calldata(no_gas_key.to_public_key().to_h160()));
    let tx: TypedTransaction = tx.into();
    let output = chain_rpc_provider.call(&tx, None).await.unwrap();
    let forwarder_nonce_no_gas_key = U256::from_big_endian(&output);
    log::info!(
        "forwarder_nonce_no_gas_key: {} {}",
        no_gas_key.to_public_key().to_h160(),
        forwarder_nonce_no_gas_key
    );

    // parsed function of "increment()"
    let func = Function {
        name: "increment".to_string(),
        inputs: vec![],
        outputs: Vec::new(),
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let arg_tokens = vec![];
    let no_gas_recipient_contract_calldata = abi::encode_calldata(func, &arg_tokens).unwrap();
    log::info!(
        "no gas recipient contract calldata: 0x{}",
        hex::encode(no_gas_recipient_contract_calldata.clone())
    );

    let mut relay_tx = Tx::new()
        //
        // make sure this matches with "registerDomainSeparator" call
        .domain_name(domain_name)
        //
        .domain_version(domain_version)
        //
        // local network
        .domain_chain_id(chain_id)
        //
        // trusted forwarder contract address
        .domain_verifying_contract(trusted_forwarder_contract_address)
        .from(no_gas_key.to_public_key().to_h160())
        //
        // contract address that this gasless transaction will interact with
        .to(recipient_contract_address)
        //
        // just some random value, otherwise, estimate gas fails
        .gas(U256::from(30000))
        //
        // contract call needs no value
        .value(U256::zero())
        //
        .nonce(forwarder_nonce_no_gas_key)
        //
        // calldata for contract calls
        .data(no_gas_recipient_contract_calldata)
        //
        .valid_until_time(U256::MAX)
        //
        .type_name(type_name)
        //
        .type_suffix_data(type_suffix_data);

    let chain_rpc_provider_arc = Arc::new(chain_rpc_provider);
    let relay_tx_request = relay_tx
        .sign_to_request_with_estimated_gas_with_retries(
            no_gas_key_signer,
            Arc::clone(&chain_rpc_provider_arc),
            Duration::from_secs(30),
            Duration::from_millis(100),
            U256::from(10000),
        )
        .await
        .unwrap();
    log::info!("relay_tx_request: {:?}", relay_tx_request);

    let signed_bytes: ethers_core::types::Bytes =
        serde_json::to_vec(&relay_tx_request).unwrap().into();

    let pending = relay_server_provider
        .send_raw_transaction(signed_bytes)
        .await
        .unwrap();
    log::info!(
        "pending tx hash {} from 0x{:x}",
        pending.tx_hash(),
        no_gas_key.to_public_key().to_h160()
    );

    Ok(())
}

fn get_nonce_calldata(addr: H160) -> Vec<u8> {
    // parsed function of "getNonce(address from)"
    let func = Function {
        name: "getNonce".to_string(),
        inputs: vec![Param {
            name: "from".to_string(),
            kind: ParamType::Address,
            internal_type: None,
        }],
        outputs: vec![Param {
            name: "nonce".to_string(),
            kind: ParamType::Uint(256),
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let arg_tokens = vec![Token::Address(addr)];
    abi::encode_calldata(func, &arg_tokens).unwrap()
}
