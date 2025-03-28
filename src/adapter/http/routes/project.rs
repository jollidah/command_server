use axum::{
    routing::{patch, post},
    Json, Router,
};

use crate::{
    adapter::http::schemas::{CreateProject, DeleteProject, ExpelMember},
    domain::project::AssignRole,
    errors::ServiceError,
};

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
pub async fn assign_role(Json(_json): Json<AssignRole>) -> Result<(), ServiceError> {
    Ok(())
}

/// Expel member
#[axum::debug_handler]
#[utoipa::path(
    patch,
    path = "/external/project/role",
    request_body(content = ExpelMember, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn expel_member(Json(_json): Json<ExpelMember>) -> Result<(), ServiceError> {
    Ok(())
}

/// Create project
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/project",
    request_body(content = CreateProject, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn create_project(Json(_json): Json<CreateProject>) -> Result<(), ServiceError> {
    Ok(())
}

/// Delete project
#[axum::debug_handler]
#[utoipa::path(
    patch,
    path = "/external/project",
    request_body(content = DeleteProject, content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
pub async fn delete_project(Json(_json): Json<DeleteProject>) -> Result<(), ServiceError> {
    Ok(())
}

pub fn project_router() -> Router {
    Router::new()
        .route("/external/project/role", post(assign_role))
        .route("/external/project/role", patch(expel_member))
        .route("/external/project", post(create_project))
        .route("/external/project", patch(delete_project))
}
