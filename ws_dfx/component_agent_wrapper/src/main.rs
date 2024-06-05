use std::env;

use candid::{Decode, Encode, Principal};
use ic_agent::Agent;
use tokio::runtime::Runtime;
use types::CycleManagements;

mod types;

#[derive(Debug)]
pub struct Args {
    pub target: Principal,
    pub method_name: String,
}

fn main() {
    println!("Hello, world!");

    let env_args: Vec<String> = env::args().collect();
    let env_args_lens = env_args.len();
    if env_args_lens != 3 {
        println!("Usage: cargo run <target> <method_name>");
        return;
    }
    let args = Args {
        target: Principal::from_text(&env_args[1]).unwrap(),
        method_name: env_args[2].clone(),
    };

    Runtime::new()
        .unwrap()
        .block_on(async { execute(args).await.expect("Failed to execute") });
}

async fn execute(args: Args) -> anyhow::Result<()> {
    let agent = Agent::builder()
        .with_url("http://127.0.0.1:4943") // temp
        .build()
        .unwrap();
    agent.fetch_root_key().await?;

    let Args {
        target,
        method_name,
    } = args;

    let call_args = Encode!(&types::Env::LocalDevelopment, &CycleManagements::default())?;
    agent
        .update(&target, method_name)
        .with_arg(call_args)
        .call()
        .await?;

    Ok(())
}
