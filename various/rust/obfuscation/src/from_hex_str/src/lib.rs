use ic_cdk_macros::query;

const URL: &'static str = "68747470733a2f2f706f6c79676f6e2d6d756d6261692e672e616c6368656d792e636f6d2f76322f36474c497a4935704c306e3462703463336a45535a54526658784535584a5f5a";

#[query]
fn rpc_endpoint() -> String {
    let bytes = hex::decode(URL).unwrap();
    String::from_utf8(bytes).unwrap()
}
