use std::sync::Arc;

use cleanbro_core::{AppError, AppResult, CleanMode, ErrorCode, TokenGrant};
use tauri::{AppHandle, State};

use crate::adapters::SystemTrash;
use crate::commands::require_scan_id;
use crate::services::exec_service::{self, ExecContext, ExecRequest};
use crate::services::scan_runtime::{db_path, emitter_for};
use crate::services::system_service::home_dir;
use crate::services::token_service::TokenStore;

#[derive(Debug, serde::Serialize)]
pub struct ExecuteResponse {
    pub action_id: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn request_clean_token(
    app: AppHandle,
    tokens: State<'_, Arc<TokenStore>>,
    scan_id: String,
    item_ids: Vec<String>,
) -> AppResult<TokenGrant> {
    let scan_id = require_scan_id(&scan_id)?;
    let context = ExecContext {
        home: home_dir()?,
        db_path: db_path(&app)?,
    };
    exec_service::request_token(
        &context,
        &tokens,
        &scan_id,
        &item_ids,
        exec_service::now_millis(),
    )
}

#[tauri::command(rename_all = "snake_case")]
pub async fn execute_clean(
    app: AppHandle,
    tokens: State<'_, Arc<TokenStore>>,
    scan_id: String,
    item_ids: Vec<String>,
    mode: CleanMode,
    confirm_token: String,
) -> AppResult<ExecuteResponse> {
    let scan_id = require_scan_id(&scan_id)?;
    let context = ExecContext {
        home: home_dir()?,
        db_path: db_path(&app)?,
    };
    let store = Arc::clone(&tokens);
    let emitter = emitter_for(&app);
    let request = ExecRequest {
        scan_id,
        item_ids,
        mode,
        confirm_token,
    };
    let outcome = tauri::async_runtime::spawn_blocking(move || {
        exec_service::execute(
            &context,
            &store,
            request,
            emitter.as_ref(),
            &SystemTrash,
            exec_service::now_millis(),
        )
    })
    .await
    .map_err(|error| {
        AppError::new(ErrorCode::Internal, "eksekusi gagal dijalankan")
            .with_detail(error.to_string())
    })??;
    Ok(ExecuteResponse {
        action_id: outcome.action_id,
    })
}
