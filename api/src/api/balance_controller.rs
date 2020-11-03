use actix_web::{HttpResponse, Result};

use crate::{models::response::ResponseBalance, services::balance_service};

// GET api/indracore/total
pub async fn get_total() -> Result<HttpResponse> {
    match balance_service::total_issuance().await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBalance::new(message))),
        Err(err) => Ok(err.response()),
    }
}
