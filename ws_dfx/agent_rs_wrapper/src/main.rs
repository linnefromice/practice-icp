use std::{env, fs, path::PathBuf};

use anyhow::{Context, Ok};
use ic_agent::{export::Principal, identity::Secp256k1Identity, Agent, Identity};
use ic_utils::{
    interfaces::{
        management_canister::builders::{CanisterInstall, InstallMode},
        ManagementCanister, WalletCanister,
    },
    Argument, Canister,
};
use ic_wasm::{
    metadata::{add_metadata, Kind},
    shrink::shrink,
    utils::parse_wasm,
};
use tokio::runtime::Runtime;

use crate::config::Network;

mod config;
mod dfx_wrapper;

// from: dfinity/sdk/src/dfx/src/lib/operations/canister/create_canister.rs
const CANISTER_CREATE_FEE: u128 = 100_000_000_000_u128;
const CANISTER_INITIAL_CYCLE_BALANCE: u128 = 3_000_000_000_000_u128;

const ARTIFACT_PATH: &str = "./dfx-project/artifacts";
const WASM_FILENAME: &str = "backend.wasm";
const DID_FILENAME: &str = "interface.did";
const BUILDED_WASM_FILENAME: &str = "backend_builded.wasm";

#[derive(Debug)]
struct Args {
    network: Network,
    is_from_wallet: bool,
}

fn main() {
    println!("Hello, world!");

    let env_args: Vec<String> = env::args().collect();
    let env_args_lens = env_args.len();
    if env_args_lens != 3 {
        println!("Usage: cargo run <network> <is_from_wallet>");
        return;
    }
    println!("{:?}", env_args);

    let network = if let Some(network_str) = env_args.get(1) {
        Network::from(network_str.clone())
    } else {
        Network::LOCAL
    };
    let is_from_wallet = if let Some(is_from_wallet_str) = env_args.get(2) {
        is_from_wallet_str == "true"
    } else {
        false
    };

    Runtime::new()
        .expect("Unable to create a runtime")
        .block_on(async {
            execute(Args {
                network,
                is_from_wallet,
            })
            .await
            .expect("Failed to execute");
        });
}

async fn execute(args: Args) -> anyhow::Result<()> {
    println!("Args: {:?}", args);
    let Args {
        network,
        is_from_wallet,
    } = args;

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
    if &network == &Network::LOCAL {
        agent.fetch_root_key().await?;
    }

    // create/get wallet by dfx binary
    let wallet_id = dfx_wrapper::identity_get_wallet(network.clone(), ".".to_string());
    let wallet_principal = Principal::from_text(wallet_id.unwrap())?;
    println!("wallet id: {}", wallet_principal.to_text());

    // create canister by crates
    let canister_id = if network == Network::LOCAL && !is_from_wallet {
        create_canister_by_management_canister(&agent).await?
    } else {
        let wallet_canister = wallet_canister(wallet_principal, &agent).await?;
        let res = wallet_canister
            .wallet_create_canister(
                CANISTER_CREATE_FEE + CANISTER_INITIAL_CYCLE_BALANCE,
                None,
                None,
                None,
                None,
            )
            .await?;
        res.canister_id
    };
    println!("canister id (created): {:?}", canister_id.to_text());

    // build
    //// from csx
    let wasm_bytes = fs::read(&format!("{}/{}", ARTIFACT_PATH, WASM_FILENAME))?;
    let mut wasm_module = parse_wasm(&wasm_bytes, false)?;
    shrink(&mut wasm_module);
    //// from dfx: wasm_post_process
    add_metadata(
        &mut wasm_module,
        Kind::Public,
        "candid:service",
        fs::read(&format!("{}/{}", ARTIFACT_PATH, DID_FILENAME))?,
    );
    wasm_module.emit_wasm_file(&format!("{}/{}", ARTIFACT_PATH, BUILDED_WASM_FILENAME))?;

    // install canister by crates
    let wasm_data = fs::read(&format!("{}/{}", ARTIFACT_PATH, BUILDED_WASM_FILENAME))?;
    if network == Network::LOCAL && !is_from_wallet {
        install_canister_by_management_canister(&agent, &canister_id, &wasm_data).await?;
    } else {
        let wallet_canister = wallet_canister(wallet_principal, &agent).await?;
        let install_args = CanisterInstall {
            mode: InstallMode::Install,
            canister_id,
            wasm_module: wasm_data,
            arg: Vec::new(),
        };
        wallet_canister
            .call(
                Principal::management_canister(),
                "install_code",
                Argument::from_candid((install_args,)),
                0,
            )
            .call_and_wait()
            .await?;
    }

    Ok(())
}

async fn create_canister_by_management_canister(agent: &Agent) -> anyhow::Result<Principal> {
    let mgr_canister = ManagementCanister::create(agent);
    let builder = mgr_canister
        .create_canister()
        .as_provisional_create_with_amount(None); // for local
    let res = builder.call_and_wait().await?;
    Ok(res.0)
}

async fn install_canister_by_management_canister(
    agent: &Agent,
    canister_id: &Principal,
    wasm_module: &[u8],
) -> anyhow::Result<()> {
    let mgr_canister = ManagementCanister::create(agent);
    let builder = mgr_canister.install(canister_id, wasm_module);
    builder.call_and_wait().await?;
    Ok(())
}

async fn wallet_canister(id: Principal, agent: &Agent) -> anyhow::Result<WalletCanister> {
    let canister = Canister::builder()
        .with_agent(agent)
        .with_canister_id(id)
        .build()?;
    let wallet_canister = WalletCanister::from_canister(canister).await?;
    Ok(wallet_canister)
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
