mod model;
mod resolver;
mod service;

use axum::extract::{FromRef, State};
use axum::routing::{on, MethodFilter};
use axum::{routing::get, Router};

use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_axum::{graphiql, playground};
use sea_orm::{Database, DatabaseConnection};
use service::juniper::JuniperState;

use std::env;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    database: Arc<DatabaseConnection>,
    juniper: service::juniper::JuniperState,
}

impl AppState {
    pub fn init(database: Arc<DatabaseConnection>) -> Self {
        Self {
            database: Arc::clone(&database),
            juniper: JuniperState::init(Arc::clone(&database)),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").unwrap();

    let server_port = env::var("SERVER_PORT").unwrap();

    let database = Arc::new(Database::connect(db_url).await.unwrap());

    let state = AppState::init(database);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
        )
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{server_port}"))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();

    tracing::info!("Starting server on http://127.0.0.1:{server_port}");
}

pub async fn graphql_handler(
    State(state): State<JuniperState>,
    JuniperRequest(req): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(req.execute(&state.schema, &state.context).await)
}
