use crate::operation::{ListWallet, Wallet};
use indracore_api::{
    balance::check_balance::free_balance,
    keyring::accounid32,
    primitives::{token_type, Token},
    wallet::{crypto::*, wallet::*},
};

pub fn get_wallet(wallet: Wallet) {
    let store = WalletStore::init(wallet.location.as_deref(), wallet.name.as_deref());
    let mut address = match wallet.phrase {
        Some(phrase) => {
            let address = if wallet.label == "ed25519" {
                Address::from_phrase::<Sr25519>(&phrase, wallet.password.as_deref())
            } else if wallet.label == "ecdsa" {
                Address::from_phrase::<Ed25519>(&phrase, wallet.password.as_deref())
            } else {
                Address::from_phrase::<Sr25519>(&phrase, wallet.password.as_deref())
            };
            match address {
                Ok(addr) => addr,
                Err(_) => {
                    println!("Invalid Phrase");
                    std::process::exit(1)
                }
            }
        }
        None => {
            let address = if wallet.label == "ed25519" {
                Address::generate::<Ed25519>(wallet.password.as_deref())
            } else if wallet.label == "ecdsa" {
                Address::generate::<Ecdsa>(wallet.password.as_deref())
            } else {
                Address::generate::<Sr25519>(wallet.password.as_deref())
            };
            address
        }
    };

    let name = match wallet.name {
        Some(name) => name,
        None => "USER".to_string(),
    };
    address.label = name.to_uppercase();
    store.save(address.clone());
    println!("{}", address.addr);
}

pub fn list_wallet(wallet: ListWallet) {
    let store = WalletStore::init(wallet.location.as_deref(), None);
    let addresses = store.read_all();
    println!(
        "{:<15} {:<55} {:<12} {:<25}",
        "Name", "Address", "Crypto", "Balance"
    );
    for address in addresses {
        let id = accounid32(&address.addr).unwrap();
        let balance = free_balance(id);
        let amount = Token::famount(balance.unwrap());
        let bl = format!("{:?} {}", amount, token_type());
        address.print(bl);
    }
}
