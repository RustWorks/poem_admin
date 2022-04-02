//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_dept"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    DeptId,
    ParentId,
    DeptName,
    OrderNum,
    Leader,
    Phone,
    Email,
    Status,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    DeptId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::DeptId => ColumnType::String(Some(32u32)).def(),
            Self::ParentId => ColumnType::String(Some(32u32)).def(),
            Self::DeptName => ColumnType::String(Some(30u32)).def(),
            Self::OrderNum => ColumnType::Integer.def(),
            Self::Leader => ColumnType::String(Some(20u32)).def().null(),
            Self::Phone => ColumnType::String(Some(11u32)).def().null(),
            Self::Email => ColumnType::String(Some(50u32)).def().null(),
            Self::Status => ColumnType::Char(Some(1u32)).def(),
            Self::CreatedBy => ColumnType::String(Some(32u32)).def(),
            Self::UpdatedBy => ColumnType::String(Some(32u32)).def().null(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def().null(),
            Self::DeletedAt => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
