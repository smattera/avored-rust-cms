//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::admin_users::Entity> for Entity {
    fn to() -> RelationDef {
        super::admin_users_roles::Relation::AdminUsers.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::admin_users_roles::Relation::Roles.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
