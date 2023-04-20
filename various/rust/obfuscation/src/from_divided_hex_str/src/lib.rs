use ic_cdk_macros::query;

const PROTOCOL: &'static str = "6874747073";
const HOST: &'static str = "706f6c79676f6e2d6d756d6261692e672e616c6368656d792e636f6d";
const PATH: &'static str = "7632";
const KEY: &'static str = "36474c497a4935704c306e3462703463336a45535a54526658784535584a5f5a";
const COLON: &'static str = "3a";
const SLASH: &'static str = "2f";

#[query]
fn rpc_endpoint() -> String {
    let hex_str = format!("{}{}{}{}{}{}{}{}{}", PROTOCOL, COLON, SLASH, SLASH, HOST, SLASH, PATH, SLASH, KEY);
    let bytes = hex::decode(hex_str.as_bytes()).unwrap();
    String::from_utf8(bytes).unwrap()
}
