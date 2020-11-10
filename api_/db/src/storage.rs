// extern crate subwallet;

use subwallet::wallet::*;
use rustbreak::FileDatabase;
use rustbreak::deser::Bincode;
use std::path::PathBuf;
use std::fs;

const DEFAULT_WALLET_NAME: &'static str = "xt-selendra-db";
// const DEFAULT_WALLET_NAME: &'static str = "polkadot";

pub fn save_to_db(mut address: Address, label: String) {
    let path = Some(&"/home/shing");
    let file = path.map(|v| {
      let mut file = PathBuf::from(v);
      file.push(DEFAULT_WALLET_NAME);
      file
    }).unwrap_or_else(|| {
      let mut file = dirs::home_dir().unwrap();
      file.push(".sel_db");
      file.push(DEFAULT_WALLET_NAME);
      file
    });

    // if !file.exists() {
    //     fs::create_dir_all(file.parent().unwrap()).expect("Failed to create wallet file");
    //     // fs::create_dir(file.parent().unwrap()).expect("Failed to create wallet file");
    // }
  
    let backend = Wallet::new(DEFAULT_WALLET_NAME.to_owned());
    let db = FileDatabase::<Wallet, Bincode>::load_from_path_or(file, backend.clone()).expect("Failed to initialize file database.");

    let data_path = default_path();
    let store = WalletStore::init(data_path.as_path().to_str());

    address.label = label.to_string();
    store.save(address.clone());
}

pub fn read_by_label(label: String) -> Option<Address> {
    let store = WalletStore::init(default_path().as_path().to_str());
    store.read(&label)
}


fn default_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".sel_db");
    if !path.exists() {
      fs::create_dir_all(path.clone()).expect("Failed to create default data path");
    }
    path
}