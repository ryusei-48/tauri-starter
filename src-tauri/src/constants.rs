use std::env::current_dir;
use std::path::PathBuf;
use tauri::Config;
use tauri::api::path::app_config_dir;

pub const APP_PATH: PathBuf = current_dir().unwrap();
pub const APP_CONFIG_PATH: PathBuf = app_config_dir( &Config::default() ).unwrap();