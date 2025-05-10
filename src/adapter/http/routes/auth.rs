use axum::{
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    adapter::http::conversion::WebResponse,
    domain::auth::{
        commands::{CheckVerification, CreateUserAccount, IssueTokens, RefreshTokens},
        AuthenticationTokens,
    },
    errors::ServiceError,
    service::auth::{
        handle_check_verification_email, handle_create_user_account, handle_get_public_key,
        handle_issue_tokens, handle_refresh_tokens,
    },
};

use super::middleware::auth_middleware;

/// Create User Account (Sign up)
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth/account",                 
    request_body(content = CreateUserAccount, content_type = "application/json"),
    responses(
        (status = 200, body = Uuid)
    )
)]
pub async fn create_user_account(
    Json(cmd): Json<CreateUserAccount>,
) -> Result<WebResponse<Uuid>, ServiceError> {
    let user_id = handle_create_user_account(cmd).await?;
    Ok(WebResponse(user_id))
}

/// Sign in
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth/login",
    request_body(content = IssueTokens, content_type = "application/json"),
    responses(
        (status = 200, body = AuthenticationTokens)
    )
)]
pub async fn issue_tokens(
    Json(cmd): Json<IssueTokens>,
) -> Result<WebResponse<AuthenticationTokens>, ServiceError> {
    let tokens = handle_issue_tokens(cmd).await?;

    Ok(WebResponse(tokens))
}

/// Refresh tokens
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth/refresh",
    request_body(content = RefreshTokens, content_type = "application/json"),
    responses(
        (status = 200, body = AuthenticationTokens)
    )
)]
pub async fn refresh_tokens(
    Json(cmd): Json<RefreshTokens>,
) -> Result<WebResponse<AuthenticationTokens>, ServiceError> {
    let tokens = handle_refresh_tokens(cmd).await?;

    Ok(WebResponse(tokens))
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
pub async fn check_verification_email(
    Json(cmd): Json<CheckVerification>,
) -> Result<(), ServiceError> {
    handle_check_verification_email(cmd).await
}

/// Get public key
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/external/auth/public-key",
    responses(
        (status = 200, body = String)
    )
)]
pub async fn get_public_key() -> Result<WebResponse<String>, ServiceError> {
    let public_key = handle_get_public_key().await?;
    Ok(WebResponse(public_key))
}

pub fn auth_router() -> Router {
    let non_auth_router = Router::new()
        .route("/external/auth/login", post(issue_tokens))
        .route("/external/auth/refresh", post(refresh_tokens))
        .route("/external/auth/account", post(create_user_account))
        .route(
            "/external/auth/verification/check",
            post(check_verification_email),
        );
    let auth_router = Router::new()
        .route("/external/auth/vult-api-key", get(get_public_key))
        .route_layer(axum::middleware::from_fn(auth_middleware));
    non_auth_router.merge(auth_router)
}
