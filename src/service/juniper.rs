use axum::extract::FromRef;
use juniper::EmptySubscription;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Default)]
pub struct JuniperContext {
    pub database: Arc<DatabaseConnection>,
}

impl juniper::Context for JuniperContext {}

impl JuniperContext {
    fn init(database: Arc<DatabaseConnection>) -> Arc<Self> {
        Arc::new(Self { database })
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

#[derive(Clone, FromRef)]
pub struct JuniperState {
    pub context: Arc<JuniperContext>,
    pub schema: &'static JuniperSchema,
}

static JUNIPER_SCHEMA: Lazy<JuniperSchema> = Lazy::new(|| {
    JuniperSchema::new(JuniperQuery, JuniperMutation, EmptySubscription::new())
});

impl JuniperState {
    pub fn init(database: Arc<DatabaseConnection>) -> Self {
        Self {
            context: JuniperContext::init(database),
            schema: &JUNIPER_SCHEMA,
        }
    }
}
