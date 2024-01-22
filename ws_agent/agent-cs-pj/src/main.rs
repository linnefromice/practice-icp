use std::{path::Path, process::{Command, Output}};

use ic_agent::{export::Principal, Agent};

const IC_URL: &str = "https://ic0.app";

fn main() {
    let _agent = agent(IC_URL);

    let canister_id = "q6yxa-fiaaa-aaaag-qc5sq-cai";
    let _principal = Principal::from_text(canister_id).unwrap();

    let metadata = get_metadata(canister_id);
    println!("{:?}", metadata);
}

fn agent(url: &str) -> Agent {
    Agent::builder()
        .with_url(url)
        .with_verify_query_signatures(false)
        .build()
        .unwrap()
}

#[derive(Debug)]
struct Metadata {
    pub id: String,
    pub label: String,
    pub description: String,
    pub type_: String,
    pub interval_sec: String,
    pub tags: String
}
fn get_metadata(canister_id: &str) -> Metadata {
    let current_dir = Path::new(".");
    let base_args = vec!["canister", "--ic", "metadata", canister_id];

    let label = output_by_exec_cmd(
        "dfx",
        current_dir,
        vec![base_args.clone(), vec!["chainsight:label"]].concat()
    ).unwrap();
    let description = output_by_exec_cmd(
        "dfx",
        current_dir,
        vec![base_args.clone(), vec!["chainsight:description"]].concat()
    ).unwrap();
    let component_type = output_by_exec_cmd(
        "dfx",
        current_dir,
        vec![base_args.clone(), vec!["chainsight:component_type"]].concat()
    ).unwrap();
    let interval_sec = output_by_exec_cmd(
        "dfx",
        current_dir,
        vec![base_args.clone(), vec!["chainsight:intervalSec"]].concat()
    ).unwrap();
    let tags = output_by_exec_cmd(
        "dfx",
        current_dir,
        vec![base_args.clone(), vec!["chainsight:tags"]].concat()
    ).unwrap();

    Metadata {
        id: canister_id.to_string(),
        label: std::str::from_utf8(&label.stdout).unwrap().to_string(),
        description: std::str::from_utf8(&description.stdout).unwrap().to_string(),
        type_: std::str::from_utf8(&component_type.stdout).unwrap().to_string(),
        interval_sec: std::str::from_utf8(&interval_sec.stdout).unwrap().to_string(),
        tags: std::str::from_utf8(&tags.stdout).unwrap().to_string(),
    }
}

pub fn output_by_exec_cmd(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> std::io::Result<Output> {
    Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        .output()
}