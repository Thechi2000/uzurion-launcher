pub mod files {
    pub const SETTINGS: &str = "../settings.json";
    pub const GAME_DIR: &str = "../game";
}

pub mod events {
    pub const SERVER_STATUS_REFRESH: &str = "server-status";
    pub const SETTINGS_UPDATE: &str = "settings-update";
    pub const GAME_UPDATE: &str = "game-update";
}

pub mod windows {
    pub const MAIN: &str = "main";
    pub const MICROSOFT_LOGIN: &str = "microsoft-login";
}

pub mod microsoft {
    pub const TENANT: &str = "consumers";
    pub const CLIENT_ID: &str = env!("CLIENT_ID");
}

pub const GAME_UPDATE_URL: &str = "http://127.0.0.1:8000";
pub const LOCAL_WEBSERVER_URL: &str = "http://127.0.0.1:3000";

#[macro_export]
macro_rules! send_event {
    ($app: expr, $event: expr, $payload: expr) => {
        {
            ::log::debug!("Sending event {} with payload {:?}", $event, ::serde_json::to_string(&$payload));
            if let Err(e) = ::tauri::Manager::emit_all(&$app, $event, $payload){
                ::log::error!("Could not send {} event: {:?}", $event, e);
            }
        }
    };
}