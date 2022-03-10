use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_dict_data,
        models::sys_dict_data::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Query},
};
use validator::Validate;

use super::super::service;
use crate::utils::jwt::Claims;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>) -> Res<ListData<sys_dict_data::Model>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>, user: Claims) -> Res<String> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(req): Json<EditReq>, user: Claims) -> Res<String> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::edit(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Res<sys_dict_data::Model> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_type(Query(req): Query<SearchReq>) -> Res<Vec<sys_dict_data::Model>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    match service::sys_dict_data::get_by_type(db, req).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_all() -> Res<Vec<Resp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
