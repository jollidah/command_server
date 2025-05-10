use std::time::Duration;

use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use uuid::Uuid;

use crate::{
    adapter::{
        http::conversion::WebResponse,
        kv_store::{interfaces::SessionStore, rocks_db::get_rocks_db},
    },
    domain::project::commands::{
        AssignRole, CreateProject, DeleteProject, ExpelMember, RegisterVultApiKey,
    },
    errors::ServiceError,
    service::project::{
        handle_assign_role, handle_create_project, handle_delete_project, handle_expel_member,
        handle_register_vult_api_key, handle_session_sse,
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
    path = "/external/project/vult-api-key",
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

use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::Stream;
use std::convert::Infallible;
/// Start session
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/external/project/{project_id}/session",
    request_body(content = (), content_type = "application/json"),
    responses(
        (status = 200, body = ())
    )
)]
async fn session_sse(
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rocks_db = get_rocks_db().await;

    // Session 정보(project_id, user_id) 추가
    rocks_db
        .add_user_to_session(project_id, current_user.user_id)
        .await
        .unwrap();
    let user_id = current_user.user_id;
    let stream = async_stream::try_stream! {
        let message = handle_session_sse(&current_user, project_id).await.unwrap();
        yield Event::default().json_data(message).unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    };

    // Session 정보(project_id, user_id) 제거
    let rocks_db = get_rocks_db().await;
    rocks_db
        .remove_user_from_session(project_id, user_id)
        .await
        .unwrap();

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub fn project_router() -> Router {
    Router::new()
        .route("/external/project/role", put(assign_role))
        .route(
            "/external/project/{project_id}/member/{email}",
            delete(expel_member),
        )
        .route("/external/project", post(create_project))
        .route("/external/project/{project_id}", delete(delete_project))
        .route("/external/project/vult-api-key", put(register_vult_api_key))
        .route("/external/project/{project_id}/session", get(session_sse))
        .route_layer(axum::middleware::from_fn(auth_middleware))
}
