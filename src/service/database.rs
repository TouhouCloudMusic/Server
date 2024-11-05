use std::{env, sync::Arc};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_db_connection() -> Arc<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").unwrap();

    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    Arc::new(Database::connect(opt).await.unwrap())
}
