use log::{error, trace};
use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl, Wry};
use url::{ParseError, Url};


fn generate_microsoft_auth_url(state: &str, code_challenge: &str) -> Result<Url, ParseError> {
    Url::parse_with_params(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize",
        [
            ("client_id", env!("CLIENT_ID")),
            ("response_type", "code"),
            ("redirect_url", "http://localhost:1420"), // TODO use dynamic port
            ("redirect_uri", "http://localhost:1420/microsoft-auth/"), // TODO use dynamic port
            ("scope", "XboxLive.signin"),
            ("code_challenge_method", "S256"),
            ("code_challenge", code_challenge),
            ("state", state)
        ],
    )
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

    let window = match WindowBuilder::new(
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