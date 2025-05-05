pub mod commands;
pub mod conversion;
pub mod jwt;
pub mod private_key;

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::errors::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAccountAggregate {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub phone_num: String,
    pub password: String,
    pub verified: bool,
    pub create_dt: DateTime<Utc>,
}

impl UserAccountAggregate {
    pub fn set_account_verified(&mut self) {
        self.verified = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationCode {
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

impl VerificationCode {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let code = rng.random_range(100000..999999).to_string();
        let expires_at = Utc::now() + Duration::minutes(5);
        Self { code, expires_at }
    }

    pub fn verify_code(&self, code: &str) -> Result<(), ServiceError> {
        if self.code != code {
            return Err(ServiceError::InvalidVerificationCode);
        }
        if self.expires_at < Utc::now() {
            return Err(ServiceError::VerificationCodeExpired);
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ServiceError> {
        serde_json::to_vec(self).map_err(|err| ServiceError::ParsingError(Box::new(err)))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ServiceError> {
        serde_json::from_slice(bytes).map_err(|err| ServiceError::ParsingError(Box::new(err)))
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct AuthenticationTokens {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

impl AuthenticationTokens {
    pub(crate) fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VultrAPIKey {
    pub(crate) api_key: String,
}
