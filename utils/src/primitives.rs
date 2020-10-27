use substrate_subxt::{DefaultNodeRuntime, PairSigner};

pub type Client = substrate_subxt::Client<DefaultNodeRuntime>;
pub type AccountId = pallet_indices::address::Address<sp_core::crypto::AccountId32, u32>;
pub type Signer = PairSigner<DefaultNodeRuntime, sp_core::sr25519::Pair>;

pub struct Token {}

impl Token {
    pub fn amount(token: f64) -> u128 {
        dotenv::dotenv().ok();
        let decimal = std::env::var("DECIMAL").unwrap();
        let decimal = 10u128.pow(decimal.parse::<u32>().unwrap() - 9);
        let amount = (token * 1_000_000_000.0) as u128;

        amount * decimal
    }
}
