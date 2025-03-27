use axum::{response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde_json::json;

use crate::errors::ServiceError;

use super::schemas::UserAccount;

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            Self::_InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}

impl IntoResponse for UserAccount {
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}