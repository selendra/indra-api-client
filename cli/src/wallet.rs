use crate::models::{Backup, ListWallet, RestoreWallet, Wallet, WatchWallet, AddrMnemonic};
use indracore_api::{
    balance::check_balance::free_balance,
    keyring::accounid32,
    primitives::{token_type, Token},
    wallet::{crypto::*, keystore::Keystore, wallet::*},
};
use colour::dark_cyan_ln;

pub fn get_wallet(wallet: Wallet) -> Result<AddrMnemonic, String> {
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
                    return Err(format!("Invalid Phrase"))
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
    let mnemonic = address.mnemonic.clone();

    address.label = name.to_uppercase();
    address.mnemonic = "".to_string();
    store.save(address.clone());

    return Ok(AddrMnemonic {
        address: address.addr,
        mnemonic: mnemonic
    })
}

pub fn op_get_wallet(wallet: Wallet) {
    match get_wallet(wallet){
        Ok(res) => {
            dark_cyan_ln!(
                "\t>> Address: {}\n\t>> Mnemonic: {}\n\t",
                res.address, res.mnemonic
            );
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
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

pub fn watch_wallet(wl: WatchWallet) {
    let store = WalletStore::init(wl.location.as_deref(), wl.name.as_deref());
    let addr = match wl.address {
        Some(addr) => addr,
        None => {
            println!("Please input address -a <address>");
            std::process::exit(1)
        }
    };
    let _ = match accounid32(&addr) {
        Ok(addr) => addr,
        Err(_) => {
            println!("Invalid address");
            std::process::exit(1)
        }
    };
    let name = match wl.name {
        Some(name) => name,
        None => "USER".to_string(),
    };
    let mut address = Address::default();
    address.addr = addr.clone();
    address.label = name.to_uppercase();
    store.save(address.clone());
    println!("{} is added", addr.clone());
}

pub fn restore_wallet(rw: RestoreWallet) {
    let store = WalletStore::init(rw.location.as_deref(), None);

    let file = match rw.file {
        Some(f) => f,
        None => {
            println!("Input file to restore");
            std::process::exit(1)
        }
    };
    println!("{:?}", file);
    let keystore = match Keystore::parse_from_file(file) {
        Ok(keystore) => keystore,
        Err(_) => {
            println!("Failed to parse keystore file");
            std::process::exit(1)
        }
    };

    if let Ok(address) = Address::from_keystore(keystore, rw.password) {
        store.save(address.clone());
        println!("{} is restored", address.addr);
    } else {
        println!("Failed to recover address");
    }
}

pub fn backup(bp: Backup) {
    let store = WalletStore::init(bp.location.as_deref(), None);
    let file = match bp.file {
        Some(f) => f,
        None => {
            println!("Input file to location to backup");
            std::process::exit(1)
        }
    };

    let addr = match bp.address {
        Some(addr) => addr,
        None => {
            println!("Input addresss to backup");
            std::process::exit(1)
        }
    };

    let address = match store.read(&addr) {
        Some(address) => address,
        None => {
            println!("`{}` related address does not exist.", addr);
            std::process::exit(1)
        }
    };
    let path = std::path::Path::new(&file);
    let full_path = if path.ends_with("/") || path.is_dir() {
        let file_name = format!("{}.json", address.addr.as_str());
        let mut path = path.to_path_buf();
        path.push(file_name);
        path
    } else {
        path.to_path_buf()
    };
    if full_path.exists() {
        eprintln!("File `{}` aleady exists", full_path.to_str().unwrap());
        std::process::exit(1)
    };
    let keystore = address.into_keystore(bp.password);
    if let Err(e) = std::fs::write(full_path.clone(), keystore.to_json()) {
        println!("Failed to write to file: {:?}", e);
    } else {
        println!(
            "Address `{}` is backed up to file `{}`",
            address.addr,
            full_path.to_str().unwrap()
        );
    }
}
