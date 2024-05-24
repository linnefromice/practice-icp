use std::env;

use ic_agent::export::Principal;
use tokio::runtime::Runtime;

#[derive(Debug)]
enum NetworkEnv {
    LOCAL,
    IC,
}
impl From<String> for NetworkEnv {
    fn from(env: String) -> Self {
        match env.as_str() {
            "local" => NetworkEnv::LOCAL,
            "ic" => NetworkEnv::IC,
            _ => panic!("Invalid network environment"),
        }
    }
}
impl NetworkEnv {
    fn url(&self) -> &str {
        match self {
            NetworkEnv::LOCAL => "http://localhost:4943",
            NetworkEnv::IC => "https://ic0.app",
        }
    }
}

#[derive(Debug)]
struct Args {
    env: NetworkEnv,
    command: String,
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    if env_args.len() != 3 {
        println!("Usage: ic-cli <env> <command>");
        return;
    }
    let args = Args {
        env: NetworkEnv::from(env_args[1].clone()),
        command: env_args[2].clone(),
    };

    let runtime = Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(execute(args));
}

async fn execute(args: Args) {
    let agent = generate_agent(args.env.url());
    if let NetworkEnv::LOCAL = args.env {
        agent.fetch_root_key().await.unwrap();
    }

    let subnet_id =
        Principal::from_text("2fq7c-slacv-26cgz-vzbx2-2jrcs-5edph-i5s2j-tck77-c3rlz-iobzx-mqe")
            .unwrap();
    let metrics = agent.read_state_subnet_metrics(subnet_id).await.unwrap();
    println!("{:?}", metrics);
}

pub fn generate_agent(url: &str) -> ic_agent::Agent {
    ic_agent::Agent::builder()
        .with_url(url)
        .with_verify_query_signatures(false)
        .build()
        .unwrap()
}
