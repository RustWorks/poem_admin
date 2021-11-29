//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "casbin_rule"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: String,
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct NewCasbinRule {
    pub ptype: String,
    pub v0: String,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Ptype,
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
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
            Self::Ptype => ColumnType::String(Some(10u32)).def().null(),
            Self::V0 => ColumnType::String(Some(256u32)).def().null(),
            Self::V1 => ColumnType::String(Some(256u32)).def().null(),
            Self::V2 => ColumnType::String(Some(256u32)).def().null(),
            Self::V3 => ColumnType::String(Some(256u32)).def().null(),
            Self::V4 => ColumnType::String(Some(256u32)).def().null(),
            Self::V5 => ColumnType::String(Some(256u32)).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
