use std::string::FromUtf8Error;

use crate::errors::ServiceError;

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("Request error while sending request to Vultr: {:?}", error);
        ServiceError::RequestError(Box::new(error))
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(_: serde_json::Error) -> Self {
        ServiceError::ParseError
    }
}

impl From<FromUtf8Error> for ServiceError {
    fn from(error: FromUtf8Error) -> Self {
        tracing::error!("Failed to convert from UTF-8: {:?}", error);
        ServiceError::ParseError
    }
}
