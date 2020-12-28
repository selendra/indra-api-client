use actix_web::{web, Result, HttpResponse};
use cli::{ 
    models::{ Wallet, PublicAddress, Transaction},
    wallet::get_wallet,
    balances::{ check_balance, run_transaction},
};

use crate::errors::CustomeError;

pub async fn http_get_wallet(wl: web::Json<Wallet>) -> Result<HttpResponse, CustomeError> {
    let wallet = Wallet {
        label: wl.label.clone(),
        name: wl.name.clone(),
        location: wl.location.clone(),
        phrase: wl.phrase.clone(),
        password: wl.password.clone(),
    };
    match get_wallet(wallet){
        Ok(res) => {
            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => {
            Err(CustomeError { error: e })
        }
    }
}

pub async fn http_check_balance(id: web::Json<PublicAddress>) -> Result<HttpResponse, CustomeError> {
    match check_balance(id.address.clone()) {
        Ok(res) => {
            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => {
            Err(CustomeError { error: e })
        }
    }
}

pub async fn http_transfer(tx: web::Json<Transaction>) -> Result<HttpResponse, CustomeError> {
    let transfer = Transaction {
        sender: tx.sender.clone(),
        receiver: tx.receiver.clone(),
        amount: tx.amount.clone(),
        location: tx.location.clone(),
    };
    match run_transaction(transfer).await {
        Ok(res) => {
            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => {
            Err(CustomeError { error: e })
        }
    }
}