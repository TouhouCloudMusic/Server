//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::EntityStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "credit_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub entity_id: i32,
    pub status: EntityStatus,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub short_description: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::release_credit::Entity")]
    ReleaseCredit,
    #[sea_orm(has_many = "super::song_credit::Entity")]
    SongCredit,
}

impl Related<super::release_credit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReleaseCredit.def()
    }
}

impl Related<super::song_credit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongCredit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
