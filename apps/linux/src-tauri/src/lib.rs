pub mod adapters;
pub mod commands;
pub mod services;

use std::sync::Arc;

pub use commands::system::current_app_info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(services::scan_runtime::ScanState::default()))
        .manage(Arc::new(services::token_service::TokenStore::new()))
        .invoke_handler(tauri::generate_handler![
            commands::system::app_info,
            commands::system::disk_summary,
            commands::docker::docker_status,
            commands::docker::docker_prune,
            commands::scan::start_scan,
            commands::scan::cancel_scan,
            commands::preview::preview_selection,
            commands::preview::set_item_excluded,
            commands::execute::request_clean_token,
            commands::execute::execute_clean,
            commands::history::list_history,
            commands::history::undo_action,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Sapu Bersih");
}

#[cfg(test)]
mod tests {
    use super::*;
    use cleanbro_core::Platform;

    #[test]
    fn app_info_matches_this_app() {
        let info = current_app_info();
        assert_eq!(info.name, "Sapu Bersih");
        assert_eq!(info.platform, Platform::Linux);
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.rules_version, "rules-v1");
    }
}
