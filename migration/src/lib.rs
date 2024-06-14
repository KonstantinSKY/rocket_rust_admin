pub use sea_orm_migration::prelude::*;
mod field_types;
mod m20240613_064555_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240613_064555_create_users_table::Migration),
        ]
    }
}
