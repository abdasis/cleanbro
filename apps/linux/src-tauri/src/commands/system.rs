use cleanbro_core::{AppInfo, AppResult, DiskSummary, Platform};

use crate::services::system_service::{disk_summary_for, home_dir};

/// Identitas app ini; dipisah dari command agar bisa dites tanpa runtime Tauri.
pub fn current_app_info() -> AppInfo {
    AppInfo::new("Sapu Bersih", env!("CARGO_PKG_VERSION"), Platform::Linux)
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    current_app_info()
}

/// Thin command: resolve `$HOME`, lalu serahkan pengukuran ke service.
#[tauri::command(rename_all = "snake_case")]
pub fn disk_summary() -> AppResult<DiskSummary> {
    let home = home_dir()?;
    disk_summary_for(&home)
}
