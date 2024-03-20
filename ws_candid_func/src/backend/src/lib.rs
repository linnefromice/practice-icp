#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query(name = "greet")]
#[candid::candid_method(query)]
fn greet_with_msg(name: String, msg: String) -> String {
    format!("Hello, {}! {}.", name, msg)
}

#[cfg(test)]
mod tests {
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}