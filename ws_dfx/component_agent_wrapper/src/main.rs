use std::env;

use candid::{Encode, Principal};
use ic_agent::{Agent, AgentError, Identity};
use ic_utils::Argument;
use tokio::runtime::Runtime;
use types::{CycleManagements, FunctionName};

mod identity;
mod types;
mod wallet;

#[derive(Debug)]
pub struct Args {
    pub target: Principal,
    pub method_name: FunctionName,
    pub from_anonymous: bool,
}

fn main() {
    println!("Hello, world!");

    let env_args: Vec<String> = env::args().collect();
    let env_args_lens = env_args.len();
    if env_args_lens != 4 {
        println!("Usage: cargo run <target> <method_name> <from_anonymous>");
        return;
    }
    let from_anonymous = if let Some(flag) = env_args.get(3) {
        flag == "true"
    } else {
        false
    };
    let args = Args {
        target: Principal::from_text(&env_args[1]).unwrap(),
        method_name: FunctionName::from(env_args[2].as_str()),
        from_anonymous,
    };

    Runtime::new()
        .unwrap()
        .block_on(async { execute(args).await.expect("Failed to execute") });
}

async fn execute(args: Args) -> anyhow::Result<()> {
    let Args {
        target,
        method_name,
        from_anonymous,
    } = args;

    let (call_args, with_cycles) = match method_name {
        FunctionName::InitIn => (
            Encode!(&types::Env::LocalDevelopment, &CycleManagements::default())?,
            1_600_000_000_000u128,
        ),
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
            (
                Encode!(&contract_addr, &web3_ctx_param, &src_component_id)?,
                0u128,
            )
        }
        FunctionName::SetTask => (Encode!(&60u32, &0u32, &false)?, 0u128),
    };
    println!("method_name: {:?}", &method_name.to_string());
    println!("call_args: {:?}", call_args);

    if from_anonymous {
        println!("> Call by anonymous identity");
        let agent = Agent::builder()
            .with_url("http://127.0.0.1:4943") // temp
            .build()
            .unwrap();
        agent.fetch_root_key().await?;

        let request_id = agent
            .update(&target, method_name.to_string())
            .with_arg(call_args)
            .call()
            .await?;
        println!("{:?}", request_id);
        let res = agent.wait(request_id, target).await?;
        println!("{:?}", res);
    } else {
        println!("> Call by wallet");
        let caller_identity = identity::default_identity()?;
        println!("caller: {:?}", caller_identity.sender().unwrap().to_text());

        let agent = Agent::builder()
            .with_url("http://127.0.0.1:4943") // temp
            .with_identity(caller_identity)
            .build()
            .unwrap();
        agent.fetch_root_key().await?;

        let wallet_canister = wallet::default_wallet(
            &agent,
            ".".to_string(), // temp
        )
        .await?;
        println!("wallet: {:?}", wallet_canister.canister_id().to_text());

        let argument = Argument::from_raw(call_args);
        let res: Result<((),), AgentError> = wallet_canister
            .call128(target, method_name.to_string(), argument, with_cycles)
            .call_and_wait()
            .await;
        println!("{:?}", res);
    }

    Ok(())
}
