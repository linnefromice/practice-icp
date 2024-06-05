use candid::Principal;
use ic_utils::interfaces::management_canister::builders::CanisterSettings;

#[derive(Debug)]
pub struct Args {
    pub network: Network,
    pub is_from_wallet: bool,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(candid::CandidType)]
pub struct UpdateSettingsArgs {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}
