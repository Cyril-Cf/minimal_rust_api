use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_permission::Entity")]
    Permissions,
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_permission::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_permission::Relation::Permission.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateModel {
    pub name: String,
}
