use crate::models::balance::{AccountId, Transaction, TransactionOutput};
use crate::utils::{
    keyring,
    primitives::{Config, Token},
};
use crate::{constants, error::ServiceError};

use actix_web::http::StatusCode;
use substrate_subxt::{
    balances::*, sp_core::Decode, system::*, ClientBuilder, EventSubscription, EventsDecoder,
    IndracoreNodeRuntime,
};

pub async fn total_issuance() -> Result<u128, ServiceError> {
    let client = ClientBuilder::<IndracoreNodeRuntime>::new()
        .set_url(Config::url())
        .build()
        .await;
    let client = match client {
        Ok(client) => client,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::CONNECT_REFUSE.to_string(),
            ))
        }
    };
    let total = client.total_issuance(None).await;
    let total = match total {
        Ok(total) => total,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Get token info error".to_string(),
            ))
        }
    };
    let amount = Token::lamount(total);
    Ok(amount)
}

pub async fn free_balance(id: AccountId) -> Result<f64, ServiceError> {
    let client = ClientBuilder::<IndracoreNodeRuntime>::new()
        .set_url(Config::url())
        .build()
        .await;
    let client = match client {
        Ok(client) => client,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::CONNECT_REFUSE.to_string(),
            ))
        }
    };
    let account = keyring::AccountId::new(id.accountid);
    let accountid32 = match account.accountid32() {
        Ok(a) => a,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ))
        }
    };
    let info = client.account(&accountid32, None).await;
    let info = match info {
        Ok(info) => info,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Get account info error".to_string(),
            ))
        }
    };
    let amount = Token::amount(info.data.free);

    Ok(amount)
}

pub async fn transfer(tx: Transaction) -> Result<TransactionOutput, ServiceError> {
    let client = ClientBuilder::<IndracoreNodeRuntime>::new()
        .set_url(Config::url())
        .build()
        .await;
    let client = match client {
        Ok(client) => client,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::CONNECT_REFUSE.to_string(),
            ))
        }
    };
    let sender = keyring::Signer::new(tx.sender);

    let accountid32 = match sender.accountid32() {
        Ok(a) => a,
        Err(e) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ))
        }
    };
    let info = client.account(&accountid32, None).await;
    let info = match info {
        Ok(info) => info,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Get account info error.".to_string(),
            ))
        }
    };
    let my_balance = Token::amount(info.data.free);

    if tx.amount > my_balance {
        return Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Your balance inefficient {:?}.", my_balance).to_string(),
        ));
    } else {
        let amount = Token::get(tx.amount);

        let signer = match sender.pair() {
            Ok(pair) => pair,
            Err(e) => {
                return Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                ))
            }
        };

        let reciever = keyring::AccountId::new(tx.receiver);
        let dest = reciever.accountid().unwrap();

        let sub = client.subscribe_events().await;
        let sub = match sub {
            Ok(sub) => sub,
            Err(_) => {
                return Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to make event subscribe".to_string(),
                ))
            }
        };
        let mut decoder = EventsDecoder::<IndracoreNodeRuntime>::new(client.metadata().clone());
        decoder.with_balances();
        let mut sub = EventSubscription::<IndracoreNodeRuntime>::new(sub, decoder);
        sub.filter_event::<TransferEvent<_>>();

        let hash = client.transfer(&signer, &dest, amount.pay()).await;
        let hash = match hash {
            Ok(h) => h,
            Err(_) => {
                return Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "make transaction failed".to_string(),
                ))
            }
        };

        let raw = sub.next().await.unwrap().unwrap();

        let event = TransferEvent::<IndracoreNodeRuntime>::decode(&mut &raw.data[..]);
        if let Ok(event) = event {
            Ok(TransactionOutput {
                hash: hash,
                sender: event.from,
                receiver: event.to,
                amount: tx.amount,
            })
        } else {
            return Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("!!! Failed to subscribe to Balances::Transfer Event").to_string(),
            ));
        }
    }
}
