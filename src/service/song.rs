use std::sync::Arc;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Order, QueryOrder, QuerySelect};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::{Func, SimpleExpr};
use entity::{song};
use entity::sea_orm_active_enums::EntityStatus;

#[derive(Default, Clone)]
pub struct SongService {
    database: Arc<DatabaseConnection>,
}

impl SongService {
    pub fn new(database: &Arc<DatabaseConnection>) -> Self {
        Self {
            database: Arc::clone(database),
        }
    }

    pub async fn find_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<song::Model, DbErr> {
        song::Entity::find_by_id(id)
            .one(self.database.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Song not found by id".to_string(),
            ))
    }

    pub async fn random(
        &self,
        count: u64,
    ) -> anyhow::Result<Vec<song::Model>, DbErr> {
        song::Entity::find()
            .order_by(
                SimpleExpr::FunctionCall(Func::random()),
                Order::Desc
            )
            .limit(count)
            .all(self.database.as_ref())
            .await
    }

    pub async fn create(
        &self,
        status: EntityStatus,
        title: String,
        created_at: DateTimeWithTimeZone,
        updated_at: DateTimeWithTimeZone,
    ) -> anyhow::Result<song::Model, DbErr> {
        let new_song = song::ActiveModel {
            status: ActiveValue::Set(status),
            title: ActiveValue::Set(title),
            created_at: ActiveValue::Set(created_at),
            updated_at: ActiveValue::Set(updated_at),
            ..Default::default()
        };
        song::Entity::insert(new_song)
            .exec_with_returning(self.database.as_ref())
            .await
    }
}