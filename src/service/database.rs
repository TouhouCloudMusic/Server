use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_db_connection(url: &str) -> DatabaseConnection {
    let opt = ConnectOptions::new(url)
        .sqlx_logging(false)
        .min_connections(1)
        .to_owned();

    Database::connect(opt).await.unwrap()
}
