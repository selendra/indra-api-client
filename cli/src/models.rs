#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    pub location: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Wallet {
    pub label: String,
    pub password: Option<String>,
    pub name: Option<String>,
    pub location: Option<String>,
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