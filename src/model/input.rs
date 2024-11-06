use entity::sea_orm_active_enums::{DatePrecision, EntityStatus, ReleaseType};
use juniper::GraphQLInputObject;
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use entity::release;

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
    #[graphql(description = "The number of random songs to return.")]
    pub count: i32,
}

#[derive(GraphQLInputObject)]
pub struct CreateSongInput {
    #[graphql(description = "The review status.")]
    pub status: EntityStatus,
    pub title: String,
    pub created_at: DateTimeWithTimeZone,
    #[graphql(description = "The latest update time.")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(GraphQLInputObject)]
pub struct RetrieveReleaseInput {
    pub id: i32,
}