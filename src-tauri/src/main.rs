#![cfg_attr(
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
        user::{add_user, check_auth, delete_users, get_user, get_users},
    },
    migration::run_migration,
};
// * refactor everything
//TODO minimize the root files
// TODO document everything
// * ADD the graphs and diagrams

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            //for production
            // let config = app.config();
            let mut store = Store::new().unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(1);
            });
            run_migration(&mut store.conn);
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
            delete_users
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
