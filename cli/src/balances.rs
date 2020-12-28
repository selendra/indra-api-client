use crate::models::{ Transaction, BalanceOutput, TransactionOutput };
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

pub async fn run_transaction(tx: Transaction) -> Result<TransactionOutput, String> {
    let store = WalletStore::init(tx.location.as_deref(), None);

    let from_address = match store.read(&tx.sender).ok_or("") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("address not exit".to_string())
        }
    };
    if from_address.is_watchonly() {
        return Err(format!("watch only address"))
    }

    let public_id = from_address.addr.clone();
    let reciever = match indracoreid(&tx.receiver) {
        Ok(p) => p,
        Err(e) => {
            return Err(format!("{:?}", e))
        }
    };
    let amount = Token::get(tx.amount.clone());

    let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
        .set_url(url())
        .build()
        .await
    {
        Ok(client) => client,
        Err(e) => {
            return Err(format!("{:?}", e))
        }
    };
    let hash = match from_address.crypto_type.as_str() {
        "sr25519" => {
            let pair = from_address.into_pair::<Sr25519>();
            let signer = PairSigner::<IndracoreNodeRuntime, Sr::Pair>::new(pair);
            let hash = match client.transfer(&signer, &reciever, amount.pay()).await {
                Ok(hash) => hash,
                Err(e) => {
                    return Err(format!("{}", e))
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
                    return Err(format!("{}", e))
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
                    return Err(format!("{}", e))
                }
            };
            hash
        }
        _ => unreachable!(),
    };
    return Ok(TransactionOutput {
        hash: format!("{:?}", hash),
        sender: public_id,
        receiver: tx.receiver,
        amount: tx.amount.parse::<f64>().unwrap(),
        symbol:  token_type()
    })
}

pub async fn op_run_transaction(tx: Transaction){
    match run_transaction(tx).await {
        Ok(res) => {
            dark_cyan_ln!(
                ">> Balance transfer extrinsic submitted: {:?}\n\t** from: {}\n\t** to: {}\n\t** amount {} {}",
                res.hash, res.sender, res.receiver, res.amount, res.symbol
            );
        }
        Err(e) => {
            println!("{:?}", format!("{:?}", e));
            std::process::exit(1)
        }
    } 
}

pub fn check_balance(id: String) -> Result<BalanceOutput, String> {
    let id = match accounid32(&id) {
        Ok(id) => id,
        Err(e) => {
            return Err(format!("{:?}", e))
        }
    };
    let amount = match free_balance(id) {
        Ok(amount) => amount,
        Err(e) => {
            return Err(format!("{:?}", e))
        }
    };
    let amount = Token::famount(amount);
    return Ok(
        BalanceOutput {
        token: amount,
        symbol: token_type()
    })
}

pub fn op_check_balance(cmd: String){
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