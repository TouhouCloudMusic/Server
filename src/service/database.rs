use std::{env, sync::Arc};

use sea_orm::{Database, DatabaseConnection};

pub async fn get_db_connection() -> Arc<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").unwrap();
    

    Arc::new(Database::connect(db_url).await.unwrap())
}
