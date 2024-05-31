#[derive(Debug, PartialEq)]
pub enum Network {
    LOCAL,
    IC,
}
impl From<String> for Network {
    fn from(env: String) -> Self {
        match env.as_str() {
            "local" => Network::LOCAL,
            "ic" => Network::IC,
            _ => panic!("Invalid network environment"),
        }
    }
}
impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::LOCAL => "http://localhost:4943",
            Network::IC => "https://ic0.app",
        }
    }

    pub fn args(&self) -> Vec<&str> {
        match self {
            Network::LOCAL => vec![],
            Network::IC => vec!["--network", "ic"],
        }
    }
}
