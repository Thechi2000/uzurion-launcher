use std::convert::Infallible;
use hyper::{Request, Response};
use hyper::body::Bytes;
use log::debug;
use tauri::{AppHandle, Wry};

pub async fn microsoft_auth(
    req: Request<hyper::body::Incoming>,
    handle: AppHandle<Wry>,
) -> Result<Response<String>, Infallible> {
    debug!("{:?}", req.uri().path_and_query().unwrap().query());
    Ok(Response::new("hi".to_owned()))
}