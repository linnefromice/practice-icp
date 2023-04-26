use candid::CandidType;
use ic_web3::{
    contract::{tokens::Tokenizable, Error},
    ethabi::Token,
};

#[derive(Copy, Clone, Debug, Default, CandidType)]
pub struct ExchangeRate {
    pub rate: u128,
    pub from_time: u128,
    pub to_time: u128,
}

impl Tokenizable for ExchangeRate {
    fn from_token(token: ic_web3::ethabi::Token) -> Result<Self, ic_web3::contract::Error>
    where
        Self: Sized,
    {
        match token {
            Token::Tuple(tokens) => {
                let rate = tokens
                    .get(0)
                    .and_then(|v| Token::into_uint(v.clone()))
                    .unwrap()
                    .as_u128();

                let from_time = tokens
                    .get(1)
                    .and_then(|v| Token::into_int(v.clone()))
                    .unwrap()
                    .as_u128();

                let to_time = tokens
                    .get(2)
                    .and_then(|v| Token::into_uint(v.clone()))
                    .unwrap()
                    .as_u128();

                Ok(Self {
                    rate,
                    from_time,
                    to_time,
                })
            }
            other => Err(Error::InvalidOutputType(format!(
                "Expected `Tuple`, got {:?}",
                other
            ))),
        }
    }

    fn into_token(self) -> ic_web3::ethabi::Token {
        Token::Tuple(vec![
            Token::Uint(self.rate.into()),
            Token::Uint(self.from_time.into()),
            Token::Uint(self.to_time.into()),
        ])
    }
}
