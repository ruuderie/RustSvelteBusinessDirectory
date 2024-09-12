use sea_orm_migration::prelude::*;

mod m20230910_create_businesses_table;
mod m20230912_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230910_create_businesses_table::Migration),
            Box::new(m20230912_create_users_table::Migration),
        ]
    }
}
