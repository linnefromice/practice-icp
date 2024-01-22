use std::{collections::BTreeMap, fs::File, io::Write, path::Path, process::{Command, Output}};

use candid::{Encode, Decode};
use ic_agent::{export::Principal, Agent};
use serde::{Deserialize, Serialize};

const IC_URL: &str = "https://ic0.app";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Summary {
    pub info: CanisterInfo,
    pub metadata: Metadata,
}
#[tokio::main]
async fn main() {
    // Get canister ids from canister_ids.json
    let file = File::open("resources/canister_ids.json").unwrap();
    let canisters = get_canister_infos(file);

    // Get metadata from canister
    for canister_info in canisters {
        let agent = agent(IC_URL);
        let canister_id = &canister_info.id.replace("\"", "");
        // let canister_id = "q6yxa-fiaaa-aaaag-qc5sq-cai";
        println!("canister_id: {}", canister_id);
        let (metadata, snapshot) = check_data(canister_id, &canister_info.name, &agent).await;

        // write data
        let summary = Summary {
            info: canister_info.clone(),
            metadata
        };
        let path = format!("output/{}.json", &canister_info.name);
        let mut f = std::fs::OpenOptions::new().create_new(true).write(true).open(&path).unwrap();
        let serialized: String = serde_json::to_string(&summary).unwrap();
        f.write_all(serialized.as_bytes()).unwrap();
        f.flush().unwrap();
        if let Some(snapshot) = snapshot {
            let path = format!("output/{}_snapshot.json", &canister_info.name);
            let mut f = std::fs::OpenOptions::new().create_new(true).write(true).open(&path).unwrap();
            let serialized: String = serde_json::to_string(&snapshot).unwrap();
            f.write_all(serialized.as_bytes()).unwrap();
            f.flush().unwrap();
        }
    }
}

fn agent(url: &str) -> Agent {
    Agent::builder()
        .with_url(url)
        .with_verify_query_signatures(false)
        .build()
        .unwrap()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CanisterInfo {
    pub name: String,
    pub id: String
}
fn get_canister_infos(file: File) -> Vec<CanisterInfo> {
    let canisters: BTreeMap<String, serde_json::Value> = serde_json::from_reader(file).unwrap();
    canisters.iter().map(|(name, info)| {
        CanisterInfo {
            name: name.to_string(),
            id: info["ic"].to_string()
        }
    }).collect()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SnapshotInfo {
    pub last_snapshot: String,
    pub snapshots_len: u64
}
async fn check_data(canister_id: &str, canister_name: &str, agent: &Agent) -> (Metadata, Option<SnapshotInfo>) {
    let metadata = get_metadata(canister_id);
    let principal = Principal::from_text(canister_id).unwrap();
    let snapshot_info = if metadata.type_.starts_with("snapshot_indexer_") {
        let snapshots_len = call_snapshots_len(&agent, &principal).await.unwrap();
        let last_snapshot = call_last_snapshot(
            canister_id,
            Path::new("."),
            format!("resources/{}.did", canister_name)
        ).unwrap();
        Some(SnapshotInfo {
            last_snapshot: std::str::from_utf8(&last_snapshot.stdout).unwrap().to_string(),
            snapshots_len
        })
    } else {
        None
    };
    (metadata, snapshot_info)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

fn output_by_exec_cmd(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> std::io::Result<Output> {
    Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        .output()
}

fn call_last_snapshot(
    canister_id: &str,
    execution_dir: &Path,
    did_path: String,
) -> std::io::Result<Output> {
    let args = vec!["canister", "--ic", "call", canister_id, "get_last_snapshot_value", "--candid", &did_path];
    output_by_exec_cmd("dfx", execution_dir, args)
}

async fn call_snapshots_len(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<u64> {
    let res = agent
        .query(principal, "snapshots_len")
        .with_arg(Encode!().unwrap())
        .call()
        .await?;
    Ok(Decode!(res.as_slice(), u64).unwrap())
}
