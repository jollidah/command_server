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
