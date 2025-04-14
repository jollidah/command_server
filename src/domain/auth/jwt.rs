use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::get_config, errors::ServiceError};

const ACCESS_TOKEN_EXPIRATION_MINUTES: i64 = 10;
const REFRESH_TOKEN_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,  // user id
    pub email: String,  // user email
    pub exp: i64,       // expiration time
    pub iat: i64,       // issued at
    pub typ: TokenType, // token type
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

impl TokenType {
    pub fn get_duration(&self) -> Duration {
        match self {
            TokenType::Access => Duration::minutes(ACCESS_TOKEN_EXPIRATION_MINUTES),
            TokenType::Refresh => Duration::hours(REFRESH_TOKEN_EXPIRATION_HOURS),
        }
    }
}
pub struct JwtToken {
    secret: String,
    header: Header,
    validation: Validation,
}

impl JwtToken {
    pub fn new() -> Self {
        let header = Header {
            typ: Some(String::from("JWT")),
            alg: Algorithm::HS256,
            ..Default::default()
        };
        Self {
            secret: get_config().jwt_secret.clone(),
            header,
            validation: Validation::default(),
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<String, ServiceError> {
        self.generate_token(user_id, email, TokenType::Access)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<String, ServiceError> {
        self.generate_token(user_id, email, TokenType::Refresh)
    }

    fn generate_token(
        &self,
        user_id: Uuid,
        email: &str,
        token_type: TokenType,
    ) -> Result<String, ServiceError> {
        let now = Utc::now();
        let exp = now + token_type.get_duration();
        let claims = Claims {
            user_id,
            email: email.to_owned(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            typ: token_type,
        };

        encode(
            &self.header,
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|err| ServiceError::JwtTokenError(err.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, ServiceError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &self.validation,
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => ServiceError::InvalidJwtToken,
            _ => ServiceError::JwtTokenError(err.to_string()),
        })?;
        if token_data.claims.exp < Utc::now().timestamp() {
            return Err(ServiceError::JwtTokenExpired);
        }

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_access_token() {
        let jwt = JwtToken::new();
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let token = jwt.generate_access_token(user_id, &email).unwrap();
        let claims = jwt.verify_token(&token).unwrap();

        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.typ, TokenType::Access);
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let jwt = JwtToken::new();
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let token = jwt.generate_refresh_token(user_id, &email).unwrap();
        let claims = jwt.verify_token(&token).unwrap();

        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.typ, TokenType::Refresh);
    }
}
