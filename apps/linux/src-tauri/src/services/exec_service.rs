use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use cleanbro_core::models::{CleanFinished, CleanItemState, CleanMode, UndoEntry};
use cleanbro_core::store::action_repo::{ActionItemRow, ActionRow, AuditRow, UndoJournalRow};
use cleanbro_core::store::db;
use cleanbro_core::store::finding_repo::findings_by_ids;
use cleanbro_core::{
    AppError, AppResult, ErrorCode, Guard, Matcher, TokenGrant, CONFIRM_THRESHOLD_BYTES,
};
use serde_json::json;

use crate::adapters::TrashGateway;
use crate::services::event::EventEmitter;
use crate::services::exec_progress::Progress;
use crate::services::exec_validation::{self, unique_ids};
use crate::services::token_service::TokenStore;

const UNDO_TTL_MS: i64 = 30 * 24 * 60 * 60 * 1000;

#[derive(Debug, Clone)]
pub struct ExecContext {
    pub home: PathBuf,
    pub db_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecRequest {
    pub scan_id: String,
    pub item_ids: Vec<String>,
    pub mode: CleanMode,
    pub confirm_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecOutcome {
    pub action_id: String,
    pub deleted: u64,
    pub failed: u64,
    pub skipped: u64,
    pub freed_bytes: u64,
}

/// Terbitkan token untuk pilihan item; semua id harus milik scan.
pub fn request_token(
    context: &ExecContext,
    tokens: &TokenStore,
    scan_id: &str,
    item_ids: &[String],
    now: u64,
) -> AppResult<TokenGrant> {
    let connection = db::open(&context.db_path)?;
    let unique = unique_ids(item_ids);
    let findings = findings_by_ids(&connection, scan_id, &unique)?;
    if findings.len() != unique.len() {
        return Err(AppError::new(
            ErrorCode::NotFound,
            "sebagian item tidak ditemukan pada scan",
        ));
    }
    Ok(tokens.issue(scan_id, &unique, now))
}

/// Eksekusi dua fase: validasi ulang semua path, lalu hapus/mindahkan.
pub fn execute(
    context: &ExecContext,
    tokens: &TokenStore,
    request: ExecRequest,
    emitter: &dyn EventEmitter,
    gateway: &dyn TrashGateway,
    now: u64,
) -> AppResult<ExecOutcome> {
    let connection = db::open(&context.db_path)?;
    let item_ids = unique_ids(&request.item_ids);
    if item_ids.is_empty() {
        return Err(AppError::new(
            ErrorCode::InvalidPath,
            "tidak ada item dipilih",
        ));
    }
    let findings = findings_by_ids(&connection, &request.scan_id, &item_ids)?;
    if findings.len() != item_ids.len() {
        return Err(AppError::new(
            ErrorCode::NotFound,
            "sebagian item tidak ditemukan pada scan",
        ));
    }
    let total_bytes: u64 = findings.iter().map(|finding| finding.bytes).sum();
    let needs_token = total_bytes > CONFIRM_THRESHOLD_BYTES || request.mode == CleanMode::Delete;
    if needs_token {
        tokens.consume(&request.scan_id, &item_ids, &request.confirm_token, now)?;
    }

    let matcher = Matcher::new(&context.home);
    let guard = Guard::new(&context.home);
    let (valid, mut skipped_items, mut audit) = exec_validation::validate(
        &findings,
        &matcher,
        &guard,
        request.mode,
        now,
        &request.scan_id,
    );
    let progress = Progress::new();
    let action_id = uuid::Uuid::new_v4().to_string();

    let mut items = Vec::new();
    let mut undo_entries = Vec::new();
    let mut deleted = 0u64;
    let mut failed = 0u64;
    let mut freed_bytes = 0u64;

    for entry in &mut skipped_items {
        entry.action_id = action_id.clone();
        items.push(entry.clone());
        progress.emit(
            emitter,
            &action_id,
            &entry.path,
            CleanItemState::Skipped,
            freed_bytes,
        );
    }
    for entry in valid {
        let path = PathBuf::from(&entry.finding.path);
        let outcome = match request.mode {
            CleanMode::Trash => gateway.trash(&path).map(|reference| {
                undo_entries.push(UndoEntry {
                    item_id: entry.finding.id.clone(),
                    path: entry.finding.path.clone(),
                    trash_ref: reference.clone(),
                    bytes: entry.finding.bytes,
                });
                Some(reference)
            }),
            CleanMode::Delete => exec_validation::permanent_delete(&path).map(|()| None),
        };
        let (state, reason, trash_ref) = match outcome {
            Ok(reference) => {
                deleted += 1;
                freed_bytes += entry.finding.bytes;
                (CleanItemState::Done, None, reference)
            }
            Err(error) => {
                failed += 1;
                (CleanItemState::Failed, Some(error), None)
            }
        };
        items.push(ActionItemRow {
            id: uuid::Uuid::new_v4().to_string(),
            action_id: action_id.clone(),
            path: entry.finding.path.clone(),
            bytes: entry.finding.bytes,
            state: state.as_str().to_owned(),
            reason,
            trash_ref,
        });
        progress.emit(emitter, &action_id, &entry.finding.path, state, freed_bytes);
    }

    let skipped = items
        .iter()
        .filter(|item| item.state == CleanItemState::Skipped.as_str())
        .count() as u64;
    let journal = build_journal(&action_id, &undo_entries, now);
    let action = ActionRow {
        id: action_id.clone(),
        scan_id: request.scan_id.clone(),
        mode: request.mode.as_str().to_owned(),
        started_at: now as i64,
        finished_at: Some(now_millis() as i64),
        freed_bytes,
        status: status_of(deleted, failed, skipped).as_str().to_owned(),
        confirm_token: Some(request.confirm_token),
    };
    audit.push(AuditRow {
        ts: now as i64,
        scan_id: request.scan_id.clone(),
        item_id: String::new(),
        rule_id: String::new(),
        decision: "finish".to_owned(),
        reason: Some(format!(
            "deleted={deleted} failed={failed} skipped={skipped}"
        )),
    });
    let mut connection = connection;
    cleanbro_core::store::action_repo::save_action(
        &mut connection,
        &action,
        &items,
        &audit,
        journal.as_ref(),
    )?;

    emitter.emit(
        "clean://finished",
        json!(CleanFinished {
            action_id: action_id.clone(),
            deleted,
            failed,
            skipped,
            freed_bytes,
        }),
    );
    Ok(ExecOutcome {
        action_id,
        deleted,
        failed,
        skipped,
        freed_bytes,
    })
}

fn build_journal(action_id: &str, entries: &[UndoEntry], now: u64) -> Option<UndoJournalRow> {
    if entries.is_empty() {
        return None;
    }
    let payload = serde_json::to_string(entries).ok()?;
    Some(UndoJournalRow {
        action_id: action_id.to_owned(),
        payload,
        created_at: now as i64,
        expires_at: now as i64 + UNDO_TTL_MS,
    })
}

fn status_of(deleted: u64, failed: u64, skipped: u64) -> cleanbro_core::ActionStatus {
    use cleanbro_core::ActionStatus;
    if deleted == 0 {
        ActionStatus::Failed
    } else if failed == 0 && skipped == 0 {
        ActionStatus::Completed
    } else {
        ActionStatus::Partial
    }
}

pub fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}
