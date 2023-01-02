mod microsoft;

use std::{convert::Infallible, net::SocketAddr};
use std::collections::HashMap;
use hyper::{Request, service::service_fn, Response, StatusCode};
use hyper::server::conn::http1;
use lazy_static::lazy_static;
use log::{error, trace};
use regex::Regex;
use tauri::{AppHandle, Wry};
use tokio::net::TcpListener;
use crate::routing::microsoft::microsoft_auth;

async fn router(
    req: Request<hyper::body::Incoming>,
    handle: AppHandle<Wry>,
) -> Result<Response<String>, Infallible> {
    trace!("Received request at {} with {:?}", req.uri(), req.body());

    if req.uri().path() == "/microsoft-auth" {
        microsoft_auth(req, handle).await
    } else {
        Ok(Response::builder().status(StatusCode::NOT_FOUND).body("".to_owned()).unwrap())
    }
}

pub async fn start_server(addr: SocketAddr, handle: AppHandle<Wry>) {
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let stream = match listener.accept().await {
            Ok((stream, _)) => stream,
            Err(e) => {
                error!("Error while accepting connection: {:?}", e);
                continue;
            }
        };

        let handle = handle.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(move |req| router(req, handle.clone())))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

fn parse_query(query: &str) -> HashMap<String, Option<String>> {
    lazy_static! {
        static ref QUERY_PARAM_REGEX: Regex = Regex::new(r"(\w+)(?:=([^&]*))?").unwrap();
    }

    let parameters = query.split('&');
    let mut map = HashMap::new();

    for param in parameters {
        if let Some(m) = QUERY_PARAM_REGEX.captures(param) {
            if let Some(name) = m.get(1) {
                map.insert(name.as_str().to_owned(), m.get(2).map(|v| v.as_str().to_owned()));
            } else {
                error!("No name for parameter: {param}")
            }
        } else {
            error!("Non matched parameter: {param}")
        }
    }

    map
}