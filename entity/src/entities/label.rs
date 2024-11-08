//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::DatePrecision;
use super::sea_orm_active_enums::EntityStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub entity_id: i32,
    pub status: EntityStatus,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub founded_date: Option<Date>,
    pub founded_date_precision: DatePrecision,
    pub dissolved_date: Option<Date>,
    pub dissolved_date_precision: DatePrecision,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::label_founder::Entity")]
    LabelFounder,
    #[sea_orm(has_many = "super::label_localized_name::Entity")]
    LabelLocalizedName,
    #[sea_orm(has_many = "super::release_label::Entity")]
    ReleaseLabel,
}

impl Related<super::label_founder::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabelFounder.def()
    }
}

impl Related<super::label_localized_name::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabelLocalizedName.def()
    }
}

impl Related<super::release_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReleaseLabel.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        super::label_founder::Relation::Artist.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::label_founder::Relation::Label.def().rev())
    }
}

impl Related<super::release::Entity> for Entity {
    fn to() -> RelationDef {
        super::release_label::Relation::Release.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::release_label::Relation::Label.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
