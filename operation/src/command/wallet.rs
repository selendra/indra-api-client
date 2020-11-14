use crate::operation::{ListWallet, Wallet};
use indracore_api::wallet::crypto::*;
use indracore_api::wallet::wallet::*;

pub fn get_wallet(wallet: Wallet) {
    let store = WalletStore::init(wallet.location.as_deref(), wallet.name.as_deref());
    let mut address = if wallet.label == "ed25519" {
        Address::generate::<Ed25519>(wallet.advanced.as_deref())
    } else if wallet.label == "ecdsa" {
        Address::generate::<Ecdsa>(wallet.advanced.as_deref())
    } else {
        Address::generate::<Sr25519>(wallet.advanced.as_deref())
    };

    address.label = wallet.label;
    store.save(address.clone());
    println!("{}", address.addr);
}

pub fn list_wallet(wallet: ListWallet) {
    let store = WalletStore::init(wallet.location.as_deref(), wallet.name.as_deref());
    let addresses = store.read_all();
    for address in addresses {
        address.print();
    }
    let from_address = match store
        .read("5GHEL3MMDMdT4cfRemAKTSLgCUpKSbV8HvjPKu97uTnoywKZ")
        .ok_or("`from` address does not exists")
    {
        Ok(fa) => fa,
        Err(_) => std::process::exit(1),
    };
    if from_address.is_watchonly() {
        println!("error");
        std::process::exit(1);
    }

    println!("{:?}", from_address)
}
