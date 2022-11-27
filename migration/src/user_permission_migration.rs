use entity::user_permission::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::PermissionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_permissions_permissions")
                            .from(Entity, Column::PermissionId)
                            .to(entity::permission::Entity, entity::permission::Column::Id),
                    )
                    .col(ColumnDef::new(Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_permissions_users")
                            .from(Entity, Column::UserId)
                            .to(entity::user::Entity, entity::user::Column::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Entity).to_owned())
            .await
    }
}
