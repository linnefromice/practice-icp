use std::{env, fs, path::PathBuf};

use anyhow::Context;
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

    let result = serde_json::json!({
        "identity_name": default_identity,
        "password": password,
    });
    println!("{}", serde_json::to_string_pretty(&result)?);

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