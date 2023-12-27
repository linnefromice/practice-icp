use core::fmt;
use std::process::Command;

use candid::{Principal, Encode, Decode, CandidType};

#[derive(Debug, PartialEq, CandidType, serde::Deserialize)]
struct Canister {
    principal: Principal,
    vault: Principal,
}

#[derive(PartialEq)]
enum Environment {
    Local,
    IC
}
impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s {
            "local" => Environment::Local,
            "ic" => Environment::IC,
            _ => panic!("Invalid environment")
        }
    }
}
impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Local => write!(f, "local"),
            Environment::IC => write!(f, "ic"),
        }
    }
}
impl Environment {
    fn _get_registry_canister_id(&self) -> Principal {
        let principal_str = match self {
            Environment::Local => "uh54g-lyaaa-aaaal-achca-cai",
            Environment::IC => "xd2oy-taaaa-aaaal-qcnva-cai",
        };
        Principal::from_text(principal_str).unwrap()
    }
}

#[tokio::main]
async fn main() {
    // About args
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
        panic!("Invalid number of arguments");
    }
    let canister_id = &args[1];
    println!("canister_id: {}", canister_id);
    let environment_str = &args[2];
    let environment = Environment::from(environment_str.as_str());
    println!("environment: {}", environment);
    let port: u16 = if args.len() == 4 {
        args[3].to_string().parse().unwrap()
    } else {
        8000
    };

    let url = match environment {
        Environment::Local => format!("http://localhost:{}", port),
        Environment::IC => format!("https://ic0.app/"),
    };
    println!("url: {}", url);

    let agent = agent(&url);
    if environment == Environment::Local {
        agent.fetch_root_key().await.unwrap();
    }
    
    let component_id = Principal::from_text(canister_id).unwrap();
    let proxy = get_proxy_from_component(&agent, &component_id).await;
    println!("proxy: {}", proxy);
    let vault = vault_from_proxy(&agent, &proxy).await;
    println!("vault: {}", vault);
    let db = db_from_proxy(&agent, &proxy).await;
    println!("db: {}", db);

    let wallet = get_wallet().unwrap();

    println!("deleting db...");
    delete_canister(&db, &wallet);
    println!("deleted db...");
    println!("deleting vault...");
    delete_canister(&vault, &wallet);
    println!("deleted vault...");
    println!("deleting proxy...");
    delete_canister(&proxy, &wallet);
    println!("deleted proxy...");
    println!("deleting component...");
    delete_canister(&component_id, &wallet);
    println!("deleted component...");

}

fn agent(url: &str) -> ic_agent::Agent {
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_verify_query_signatures(false)
        .build()
        .unwrap();
    agent
}

async fn get_proxy_from_component(agent: &ic_agent::Agent, principal: &Principal) -> Principal {
    let res = agent.update(
        &principal,
        "get_proxy",
    )
    .with_arg(Encode!().unwrap())
    .call_and_wait()
    .await
    .unwrap();
    Decode!(res.as_slice(), Principal).unwrap()
}

async fn vault_from_proxy(agent: &ic_agent::Agent, principal: &Principal) -> Principal {
    let res = agent.query(
        &principal,
        "vault",
    )
    .with_arg(Encode!().unwrap())
    .call()
    .await
    .unwrap();
    Decode!(res.as_slice(), Principal).unwrap()
}

async fn db_from_proxy(agent: &ic_agent::Agent, principal: &Principal) -> Principal {
    let res = agent.query(
        &principal,
        "db",
    )
    .with_arg(Encode!().unwrap())
    .call()
    .await
    .unwrap();
    Decode!(res.as_slice(), Principal).unwrap()
}

async fn _scan_canisters_from_registry(agent: &ic_agent::Agent, principal: &Principal) -> Vec<Canister> {
    let res = agent.update(
        &principal,
        "scanCanisters",
    )
    .with_arg(Encode!().unwrap())
    .call_and_wait()
    .await
    .unwrap();
    Decode!(res.as_slice(), Vec<Canister>).unwrap()
}

fn get_wallet() -> Result<Principal, String> {
    // TODO: consider --ic
    let output = Command::new("dfx")
        .args([
            "identity",
            "get-wallet"
        ])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let msg = std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout");
        Ok(Principal::from_text(msg.replace("\n", "")).unwrap())
    } else {
        let msg = std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr");
        Err(msg.to_string())
    }
}

fn delete_canister(target: &Principal, wallet: &Principal) {
    // TODO: consider --ic
    let output = Command::new("dfx")
        .args([
            "canister",
            "delete",
            "--wallet",
            &wallet.to_text(),
            "--yes",
            &target.to_text(),
        ])
        .output()
        .expect("failed to execute process");
    println!("> stdout");
    println!(
        "{}",
        std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
    );
    println!("> stderr");
    println!(
        "{}",
        std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr")
    );
}