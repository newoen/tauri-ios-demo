use serde::Serialize;

#[derive(Serialize)]
pub struct SystemInfo {
    os: String,
    arch: String,
    tauri_version: String,
    device_name: Option<String>,
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        tauri_version: tauri::VERSION.to_string(),
        device_name: get_device_name(),
    }
}

fn get_device_name() -> Option<String> {
    #[cfg(not(target_os = "ios"))]
    {
        return std::env::var("COMPUTERNAME")
            .or_else(|_| std::env::var("HOSTNAME"))
            .ok();
    }
    #[cfg(target_os = "ios")]
    {
        None
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}