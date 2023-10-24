use std::path::Path;

use anyhow::Result;
use candid::pretty_check_file;

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    #[test]
    fn test_env_from_file() {
        let candid_path = "assets/sample.did";
        let (env, _) = pretty_check_file(Path::new(candid_path)).unwrap();
        let config = candid::bindings::rust::Config::new();
        let result = candid::bindings::rust::compile(&config, &env, &None);
        assert_snapshot!(result);
    }

    #[test]
    fn test_actor_from_file() {
        let candid_path = "assets/sample.did";
        let (env, actor) = pretty_check_file(Path::new(candid_path)).unwrap();
        let config = candid::bindings::rust::Config::new();
        let result = candid::bindings::rust::compile(&config, &env, &Some(actor.unwrap()));
        assert_snapshot!(result);
    }
}