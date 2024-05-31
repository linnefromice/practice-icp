use std::{env, fs, path::PathBuf};

use anyhow::Context;
use ic_agent::{identity::Secp256k1Identity, Agent, Identity};
use ic_utils::interfaces::ManagementCanister;
use tokio::runtime::Runtime;

use crate::config::Network;

mod config;
mod dfx_wrapper;

#[derive(Debug)]
struct Args {
    network: Network,
}


fn main() {
    println!("Hello, world!");

    let env_args: Vec<String> = env::args().collect();
    let env_args_lens = env_args.len();
    if env_args_lens != 2 {
        println!("Usage: cargo run <network>");
        return;
    }

    let network = if let Some(network_str) = env_args.get(2) {
        Network::from(network_str.clone())
    } else {
        Network::LOCAL
    };

    Runtime::new().expect("Unable to create a runtime").block_on(async {
        execute(Args { network }).await.expect("Failed to execute");
    });
}

async fn execute(args: Args) -> anyhow::Result<()> {
    println!("Args: {:?}", args);
    let Args { network } = args;

    let path = get_path_to_home("~/.config/dfx/identity.json")
        .context("Not found: ~/.config/dfx/identity.json")?;
    let identity_json: serde_json::Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let default_identity = identity_json["default"]
        .as_str()
        .context("No default identity found")?;

    println!("identity: {}", default_identity);

    let entry = keyring::Entry::new(
        "internet_computer_identities",
        &format!("internet_computer_identity_{}", default_identity),
    )?;
    let password = entry.get_password()?;

    let pem = hex::decode(password.clone())?;

    let identity = Secp256k1Identity::from_pem(pem.as_slice())?;

    let result = serde_json::json!({
        "identity_name": default_identity,
        "password": password,
        "pem": String::from_utf8(pem).unwrap(),
        "identity-principal": identity.sender().unwrap().to_string(),
    });
    println!("{}", serde_json::to_string_pretty(&result)?);

    let agent = Agent::builder()
        .with_url(network.url())
        .with_identity(identity)
        .build()?;
    if network == Network::LOCAL {
        agent.fetch_root_key().await?;
    }
    let mgr_canister = ManagementCanister::create(&agent);
    let builder_create_canister = mgr_canister
        .create_canister()
        .as_provisional_create_with_amount(None); // for local
    let create_canister_response = builder_create_canister.call_and_wait().await?;
    println!("create_canister_response: {:?}", create_canister_response.0.to_text());

    let wallet_id = dfx_wrapper::identity_get_wallet(network, ".".to_string());
    println!("wallet id: {}", wallet_id.unwrap());

    Ok(())
}

fn get_home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
fn get_path_to_home(path: &str) -> Option<PathBuf> {
    if path.starts_with('~') {
        get_home_dir().map(|home| home.join(path.trim_start_matches("~/")))
    } else {
        Some(PathBuf::from(path))
    }
}