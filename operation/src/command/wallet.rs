use crate::operation::{ListWallet, Wallet};
use indracore_api::wallet::crypto::*;
use indracore_api::wallet::wallet::*;

pub fn get_wallet(wallet: Wallet) {
    let store = WalletStore::init(wallet.location.as_deref(), wallet.name.as_deref());
    let mut address = if wallet.label == "ed25519" {
        Address::generate::<Ed25519>(wallet.password.as_deref())
    } else if wallet.label == "ecdsa" {
        Address::generate::<Ecdsa>(wallet.password.as_deref())
    } else {
        Address::generate::<Sr25519>(wallet.password.as_deref())
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
}
