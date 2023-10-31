use anyhow::Result;

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use candid::{pretty_check_file, IDLProg, TypeEnv, check_prog};
    use insta::assert_snapshot;

    #[test]
    fn test_compile_from_file() {
        let candid_path = "assets/sample.did";
        let (env, _) = pretty_check_file(Path::new(candid_path)).unwrap();
        let config = candid::bindings::rust::Config::new();
        let result = candid::bindings::rust::compile(&config, &env, &None);
        assert_snapshot!(result);
    }

    #[test]
    fn test_compile_with_actor_from_file() {
        let candid_path = "assets/sample.did";
        let (env, actor) = pretty_check_file(Path::new(candid_path)).unwrap();
        let config = candid::bindings::rust::Config::new();
        let result = candid::bindings::rust::compile(&config, &env, &Some(actor.unwrap()));
        assert_snapshot!(result);
    }

    #[test]
    fn test_compile_from_text() {
        let did = r#"
        type RequestArgsType = nat64;
        type ResponseType = record { value : text; timestamp : nat64 };
        type FunctionType = func (RequestArgsType) -> (ResponseType) query;
    "#;
        let ast: IDLProg = did.to_string().parse().unwrap();

        let mut te = TypeEnv::new();
        let _ = check_prog(&mut te, &ast);
        let config = candid::bindings::rust::Config::new();
        let result = candid::bindings::rust::compile(&config, &te, &None);
        assert_snapshot!(result);
    }

    #[test]
    fn test_prog_from_text() {
        let did: &str = r#"
        type RequestArgsType = nat64;
        type ResponseType = record { value : text; timestamp : nat64 };
        type FunctionType = func (RequestArgsType) -> (ResponseType) query;
    "#;
        let ast: IDLProg = did.to_string().parse().unwrap();
        let mut te = TypeEnv::new();
        let _ = check_prog(&mut te, &ast);
        println!("{:?}", ast);
        println!("{:?}", te.find_type("RequestArgsType").unwrap().to_string());
    }
}