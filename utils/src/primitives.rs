use std::env;
use substrate_subxt::{DefaultNodeRuntime, PairSigner};

pub type Client = substrate_subxt::Client<DefaultNodeRuntime>;
pub type AccountId = pallet_indices::address::Address<sp_core::crypto::AccountId32, u32>;
pub type Signer = PairSigner<DefaultNodeRuntime, sp_core::sr25519::Pair>;

#[derive(Debug)]
pub struct Config {}

impl Config {
    pub fn url() -> String {
        dotenv::dotenv().expect("Failed to read .env file");
        let url = env::var("RPC");
        url.unwrap()
    }

    pub fn decimal() -> u32 {
        let decimal = std::env::var("DECIMAL").unwrap();
        decimal.parse::<u32>().unwrap()
    }

    pub fn token() -> String {
        let token = env::var("TOKEN");
        token.unwrap()
    }
}

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
        let decimal = 10u128.pow(Config::decimal() - 9);
        let amount = (self.token * 1_000_000_000.0) as u128;

        amount * decimal
    }

    pub fn token(&self) -> f64 {
        self.token
    }

    pub fn amount(amount: u128) -> u128 {
        amount.checked_div(10u128.pow(Config::decimal())).unwrap()
    }

    pub fn low_amount(amount: u128) -> f64 {
        let low: f64 = amount as f64 / 10u128.pow(Config::decimal()) as f64;
        low
    }
}
