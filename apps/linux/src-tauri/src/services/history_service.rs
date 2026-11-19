use cleanbro_core::models::{
    ActionStatus, ActionSummary, CleanMode, HistoryPage, UndoEntry, UndoOutcome,
};
use cleanbro_core::store::action_repo::{self, ActionRow};
use cleanbro_core::store::db;
use cleanbro_core::{AppError, AppResult, ErrorCode};

use crate::adapters::TrashGateway;
use crate::services::exec_service::ExecContext;

const MAX_LIMIT: u32 = 200;

/// Riwayat aksi terbaru dulu dengan status undo dihitung dari jurnal + trash.
pub fn list_history(
    context: &ExecContext,
    limit: u32,
    offset: u32,
    gateway: &dyn TrashGateway,
    now: u64,
) -> AppResult<HistoryPage> {
    let connection = db::open(&context.db_path)?;
    let limit = limit.clamp(1, MAX_LIMIT);
    let (actions, total) = action_repo::list_actions(&connection, limit, offset)?;
    let mut items = Vec::with_capacity(actions.len());
    for action in actions {
        let item_count = action_repo::item_count(&connection, &action.id)?;
        let journal = action_repo::journal_for(&connection, &action.id)?;
        let undoable = is_undoable(&action, journal, gateway, now);
        items.push(summary(action, item_count, undoable));
    }
    Ok(HistoryPage { items, total })
}

/// Pulihkan item yang masih ada di trash; kegagalan per item tidak fatal.
pub fn undo_action(
    context: &ExecContext,
    action_id: &str,
    gateway: &dyn TrashGateway,
    now: u64,
) -> AppResult<UndoOutcome> {
    let connection = db::open(&context.db_path)?;
    let action = action_repo::action_by_id(&connection, action_id)?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, format!("aksi {action_id} tidak ada")))?;
    if action.status == ActionStatus::Undone.as_str() {
        return Err(rejected("aksi sudah di-undo"));
    }
    let journal = action_repo::journal_for(&connection, action_id)?
        .ok_or_else(|| rejected("tidak ada jurnal undo untuk aksi ini"))?;
    if journal.expires_at <= now as i64 {
        action_repo::delete_journal(&connection, action_id)?;
        return Err(rejected("jurnal undo sudah kedaluwarsa"));
    }
    let entries: Vec<UndoEntry> = serde_json::from_str(&journal.payload).map_err(|error| {
        AppError::new(ErrorCode::Internal, "jurnal undo rusak").with_detail(error.to_string())
    })?;

    let mut restored = 0u64;
    let mut failed = 0u64;
    for entry in entries {
        if gateway.exists(&entry.trash_ref)
            && gateway
                .restore(&entry.trash_ref, std::path::Path::new(&entry.path))
                .is_ok()
        {
            restored += 1;
        } else {
            failed += 1;
        }
    }
    if restored > 0 {
        action_repo::mark_status(&connection, action_id, ActionStatus::Undone.as_str())?;
    }
    action_repo::delete_journal(&connection, action_id)?;
    Ok(UndoOutcome { restored, failed })
}

fn is_undoable(
    action: &ActionRow,
    journal: Option<action_repo::UndoJournalRow>,
    gateway: &dyn TrashGateway,
    now: u64,
) -> bool {
    if action.status == ActionStatus::Undone.as_str() {
        return false;
    }
    let Some(journal) = journal else {
        return false;
    };
    if journal.expires_at <= now as i64 {
        return false;
    }
    let entries: Vec<UndoEntry> = match serde_json::from_str(&journal.payload) {
        Ok(entries) => entries,
        Err(_) => return false,
    };
    entries.iter().any(|entry| gateway.exists(&entry.trash_ref))
}

fn summary(action: ActionRow, item_count: u64, undoable: bool) -> ActionSummary {
    ActionSummary {
        mode: CleanMode::from_db(&action.mode).unwrap_or(CleanMode::Trash),
        status: ActionStatus::from_db(&action.status).unwrap_or(ActionStatus::Failed),
        id: action.id,
        scan_id: action.scan_id,
        started_at: action.started_at.max(0) as u64,
        finished_at: action.finished_at.map(|value| value.max(0) as u64),
        freed_bytes: action.freed_bytes,
        item_count,
        undoable,
    }
}

fn rejected(message: &str) -> AppError {
    AppError::new(ErrorCode::GuardRejected, message)
}
