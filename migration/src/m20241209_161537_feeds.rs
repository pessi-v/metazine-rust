use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Feeds::Table)
                    .col(pk_auto(Feeds::Id))
                    .col(string(Feeds::Title))
                    .col(string(Feeds::Url))
                    .col(string_null(Feeds::Description))
                    .col(timestamp_with_time_zone_null(Feeds::LastFetchedAt))
                    .col(boolean_null(Feeds::Active))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Feeds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Feeds {
    Table,
    Id,
    Title,
    Url,
    Description,
    LastFetchedAt,
    Active,
    
}

