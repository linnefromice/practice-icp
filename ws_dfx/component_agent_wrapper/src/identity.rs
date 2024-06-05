use std::{env, fs, path::PathBuf};

use anyhow::Context;
use ic_agent::identity::Secp256k1Identity;

// todo: support only identity by PEM files are stored in the OS-provided keyring
// https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-identity#dfx-identity-new
pub fn default_identity() -> anyhow::Result<Secp256k1Identity> {
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

    Ok(identity)
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
