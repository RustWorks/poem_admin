use sea_orm::{entity::prelude::DateTime, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub dict_data_id: Option<String>,
    pub dict_type: Option<String>,
    pub dict_label: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct AddReq {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: String,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub dict_data_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct EditReq {
    pub dict_data_id: String,
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: String,
    pub status: String,
    pub remark: String,
}

#[derive(Debug, Serialize, FromQueryResult, Clone)]
pub struct Resp {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub is_default: String,
    pub status: String,
    pub remark: String,
    pub created_at: DateTime,
}
