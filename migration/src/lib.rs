#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241209_161537_feeds;
mod m20241209_162337_articles;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241209_161537_feeds::Migration),
            Box::new(m20241209_162337_articles::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}