use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::FirstName).string().not_null().default(""))
                    .add_column(ColumnDef::new(User::LastName).string().not_null().default(""))
                    .add_column(ColumnDef::new(User::LastLogin).timestamp_with_time_zone().null())
                    .add_column(ColumnDef::new(User::Phone).string().not_null().default(""))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::FirstName)
                    .drop_column(User::LastName)
                    .drop_column(User::LastLogin)
                    .drop_column(User::Phone)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    FirstName,
    LastName,
    LastLogin,
    Phone,
}