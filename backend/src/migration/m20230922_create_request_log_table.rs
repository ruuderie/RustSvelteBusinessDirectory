use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RequestLog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RequestLog::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RequestLog::UserId).uuid().null())
                    .col(ColumnDef::new(RequestLog::IpAddress).string().not_null())
                    .col(ColumnDef::new(RequestLog::UserAgent).string().null())
                    .col(ColumnDef::new(RequestLog::Path).string().not_null())
                    .col(ColumnDef::new(RequestLog::Method).string().not_null())
                    .col(ColumnDef::new(RequestLog::StatusCode).integer().not_null())
                    .col(ColumnDef::new(RequestLog::RequestType).string().not_null())
                    .col(ColumnDef::new(RequestLog::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RequestLog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RequestLog {
    Table,
    Id,
    UserId,
    IpAddress,
    UserAgent,
    Path,
    Method,
    StatusCode,
    RequestType,
    CreatedAt,
}