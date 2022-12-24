use std::time::Duration;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use tauri::{App, AppHandle, Manager, Wry};
use crate::consts::*;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ServerStatus {
    pub online: bool,

    #[serde(default)]
    pub players: Option<PlayersStatus>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PlayersStatus {
    pub max: u32,
    pub online: u32,
}

async fn fetch_server_status(ip: &str) -> Result<ServerStatus, String> {
    // Using https://api.mcsrvstat.us/ api to get server status
    reqwest::get(format!("https://api.mcsrvstat.us/2/{ip}")).await
        .map_err(|e| format!("Could not send request to https://api.mcsrvstat.us/2/: {:?}", e))?.json::<ServerStatus>().await.map_err(|e| format!("Could not parse response: {:?}", e))
}

pub async fn refresh_server_status(ip: &str, tx: AppHandle<Wry>) {
// Fetch state and get lock from the state
    let state = fetch_server_status(ip).await;

    // Update the state and log results
    match state {
        Ok(status) => {
            debug!("Received state {:?}", serde_json::to_string(&status));

            if let Err(e) = tx.emit_all(events::SERVER_STATUS_REFRESH, Some(status)) {
                warn!("Could not send ServerStatus: {:?}", e);
            }
        }
        Err(e) => {
            warn!("Could not fetch server status: {:?}", e);
            if let Err(e) = tx.emit_all(events::SERVER_STATUS_REFRESH, Option::<ServerStatus>::None) {
                warn!("Could not send ServerStatus: {:?}", e);
            }
        }
    }
}

async fn fetch_server_status_task(ip: String, tx: AppHandle<Wry>) {
    info!("Server status fetching task started");

    // Uses interval to fetch every 5 seconds
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        refresh_server_status(ip.as_str(), tx.clone()).await;
        interval.tick().await;
    }
}

pub fn start_fetch_server_status_task(app: &mut App<Wry>, ip: &str) {
    tokio::task::spawn(fetch_server_status_task(ip.to_string(), app.handle()));
}
