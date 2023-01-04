use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use tauri::{State, Wry, AppHandle, Manager};
use tokio::fs::read_to_string;
use crate::{AppState, send_error, send_event};
use crate::consts::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub game: GameSettings,
    pub launcher: LauncherSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            game: GameSettings {
                resolution: (1920, 1080),
                ram: 1024,
            },
            launcher: LauncherSettings {},
        }
    }
}

impl Settings {
    pub async fn load() -> Option<Self> {
        match read_to_string(files::SETTINGS).await {
            Ok(s) => {
                match serde_json::from_str(s.as_str()) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        error!("Could not parse settings file: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Could not read from settings file: {:?}", e);
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameSettings {
    pub resolution: (u16, u16),
    pub ram: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LauncherSettings {}

#[tauri::command]
pub async fn set_settings(settings: Settings, state: State<'_, AppState>, app: AppHandle<Wry>) -> Result<(), ()> {
    *state.settings.lock().unwrap() = settings.clone();
    debug!("Updating settings to {:?}", settings);

    let settings_json = match serde_json::to_string(&settings) {
        Ok(s) => s,
        Err(e) => {
            error!("Could not convert settings {:?} to json: {:?}", settings, e);
            send_error!(app, "Invalid settings file", e);
            return Err(());
        }
    };

    send_event!(app, events::SETTINGS_UPDATE, settings);

    if let Err(e) = tokio::fs::write(files::SETTINGS, settings_json).await {
        error!("Could not write settings to {}: {:?}", files::SETTINGS, e);
        send_error!(app, "Cannot save settings", e);
        Err(())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>, app: AppHandle<Wry>) -> Result<(), ()> {
    if let Err(e) = app.emit_all(events::SETTINGS_UPDATE, state.settings.lock().unwrap().clone()) {
        error!("Could not send {} event: {:?}", events::SETTINGS_UPDATE, e);
        Err(())
    } else {
        Ok(())
    }
}