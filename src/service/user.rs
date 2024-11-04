use sea_orm::{ActiveValue, ColumnTrait, DbErr};
use sea_orm::QueryFilter;
use sea_orm::EntityTrait;
use entity::user;
use crate::service::juniper::JuniperContext;

pub struct UserService;
impl UserService {
    pub async fn is_user_exit(
        username: String,
        context: &JuniperContext,
    )
    -> bool {
        let user = user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(context.database.as_ref())
            .await;

        match user {
            Ok(user) => {user.is_some()}
            Err(_) => {false}
        }
    }

    pub async fn create_user(
        username: String,
        password: String,
        context: &JuniperContext,
    ) -> Result<user::Model, DbErr> {
        let new_user = user::ActiveModel {
            name: ActiveValue::Set(username),
            password: ActiveValue::Set(password),
            ..Default::default()
        };

        let user_id = user::Entity::insert(new_user)
            .exec(context.database.as_ref())
            .await?.last_insert_id;

        let user = user::Entity::find_by_id(user_id)
            .one(context.database.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("User not found after insert".to_string()))?;

        Ok(user)
    }

    pub async fn is_password_correct(
        username: String,
        password: String,
        context: &JuniperContext,
    )
        -> bool {
        let user = user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .filter(user::Column::Password.eq(password))
            .one(context.database.as_ref())
            .await;

        match user {
            Ok(user) => {user.is_some()}
            Err(_) => {false}
        }
    }

    pub async fn get_user(
        username: String,
        context: &JuniperContext,
    ) -> Result<user::Model, DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .one(context.database.as_ref())
            .await?.ok_or(DbErr::RecordNotFound("User not found after login".to_string()))?;

        Ok(user)
    }
}