use entity::user_permission::{Entity, Model};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};

pub async fn create(
    permission_id: i32,
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<Model, DbErr> {
    entity::user_permission::ActiveModel {
        permission_id: ActiveValue::Set(permission_id),
        user_id: ActiveValue::Set(user_id),
    }
    .insert(conn)
    .await
}

pub async fn find_all_for_user(
    id_user: i32,
    conn: &DatabaseConnection,
) -> Result<Vec<Model>, DbErr> {
    Entity::find()
        .filter(entity::user_permission::Column::UserId.eq(id_user))
        .all(conn)
        .await
}
