#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[tauri::command]
fn play() {
    println!("Have fun :)")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
