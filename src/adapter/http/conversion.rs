use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;

use crate::errors::ServiceError;

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            Self::_InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Self::DatabaseConnectionError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)).into_response()
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Row not found").into_response(),
            Self::KVStoreError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)).into_response()
            }
            Self::ParsingError(err) => {
                (StatusCode::BAD_REQUEST, format!("{:?}", err)).into_response()
            }
            Self::EmailError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)).into_response()
            }
            Self::InvalidVerificationCode => {
                (StatusCode::BAD_REQUEST, "Invalid verification code").into_response()
            }
            Self::VerificationCodeExpired => {
                (StatusCode::GONE, "Verification code expired").into_response()
            }
            Self::InvalidJwtToken => {
                (StatusCode::UNAUTHORIZED, "Invalid JWT token").into_response()
            }
            Self::JwtTokenExpired => {
                (StatusCode::UNAUTHORIZED, "JWT token expired").into_response()
            }
            Self::JwtTokenError(err) => (
                StatusCode::UNAUTHORIZED,
                format!("JWT token error: {}", err),
            )
                .into_response(),
            Self::UserNotVerified => {
                (StatusCode::UNAUTHORIZED, "User not verified").into_response()
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            Self::RequestError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)).into_response()
            }
            Self::ParseError => (StatusCode::BAD_REQUEST, "Parse error").into_response(),
            Self::PemKeyError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while handling pem key: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct WebResponse<T: Serialize>(pub(crate) T);

impl<T> IntoResponse for WebResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}
