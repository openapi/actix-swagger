use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serde json failure: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Serde url encoded deserialization failure: {0}")]
    SerdeUrlEncodedDeError(#[from] serde_urlencoded::de::Error),
    #[error("Serde url encoded serialization failure: {0}")]
    SerdeUrlEncodedSerError(#[from] serde_urlencoded::ser::Error),
}

impl ResponseError for Error {}