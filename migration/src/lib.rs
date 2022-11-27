pub use sea_orm_migration::*;

mod permission_migration;
mod user_migration;
mod user_permission_migration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user_migration::Migration),
            Box::new(permission_migration::Migration),
            Box::new(user_permission_migration::Migration),
        ]
    }
}
