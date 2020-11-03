use substrate_subxt::{balances::*, ClientBuilder, IndracoreNodeRuntime};
use utils::primitives::{Config, Token};

use crate::{constants, error::ServiceError};
use actix_web::http::StatusCode;

pub async fn total_issuance() -> Result<u128, ServiceError> {
    let client = ClientBuilder::<IndracoreNodeRuntime>::new()
        .set_url(Config::url())
        .build()
        .await;
    match client {
        Ok(client) => {
            let total = client.total_issuance(None).await.unwrap();
            let amount = Token::amount(total);
            Ok(amount)
        }
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::CONNECT_REFUSE.to_string(),
        )),
    }
}
