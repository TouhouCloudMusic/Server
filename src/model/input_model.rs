use juniper::GraphQLInputObject;

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
pub struct SongInput {
    pub id: i32,
}

#[derive(GraphQLInputObject)]
pub struct RandomSongInput {
    pub count: i32,
}