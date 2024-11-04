use sea_orm::{ActiveValue, QueryFilter};
use sea_orm::ColumnTrait;
use entity::user;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};
use sea_orm::EntityTrait;
use crate::{JuniperContext, JuniperMutation, JuniperQuery};
use crate::model::graphql_input::*;
use crate::model::graphql_output::*;

#[graphql_object]
#[graphql(context = JuniperContext)]
impl JuniperQuery {
    async fn login(
        input: LoginInput,
        context: &JuniperContext,
    ) -> FieldResult<LoginOutput> {
        let user = user::Entity::find()
            .filter(user::Column::Name.eq(input.username.clone()))
            .filter(user::Column::Password.eq(input.password.clone()))
            .one(&context.db)
            .await?;

        if user.is_some() {
            return Ok(LoginOutput{id: user.unwrap().id});
        }
        Err(FieldError::new("Incorrect username or password", graphql_value!({"status": "AUTHORIZATION FAILURE"})))
    }
}
#[graphql_object]
#[graphql(context = JuniperContext)]
impl JuniperMutation {
    async fn signup(
        input: SignupInput,
        context: &JuniperContext,
    ) -> FieldResult<SignupOutput> {
        let user = user::Entity::find()
            .filter(user::Column::Name.eq(input.username.clone()))
            .one(&context.db)
            .await?;

        if user.is_some() {
            return Err(FieldError::new("Username already taken", graphql_value!({"status": "USER_EXISTS"})));
        }

        let new_user = user::ActiveModel {
            name: ActiveValue::Set(input.username.clone()),
            password: ActiveValue::Set(input.password),
            ..Default::default()
        };
        let user = user::Entity::insert(new_user)
            .exec(&context.db)
            .await?;
        Ok(SignupOutput {
            id: user.last_insert_id,
        })
    }
}
