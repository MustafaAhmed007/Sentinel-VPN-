use tauri::{Emitter, Manager};

#[derive(Clone, serde::Serialize)]
struct TrayStatus {
    connected: bool,
}

#[tauri::command]
fn service_status() -> TrayStatus {
    // Production builds query the local privileged service over authenticated
    // IPC. The UI never gains SYSTEM privileges and never owns firewall state.
    TrayStatus { connected: false }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![service_status])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("sentinel-ready", true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Sentinel-VPN Ω");
}
