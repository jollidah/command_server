use axum::{routing::{get, post}, Json, Router};

use crate::{adapter::http::schemas::{CheckVerification, CreateVerification, SignIn, SignUp, Tokens}, errors::ServiceError};

/// Create User Account (Sign up)
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth",                 
    request_body(content = SignUp, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn sign_up(Json(_json): Json<SignUp>) -> Result<(), ServiceError> {
    Ok(())
}

/// Sign in
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/external/auth",
    request_body(content = SignIn, content_type = "application/json"),
    responses(
        (status = 200, body = Tokens)
    )
)]
pub async fn sign_in(Json(_json): Json<SignIn>) -> Result<Tokens, ServiceError> {
    Ok(
        Tokens{
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string()
        }
    )
}

/// Send Verification email
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth/verification",
    request_body(content = CreateVerification, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn create_verification_email(Json(_json): Json<CreateVerification>) -> Result<(), ServiceError> {
    Ok(())
}

/// Check email verification
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth/verification/check",
    request_body(content = CheckVerification, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn check_verification_email(Json(_json): Json<CheckVerification>) -> Result<(), ServiceError> {
    Ok(())
}

pub fn auth_router() -> Router {
    Router::new()
    .route("/external/auth", post(sign_up))
    .route("/external/auth", get(sign_in))
    .route("/external/auth/verification", post(create_verification_email))
    .route("/external/auth/verification/check", post(check_verification_email))
}
