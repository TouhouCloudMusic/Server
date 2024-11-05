use juniper::EmptySubscription;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::AppState;
use crate::resolver::juniper::{JuniperMutation, JuniperQuery};

#[derive(Default)]
pub struct JuniperContext {
    #[allow(dead_code)]
    pub database: Arc<DatabaseConnection>,
    pub user_service: crate::service::user::UserService,
    pub song_service: crate::service::song::SongService,
    pub release_service: crate::service::release::ReleaseService,
}

impl juniper::Context for JuniperContext {}

impl From<AppState> for JuniperContext {
    fn from(state: AppState) -> Self {
        Self {
            database: Arc::clone(&state.database),
            user_service: state.user_service,
            song_service: state.song_service,
            release_service: state.release_service
        }
    }
}

pub struct _JuniperSubscription;

pub type JuniperSchema = juniper::RootNode<
    'static,
    JuniperQuery,
    JuniperMutation,
    EmptySubscription<JuniperContext>,
>;
