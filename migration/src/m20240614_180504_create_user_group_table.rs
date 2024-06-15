use sea_orm_migration::prelude::*;
use crate::m20240613_064555_create_users_table::User;
use crate::m20240614_173112_create_group_table::Group;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserGroup::UserId).integer().not_null())
                    .col(ColumnDef::new(UserGroup::GroupId).integer().not_null())
                    .primary_key(Index::create()
                        .name("pk_user_group")
                        .col(UserGroup::UserId)
                        .col(UserGroup::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group_user_id")
                            .from(UserGroup::Table, UserGroup::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group_group_id")
                            .from(UserGroup::Table, UserGroup::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserGroup {
    Table,
    UserId,
    GroupId,
}
