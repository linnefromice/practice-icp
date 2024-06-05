use std::env;

use candid::{Encode, Principal};
use ic_agent::Agent;
use tokio::runtime::Runtime;
use types::{CycleManagements, FunctionName};

mod types;

#[derive(Debug)]
pub struct Args {
    pub target: Principal,
    pub method_name: FunctionName,
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
        method_name: FunctionName::from(env_args[2].as_str()),
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

    let call_args = match method_name {
        FunctionName::InitIn => {
            Encode!(&types::Env::LocalDevelopment, &CycleManagements::default())
        }
        FunctionName::Setup => {
            // note: support only relayer
            let contract_addr = "0539a0EF8e5E60891fFf0958A059E049e43020d9";
            let web3_ctx_param = types::Web3CtxParam {
                env: types::Env::LocalDevelopment,
                url: "https://eth.llamarpc.com".to_string(),
                from: None,
                chain_id: 1u64,
            };
            let src_component_id = target.to_text(); // ex: "bnz7o-iuaaa-aaaaa-qaaaa-cai";
            Encode!(&contract_addr, &web3_ctx_param, &src_component_id)
        }
        FunctionName::SetTask => Encode!(&60u32, &0u32, &false),
    }?;
    println!("method_name: {:?}", &method_name.to_string());
    println!("call_args: {:?}", call_args);

    let request_id = agent
        .update(&target, method_name.to_string())
        .with_arg(call_args)
        .call()
        .await?;
    println!("{:?}", request_id);
    let res = agent.wait(request_id, target).await?;
    println!("{:?}", res);

    Ok(())
}
