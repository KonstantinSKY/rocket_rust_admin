use sea_orm_migration::prelude::*;
use crate::field_types::{current_timestamp, id, name};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
            Table::create()
                .table(Group::Table)
                .if_not_exists()
                .col(&mut id(Group::Id))
                .col(&mut name(Group::Name))
                .col(&mut current_timestamp(Group::CreatedAt))
            .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Group {
    Table,
    Id,
    Name,
    CreatedAt,
}
