use axum::{
    extract::Path,
    routing::{delete, post, put},
    Extension, Json, Router,
};
use uuid::Uuid;

use crate::{
    adapter::http::conversion::WebResponse,
    domain::project::commands::{
        AssignRole, CreateProject, DeleteProject, ExpelMember, RegisterVultApiKey,
    },
    errors::ServiceError,
    service::project::{
        handle_assign_role, handle_create_project, handle_delete_project, handle_expel_member,
        handle_register_vult_api_key,
    },
    CurrentUser,
};

use super::middleware::auth_middleware;

/// Assign role
#[axum::debug_handler]
#[utoipa::path(
    put,
    path = "/external/project/role",
    request_body(content = AssignRole, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn assign_role(
    Extension(current_user): Extension<CurrentUser>,
    Json(cmd): Json<AssignRole>,
) -> Result<(), ServiceError> {
    handle_assign_role(cmd, current_user).await?;
    Ok(())
}

/// Expel member
#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/external/project/{project_id}/member/{email}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn expel_member(
    Extension(current_user): Extension<CurrentUser>,
    Path((project_id, email)): Path<(Uuid, String)>,
) -> Result<(), ServiceError> {
    handle_expel_member(
        ExpelMember {
            project_id,
            expelled_email: email,
        },
        current_user,
    )
    .await?;
    Ok(())
}

/// Create project
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/project",
    request_body(content = CreateProject, content_type = "application/json"),
    responses(
        (status = 200, body = Uuid)
    )
)]
pub async fn create_project(
    Extension(current_user): Extension<CurrentUser>,
    Json(cmd): Json<CreateProject>,
) -> Result<WebResponse<Uuid>, ServiceError> {
    let project_id = handle_create_project(cmd, current_user).await.unwrap();
    Ok(WebResponse(project_id))
}

/// Delete project
#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/external/project/{project_id}",
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_project(Path(project_id): Path<Uuid>) -> Result<(), ServiceError> {
    handle_delete_project(DeleteProject { project_id }).await?;
    Ok(())
}

/// Register (updsert) vult api key for admin
#[axum::debug_handler]
#[utoipa::path(
    put,
    path = "/external/auth/vult-api-key",
    request_body(content = RegisterVultApiKey, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn register_vult_api_key(
    Extension(current_user): Extension<CurrentUser>,
    Json(cmd): Json<RegisterVultApiKey>,
) -> Result<(), ServiceError> {
    handle_register_vult_api_key(cmd, current_user).await?;
    Ok(())
}

pub fn project_router() -> Router {
    Router::new()
        .route("/external/project/role", put(assign_role))
        .route(
            "/external/project/:project_id/member/:email",
            delete(expel_member),
        )
        .route("/external/project", post(create_project))
        .route("/external/project/:project_id", delete(delete_project))
        .route("/external/auth/vult-api-key", put(register_vult_api_key))
        .route_layer(axum::middleware::from_fn(auth_middleware))
}
