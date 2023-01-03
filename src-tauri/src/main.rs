#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use log::{trace, warn};
use tauri::{Manager, WindowEvent};
use crate::consts::*;
use crate::login::MicrosoftLoginData;
use crate::routing::start_server;
use crate::server_status::{refresh_server_status, start_fetch_server_status_task};
use crate::settings::{Settings};

mod server_status;
mod login;
mod settings;
mod consts;
mod update;
mod routing;
mod event;

#[tauri::command]
fn play() {
    println!("Have fun :)")
}

pub struct AppState {
    pub settings: Arc<Mutex<Settings>>,
    pub microsoft_login: Arc<Mutex<MicrosoftLoginData>>,
    pub client: reqwest::Client,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            settings: Arc::new(Mutex::default()),
            microsoft_login: Arc::new(Mutex::default()),
            client: reqwest::Client::new(),
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
            let main_window = app.get_window(windows::MAIN).unwrap();

            // Start server status fetching task
            start_fetch_server_status_task(app, IP);

            let handle = app.handle();
            tokio::spawn(start_server(SocketAddr::from(([127, 0, 0, 1], 3000)), handle));

            let handle = app.handle();
            tokio::spawn(async move {
                *handle.state::<AppState>().settings.lock().unwrap() = Settings::load().await.unwrap_or_default()
            });

            let handle = app.handle();
            app.listen_global("refresh-server-status", move |_| {
                trace!("Manually refreshing server status");
                tokio::spawn(refresh_server_status(IP, handle.clone()));
            });

            // Force to give focus to microsoft-login if it exists
            let handle = app.app_handle();
            main_window.on_window_event(move |e| {
                if let WindowEvent::Focused(_) = e {
                    if let Some(window) = handle.get_window(windows::MICROSOFT_LOGIN) {
                        if let Err(e) = window.set_focus() {
                            warn!("Could not set focus to microsoft-login window: {:?}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play,
            login::mojang_login,
            login::microsoft_login,
            settings::set_settings,
            settings::get_settings,
            update::check_update
        ])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
