use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Directory::Table)
                    .modify_column(ColumnDef::new(Directory::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(Directory::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Listing::Table)
                    .modify_column(ColumnDef::new(Listing::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(Listing::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Profile::Table)
                    .modify_column(ColumnDef::new(Profile::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(Profile::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Account::Table)
                    .modify_column(ColumnDef::new(Account::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(Account::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserAccount::Table)
                    .modify_column(ColumnDef::new(UserAccount::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(UserAccount::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdPurchase::Table)
                    .modify_column(ColumnDef::new(AdPurchase::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(AdPurchase::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Category::Table)
                    .modify_column(ColumnDef::new(Category::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(Category::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Template::Table)
                    .modify_column(ColumnDef::new(Template::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(Template::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ListingAttribute::Table)
                    .modify_column(ColumnDef::new(ListingAttribute::CreatedAt).timestamp_with_time_zone().not_null())
                    .modify_column(ColumnDef::new(ListingAttribute::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Directory::Table)
                    .modify_column(ColumnDef::new(Directory::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(Directory::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Listing::Table)
                    .modify_column(ColumnDef::new(Listing::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(Listing::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Profile::Table)
                    .modify_column(ColumnDef::new(Profile::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(Profile::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Account::Table)
                    .modify_column(ColumnDef::new(Account::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(Account::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserAccount::Table)
                    .modify_column(ColumnDef::new(UserAccount::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(UserAccount::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdPurchase::Table)
                    .modify_column(ColumnDef::new(AdPurchase::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(AdPurchase::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;


        manager
            .alter_table(
                Table::alter()
                    .table(Category::Table)
                    .modify_column(ColumnDef::new(Category::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(Category::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Template::Table)
                    .modify_column(ColumnDef::new(Template::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(Template::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ListingAttribute::Table)
                    .modify_column(ColumnDef::new(ListingAttribute::CreatedAt).timestamp().not_null())
                    .modify_column(ColumnDef::new(ListingAttribute::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Directory {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Listing {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Profile {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Account {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UserAccount {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AdPurchase {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Category {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Template {
    Table,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ListingAttribute {
    Table,
    CreatedAt,
    UpdatedAt,
}

