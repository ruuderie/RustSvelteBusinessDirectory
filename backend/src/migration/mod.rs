use sea_orm_migration::prelude::*;

mod m20230912_create_users_table;
mod m20230911_create_accounts_table;
mod m20230912_create_user_accounts_table;
mod m20230913_create_directory_types_table;
mod m20230914_create_directories_table;
mod m20230915_create_profiles_table;
mod m20230916_create_categories_table;
mod m20230917_create_templates_table;
mod m20230918_create_listings_table;
mod m20230919_create_listing_attributes_table;
mod m20230920_create_ad_purchases_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230912_create_users_table::Migration),
            Box::new(m20230911_create_accounts_table::Migration),
            Box::new(m20230912_create_user_accounts_table::Migration),
            Box::new(m20230913_create_directory_types_table::Migration),
            Box::new(m20230914_create_directories_table::Migration),
            Box::new(m20230915_create_profiles_table::Migration),
            Box::new(m20230916_create_categories_table::Migration),
            Box::new(m20230917_create_templates_table::Migration),
            Box::new(m20230918_create_listings_table::Migration),
            Box::new(m20230919_create_listing_attributes_table::Migration),
            Box::new(m20230920_create_ad_purchases_table::Migration),
        ]
    }
}