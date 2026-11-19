use cleanbro_core::{AppInfo, Platform};

/// Identitas app ini; dipisah dari command agar bisa dites tanpa runtime Tauri.
pub fn current_app_info() -> AppInfo {
    AppInfo::new("Sapu Bersih", env!("CARGO_PKG_VERSION"), Platform::Linux)
}

#[tauri::command]
fn app_info() -> AppInfo {
    current_app_info()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_info])
        .run(tauri::generate_context!())
        .expect("failed to run Sapu Bersih");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_matches_this_app() {
        let info = current_app_info();
        assert_eq!(info.name, "Sapu Bersih");
        assert_eq!(info.platform, Platform::Linux);
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.rules_version, "rules-v1");
    }
}
