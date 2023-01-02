use log::debug;
use reqwest::header::HeaderValue;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum MicrosoftTokenResponse {
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

#[derive(Serialize)]
#[allow(dead_code)]
pub struct MicrosoftTokenForm {
    pub tenant: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub redirect_uri: String,
    pub grant_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

pub async fn microsoft_token_request(client: &Client, tenant: &str, form: MicrosoftTokenForm) -> Result<MicrosoftTokenResponse, reqwest::Error> {
    debug!("Sending JSON {:?}", serde_json::to_string(&form));

    let content_type = HeaderValue::from_str(&format!("{}", "application/x-www-form-urlencoded")).expect("Header value creation bug");

    client
        .request(
            reqwest::Method::POST,
            format!("https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token").parse::<Url>().unwrap(),
        )
        .header(header::CONTENT_TYPE, content_type)
        .body(serde_urlencoded::to_string(form).unwrap())
        .send()
        .await?
        .json::<MicrosoftTokenResponse>()
        .await
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XBoxLiveAuthenticationResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: XBoxLiveAuthenticationDisplayClaims,
}
#[derive(Deserialize)]
pub struct XBoxLiveAuthenticationDisplayClaims {
    pub xui: Vec<XBoxLiveAuthenticationClaim>,
}
#[derive(Deserialize)]
pub struct XBoxLiveAuthenticationClaim {
    pub uhs: String,
}
pub async fn request_xbox_live_token(client: &Client, access_token: String) -> Result<XBoxLiveAuthenticationResponse, reqwest::Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct XBoxLiveAuthProps {
        auth_method: String,
        site_name: String,
        rps_ticket: String,
    }
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct XBoxLiveAuthForm {
        properties: XBoxLiveAuthProps,
        relying_party: String,
        token_type: String,
    }

    let form = XBoxLiveAuthForm {
        properties: XBoxLiveAuthProps {
            auth_method: "RPS".to_owned(),
            site_name: "user.auth.xboxlive.com".to_owned(),
            rps_ticket: format!("d={access_token}"),
        },
        relying_party: "http://auth.xboxlive.com".to_string(),
        token_type: "JWT".to_string(),
    };

    debug!("Sending request to https://user.auth.xboxlive.com/user/authenticate with json {:?}", serde_json::to_string(&form));

    client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&form)
        .send()
        .await?
        .json()
        .await
}
