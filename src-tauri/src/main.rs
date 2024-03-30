// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod database;
mod functions;
#[allow(warnings, unused)]
mod prisma;

//use database::Database;
use functions::*;
use std::env;
//use std::sync::RwLock;
use prisma::PrismaClient;
use prisma_client_rust::{ NewClientError, Raw };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn quit(state: tauri::State<AnyDrop>) {
  std::process::exit(0);
}

struct AnyDrop {
  db: PrismaClient,
}

impl AnyDrop {

  async fn new() -> Self {
    let client: PrismaClient = PrismaClient::_builder().build().await.unwrap();
    //client._migrate_deploy().await.unwrap();
    let result_a = client._query_raw::<serde_json::Value>( Raw::new("PRAGMA journal_mode = WAL;", Vec::new()) ).exec().await;
    //let result = client._execute_raw( Raw::new("CREATE VIRTUAL TABLE user_idx USING fts5(id, 'name', 'email', content='User');", Vec::new())).exec().await;
    //println!("create: {:?}", result);
    println!("create: {:?}", result_a);
    println!("config path: {:?}", get_app_config_path());
    println!("app path: {:?}", get_app_path());
    AnyDrop { db: client }
  }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
  let app = AnyDrop::new().await;
  println!("config path: {:?}", get_app_config_path());
  println!("app path: {:?}", get_app_path());

  tauri::Builder::default()
    .manage(app)
    .invoke_handler(tauri::generate_handler![greet, quit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}
