use crate::{
    connect,
    db::ConnPool,
    models::users::{User, UserBmc, UserForCreate},
    utils::error::AuthData,
};

use super::response::IpcResponse;

#[tauri::command]
pub fn add_user(store: tauri::State<'_, ConnPool>, params: UserForCreate) -> IpcResponse<()> {
    let conn = connect!(store);
    UserBmc::insert(conn, params).into()
}

#[tauri::command]
pub fn get_user(store: tauri::State<'_, ConnPool>, params: i32) -> IpcResponse<Vec<User>> {
    let conn = connect!(store);
    UserBmc::get_user(conn, params).into()
}
#[tauri::command]
pub fn get_users(store: tauri::State<'_, ConnPool>) -> IpcResponse<Vec<User>> {
    let conn = connect!(store);
    UserBmc::get_users(conn).into()
}

#[tauri::command]
pub fn check_auth(store: tauri::State<'_, ConnPool>) -> IpcResponse<AuthData> {
    let conn = connect!(store);
    UserBmc::check_authentication(conn).into()
}

#[tauri::command]
pub fn delete_users(store: tauri::State<'_, ConnPool>) -> IpcResponse<bool> {
    let conn = connect!(store);
    UserBmc::delete_all(conn).into()
}

#[tauri::command]
pub fn delete_user(store: tauri::State<'_, ConnPool>, params: i32) -> IpcResponse<()> {
    let conn = connect!(store);
    UserBmc::delete(conn, params).into()
}
