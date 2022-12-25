#![allow(unused)]

///! Imported from https://www.github.com/Thechi2000/bootstrap

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf};


use log::{error};
use ring::digest::{Algorithm, Context, Digest, SHA256, SHA384, SHA512, SHA512_256};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Wry};
use tokio::sync::mpsc;

pub use error::Error;
use crate::consts::*;
use crate::send_event;
use crate::update::updater::update;

pub mod error;
pub mod updater;

#[derive(Serialize, Deserialize)]
/// Represents a file in the Info struct
struct FileInfo {
    /// The path of the file relative to the root of the program
    pub path: String,
    /// The hash of the file (using Crockford representation)
    pub hash: String,
    /// A placeholder file will never be overwritten, only downloaded when absent
    pub placeholder: bool,
}

#[derive(Serialize, Deserialize)]
/// Represents the JSON sent by the server to compute which files must be updated
struct Info {
    /// Url of the root of the program on the remote server
    pub base_url: String,
    /// Algorithm to generate hashes
    pub algorithm: String,
    /// Vector of the file info of all files
    pub files: Vec<FileInfo>,
    /// Vector of all the files/dirs that won't be modified
    pub ignored_files: Vec<PathBuf>,
}

/// Compute the hash of a file
/// # Arguments
/// * 'path' - The path of the file to hash
/// * 'digest' - The hash algorithm to use
fn hash_file(path: &PathBuf, algo: &'static Algorithm) -> Result<Digest, Error> {
    let mut file = File::open(path)?;
    let mut context = Context::new(algo);
    let mut buffer = [0; 1024];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn scan_dir(path: PathBuf, ignored: &Vec<PathBuf>) -> Result<Vec<PathBuf>, Error> {
    if ignored.contains(&path) {
        Ok(Vec::new())
    } else if path.is_dir() {
        path.read_dir()?
            .map(|d|
                scan_dir(d.map_err(|_| Error::Other("".to_string()))?.path(), ignored))
            .collect::<Result<Vec<Vec<PathBuf>>, Error>>()
            .map(|v|
                v.into_iter()
                    .flatten()
                    .collect())
    } else {
        Ok(Vec::from([path]))
    }.map(|v| v.into_iter().filter(|p| p.is_file()).collect())
}

fn convert_hash_algorithm(name: &str) -> Option<&'static Algorithm> {
    match name.to_lowercase().as_str() {
        "sha256" => Some(&SHA256),
        "sha384" => Some(&SHA384),
        "sha512" => Some(&SHA512),
        "sha512_256" => Some(&SHA512_256),
        _ => None,
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Message {
    /// Updated total downloads with done bytes downloaded
    UpdateState { name: Option<String>, done: u64, total: u64 },

    /// Starting update process
    Start,
    /// Fetch of files to updated completed
    FetchDone,
    /// Unnecessary files deletion completed
    CleanDone,
    /// Update completed
    DownloadDone,
    /// Update failed
    Failure,
}

#[tauri::command]
pub async fn check_update(app: AppHandle<Wry>) {
    let (tx, mut rx) = mpsc::channel(10);
    let mut map = HashMap::new();
    let mut total = 0;
    let mut total_done = 0;

    tokio::spawn(update(tx, GAME_UPDATE_URL.into(), files::GAME_DIR.into()));

    send_event!(app, events::SETTINGS_UPDATE, Message::Start);

    while let Some(msg) = rx.recv().await {
        let msg = match msg {
            updater::Message::AddState(id, size, name) => {
                map.insert(id, (name, 0, size));
                total += size;
                continue;
            }
            updater::Message::UpdateState(id, done) => {
                let Some((name, already_done, size)) = map.get(&id).cloned() else {
                    error!("Unknown id received");
                    continue;
                };

                total_done = total_done - already_done + done;
                map.insert(id, (name.clone(), done, size));

                Message::UpdateState { name, done: total_done, total }
            }
            updater::Message::FetchDone => {
                Message::FetchDone
            }
            updater::Message::CleanDone => {
                Message::CleanDone
            }
            updater::Message::DownloadDone => {
                Message::DownloadDone
            }
            updater::Message::Interrupted(e) => {
                error!("Could not update game: {:?}", e);
                Message::Failure
            }
        };

        send_event!(app, events::GAME_UPDATE, msg)
    }
}
