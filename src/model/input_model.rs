use juniper::GraphQLInputObject;
use sea_orm::prelude::DateTimeWithTimeZone;
use entity::sea_orm_active_enums::EntityStatus;

#[derive(GraphQLInputObject)]
pub struct SignupInput {
    pub username: String,
    pub password: String,
}
#[derive(GraphQLInputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(GraphQLInputObject)]
pub struct RetrieveSongInput {
    pub id: i32,
}

#[derive(GraphQLInputObject)]
pub struct RandomSongInput {
    pub count: i32,
}

#[derive(GraphQLInputObject)]
pub struct CreateSongInput {
    pub status: EntityStatus,
    pub title: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}