use std::cell::Cell;
use std::sync::Arc;
use std::time::Duration;
use log::{debug, info, trace, warn};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct ServerStatus {
    pub online: bool,

    #[serde(default)]
    pub players: Option<PlayersStatus>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayersStatus {
    pub max: u32,
    pub online: u32,
}

async fn fetch_server_status(ip: &str) -> Result<ServerStatus, String> {
    // Using https://api.mcsrvstat.us/ api to get server status
    reqwest::get(format!("https://api.mcsrvstat.us/2/{ip}")).await
        .map_err(|e| format!("Could not send request to mcapi: {:?}", e))?.json::<ServerStatus>().await.map_err(|e| format!("Could not parse response: {:?}", e))
}

pub async fn fetch_server_status_task(ip: &str, app_state: Arc<Mutex<Cell<Option<ServerStatus>>>>) {
    info!("Server status fetching task started");

    // Uses interval to fetch every 5 seconds
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        // Fetch state and get lock from the state
        let state = fetch_server_status(ip).await;
        let lock = app_state.lock().await;

        // Update the state and log results
        match state {
            Ok(status) => {
                debug!("Received state {:?}", serde_json::to_string(&status));
                lock.set(Some(status));
            }
            Err(e) => {
                warn!("Could not fetch server status: {:?}", e);
                lock.set(None);
            }
        }

        interval.tick().await;
    }
}