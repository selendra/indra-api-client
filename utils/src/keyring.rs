use sp_core::{sr25519::Pair, Pair as TraitPair};
use substrate_subxt::{DefaultNodeRuntime, PairSigner};

use colour::dark_red_ln;
use std::panic;
use std::str::FromStr;

use crate::primitives;
pub struct Signer {}

impl Signer {
    pub fn from_mnemonic(mnemonic: &str) -> primitives::Signer {
        let pair = Pair::from_string(mnemonic.as_ref(), None).unwrap();
        PairSigner::<DefaultNodeRuntime, Pair>::new(pair)
    }
    pub fn from_accountid(accountid: &str) -> primitives::AccountId {
        let id = sp_runtime::AccountId32::from_str(accountid).unwrap();
        pallet_indices::address::Address::from(id)
    }

    pub fn to_accountid(mnemonic: &str) -> sp_core::crypto::AccountId32 {
        panic::set_hook(Box::new(|_| {
            dark_red_ln!("!!! something went wrong with your account");
        }));

        let pair = Pair::from_string(mnemonic.as_ref(), None);
        // match pair {
        //     Ok(data) => Ok(sp_core::crypto::AccountId32::from(data.public())),
        //     Err(e) => Err(e),
        // }
        let data = match pair {
            Ok(data) => data.public(),
            Err(_) => panic!(),
        };

        sp_core::crypto::AccountId32::from(data)
    }
}
