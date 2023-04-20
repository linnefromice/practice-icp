use ic_cdk_macros::query;

const URL: &'static str = "https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z";

#[query]
fn rpc_endpoint() -> String {
    URL.to_string()
}
