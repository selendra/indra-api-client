use crate::error::ServiceError;
use crate::models::balance::{AccountId, Transaction, TransactionOutput};

use indracore_api::{
    balance::{
        check_balance::{free_balance as freeApi, total_issuance as totalApi},
        transaction::Transaction as TransactionApi,
    },
    keyring::{accounid32, indracoreid, Sr25519},
    primitives::Token,
};

use actix_web::http::StatusCode;

pub async fn total_issuance() -> Result<u128, ServiceError> {
    let total = match totalApi() {
        Ok(total) => total,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:?}", e),
            ))
        }
    };
    let amount = Token::uamount(total);
    Ok(amount)
}

pub async fn free_balance(id: AccountId) -> Result<f64, ServiceError> {
    let id = match accounid32(&id.accountid) {
        Ok(id) => id,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:?}", e),
            ))
        }
    };
    let amount = match freeApi(id) {
        Ok(amount) => amount,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:?}", e),
            ))
        }
    };
    let amount = Token::famount(amount);

    Ok(amount)
}

pub async fn transfer(tx: Transaction) -> Result<TransactionOutput, ServiceError> {
    let token = Token {
        token: tx.amount.clone(),
    };
    let amount = token.pay();
    let sender = Sr25519 {
        suri: tx.sender.clone(),
    };
    let reciever = match indracoreid(&tx.receiver.clone()) {
        Ok(p) => p,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:?}", e),
            ))
        }
    };

    let pubid = sender.to_accountid().unwrap();

    let result = TransactionApi {
        amount,
        sender,
        reciever: reciever.clone(),
    };

    let hash = match result.run(None) {
        Ok(hash) => hash,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:?}", e),
            ))
        }
    };
    Ok(TransactionOutput {
        hash: format!("{:?}", hash),
        sender: format!("{:?}", pubid),
        receiver: tx.receiver.clone(),
        amount: tx.amount.clone(),
    })
}
