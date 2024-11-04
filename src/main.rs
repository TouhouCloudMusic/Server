mod resolver;
mod model;

use axum::{routing::get, Extension, Router};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use axum::routing::{on, MethodFilter};
use juniper::{EmptySubscription};
use juniper_axum::{graphiql, playground};
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;

#[derive(Default)]
pub struct JuniperContext {
    db: DatabaseConnection
}
impl juniper::Context for JuniperContext {}
pub struct JuniperQuery;
pub struct JuniperMutation;
pub struct JuniperSubscription;
type JuniperSchema = juniper::RootNode<'static, JuniperQuery, JuniperMutation, EmptySubscription<JuniperContext>>;

#[tokio::main]
async fn main() {
    let context = get_db_connectin().await
        .map(|db| JuniperContext { db })
        .map_err(|e| eprintln!("Failed to get database connection: {:?}", e))
        .ok().unwrap();

    let schema = JuniperSchema::new(JuniperQuery, JuniperMutation, EmptySubscription::new());
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/graphql",
            on(
                MethodFilter::GET.or(MethodFilter::POST),
                graphql_handler,
            ),
        )
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(context)))
        .layer(Extension(Arc::new(schema)));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_db_connectin(
) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let db_url = env::var("DATABASE_URL")?;

    let connection: DatabaseConnection = Database::connect(db_url).await?;

    Ok(connection)
}

pub async fn graphql_handler(
    Extension(schema): Extension<Arc<JuniperSchema>>,
    Extension(context): Extension<Arc<JuniperContext>>,
    JuniperRequest(req): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(
        req.execute(&*schema, &*context).await,
    )
}