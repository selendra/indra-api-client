use std::env;
use substrate_subxt::{
    sp_core::{ed25519, sr25519},
    IndracoreNodeRuntime, PairSigner,
};

pub type Sr25519 = PairSigner<IndracoreNodeRuntime, sr25519::Pair>;
pub type Ed25519 = PairSigner<IndracoreNodeRuntime, ed25519::Pair>;
pub type Client = substrate_subxt::Client<IndracoreNodeRuntime>;
pub type IndracoreId = pallet_indices::address::Address<sp_core::crypto::AccountId32, u32>;

pub fn url() -> String {
    dotenv::dotenv().expect("!!! Failed to read .env file");
    let url = env::var("RPC");
    url.unwrap_or("ws://127.0.0.1:9944".to_string())
}
pub fn decimal() -> u32 {
    dotenv::dotenv().expect("!!! Failed to read .env file");
    let decimal = std::env::var("DECIMAL").unwrap();
    decimal.parse::<u32>().unwrap_or(15)
}

pub fn token_type() -> String {
    dotenv::dotenv().expect("!!! Failed to read .env file");
    let token = env::var("TOKEN");
    token.unwrap_or("unit".into())
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
        let decimal = 10u128.pow(decimal() - 9);
        let amount = (self.token * 1_000_000_000.0) as u128;

        amount * decimal
    }

    pub fn token(&self) -> f64 {
        self.token
    }

    pub fn uamount(amount: u128) -> u128 {
        amount.checked_div(10u128.pow(decimal())).unwrap()
    }

    pub fn famount(amount: u128) -> f64 {
        amount as f64 / 10u128.pow(decimal()) as f64
    }
}
