use extension::postgres::Type;
use sea_orm::Iterable;
use sea_orm_migration::{prelude::*, schema::*};

use crate::{date_precision, default_self_id, CreatedAndUpdatedAt};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ArtistType)
                    .values(ArtistTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Artist::Table)
                    .if_not_exists()
                    .col(pk_auto(Artist::Id))
                    .col(default_self_id(Artist::EntityId, Artist::Id))
                    .col(integer_null(Artist::PrevId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_artist_prev_id")
                            .from(Artist::Table, Artist::PrevId)
                            .to(Artist::Table, Artist::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Artist::NextId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_artist_next_id")
                            .from(Artist::Table, Artist::NextId)
                            .to(Artist::Table, Artist::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(text(Artist::Name))
                    .col(ColumnDef::new(Artist::ArtistType).custom(ArtistType))
                    .col(ColumnDef::new(Artist::AliasGroupId).integer().null())
                    .col(array_null(Artist::TextAlias, ColumnType::Text))
                    .col(date_null(Artist::StartDate))
                    .col(date_precision(Artist::StartDatePrecision))
                    .col(date_null(Artist::EndDate))
                    .col(date_precision(Artist::EndDatePrecision))
                    .created_at(Artist::CreatedAt)
                    .updated_at(Artist::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AliasGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(AliasGroup::Id))
                    .col(default_self_id(AliasGroup::EntityId, AliasGroup::Id))
                    .col(integer_null(AliasGroup::PrevId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alias_group_prev_id")
                            .from(AliasGroup::Table, AliasGroup::PrevId)
                            .to(AliasGroup::Table, AliasGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(AliasGroup::NextId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alias_group_next_id")
                            .from(AliasGroup::Table, AliasGroup::NextId)
                            .to(AliasGroup::Table, AliasGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(AliasGroup::ArtistId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_alias_group_artist_id")
                            .from(AliasGroup::Table, AliasGroup::ArtistId)
                            .to(Artist::Table, Artist::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Artist::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_artist_alias_group_id")
                            .from_tbl(Artist::Table)
                            .from_col(Artist::AliasGroupId)
                            .to_tbl(AliasGroup::Table)
                            .to_col(AliasGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Artist::Table)
                    .name("fk_artist_alias_group_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AliasGroup::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Artist::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ArtistType).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Artist {
    Table,
    Id,
    EntityId,
    PrevId,
    NextId,
    Name,
    ArtistType,
    TextAlias,
    AliasGroupId,
    StartDate,
    StartDatePrecision,
    EndDate,
    EndDatePrecision,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub struct ArtistType;

#[derive(DeriveIden, sea_orm::EnumIter)]
enum ArtistTypeVariants {
    Group,
    Person,
}

#[derive(DeriveIden)]
enum AliasGroup {
    Table,
    Id,
    EntityId,
    PrevId,
    NextId,
    ArtistId,
}
