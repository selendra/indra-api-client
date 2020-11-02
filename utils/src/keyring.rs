use sp_core::{sr25519::Pair, Pair as TraitPair};
use substrate_subxt::{IndracoreNodeRuntime, PairSigner};

use colour::e_red_ln;
use std::str::FromStr;

use crate::primitives;

pub struct Signer {
    mnemonic: String,
}

pub struct AccountId {
    pubkey: String,
}

impl AccountId {
    pub fn new(pubkey: String) -> AccountId {
        AccountId { pubkey: pubkey }
    }

    pub fn accounid(&self) -> primitives::AccountId {
        let id = sp_runtime::AccountId32::from_str(&self.pubkey);
        match id {
            Ok(id) => pallet_indices::address::Address::from(id),
            Err(_) => {
                e_red_ln!("!!! The account id provided is invalid");
                std::process::exit(1)
            }
        }
    }

    pub fn accounid32(&self) -> sp_core::crypto::AccountId32 {
        let id = sp_runtime::AccountId32::from_str(&self.pubkey);
        match id {
            Ok(id) => id,
            Err(_) => {
                e_red_ln!("!!! The account id provided is invalid");
                std::process::exit(1)
            }
        }
    }
}

impl Signer {
    pub fn new(mnemonic: String) -> Signer {
        Signer { mnemonic: mnemonic }
    }
    pub fn pair(&self) -> primitives::Signer {
        let pair = Pair::from_string(&self.mnemonic, None);
        match pair {
            Ok(p) => PairSigner::<IndracoreNodeRuntime, Pair>::new(p),
            Err(_) => {
                e_red_ln!("!!! The mnemonic provided is invalid");
                std::process::exit(1)
            }
        }
    }

    pub fn accountid(&self) -> sp_core::crypto::AccountId32 {
        let pair = Pair::from_string(&self.mnemonic, None);
        match pair {
            Ok(data) => sp_core::crypto::AccountId32::from(data.public()),
            Err(_) => {
                e_red_ln!("!!! The mnemonic provided is invalid");
                std::process::exit(1)
            }
        }
    }
}
