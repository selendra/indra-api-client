use std::env;
use substrate_subxt::{IndracoreNodeRuntime, PairSigner};

pub type AccountId = pallet_indices::address::Address<sp_core::crypto::AccountId32, u32>;
pub type Signer = PairSigner<IndracoreNodeRuntime, sp_core::sr25519::Pair>;
pub type Hash = sp_core::H256;

pub struct Token {
    pub token: f64,
}
#[derive(Debug)]
pub struct Config {}

impl Config {
    pub fn url() -> String {
        dotenv::dotenv().expect("!!! Failed to read .env file");
        let url = env::var("RPC");
        url.unwrap()
    }

    pub fn decimal() -> u32 {
        let decimal = std::env::var("DECIMAL").unwrap();
        decimal.parse::<u32>().unwrap()
    }
}

impl Token {
    pub fn get(token: f64) -> Token {
        Token { token: token }
    }

    pub fn pay(&self) -> u128 {
        let decimal = 10u128.pow(Config::decimal() - 9);
        let amount = (self.token * 1_000_000_000.0) as u128;

        amount * decimal
    }

    pub fn lamount(amount: u128) -> u128 {
        amount.checked_div(10u128.pow(Config::decimal())).unwrap()
    }

    pub fn amount(amount: u128) -> f64 {
        amount as f64 / 10u128.pow(Config::decimal()) as f64
    }
}
