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
