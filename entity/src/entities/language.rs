//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "language")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::artist_localized_name::Entity")]
    ArtistLocalizedName,
    #[sea_orm(has_many = "super::label_localized_name::Entity")]
    LabelLocalizedName,
    #[sea_orm(has_many = "super::release_localized_title::Entity")]
    ReleaseLocalizedTitle,
    #[sea_orm(has_many = "super::song_localized_title::Entity")]
    SongLocalizedTitle,
}

impl Related<super::artist_localized_name::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ArtistLocalizedName.def()
    }
}

impl Related<super::label_localized_name::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabelLocalizedName.def()
    }
}

impl Related<super::release_localized_title::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReleaseLocalizedTitle.def()
    }
}

impl Related<super::song_localized_title::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongLocalizedTitle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
