use crate::consts::{microsoft, LOCAL_WEBSERVER_URL};
use crate::login::microsoft::requests::{microsoft_token_request, MicrosoftTokenForm, MicrosoftTokenResponse};
use crate::{AppState, send_error};
use chrono::{DateTime, Duration, Local};
use log::{debug, error, trace, warn};
use oauth2::{PkceCodeChallenge, PkceCodeVerifier};
use reqwest::Client;
use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl, Wry};
use url::{ParseError, Url};

pub mod requests;

// TODO Check that no confidential info gets logged

#[derive(Default)]
pub struct MicrosoftLoginData {
    challenge: Option<PkceCodeChallenge>,
    verifier: Option<PkceCodeVerifier>,
    access_token: Option<(String, DateTime<Local>)>,
    refresh_token: Option<String>,
}

impl MicrosoftLoginData {
    pub fn access_token(&self) -> Option<String> {
        debug!("###3");
        self.access_token.as_ref().map(|s| s.0.clone())
    }

    pub async fn access_token_or_refresh(&mut self, client: &Client) -> Option<String> {
        debug!("###0");
        self.refresh_access_token_if_needed(client).await;
        debug!("###1");
        self.access_token()
    }

    async fn refresh_access_token_if_needed(&mut self, client: &Client) {
        if self.access_token.as_ref().map(|p| p.1 < Local::now()).unwrap_or(true) {
            trace!("Refreshing Microsoft access token");
            if let Some(refresh_token) = &self.refresh_token {
                match microsoft_token_request(
                    client,
                    microsoft::TENANT,
                    MicrosoftTokenForm {
                        tenant: microsoft::TENANT.to_owned(),
                        client_id: microsoft::CLIENT_ID.to_string(),
                        scope: None,
                        code: None,
                        refresh_token: Some(refresh_token.clone()),
                        redirect_uri: format!("{LOCAL_WEBSERVER_URL}/microsoft-auth"),
                        grant_type: "refresh_token".to_string(),
                        code_verifier: None,
                        client_secret: None,
                    },
                )
                    .await
                {
                    Ok(MicrosoftTokenResponse::Success { access_token, expires_in, .. }) => {
                        trace!("Microsoft access token refreshed");
                        self.access_token = Some((access_token, Local::now() + Duration::seconds(expires_in as i64)))
                    }
                    Ok(MicrosoftTokenResponse::Error { error, error_description, .. }) => {
                        error!("Request for access_token failed with {error}: {error_description}");
                        return;
                    }
                    Err(e) => {
                        error!("Request for access_token failed: {:?}", e);
                        return;
                    }
                };
            } else {
                warn!("Tried to refresh access token, but no refresh token was available");
            }
        }
    }
}

fn generate_microsoft_auth_url(state: &str, code_challenge: &str) -> Result<Url, ParseError> {
    Url::parse_with_params(
        format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", microsoft::TENANT).as_str(),
        [
            ("client_id", microsoft::CLIENT_ID),
            ("response_type", "code"),
            ("redirect_url", LOCAL_WEBSERVER_URL),                                      // TODO use dynamic port
            ("redirect_uri", format!("{LOCAL_WEBSERVER_URL}/microsoft-auth").as_str()), // TODO use dynamic port
            ("scope", "XboxLive.signin offline_access"),
            ("code_challenge_method", "S256"),
            ("code_challenge", code_challenge),
            ("state", state),
        ],
    )
}

#[tauri::command]
pub async fn microsoft_login(app: AppHandle<Wry>) {
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256_len(64);
    let url = match generate_microsoft_auth_url("", challenge.as_str()) {
        Ok(u) => u,
        Err(e) => {
            error!("Could not generate microsoft login url: {:?}", e);
            send_error!(app, "Microsoft login failed", e);
            return;
        }
    };

    let state = app.state::<AppState>();
    let mut lock = state.microsoft_login.lock().unwrap();
    lock.challenge = Some(challenge);
    lock.verifier = Some(verifier);

    match WindowBuilder::new(&app, "microsoft-login", WindowUrl::External(url))
        .focused(true)
        .title("Microsoft login")
        .build()
    {
        Ok(w) => w,
        Err(e) => {
            error!("Could not open window for microsoft login: {:?}", e);
            send_error!(app, "Cannot create window", e);
            return;
        }
    };
}

pub async fn receive_auth_code(code: String, app: AppHandle<Wry>) {
    let state = app.state::<AppState>();
    let verifier = {
        let lock = state.microsoft_login.lock().unwrap();
        if let Some(verifier) = &lock.verifier {
            verifier.secret().clone()
        } else {
            error!("Missing PKCE code verifier");
            return;
        }
    };

    trace!("Requesting Microsoft access token");
    let res = match microsoft_token_request(
        &state.client,
        microsoft::TENANT,
        MicrosoftTokenForm {
            tenant: microsoft::TENANT.to_owned(),
            client_id: microsoft::CLIENT_ID.to_string(),
            scope: None,
            code: Some(code),
            refresh_token: None,
            redirect_uri: format!("{LOCAL_WEBSERVER_URL}/microsoft-auth"),
            grant_type: "authorization_code".to_string(),
            code_verifier: Some(verifier),
            client_secret: None,
        },
    )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("Request for access_token failed: {:?}", e);
            send_error!(app, "Microsoft login failed", e);
            return;
        }
    };

    debug!("Received response: {:?}", res);
    match res {
        MicrosoftTokenResponse::Success {
            access_token,
            expires_in,
            refresh_token,
            ..
        } => {
            let state = app.state::<AppState>();
            let mut state = state.microsoft_login.lock().unwrap();
            state.access_token = Some((access_token, Local::now() + Duration::seconds(expires_in as i64)));
            state.refresh_token = refresh_token;
        }
        MicrosoftTokenResponse::Error { error, error_description, .. } => {
            warn!("Received error {error} from Microsoft: {error_description}");
            send_error!(app, "Microsoft login failed", format!("{}: {}", error, error_description));
            return;
        }
    }
}
