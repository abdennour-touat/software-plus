/// This is the main entry point of your Tauri application.
/// #SOFTWARE PLUS
#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate magic_crypt;
mod db;
mod ipc;
mod models;
mod prelude;
mod schema;
mod utils;

use db::{ConnPool, Store};
use std::process;
use tauri::Manager;
use utils::migration;

use crate::{
    ipc::{
        key::{add_key, delete_keys, get_key},
        user::{add_user, check_auth, delete_user, delete_users, get_user, get_users},
    },
    migration::run_migration,
};
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            //for production
            // let config = app.config();
            let store = Store::new().unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(1);
            });
            run_migration(&store.conn);
            app.manage::<ConnPool>(store.conn);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_key,
            get_key,
            add_user,
            get_user,
            get_users,
            check_auth,
            delete_keys,
            delete_users,
            delete_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
