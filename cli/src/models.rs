use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTx {
    pub sender: String,
    pub receiver: String,
    pub amount: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddrMnemonic {
    pub address: String,
    pub mnemonic: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAddress {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceOutput {
    pub token: f64,
    pub symbol: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub hash: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub symbol: String
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    #[serde(default)]
    pub location: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub label: String,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub phrase: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WatchWallet {
    pub address: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RestoreWallet {
    pub file: Option<String>,
    pub location: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ListWallet {
    pub location: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Backup {
    pub address: Option<String>,
    pub file: Option<String>,
    pub location: Option<String>,
    pub password: Option<String>,
}