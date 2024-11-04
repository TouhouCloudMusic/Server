use crate::model::input_model::*;
use crate::model::output_model::*;
use crate::service::juniper::*;

use entity::user;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::{ActiveValue, QueryFilter};

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
            .one(context.database.as_ref())
            .await?;

        if let Some(u) = user {
            return Ok(LoginOutput { id: u.id });
        }

        Err(FieldError::new(
            "Incorrect username or password",
            graphql_value!({"status": "AUTHORIZATION FAILURE"}),
        ))
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
            .one(context.database.as_ref())
            .await?;

        if user.is_some() {
            return Err(FieldError::new(
                "Username already taken",
                graphql_value!({"status": "USER_EXISTS"}),
            ));
        }

        let new_user = user::ActiveModel {
            name: ActiveValue::Set(input.username.clone()),
            password: ActiveValue::Set(input.password),
            ..Default::default()
        };
        let user = user::Entity::insert(new_user)
            .exec(context.database.as_ref())
            .await?;
        Ok(SignupOutput {
            id: user.last_insert_id,
        })
    }
}
