use std::sync::Arc;

use cleanbro_core::{AppResult, ScanId, ScanMode};
use tauri::{AppHandle, State};

use crate::commands::require_scan_id;
use crate::services::scan_runtime::{db_path, spawn_scan, ScanState};
use crate::services::scan_service::ScanRequest;
use crate::services::system_service::home_dir;

#[derive(Debug, serde::Serialize)]
pub struct StartScanResponse {
    pub scan_id: ScanId,
}

#[derive(Debug, serde::Serialize)]
pub struct CancelScanResponse {
    pub cancelled: bool,
}

/// Mulai scan di background; hasil dikirim lewat event `scan://*`.
/// Frontend mengirim argumen snake_case (`scan_id`, `mode`, `categories`).
#[tauri::command(rename_all = "snake_case")]
pub async fn start_scan(
    app: AppHandle,
    state: State<'_, Arc<ScanState>>,
    mode: ScanMode,
    categories: Option<Vec<String>>,
) -> AppResult<StartScanResponse> {
    let home = home_dir()?;
    let db = db_path(&app)?;
    let scan_id = spawn_scan(
        app,
        Arc::clone(&state),
        home,
        db,
        ScanRequest { mode, categories },
    )?;
    Ok(StartScanResponse { scan_id })
}

#[tauri::command(rename_all = "snake_case")]
pub fn cancel_scan(
    state: State<'_, Arc<ScanState>>,
    scan_id: String,
) -> AppResult<CancelScanResponse> {
    let scan_id = require_scan_id(&scan_id)?;
    Ok(CancelScanResponse {
        cancelled: state.cancel(&scan_id),
    })
}
