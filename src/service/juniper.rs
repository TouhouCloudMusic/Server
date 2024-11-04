use juniper::EmptySubscription;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::AppState;

#[derive(Default)]
pub struct JuniperContext {
    #[allow(dead_code)]
    pub database: Arc<DatabaseConnection>,
    pub user_service: crate::service::user::UserService,
}

impl juniper::Context for JuniperContext {}

impl From<AppState> for JuniperContext {
    fn from(state: AppState) -> Self {
        Self {
            database: Arc::clone(&state.database),
            user_service: state.user_service,
        }
    }
}

pub struct JuniperQuery;

pub struct JuniperMutation;

pub struct _JuniperSubscription;

pub type JuniperSchema = juniper::RootNode<
    'static,
    JuniperQuery,
    JuniperMutation,
    EmptySubscription<JuniperContext>,
>;
