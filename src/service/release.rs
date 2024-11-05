use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use entity::release;

#[derive(Default, Clone)]
pub struct ReleaseService {
    database: Arc<DatabaseConnection>,
}

impl ReleaseService {
    pub fn new(database: &Arc<DatabaseConnection>) -> Self {
        Self {
            database: Arc::clone(database),
        }
    }

    pub async fn find_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<release::Model, DbErr> {
        release::Entity::find_by_id(id)
            .one(self.database.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Release not found by id".to_string(),
            ))
    }
}