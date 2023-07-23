#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}! I'm callee.", name)
}
