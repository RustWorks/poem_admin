//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_user_online"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub uuid: String,
    pub token: String,
    pub create_time: Option<DateTime>,
    pub user_name: String,
    pub ip: String,
    pub explorer: String,
    pub os: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Uuid,
    Token,
    CreateTime,
    UserName,
    Ip,
    Explorer,
    Os,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
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
            Self::Id => ColumnType::String(Some(32u32)).def(),
            Self::Uuid => ColumnType::String(Some(32u32)).def(),
            Self::Token => ColumnType::String(Some(255u32)).def().unique(),
            Self::CreateTime => ColumnType::DateTime.def().null(),
            Self::UserName => ColumnType::String(Some(255u32)).def(),
            Self::Ip => ColumnType::String(Some(120u32)).def(),
            Self::Explorer => ColumnType::String(Some(30u32)).def(),
            Self::Os => ColumnType::String(Some(30u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
