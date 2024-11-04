mod controller;
mod model;
mod resolver;
mod service;

use axum::extract::FromRef;
use axum::{routing::get, Router};

use sea_orm::DatabaseConnection;

use service::database::get_db_connection;
use service::user::UserService;

use std::env;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    database: Arc<DatabaseConnection>,
    user_service: UserService,
}

impl AppState {
    pub async fn init() -> Self {
        let database = get_db_connection().await;

        Self {
            database: Arc::clone(&database),
            user_service: UserService::new(&database),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt::init();

    let server_port = env::var("SERVER_PORT").unwrap();

    let state = AppState::init().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/", controller::graphql::router())
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{server_port}"))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();

    tracing::info!("Starting server on http://127.0.0.1:{server_port}");
}
