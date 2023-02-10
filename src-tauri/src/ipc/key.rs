use crate::{
    connect,
    db::ConnPool,
    models::key::{Key, KeyBmc, KeyForCreate},
};

use super::response::IpcResponse;

#[tauri::command]
pub fn add_key(store: tauri::State<'_, ConnPool>, params: KeyForCreate) -> IpcResponse<String> {
    let conn = connect!(store);
    let res = match KeyBmc::insert(conn, params) {
        license_key::Status::Valid => Ok("valid".to_owned()),
        license_key::Status::Invalid => Ok("invalid".to_owned()),
        license_key::Status::Blocked => Ok("blocked".to_owned()),
        license_key::Status::Forged => Ok("forged".to_owned()),
    };
    res.into()
}

#[tauri::command]
pub fn get_key(store: tauri::State<'_, ConnPool>) -> IpcResponse<Vec<Key>> {
    let conn = connect!(store);
    KeyBmc::get(conn).into()
}

#[tauri::command]
pub fn delete_keys(store: tauri::State<'_, ConnPool>) -> IpcResponse<bool> {
    let conn = connect!(store);
    KeyBmc::delete_all(conn).into()
}
