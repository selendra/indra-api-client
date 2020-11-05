use crate::utils::primitives;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::str::FromStr;
use substrate_subxt::{IndracoreNodeRuntime, PairSigner};

pub struct AccountId {
    pubkey: String,
}

impl AccountId {
    pub fn new(pubkey: String) -> AccountId {
        AccountId { pubkey: pubkey }
    }

    pub fn accountid(&self) -> Result<primitives::AccountId, &str> {
        let id = sp_runtime::AccountId32::from_str(&self.pubkey);
        match id {
            Ok(id) => Ok(pallet_indices::address::Address::from(id)),
            Err(e) => Err(e),
        }
    }

    pub fn accountid32(&self) -> Result<sp_core::crypto::AccountId32, &str> {
        let id = sp_runtime::AccountId32::from_str(&self.pubkey);
        match id {
            Ok(id) => Ok(id),
            Err(e) => Err(e),
        }
    }
}

pub struct Signer {
    mnemonic: String,
}

impl Signer {
    pub fn new(mnemonic: String) -> Signer {
        Signer { mnemonic: mnemonic }
    }
    pub fn pair(&self) -> Result<primitives::Signer, &str> {
        let pair = Pair::from_string(&self.mnemonic, None);
        match pair {
            Ok(p) => Ok(PairSigner::<IndracoreNodeRuntime, Pair>::new(p)),
            Err(_) => Err("Invalid account"),
        }
    }

    pub fn accountid32(&self) -> Result<sp_core::crypto::AccountId32, &str> {
        let pair = Pair::from_string(&self.mnemonic, None);
        match pair {
            Ok(data) => Ok(sp_core::crypto::AccountId32::from(data.public())),
            Err(_) => Err("Invalid account"),
        }
    }
}
