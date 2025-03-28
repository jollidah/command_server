use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;

use crate::errors::ServiceError;

use super::schemas::{CheckVerification, CreateVerification, SignIn, SignUp, Tokens};

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            Self::_InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}

impl IntoResponse for SignUp{
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}

impl IntoResponse for SignIn{
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}

impl IntoResponse for Tokens{
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}

impl IntoResponse for CreateVerification{
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}

impl IntoResponse for CheckVerification{
    fn into_response(self) -> Response {
        Json(json!(self)).into_response()
    }
}
