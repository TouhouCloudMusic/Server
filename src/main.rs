mod controller;
mod model;
mod resolver;
mod service;

use axum::extract::FromRef;
use axum::{routing::get, Router};

use sea_orm::DatabaseConnection;

use service::database::get_db_connection;
use service::UserService;

use std::env;
use std::sync::Arc;
use tracing_subscriber::fmt::time::ChronoLocal;
use crate::service::SongService;

#[derive(Clone, FromRef)]
pub struct AppState {
    database: Arc<DatabaseConnection>,
    user_service: UserService,
    song_service: SongService,
}

impl AppState {
    pub async fn init() -> Self {
        let database = get_db_connection().await;

        Self {
            database: Arc::clone(&database),
            user_service: UserService::new(&database),
            song_service: SongService::new(&database),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

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
