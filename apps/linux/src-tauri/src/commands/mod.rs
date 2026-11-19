use cleanbro_core::{AppError, AppResult, ErrorCode};

pub mod docker;
pub mod execute;
pub mod history;
pub mod preview;
pub mod scan;
pub mod system;

pub use docker::{docker_prune, docker_status};
pub use execute::{execute_clean, request_clean_token};
pub use history::{list_history, undo_action};
pub use preview::{preview_selection, set_item_excluded};
pub use scan::{cancel_scan, start_scan};
pub use system::{app_info, disk_summary};

/// Validasi id scan dari UI sebelum menyentuh registry.
pub fn require_scan_id(raw: &str) -> AppResult<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AppError::new(ErrorCode::InvalidPath, "scan_id kosong"));
    }
    Ok(trimmed.to_owned())
}
