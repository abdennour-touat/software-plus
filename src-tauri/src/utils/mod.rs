pub mod error;
pub mod keyverifier;
pub mod macros;
pub mod migration;
use std::{fs, path::PathBuf, sync::Arc};

use tauri::{api::path::app_data_dir, Config};

pub fn get_app_path(config: &Arc<Config>) -> PathBuf {
    // debug!("get_app_path.");
    let path = app_data_dir(config).expect("Failed to get application data path.");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path
}
