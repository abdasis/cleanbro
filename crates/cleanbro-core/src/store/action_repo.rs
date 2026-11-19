use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{AppError, AppResult, ErrorCode};

/// Satu baris tabel `actions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRow {
    pub id: String,
    pub scan_id: String,
    pub mode: String,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub freed_bytes: u64,
    pub status: String,
    pub confirm_token: Option<String>,
}

/// Satu baris tabel `action_items`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionItemRow {
    pub id: String,
    pub action_id: String,
    pub path: String,
    pub bytes: u64,
    pub state: String,
    pub reason: Option<String>,
    pub trash_ref: Option<String>,
}

/// Satu baris tabel `audit` (keputusan guard).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRow {
    pub ts: i64,
    pub scan_id: String,
    pub item_id: String,
    pub rule_id: String,
    pub decision: String,
    pub reason: Option<String>,
}

/// Satu baris tabel `undo_journal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndoJournalRow {
    pub action_id: String,
    pub payload: String,
    pub created_at: i64,
    pub expires_at: i64,
}

/// Simpan aksi beserta item, audit, dan jurnal undo dalam satu transaksi.
pub fn save_action(
    connection: &mut Connection,
    action: &ActionRow,
    items: &[ActionItemRow],
    audit: &[AuditRow],
    journal: Option<&UndoJournalRow>,
) -> AppResult<()> {
    let transaction = connection
        .transaction()
        .map_err(|error| map("memulai transaksi aksi", error))?;
    transaction
        .execute(
            "INSERT INTO actions
             (id, scan_id, mode, started_at, finished_at, freed_bytes, status, confirm_token)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                action.id,
                action.scan_id,
                action.mode,
                action.started_at,
                action.finished_at,
                action.freed_bytes as i64,
                action.status,
                action.confirm_token,
            ],
        )
        .map_err(|error| map("menyimpan aksi", error))?;
    for item in items {
        transaction
            .execute(
                "INSERT INTO action_items
                 (id, action_id, path, bytes, state, reason, trash_ref)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    item.id,
                    item.action_id,
                    item.path,
                    item.bytes as i64,
                    item.state,
                    item.reason,
                    item.trash_ref,
                ],
            )
            .map_err(|error| map("menyimpan item aksi", error))?;
    }
    for entry in audit {
        transaction
            .execute(
                "INSERT INTO audit (ts, scan_id, item_id, rule_id, decision, reason)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    entry.ts,
                    entry.scan_id,
                    entry.item_id,
                    entry.rule_id,
                    entry.decision,
                    entry.reason,
                ],
            )
            .map_err(|error| map("menyimpan audit", error))?;
    }
    if let Some(journal) = journal {
        transaction
            .execute(
                "INSERT OR REPLACE INTO undo_journal
                 (action_id, payload, created_at, expires_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    journal.action_id,
                    journal.payload,
                    journal.created_at,
                    journal.expires_at
                ],
            )
            .map_err(|error| map("menyimpan jurnal undo", error))?;
    }
    transaction
        .commit()
        .map_err(|error| map("commit aksi", error))
}

/// Riwayat terbaru dulu, plus total keseluruhan.
pub fn list_actions(
    connection: &Connection,
    limit: u32,
    offset: u32,
) -> AppResult<(Vec<ActionRow>, u64)> {
    let total: i64 = connection
        .query_row("SELECT COUNT(*) FROM actions", [], |row| row.get(0))
        .map_err(|error| map("menghitung aksi", error))?;
    let mut statement = connection
        .prepare(
            "SELECT id, scan_id, mode, started_at, finished_at, freed_bytes,
                    status, confirm_token
             FROM actions ORDER BY started_at DESC, id DESC LIMIT ?1 OFFSET ?2",
        )
        .map_err(|error| map("menyiapkan riwayat", error))?;
    let rows = statement
        .query_map(params![limit, offset], row_to_action)
        .map_err(|error| map("membaca riwayat", error))?;
    let mut actions = Vec::new();
    for row in rows {
        actions.push(row.map_err(|error| map("membaca baris aksi", error))?);
    }
    Ok((actions, total as u64))
}

pub fn action_by_id(connection: &Connection, action_id: &str) -> AppResult<Option<ActionRow>> {
    connection
        .query_row(
            "SELECT id, scan_id, mode, started_at, finished_at, freed_bytes,
                    status, confirm_token
             FROM actions WHERE id = ?1",
            params![action_id],
            row_to_action,
        )
        .optional()
        .map_err(|error| map("membaca aksi", error))
}

pub fn item_count(connection: &Connection, action_id: &str) -> AppResult<u64> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM action_items WHERE action_id = ?1",
            params![action_id],
            |row| row.get(0),
        )
        .map_err(|error| map("menghitung item aksi", error))?;
    Ok(count as u64)
}

pub fn journal_for(connection: &Connection, action_id: &str) -> AppResult<Option<UndoJournalRow>> {
    connection
        .query_row(
            "SELECT action_id, payload, created_at, expires_at
             FROM undo_journal WHERE action_id = ?1",
            params![action_id],
            |row| {
                Ok(UndoJournalRow {
                    action_id: row.get(0)?,
                    payload: row.get(1)?,
                    created_at: row.get(2)?,
                    expires_at: row.get(3)?,
                })
            },
        )
        .optional()
        .map_err(|error| map("membaca jurnal undo", error))
}

pub fn mark_status(connection: &Connection, action_id: &str, status: &str) -> AppResult<()> {
    connection
        .execute(
            "UPDATE actions SET status = ?1 WHERE id = ?2",
            params![status, action_id],
        )
        .map_err(|error| map("memperbarui status aksi", error))?;
    Ok(())
}

pub fn delete_journal(connection: &Connection, action_id: &str) -> AppResult<()> {
    connection
        .execute(
            "DELETE FROM undo_journal WHERE action_id = ?1",
            params![action_id],
        )
        .map_err(|error| map("menghapus jurnal undo", error))?;
    Ok(())
}

fn row_to_action(row: &rusqlite::Row<'_>) -> rusqlite::Result<ActionRow> {
    Ok(ActionRow {
        id: row.get(0)?,
        scan_id: row.get(1)?,
        mode: row.get(2)?,
        started_at: row.get(3)?,
        finished_at: row.get(4)?,
        freed_bytes: row.get::<_, i64>(5)? as u64,
        status: row.get(6)?,
        confirm_token: row.get(7)?,
    })
}

fn map(operation: &str, error: rusqlite::Error) -> AppError {
    AppError::new(ErrorCode::Internal, format!("{operation} gagal")).with_detail(error.to_string())
}
