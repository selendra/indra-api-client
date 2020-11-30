use crate::operation::Transaction;
use colour::{dark_cyan_ln, e_red_ln};
use indracore_api::wallet::{crypto::*, wallet::*};
use indracore_api::{
    balance::check_balance::{free_balance, total_issuance},
    keyring::{accounid32, indracoreid},
    primitives::{token_type, url, Token},
    substrate_subxt::{
        balances::*,
        sp_core::{ecdsa as Ec, ed25519 as Ed, sr25519 as Sr},
        ClientBuilder, IndracoreNodeRuntime, PairSigner,
    },
};

pub fn run_transaction(tx: Transaction) {
    let store = WalletStore::init(tx.location.as_deref(), None);

    let from_address = store
        .read(&tx.sender)
        .ok_or("This Addrees not exit")
        .unwrap();
    if from_address.is_watchonly() {
        e_red_ln!("This account only for watch");
        std::process::exit(1);
    }

    let public_id = from_address.addr.clone();
    let reciever = match indracoreid(&tx.receiver) {
        Ok(p) => p,
        Err(e) => {
            e_red_ln!("{:?}", format!("{:?}", e));
            std::process::exit(1)
        }
    };
    let amount = Token::get(tx.amount.clone());
    let hash = async_std::task::block_on(async move {
        let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
            .set_url(url())
            .build()
            .await
        {
            Ok(cli) => cli,
            Err(e) => {
                println!("{:?}", format!("{:?}", e));
                std::process::exit(1)
            }
        };
        match from_address.crypto_type.as_str() {
            "sr25519" => {
                let pair = from_address.into_pair::<Sr25519>();
                let signer = PairSigner::<IndracoreNodeRuntime, Sr::Pair>::new(pair);
                let hash = match client.transfer(&signer, &reciever, amount.pay()).await {
                    Ok(hash) => hash,
                    Err(e) => {
                        e_red_ln!("{:?}", format!("{:?}", e));
                        std::process::exit(1)
                    }
                };
                hash
            }
            "ed25519" => {
                let pair = from_address.into_pair::<Ed25519>();
                let signer = PairSigner::<IndracoreNodeRuntime, Ed::Pair>::new(pair);
                let hash = match client.transfer(&signer, &reciever, amount.pay()).await {
                    Ok(hash) => hash,
                    Err(e) => {
                        e_red_ln!("{:?}", format!("{:?}", e));
                        std::process::exit(1)
                    }
                };
                hash
            }
            "ecdsa" => {
                let pair = from_address.into_pair::<Ecdsa>();
                let signer = PairSigner::<IndracoreNodeRuntime, Ec::Pair>::new(pair);
                let hash = match client.transfer(&signer, &reciever, amount.pay()).await {
                    Ok(hash) => hash,
                    Err(e) => {
                        e_red_ln!("{:?}", format!("{:?}", e));
                        std::process::exit(1)
                    }
                };
                hash
            }
            _ => unreachable!(),
        };
    });
    dark_cyan_ln!(
        ">> Balance transfer extrinsic submitted: {:?}\n\t** from: {}\n\t** to: {}\n\t** amount {} {}",
        hash, public_id, tx.receiver, tx.amount, token_type()
    );
}

pub async fn check_balance(cmd: String) {
    if cmd.eq("total-issuance") {
        let total = match total_issuance() {
            Ok(total) => total,
            Err(e) => {
                e_red_ln!("{:?}", format!("{:?}", e));
                std::process::exit(1)
            }
        };
        let amount = Token::uamount(total);
        dark_cyan_ln!("total supply {:?} {}", amount, token_type())
    } else {
        let id = match accounid32(&cmd) {
            Ok(id) => id,
            Err(e) => {
                e_red_ln!("{:?}", format!("{:?}", e));
                std::process::exit(1)
            }
        };
        let amount = match free_balance(id) {
            Ok(amount) => amount,
            Err(e) => {
                e_red_ln!("{:?}", format!("{:?}", e));
                std::process::exit(1)
            }
        };
        let amount = Token::famount(amount);
        dark_cyan_ln!("balance {:?} {}", amount, token_type())
    }
}
