use serde::de::DeserializeOwned;

use crate::errors::ServiceError;

pub mod block_storage;
pub mod conversions;
pub mod firewall;
pub mod instance;
pub mod managed_database;
pub mod object_storage;

pub const BASE_URL: &str = "https://api.vultr.com/v2";

pub async fn extract_schema_from_response<T: DeserializeOwned>(
    response: reqwest::Response,
    field_name: &str,
) -> Result<T, ServiceError> {
    let response_json: serde_json::Value = response.json().await?;
    let schema_json = response_json
        .get(field_name)
        .ok_or(ServiceError::ParseError)?;
    let schema: T = serde_json::from_value(schema_json.clone())?;
    Ok(schema)
}
