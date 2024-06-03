use std::path::Path;

use crate::types::Network;

#[allow(dead_code)]
pub fn version() -> Result<String, String> {
    exec_cmd_string_output("dfx", &Path::new("."), vec!["--version"]).map(remove_trailing_newline)
}

pub fn identity_get_wallet(network: Network, path: String) -> Result<String, String> {
    let args = vec![vec!["identity", "get-wallet"], network.args()].concat();
    exec_cmd_string_output("dfx", &Path::new(&path), args).map(remove_trailing_newline)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
