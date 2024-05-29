use std::{env, fmt::Debug};

use config::Network;
use tokio::runtime::Runtime;

use crate::dfx_wrapper::{
    build, canister_call, canister_create, canister_id, canister_info, canister_install,
    identity_get_wallet, identity_whoami, ping,
};

mod config;
mod dfx_wrapper;

#[derive(Debug)]
struct Args {
    command: String,
    path: String,
    network: Network,
    canister_name_or_id: Option<String>,
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let env_args_lens = env_args.len();
    if env_args_lens <= 1 || 5 < env_args_lens {
        println!("Usage: cargo run <command> <env>");
        return;
    }

    let network = if let Some(network_str) = env_args.get(3) {
        Network::from(network_str.clone())
    } else {
        Network::LOCAL
    };
    let args = Args {
        command: env_args[1].clone(),
        path: env_args[2].clone(),
        network,
        canister_name_or_id: env_args.get(4).cloned(),
    };
    let runtime = Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(execute(args));
}

async fn execute(args: Args) {
    println!("Network: {:?}", args.network);
    println!("Command: {:?}", args.command);

    let res: Box<dyn Debug> = match args.command.as_str() {
        "ping" => Box::new(ping(args.network, args.path)),
        "canister_create" => Box::new(canister_create(args.network, args.path)),
        "build" => Box::new(build(args.network, args.path)),
        "canister_install" => Box::new(canister_install(args.network, args.path)),
        "canister_call" => Box::new(canister_call(args.network, args.path)),
        // for cli
        "identity_whoami" => Box::new(identity_whoami(args.path)),
        "identity_get-wallet" => Box::new(identity_get_wallet(args.network, args.path)),
        "canister_info" => Box::new(canister_info(
            args.network,
            args.path,
            args.canister_name_or_id.unwrap(),
        )),
        "canister_id" => Box::new(canister_id(
            args.network,
            args.path,
            args.canister_name_or_id.unwrap(),
        )),
        _ => Box::new("Invalid command"),
    };
    println!("{:?}", res)
}
