use axum::{routing::post, Json, Router};

use crate::{adapter::http::schemas::UserAccount, errors::ServiceError};

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/auth/test",                 
    request_body(content = UserAccount, content_type = "application/json"),  
    responses(
        (status = 200, body = UserAccount)
    )
)]
pub async fn test_auth(Json(json): Json<UserAccount>) -> Result<UserAccount, ServiceError> {
    Ok(json)
}

pub fn auth_router() -> Router {
    Router::new().route("/external/auth/test", post(test_auth))
}
