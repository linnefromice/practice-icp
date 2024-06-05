use std::path::Path;

use anyhow::anyhow;
use candid::Principal;
use ic_agent::Agent;
use ic_utils::{interfaces::WalletCanister, Canister};

pub async fn default_wallet(agent: &Agent, path: String) -> anyhow::Result<WalletCanister> {
    let wallet_identity_str = identity_get_wallet(path).map_err(|e| anyhow!(e))?;
    let wallet_identity = Principal::from_text(wallet_identity_str)?;

    let canister = Canister::builder()
        .with_agent(agent)
        .with_canister_id(wallet_identity)
        .build()?;
    let wallet_canister = WalletCanister::from_canister(canister).await?;
    Ok(wallet_canister)
}

// note: support only default identity's wallet
fn identity_get_wallet(path: String) -> Result<String, String> {
    let args = vec!["identity", "get-wallet"];
    exec_cmd_string_output("dfx", &Path::new(&path), args).map(remove_trailing_newline)
}

fn exec_cmd_string_output(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> Result<String, String> {
    exec_cmd_generic_output(cmd, execution_dir, args, |stdout| {
        Ok(std::str::from_utf8(&stdout).unwrap().to_string())
    })
}

fn exec_cmd_generic_output<T, F>(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
    process_output: F,
) -> Result<T, String>
where
    F: FnOnce(Vec<u8>) -> Result<T, String>,
{
    let output = exec_cmd(cmd, execution_dir, args)
        .unwrap_or_else(|_| panic!("failed to execute process: {}", cmd));
    if output.status.success() {
        process_output(output.stdout)
    } else {
        Err(std::str::from_utf8(&output.stderr).unwrap().to_string())
    }
}

fn exec_cmd(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> std::io::Result<std::process::Output> {
    std::process::Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        // .stdout(Stdio::piped())
        .output()
}

fn remove_trailing_newline<T>(s: T) -> String
where
    T: AsRef<str>,
{
    let s = s.as_ref();
    let mut result = s.to_string();
    if let Some('\n') = result.chars().last() {
        result.pop();
        if let Some('\r') = result.chars().last() {
            result.pop();
        }
    }
    result
}
