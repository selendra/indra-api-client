#[derive(Deserialize)]
pub struct AccountId {
    pub accountid: String,
}

#[derive(Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    #[serde(default = "empty_value")]
    pub password: String,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionOutput {
    pub hash: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

fn empty_value() -> String {
    "".to_string()
}
