use std::path::Path;

use crate::config::Network;

#[derive(Debug, serde::Deserialize)]
pub struct ResponseTypePing {
    pub ic_api_version: String,
    pub replica_health_status: String,
    pub root_key: Vec<u8>,
    // for local
    pub certified_height: Option<u64>,
    pub impl_version: Option<String>,
    pub impl_hash: Option<String>,
}
pub fn ping(network: Network, path: String) -> Result<ResponseTypePing, String> {
    let args = vec!["ping", network.url()];
    exec_cmd_json_output::<ResponseTypePing>("dfx", &Path::new(&path), args)
}

pub fn canister_create(network: Network, path: String) -> Result<(), String> {
    let args = vec![vec!["canister", "create", "--all"], network.args()].concat();
    exec_cmd_none_output("dfx", &Path::new(&path), args)
}

pub fn build(network: Network, path: String) -> Result<(), String> {
    let args = vec![vec!["build"], network.args()].concat();
    exec_cmd_none_output("dfx", &Path::new(&path), args)
}

pub fn canister_install(network: Network, path: String) -> Result<String, String> {
    let args = vec![vec!["canister", "install", "--all"], network.args()].concat();
    exec_cmd_string_output("dfx", &Path::new(&path), args)
}

pub fn canister_call(network: Network, path: String) -> Result<String, String> {
    let canister_args = vec!["backend_candid", "hello"]; // temp

    let args = vec![vec!["canister", "call"], canister_args, network.args()].concat();
    exec_cmd_string_output("dfx", &Path::new(&path), args)
}

pub fn identity_whoami(path: String) -> Result<String, String> {
    exec_cmd_string_output("dfx", &Path::new(&path), vec!["identity", "whoami"])
        .map(remove_trailing_newline)
}

pub fn identity_get_wallet(network: Network, path: String) -> Result<String, String> {
    let args = vec![vec!["identity", "get-wallet"], network.args()].concat();
    exec_cmd_string_output("dfx", &Path::new(&path), args).map(remove_trailing_newline)
}

pub fn canister_info(
    network: Network,
    path: String,
    canister_name_or_id: String,
) -> Result<String, String> {
    let args = vec![
        vec!["canister", "info"],
        network.args(),
        vec![canister_name_or_id.as_str()],
    ]
    .concat();
    exec_cmd_string_output("dfx", &Path::new(&path), args)
}

pub fn canister_id(
    network: Network,
    path: String,
    canister_name_or_id: String,
) -> Result<String, String> {
    let args = vec![
        vec!["canister", "id"],
        network.args(),
        vec![canister_name_or_id.as_str()],
    ]
    .concat();
    exec_cmd_string_output("dfx", &Path::new(&path), args).map(remove_trailing_newline)
}

fn exec_cmd_none_output(cmd: &str, execution_dir: &Path, args: Vec<&str>) -> Result<(), String> {
    exec_cmd_generic_output(cmd, execution_dir, args, |_stdout| Ok(()))
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

pub fn exec_cmd_json_output<T>(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    exec_cmd_generic_output(cmd, execution_dir, args, |stdout| {
        Ok(serde_json::from_slice(&stdout).unwrap())
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
