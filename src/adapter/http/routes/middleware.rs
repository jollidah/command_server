use crate::domain::auth::jwt::JwtToken;
use axum::extract::Request;
use axum::http::{self, HeaderMap};
use axum::middleware::Next;
use axum::response::Response;
use reqwest::StatusCode;

#[derive(Debug, Clone)]
pub(crate) struct CurrentUser {
    pub email: String,
}

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let jwt = JwtToken::new();
    let token = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?
        .trim_start_matches("Bearer ");
    let claims = jwt
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let current_user = CurrentUser {
        email: claims.email,
    };
    request.extensions_mut().insert(current_user);
    Ok(next.run(request).await)
}
