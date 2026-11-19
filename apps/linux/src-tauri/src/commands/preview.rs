use cleanbro_core::{AppError, AppResult, CleanPlan, ErrorCode};
use tauri::AppHandle;

use crate::commands::require_scan_id;
use crate::services::plan_service::{self, PlanContext};
use crate::services::scan_runtime::db_path;

#[tauri::command(rename_all = "snake_case")]
pub fn preview_selection(
    app: AppHandle,
    scan_id: String,
    rule_ids: Vec<String>,
) -> AppResult<CleanPlan> {
    let scan_id = require_scan_id(&scan_id)?;
    let context = PlanContext::new(db_path(&app)?);
    plan_service::preview_selection(&context, &scan_id, &rule_ids)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_item_excluded(
    app: AppHandle,
    scan_id: String,
    item_id: String,
    excluded: bool,
) -> AppResult<CleanPlan> {
    let scan_id = require_scan_id(&scan_id)?;
    let item_id = item_id.trim();
    if item_id.is_empty() {
        return Err(AppError::new(ErrorCode::InvalidPath, "item_id kosong"));
    }
    let context = PlanContext::new(db_path(&app)?);
    plan_service::set_item_excluded(&context, &scan_id, item_id, excluded)
}
