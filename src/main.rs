use std::{env, net::SocketAddr};

use adapter::http::{routes::auth::auth_router, swagger_docs::AuthDoc};
use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod adapter;
mod errors;

#[tokio::main]
async fn main() {
    println!("Environment Variable Is Being Set...");
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "command_server=debug,tower_http=info,axum::rejection=trace,trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let service_routers = Router::new().nest("/api/v1", auth_router());

    let service_name = "/command_server";

    let swagger = Router::new().merge(
		SwaggerUi::new(service_name.to_string() + "/api/docs")
			.url(service_name.to_string() + "/api/docs/auth/openapi.json", AuthDoc::openapi())
	);

    let app = Router::new()
        .nest_service(service_name, service_routers)
        .layer(tower_http::trace::TraceLayer::new_for_http())
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
