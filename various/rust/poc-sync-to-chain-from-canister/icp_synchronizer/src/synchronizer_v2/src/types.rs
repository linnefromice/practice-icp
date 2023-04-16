use candid::CandidType;
use ic_web3::{contract::{tokens::Tokenizable, Error}, ethabi::Token};

#[derive(CandidType)]
pub struct AccountInfo {
    pub address: String,
    pub pub_key: String
}

#[derive(Copy, Clone, Debug, Default, CandidType)]
pub struct Round {
    pub round_id: u128,
    pub answer: i128,
    pub started_at: u64,
    pub updated_at: u64
}

impl Tokenizable for Round {
    fn from_token(token: ic_web3::ethabi::Token) -> Result<Self, ic_web3::contract::Error>
        where
            Self: Sized {
        match token {
            Token::Tuple(tokens) => {
                let round_id = tokens
                    .get(0)
                    .and_then(|v| { Token::into_uint(v.clone()) })
                    .unwrap()
                    .as_u128();

                let answer = tokens
                    .get(1)
                    .and_then(|v| { Token::into_int(v.clone()) })
                    .unwrap()
                    .as_u128() as i128; // temp

                let started_at = tokens
                    .get(2)
                    .and_then(|v| { Token::into_uint(v.clone()) })
                    .unwrap()
                    .as_u64();

                let updated_at = tokens
                    .get(3)
                    .and_then(|v| { Token::into_uint(v.clone()) })
                    .unwrap()
                    .as_u64();

                Ok(Self {
                    round_id,
                    answer,
                    started_at,
                    updated_at,
                })
            },
            other => Err(Error::InvalidOutputType(format!("Expected `Tuple`, got {:?}", other))),
        }

    }

    fn into_token(self) -> Token {
        Token::Tuple(vec![
            Token::Uint(self.round_id.into()),
            Token::Int(self.answer.into()),
            Token::Uint(self.started_at.into()),
            Token::Uint(self.updated_at.into()),
        ])
    }
}