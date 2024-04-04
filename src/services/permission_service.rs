use crate::services::user_permission_service;
use entity::permission::{Entity, Model};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    ModelTrait,
};

pub async fn find_one(id: i32, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(conn).await
}

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    entity::permission::ActiveModel {
        name: ActiveValue::Set(new_name),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match Entity::find_by_id(id).one(conn).await? {
        Some(entity) => {
            if user_permission_service::find_all_for_permission(entity.id, conn)
                .await?
                .len()
                > 0
            {
                return Ok(None);
            }
            return Ok(Some(entity.delete(conn).await?));
        }
        None => Ok(None),
    }
}
