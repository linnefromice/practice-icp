use std::env;

use tokio::runtime::Runtime;

use crate::dfx_wrapper::{canister_create, ping};

mod dfx_wrapper;

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
    println!("Network: {:?}", args.network);
    println!("Command: {:?}", args.command);

    match args.command.as_str() {
        "ping" => ping(),
        "canister_create" => canister_create(),
        _ => println!("Invalid command"),
    }
}
