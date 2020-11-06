use actix_web::{web, HttpResponse, Result};

use crate::{
    models::{
        balance::{AccountId, Transaction},
        response::ResponseBalance,
    },
    services::balance_service,
};

// GET api/indracore/total
pub async fn get_total() -> Result<HttpResponse> {
    match balance_service::total_issuance().await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBalance::new(message))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn get_balance(id_dto: web::Json<AccountId>) -> Result<HttpResponse> {
    match balance_service::free_balance(id_dto.0).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBalance::new(message))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn make_transaction(tx: web::Json<Transaction>) -> Result<HttpResponse> {
    match balance_service::transfer(tx.0).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBalance::new(message))),
        Err(err) => Ok(err.response()),
    }
}
