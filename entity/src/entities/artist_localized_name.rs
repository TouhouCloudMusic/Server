//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "artist_localized_name")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub artist_id: i32,
    pub language_id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::artist::Entity",
        from = "Column::ArtistId",
        to = "super::artist::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Artist,
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::LanguageId",
        to = "super::language::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Language,
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl Related<super::language::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Language.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}