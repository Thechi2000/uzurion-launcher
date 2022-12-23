#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use log::trace;
use tauri::Manager;
use crate::server_status::{refresh_server_status, start_fetch_server_status_task};

mod server_status;

#[tauri::command]
fn play() {
    println!("Have fun :)")
}

pub struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        AppState {}
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
            start_fetch_server_status_task(app, IP);

            let handle = app.handle();
            app.listen_global("refresh-server-status", move |_| {
                trace!("Manually refreshing server status");
                tokio::spawn(refresh_server_status(IP, handle.clone()));
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
