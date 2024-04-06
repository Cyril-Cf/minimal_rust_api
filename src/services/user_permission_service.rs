use entity::permission::Entity as PermissionEntity;
use entity::permission::Model as PermissionModel;
use entity::user_permission::{Entity, Model};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr,
    DeleteResult, EntityTrait, ModelTrait, QueryFilter,
};

pub async fn create(
    permission_id: i32,
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<Model, DbErr> {
    let entities = Entity::find()
        .filter(entity::user_permission::Column::UserId.eq(user_id))
        .filter(entity::user_permission::Column::PermissionId.eq(permission_id))
        .all(conn)
        .await?;
    if !entities.is_empty() {
        return Err(DbErr::Custom("Association already exists!".to_string()));
    }
    entity::user_permission::ActiveModel {
        permission_id: ActiveValue::Set(permission_id),
        user_id: ActiveValue::Set(user_id),
    }
    .insert(conn)
    .await
}

pub async fn delete(
    id_user: i32,
    id_permission: i32,
    conn: &DatabaseConnection,
) -> Result<Option<DeleteResult>, DbErr> {
    let entities = Entity::find()
        .filter(
            Condition::all()
                .add(entity::user_permission::Column::UserId.eq(id_user))
                .add(entity::user_permission::Column::PermissionId.eq(id_permission)),
        )
        .all(conn)
        .await?;
    if entities.is_empty() {
        return Ok(None);
    }
    Ok(Some(
        entities.into_iter().next().unwrap().delete(conn).await?,
    ))
}

pub async fn find_all_for_permission(
    id_permission: i32,
    conn: &DatabaseConnection,
) -> Result<Vec<Model>, DbErr> {
    Entity::find()
        .filter(entity::user_permission::Column::PermissionId.eq(id_permission))
        .all(conn)
        .await
}

pub async fn find_all_for_user(
    id_user: i32,
    conn: &DatabaseConnection,
) -> Result<Vec<PermissionModel>, DbErr> {
    let entities = Entity::find()
        .filter(entity::user_permission::Column::UserId.eq(id_user))
        .all(conn)
        .await?;
    let mut permissions: Vec<PermissionModel> = Vec::new();
    for entity in entities {
        let permission_entity = PermissionEntity::find()
            .filter(entity::permission::Column::Id.eq(entity.permission_id))
            .one(conn)
            .await?;
        if let Some(permission) = permission_entity {
            permissions.push(permission);
        }
    }
    Ok(permissions)
}
