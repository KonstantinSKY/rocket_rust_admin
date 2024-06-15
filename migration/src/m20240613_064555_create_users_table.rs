use sea_orm_migration::prelude::*;
use crate::field_types::{current_timestamp, email, id, name};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(&mut id(User::Id))
                .col(&mut name(User::Name))
                .col(&mut email(User::Email))
                .col(ColumnDef::new(User::Password).string_len(255).not_null())
                .col(ColumnDef::new(User::FirstName).string())
                .col(ColumnDef::new(User::LastName).string())
                .col(ColumnDef::new(User::LastLogin).timestamp().null())
                .col(ColumnDef::new(User::IsActive).boolean().not_null().default(true))
                .col(ColumnDef::new(User::IsStaff).boolean().not_null().default(false))
                .col(ColumnDef::new(User::IsSuperuser).boolean().not_null().default(false))
                .col(&mut current_timestamp(User::CreatedAt))
            .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table, 
    Id,
    Name,
    Email,
    Password,
    FirstName,
    LastName,
    CreatedAt,
    LastLogin,
    IsActive,
    IsStaff,
    IsSuperuser,
}
