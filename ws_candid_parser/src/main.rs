use std::path::Path;

use anyhow::Result;
use candid::pretty_check_file;

fn main() -> Result<()> {
    let candid_path = "assets/sample.did";
    let (env, actor) = pretty_check_file(Path::new(candid_path))?;
    let config = candid::bindings::rust::Config::new();

    let result = candid::bindings::rust::compile(&config, &env, &None);
    println!("{}", result);

    if let Some(actor) = actor {
        let result = candid::bindings::rust::compile(&config, &env, &Some(actor));
        println!("{}", result);
    }

    Ok(())
}
