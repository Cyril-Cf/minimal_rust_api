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

pub async fn create(
    new_name: String,
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<Model, DbErr> {
    entity::permission::ActiveModel {
        name: ActiveValue::Set(new_name),
        user_id: ActiveValue::Set(user_id),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}
