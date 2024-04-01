// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(warnings)]
use std::fs;

use db::{ get_client, init_db };
use state::AppState;
use commands::user;
use menu::get_menu;

mod commands;
mod db;
mod state;
mod menu;

//Change for example com.{name_creator or name_team}.{name_app}
const BUNDLE_IDENTIFIER: &str = "com.template.tauri-app";

#[tokio::main]
async fn main() {
    init_data_dir();

    let prisma_client = get_client().await;
    init_db(&prisma_client).await;

    tauri::Builder
        ::default()
        .menu(get_menu())
        .manage(AppState { prisma_client })
        .invoke_handler(
            tauri::generate_handler![
                user::create_user,
                user::delete_user,
                user::update_user,
                user::get_user,
                user::get_all_user
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_data_dir() {
    let data_dir = tauri::api::path
        ::data_dir()
        .unwrap_or(std::path::PathBuf::from("./"))
        .join(BUNDLE_IDENTIFIER);

    if !data_dir.exists() {
        fs::create_dir(data_dir).expect("Error creating application data directory");
    }
}
