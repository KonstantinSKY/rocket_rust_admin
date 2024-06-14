use sea_orm_migration::prelude::*;
use crate::field_types::id;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(&mut id(User::Id) )
                .col(ColumnDef::new(User::Name).string().not_null().unique_key())
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .col(ColumnDef::new(User::FirstName).string())
                .col(ColumnDef::new(User::LastName).string())
                .col(ColumnDef::new(User::CreatedAt)
                    .timestamp()
                    .not_null()
                    .default(Expr::current_timestamp()))
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum User {
    Table, 
    Id,
    Name,
    Email,
    FirstName,
    LastName,
    CreatedAt,
}
