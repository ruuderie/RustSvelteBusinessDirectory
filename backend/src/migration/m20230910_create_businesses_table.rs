use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Business::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Business::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Business::Name).string().not_null())
                    .col(ColumnDef::new(Business::Category).string().not_null())
                    .col(ColumnDef::new(Business::Address).string().not_null())
                    .col(ColumnDef::new(Business::Phone).string().not_null())
                    .col(ColumnDef::new(Business::Website).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Business::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Business {
    Table,
    Id,
    Name,
    Category,
    Address,
    Phone,
    Website,
}
