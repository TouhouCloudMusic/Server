use axum::body::Bytes;
use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use entity::image;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect,
};
use xxhash_rust::xxh3::xxh3_64;
#[derive(Clone)]
pub struct Service {
    db: DatabaseConnection,
}

impl Service {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        data: Bytes,
        uploader_id: i32,
    ) -> Result<image::Model, anyhow::Error> {
        let data_hash = xxh3_64(&data);
        let filename = URL_SAFE.encode(data_hash.to_be_bytes());

        if let Some(image) = self.is_exist_with_returning(&filename).await? {
            Ok(image)
        } else {
            let active_model = image::ActiveModel {
                filename: ActiveValue::Set(filename),
                upload_by: ActiveValue::Set(uploader_id),
                ..Default::default()
            };

            Ok(image::Entity::insert(active_model)
                .exec_with_returning(&self.db)
                .await?)
        }
    }

    async fn is_exist(&self, name: &str) -> Result<bool, DbErr> {
        image::Entity::find()
            .expr(Expr::value(1))
            .filter(image::Column::Filename.eq(name))
            .count(&self.db)
            .await
            .map(|count| count > 0)
    }

    async fn is_exist_with_returning(
        &self,
        name: &str,
    ) -> Result<Option<image::Model>, DbErr> {
        image::Entity::find()
            .filter(image::Column::Filename.eq(name))
            .one(&self.db)
            .await
    }
}
