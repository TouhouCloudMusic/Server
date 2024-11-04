mod model;
mod resolver;
mod service;

use axum::extract::{FromRef, State};
use axum::routing::{on, MethodFilter};
use axum::{routing::get, Router};

use juniper::EmptySubscription;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_axum::{graphiql, playground};

use sea_orm::DatabaseConnection;

use service::database::get_db_connection;
use service::juniper::{
    JuniperContext, JuniperMutation, JuniperQuery, JuniperSchema,
};
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
    State(state): State<AppState>,
    JuniperRequest(req): JuniperRequest,
) -> JuniperResponse {
    let schema = JuniperSchema::new(
        JuniperQuery,
        JuniperMutation,
        EmptySubscription::new(),
    );
    JuniperResponse(req.execute(&schema, &JuniperContext::from(state)).await)
}
