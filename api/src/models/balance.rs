use crate::utils::primitives::Hash;
#[derive(Deserialize)]
pub struct AccountId {
    pub accountid: String,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub sender: String,
    #[serde(default)]
    pub password: Option<String>,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionOutput {
    pub hash: Hash,
    pub sender: sp_core::crypto::AccountId32,
    pub receiver: sp_core::crypto::AccountId32,
    pub amount: f64,
}
