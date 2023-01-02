use log::info;

#[tauri::command]
pub async fn mojang_login(email: String, password: String, remember: bool){
    info!("Logging in with {email} {password} {remember}")
}