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

#[cfg(target_os = "ios")]
fn get_device_name() -> Option<String> {
    None // 简化处理，可扩展为调用 UIDevice.current.name
}

#[cfg(not(target_os = "ios"))]
fn get_device_name() -> Option<String> {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}