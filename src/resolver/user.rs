use crate::model::input_model::*;
use crate::model::output_model::*;
use crate::service::juniper::*;

use juniper::{graphql_object, graphql_value, FieldError, FieldResult};
use crate::service::user::UserService;

#[graphql_object]
#[graphql(context = JuniperContext)]
impl JuniperQuery {
    async fn login(
        input: LoginInput,
        context: &JuniperContext,
    ) -> FieldResult<LoginOutput> {
        if UserService::is_password_correct(input.username.clone(), input.password.clone(), context).await {
            return Err(FieldError::new(
                "Username already exits",
                graphql_value!({"status": "USER_EXISTS"}),
            ));
        }

        let user = UserService::get_user(input.username.clone(), context)
            .await
            .map_err(|err| FieldError::new(
                format!("Failed to create user: {}", err),
                graphql_value!({"status": "DATABASE_ERROR"}),
            ))?;

        Ok(LoginOutput { id: user.id})
    }
}
#[graphql_object]
#[graphql(context = JuniperContext)]
impl JuniperMutation {
    async fn signup(
        input: SignupInput,
        context: &JuniperContext,
    ) -> FieldResult<SignupOutput> {
        if UserService::is_user_exit(input.username.clone(), context).await {
            return Err(FieldError::new(
                "Username already exits",
                graphql_value!({"status": "USER_EXISTS"}),
            ));
        }

        let user = UserService::create_user(input.username.clone(), input.password.clone(), context)
            .await
            .map_err(|err| FieldError::new(
                format!("Failed to create user: {}", err),
                graphql_value!({"status": "DATABASE_ERROR"}),
            ))?;

        Ok(SignupOutput { id: user.id })
    }
}
