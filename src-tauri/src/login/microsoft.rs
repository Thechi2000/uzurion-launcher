use log::{debug, error, trace};
use oauth2::{PkceCodeChallenge, PkceCodeVerifier};
use reqwest::{Body, Client, header, Request, RequestBuilder};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl, Wry};
use url::{ParseError, Url};
use crate::AppState;
use crate::consts::LOCAL_WEBSERVER_URL;

//! TODO Check that no confidential info gets logged

#[derive(Default)]
pub struct MicrosoftLoginData {
    challenge: Option<PkceCodeChallenge>,
    verifier: Option<PkceCodeVerifier>,
}

fn generate_microsoft_auth_url(state: &str, code_challenge: &str) -> Result<Url, ParseError> {
    Url::parse_with_params(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize",
        [
            ("client_id", env!("CLIENT_ID")),
            ("response_type", "code"),
            ("redirect_url", LOCAL_WEBSERVER_URL), // TODO use dynamic port
            ("redirect_uri", format!("{LOCAL_WEBSERVER_URL}/microsoft-auth").as_str()), // TODO use dynamic port
            ("scope", "XboxLive.signin offline_access"),
            ("code_challenge_method", "S256"),
            ("code_challenge", code_challenge),
            ("state", state)
        ],
    )
}

#[derive(Deserialize, Debug)]
#[serde(untagged, )]
#[allow(dead_code)]
enum MicrosoftTokenResponse {
    Success {
        access_token: String,
        token_type: String,
        expires_in: u32,
        #[serde(default)]
        scope: Option<String>,
        #[serde(default)]
        refresh_token: Option<String>,
    },
    Error {
        error: String,
        error_description: String,
        error_codes: Vec<u32>,
        timestamp: String,
        trace_id: String,
        correlation_id: String,
    },
}

fn generate_microsoft_token_request(client: &Client, code: String, verifier: &String) -> RequestBuilder {
    #[derive(Serialize)]
    #[allow(dead_code)]
    struct MicrosoftTokenForm {
        tenant: String,
        client_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        code: String,
        redirect_uri: String,
        grant_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        code_verifier: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_secret: Option<String>,
    }

    let form = MicrosoftTokenForm {
        tenant: "common".to_owned(),
        client_id: env!("CLIENT_ID").to_string(),
        scope: None,
        code,
        redirect_uri: format!("{LOCAL_WEBSERVER_URL}/microsoft-auth"),
        grant_type: "authorization_code".to_string(),
        code_verifier: Some(verifier.clone()),
        client_secret: None,
    };

    debug!("Sending JSON {:?}", serde_json::to_string(&form));

    let content_type = HeaderValue::from_str(&format!("{}", "application/x-www-form-urlencoded")).expect("Header value creation bug");

    client.request(reqwest::Method::POST, "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".parse::<Url>().unwrap())
        .header(header::CONTENT_TYPE, content_type)
        .body(serde_urlencoded::to_string(form).unwrap())
}

#[tauri::command]
pub async fn microsoft_login(app: AppHandle<Wry>) {
    let (challenge, verifier) = oauth2::PkceCodeChallenge::new_random_sha256_len(64);
    let url = match generate_microsoft_auth_url("", challenge.as_str()) {
        Ok(u) => u,
        Err(e) => {
            error!("Could not generate microsoft login url: {:?}", e);
            return;
        }
    };

    let state = app.state::<AppState>();
    let mut lock = state.microsoft_login.lock().await;
    lock.challenge = Some(challenge);
    lock.verifier = Some(verifier);

    match WindowBuilder::new(
        &app,
        "microsoft-login",
        WindowUrl::External(url),
    )
        .focused(true)
        .title("Microsoft login")
        .build() {
        Ok(w) => w,
        Err(e) => {
            error!("Could not open window for microsoft login: {:?}", e);
            return;
        }
    };
}

pub async fn receive_auth_code(code: String, handle: AppHandle<Wry>) {
    let state = handle.state::<AppState>();
    let Some(verifier) = &state.microsoft_login.lock().await.verifier else {
        error!("Missing PKCE code verifier");
        return;
    };

    let req = generate_microsoft_token_request(&state.client, code, verifier.secret());
    trace!("Requesting access token");
    let res = match req.send().await {
        Ok(res) => res,
        Err(e) => {
            error!("Could not send request for access token: {:?}", e);
            return;
        }
    };

    let res = match res.json::<MicrosoftTokenResponse>().await {
        Ok(v) => v,
        Err(e) => {
            error!("Could not interpret response as valid JSON: {:?}", e);
            return;
        }
    };

    debug!("Received response: {:?}", res);
}