use candid::CandidType;

#[derive(Debug, Default, CandidType, serde::Serialize)]
struct Sample {
    #[serde(rename = "internet-computer")]
    token: Token
}
#[derive(Debug, Default, CandidType, serde::Serialize)]
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    price: u64,
}

fn main() {
    println!("Hello, world!");
    let sample = Sample::default();
    println!("{:?}", &sample);
    // println!("{:?}", Sample::_ty());
}
