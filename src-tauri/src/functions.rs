use std::env::current_dir;
use std::path::PathBuf;
use tauri::Config;
use tauri::api::path::app_config_dir;

pub fn get_app_path() -> PathBuf {
  current_dir().unwrap()
}

pub fn get_app_config_path() -> PathBuf {
  app_config_dir( &Config::default() ).unwrap()
}