use std::path::Path;

pub fn ping() {
    let res = exec_cmd_string_output("dfx", &Path::new("."), vec!["ping"]);
    println!("{:?}", res);
}

pub fn canister_create() {
    // note: fail because no response ("")
    let _ = exec_cmd_string_output("dfx", &Path::new("."), vec!["canister", "create", "--all"]);
    // println!("{:?}", res);
}

pub fn exec_cmd_string_output(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> Result<String, String> {
    match exec_cmd_output(cmd, execution_dir, args) {
        Ok(stdout) => Ok(std::str::from_utf8(&stdout).unwrap().to_string()),
        Err(stderr) => Err(std::str::from_utf8(&stderr).unwrap().to_string()),
    }
}

pub fn exec_cmd_json_output<T>(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    match exec_cmd_output(cmd, execution_dir, args) {
        Ok(stdout) => Ok(serde_json::from_slice(&stdout).unwrap()),
        Err(stderr) => Err(std::str::from_utf8(&stderr).unwrap().to_string()),
    }
}

pub fn exec_cmd_output(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> Result<Vec<u8>, Vec<u8>> {
    let output = exec_cmd(cmd, execution_dir, args)
        .unwrap_or_else(|_| panic!("failed to execute process: {}", cmd));
    if output.status.success() {
        Ok(output.stdout.clone())
    } else {
        Err(output.stderr.clone())
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
