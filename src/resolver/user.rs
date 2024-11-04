use crate::model::input_model::*;
use crate::model::output_model::*;
use crate::service::juniper::*;

use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

#[graphql_object]
#[graphql(context = JuniperContext)]
impl JuniperQuery {
    async fn login(
        input: LoginInput,
        context: &JuniperContext,
    ) -> FieldResult<LoginOutput> {
        let user_service = &context.user_service;
        if user_service
            .verify_password(&input.username, &input.password)
            .await?
        {
            return Err(FieldError::new(
                "Username already exits",
                graphql_value!({"status": "USER_EXISTS"}),
            ));
        }

        let user = user_service.find_by_name(&input.username).await.map_err(
            |err| {
                FieldError::new(
                    format!("Failed to create user: {}", err),
                    graphql_value!({"status": "DATABASE_ERROR"}),
                )
            },
        )?;

        Ok(LoginOutput { id: user.id })
    }
}
#[graphql_object]
#[graphql(context = JuniperContext)]
impl JuniperMutation {
    async fn signup(
        input: SignupInput,
        context: &JuniperContext,
    ) -> FieldResult<SignupOutput> {
        let user_service = &context.user_service;

        if user_service.is_exist(&input.username).await? {
            return Err(FieldError::new(
                "Username already exits",
                graphql_value!({"status": "USER_EXISTS"}),
            ));
        }

        let user = user_service
            .create(&input.username, (&input.password).into())
            .await
            .map_err(|err| {
                FieldError::new(
                    format!("Failed to create user: {}", err),
                    graphql_value!({"status": "DATABASE_ERROR"}),
                )
            })?;

        Ok(SignupOutput { id: user.id })
    }
}
