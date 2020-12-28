use actix_web::{error};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "error: {}", error)]
pub struct CustomeError {
    pub error: String,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for CustomeError {}