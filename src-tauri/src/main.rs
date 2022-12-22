#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::cell::Cell;
use std::sync::Arc;
use log::info;
use tauri::async_runtime::Mutex;
use tauri::Manager;
use crate::server_status::{fetch_server_status_task, ServerStatus};

mod server_status;

#[tauri::command]
fn play() {
    println!("Have fun :)")
}

pub struct AppState {
    pub server_status: Arc<Mutex<Cell<Option<ServerStatus>>>>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            server_status: Arc::new(Mutex::new(Cell::new(None))),
        }
    }
}

const IP: &str = "play.cubecraft.net"; // TODO Set in config file

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin( // Register logging plugin https://jonaskruckenberg.github.io/tauri-docs-wip/development/debugging.html
                 tauri_plugin_log::LoggerBuilder::new()
                     .targets([
                         // write to the OS logs folder
                         tauri_plugin_log::LogTarget::LogDir,
                         // write to stdout
                         tauri_plugin_log::LogTarget::Stdout,
                         // forward logs to the webview
                         tauri_plugin_log::LogTarget::Webview,
                     ])
                     .build(),
        )
        .setup(|app| {
            // Start server status fetching task
            tokio::task::spawn(fetch_server_status_task(IP, app.state::<AppState>().server_status.clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
