mod model;
mod resolver;
mod service;

use axum::routing::{on, MethodFilter};
use axum::{routing::get, Extension, Router};
use juniper::EmptySubscription;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_axum::{graphiql, playground};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;

#[derive(Default)]
pub struct JuniperContext {
    database: Arc<DatabaseConnection>,
}
impl juniper::Context for JuniperContext {}
pub struct JuniperQuery;
pub struct JuniperMutation;
pub struct JuniperSubscription;
type JuniperSchema = juniper::RootNode<
    'static,
    JuniperQuery,
    JuniperMutation,
    EmptySubscription<JuniperContext>,
>;

#[derive(Clone)]
pub struct AppState {
    database: Arc<DatabaseConnection>,
    juniper_context: Arc<JuniperContext>,
    juniper_schema: Arc<JuniperSchema>,
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").unwrap();

    let server_port = env::var("SERVER_PORT").unwrap();

    let database = Arc::new(Database::connect(db_url).await.unwrap());

    let context = JuniperContext {
        database: Arc::clone(&database),
    };

    let schema = JuniperSchema::new(
        JuniperQuery,
        JuniperMutation,
        EmptySubscription::new(),
    );

    let state = AppState {
        database,
        juniper_context: Arc::new(context),
        juniper_schema: Arc::new(schema),
    };

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
    Extension(schema): Extension<Arc<JuniperSchema>>,
    Extension(context): Extension<Arc<JuniperContext>>,
    JuniperRequest(req): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(req.execute(&*schema, &*context).await)
}
