use std::{env, fs, path::PathBuf};

use anyhow::Context;
use ic_agent::{identity::Secp256k1Identity, Agent, Identity};
use ic_utils::interfaces::ManagementCanister;
use tokio::runtime::Runtime;

fn main() {
    println!("Hello, world!");

    let runtime = Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(async {
        execute().await.expect("Failed to execute");
    });
}

async fn execute() -> anyhow::Result<()> {
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
        // .with_url("https://ic0.app") // ic
        .with_url("http://localhost:4943") // local
        .with_identity(identity)
        .build()?;
    agent.fetch_root_key().await?; // local
    let mgr_canister = ManagementCanister::create(&agent);
    let builder_create_canister = mgr_canister
        .create_canister()
        .as_provisional_create_with_amount(None); // for local
    let create_canister_response = builder_create_canister.call_and_wait().await?;
    println!("create_canister_response: {:?}", create_canister_response.0.to_text());

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