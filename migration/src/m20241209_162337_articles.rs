use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Articles::Table)
                    .col(pk_auto(Articles::Id))
                    .col(string(Articles::Title))
                    .col(text_null(Articles::Content))
                    .col(string(Articles::Url))
                    .col(string_null(Articles::Guid))
                    .col(string_null(Articles::Author))
                    .col(timestamp_with_time_zone_null(Articles::PublishedAt))
                    .col(integer(Articles::FeedId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-articles-feed_ids")
                            .from(Articles::Table, Articles::FeedId)
                            .to(Feeds::Table, Feeds::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Articles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Articles {
    Table,
    Id,
    Title,
    Content,
    Url,
    Guid,
    Author,
    PublishedAt,
    FeedId,
    
}

#[derive(DeriveIden)]
enum Feeds {
    Table,
    Id,
}
