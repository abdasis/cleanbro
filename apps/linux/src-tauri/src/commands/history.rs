use cleanbro_core::{AppError, AppResult, ErrorCode, HistoryPage, UndoOutcome};
use tauri::AppHandle;

use crate::adapters::SystemTrash;
use crate::services::exec_service::{self, ExecContext};
use crate::services::history_service;
use crate::services::scan_runtime::db_path;
use crate::services::system_service::home_dir;

#[tauri::command(rename_all = "snake_case")]
pub fn list_history(app: AppHandle, limit: u32, offset: u32) -> AppResult<HistoryPage> {
    let context = context(&app)?;
    history_service::list_history(
        &context,
        limit,
        offset,
        &SystemTrash,
        exec_service::now_millis(),
    )
}

#[tauri::command(rename_all = "snake_case")]
pub async fn undo_action(app: AppHandle, action_id: String) -> AppResult<UndoOutcome> {
    let action_id = action_id.trim().to_owned();
    if action_id.is_empty() {
        return Err(AppError::new(ErrorCode::NotFound, "action_id kosong"));
    }
    let context = context(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        history_service::undo_action(
            &context,
            &action_id,
            &SystemTrash,
            exec_service::now_millis(),
        )
    })
    .await
    .map_err(|error| {
        AppError::new(ErrorCode::Internal, "undo gagal dijalankan").with_detail(error.to_string())
    })?
}

fn context(app: &AppHandle) -> AppResult<ExecContext> {
    Ok(ExecContext {
        home: home_dir()?,
        db_path: db_path(app)?,
    })
}
