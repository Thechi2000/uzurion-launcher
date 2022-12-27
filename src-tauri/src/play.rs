use crate::login::requests::{request_xbox_live_token, XBoxLiveAuthenticationResponse};
use crate::AppState;
use log::{debug, error, trace, warn};
use tauri::{AppHandle, Manager, Wry};

#[tauri::command]
pub async fn play(app: AppHandle<Wry>) {
    let state = app.state::<AppState>();
    println!("Have fun :)");

    trace!("Requesting XBoxLive authentication token");
    let Some(access_token) = futures::executor::block_on(state.microsoft_login.lock().unwrap().access_token_or_refresh(&state.client)) else {
        warn!("Could not refresh access token");
        return;
    };

    debug!("Sending request with token {access_token}");
    let (xbox_auth_token, user_hash) = match request_xbox_live_token(&state.client, access_token).await {
        Ok(XBoxLiveAuthenticationResponse { token, display_claims, .. }) => {
            let Some(user_hash) = display_claims.xui.first() else {
                error!("Missing user hash in XBoxLive response");
                return;
            };
            (token, user_hash.uhs.clone())
        }
        Err(e) => {
            error!("Could not request XBoxLive authentication token: {:?}", e);
            return;
        }
    };

    debug!("Received XBoxLive auth_token: \"{xbox_auth_token}\" and user hash: \"{user_hash}\"")
}
