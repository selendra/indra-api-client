use crate::operation::Transaction;
use colour::{dark_cyan_ln, e_red_ln};
use indracore_api::{
    balance::{
        check_balance::{free_balance, total_issuance},
        transaction::Transaction as TransactionApi,
    },
    keyring::{accounid32, indracoreid, Sr25519},
    primitives::{token_type, Token},
};

pub async fn run_transaction(transaction: Transaction) {
    // let password = transaction.password.unwrap();

    let rec = transaction.receiver.unwrap();
    let amount = Token::get(transaction.amount.unwrap());
    let sender = Sr25519 {
        suri: transaction.sender.unwrap(),
    };
    let reciever = match indracoreid(&rec) {
        Ok(p) => p,
        Err(e) => {
            e_red_ln!("{:?}", format!("{:?}", e));
            std::process::exit(1)
        }
    };

    let pubid = sender.to_accountid().unwrap();
    let token = amount.token();

    let result = TransactionApi {
        amount: amount.pay(),
        sender,
        reciever: reciever.clone(),
    };

    let hash = match result.run(None) {
        Ok(hash) => hash,
        Err(e) => {
            e_red_ln!("{:?}", format!("{:?}", e));
            std::process::exit(1)
        }
    };
    dark_cyan_ln!(
        ">> Balance transfer extrinsic submitted: {:?}\n\t** from: {}\n\t** to: {}\n\t** amount {} {}",
        hash, pubid, rec, token, token_type()
    );
}

// pub fn test_transaction(transaction: Transaction) {
//     let

// }

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
        dark_cyan_ln!("{:?}", amount)
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
        dark_cyan_ln!("{:?}", amount)
    }
}
