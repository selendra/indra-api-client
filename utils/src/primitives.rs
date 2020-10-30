use std::fmt;
use substrate_subxt::{DefaultNodeRuntime, PairSigner};

pub type Client = substrate_subxt::Client<DefaultNodeRuntime>;
pub type AccountId = pallet_indices::address::Address<sp_core::crypto::AccountId32, u32>;
pub type Signer = PairSigner<DefaultNodeRuntime, sp_core::sr25519::Pair>;

#[derive(Debug, Eq, PartialEq)]
pub struct Transaction {
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub amount: Option<String>,
}
pub struct Token {
    pub token: f64,
}

impl Token {
    pub fn get(token: String) -> Token {
        Token {
            token: token.parse::<f64>().unwrap(),
        }
    }

    pub fn pay(&self) -> u128 {
        dotenv::dotenv().ok();
        let decimal = std::env::var("DECIMAL").unwrap();
        let decimal = 10u128.pow(decimal.parse::<u32>().unwrap() - 9);
        let amount = (self.token * 1_000_000_000.0) as u128;

        amount * decimal
    }

    pub fn token(&self) -> f64 {
        self.token
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pay())
    }
}
