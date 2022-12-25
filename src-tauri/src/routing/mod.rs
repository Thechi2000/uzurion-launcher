mod microsoft;

use std::{convert::Infallible, net::SocketAddr, error::Error};
use hyper::{Request, body::Bytes, service::service_fn, Response, StatusCode};
use hyper::server::conn::http1;
use log::{error, trace};
use tauri::{AppHandle, Wry};
use tokio::net::TcpListener;
use crate::routing::microsoft::microsoft_auth;

async fn router(
    req: Request<hyper::body::Incoming>,
    handle: AppHandle<Wry>,
) -> Result<Response<String>, Infallible> {
    trace!("Received request at {}", req.uri());

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