use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Account::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Account::DirectoryId).uuid().not_null())
                    .col(ColumnDef::new(Account::Name).string().not_null())
                    .col(ColumnDef::new(Account::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Account::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Account::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Account {
    Table,
    Id,
    DirectoryId,
    Name,
    IsActive,
    CreatedAt,
    UpdatedAt,
}