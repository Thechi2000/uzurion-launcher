use std::convert::Infallible;
use hyper::{Request, Response};

use hyper::http::uri::PathAndQuery;
use log::{debug, error, warn};
use tauri::{AppHandle, Manager, Wry};
use crate::consts::*;
use crate::login::receive_auth_code;

use crate::routing::parse_query;

pub async fn microsoft_auth(
    req: Request<hyper::body::Incoming>,
    handle: AppHandle<Wry>,
) -> Result<Response<String>, Infallible> {
    let Some(Some(query)) = req.uri().path_and_query().map(PathAndQuery::query) else {
        warn!("No query parameters");
        return Ok(Response::new("".to_owned()))
    };

    handle.get_window(windows::MICROSOFT_LOGIN).map(|w| {
        if let Err(e) = w.close() {
            error!("Could not close microsoft-login window: {:?}", e);
        }
    });

    let query = parse_query(query);
    debug!("{:?}", query);

    if let Some(Some(code)) = query.get("code") {
        tokio::spawn(receive_auth_code(code.to_owned(), handle));
    }

    Ok(Response::new("".to_owned()))
}