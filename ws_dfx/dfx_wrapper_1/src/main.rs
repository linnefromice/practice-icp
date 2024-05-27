use std::env;

use dfx_core::identity::{
    // keyring_mock::{keyring_available, KEYRING_IDENTITY_PREFIX, KEYRING_SERVICE_NAME},
    IdentityManager,
};
use ic_agent::Identity;
use slog::{Discard, Logger};
use tokio::runtime::Runtime;

mod commands;

#[derive(Debug)]
enum Network {
    LOCAL,
    IC,
}
impl From<String> for Network {
    fn from(env: String) -> Self {
        match env.as_str() {
            "local" => Network::LOCAL,
            "ic" => Network::IC,
            _ => panic!("Invalid network environment"),
        }
    }
}
impl Network {
    fn url(&self) -> &str {
        match self {
            Network::LOCAL => "http://localhost:4943",
            Network::IC => "https://ic0.app",
        }
    }
}

#[derive(Debug)]
struct Args {
    command: String,
    network: Network,
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let env_args_lens = env_args.len();
    if env_args_lens <= 1 || 3 < env_args_lens {
        println!("Usage: cargo run <command> <env>");
        return;
    }

    let network = if let Some(network_str) = env_args.get(2) {
        Network::from(network_str.clone())
    } else {
        Network::LOCAL
    };
    let args = Args {
        command: env_args[1].clone(),
        network,
    };

    let runtime = Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(execute(args));
}

async fn execute(args: Args) {
    println!("CARGO_PKG_VERSION is {:?}", env!("CARGO_PKG_VERSION"));

    let logger = Logger::root(Discard, slog::o!());
    let mut identity_mgr = IdentityManager::new(&logger, &None).unwrap();
    // let identity_config =
    //     identity_mgr.get_identity_config_or_default(identity_mgr.get_selected_identity_name());
    // note: if error in keychain, set "Allow all applications" to internet_computer_identities / internet_computer_identity_(name) in Keychain access.
    let identity: Box<dyn Identity + Send + Sync> =
        identity_mgr.instantiate_selected_identity(&logger).unwrap();

    let agent = generate_agent(args.network.url(), identity);
    // let agent = generate_agent(args.network.url(), _identity);
    if let Network::LOCAL = args.network {
        agent.fetch_root_key().await.unwrap();
    }

    println!("Your Command is `{}`", args.command);
    match args.command.as_str() {
        "ping" => {
            let res = commands::ping(&agent).await;
            println!("{:?}", res);
        }
        "canister_create" => {
            // let res = commands::canister_create(logger, &agent).await;
            let res = commands::canister_create(logger).await;
            println!("{:?}", res);
        }
        "build" => {}
        "canister_install" => {}
        "canister_call" => {}
        _ => {
            panic!("Invalid command");
        }
    }
}

pub fn generate_agent(url: &str, identity: Box<dyn Identity + Send + Sync>) -> ic_agent::Agent {
    ic_agent::Agent::builder()
        .with_url(url)
        .with_boxed_identity(identity)
        .with_verify_query_signatures(false)
        .build()
        .unwrap()
}
