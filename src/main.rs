use adapter::{
    http::{
        routes::{auth::auth_router, middleware::CurrentUser, project::project_router},
        swagger_docs::{AuthDoc, ProjectDoc},
    },
    repositories::connection_pool,
};
use axum::Router;
use reqwest::Method;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod adapter;
mod config;
mod domain;
mod errors;
mod service;

#[tokio::main]
async fn main() {
    println!("Environment Variable Is Being Set...");
    dotenv::dotenv().ok();
    sqlx::migrate!("./migrations")
        .run(connection_pool())
        .await
        .expect("Running Migration Script Failed!");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "command_server=debug,tower_http=debug,axum::rejection=debug,debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);
    let service_routers = Router::new()
        .nest("/api/v1", auth_router())
        .nest("/api/v1", project_router());

    let service_name = "/command_server";

    let swagger = Router::new().merge(
        SwaggerUi::new(service_name.to_string() + "/api/docs")
            .url(
                service_name.to_string() + "/api/docs/auth/openapi.json",
                AuthDoc::openapi(),
            )
            .url(
                service_name.to_string() + "/api/docs/project/openapi.json",
                ProjectDoc::openapi(),
            ),
    );

    let app = Router::new()
        .nest_service(service_name, service_routers)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors)
        .merge(swagger);

    let listener = TcpListener::bind(&env::var("SERVER_IP_PORT").unwrap_or("0.0.0.0:80".into()))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
