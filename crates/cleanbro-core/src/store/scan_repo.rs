use rusqlite::{params, Connection};

use crate::error::{AppError, AppResult, ErrorCode};
use crate::models::Finding;

/// Ringkasan satu scan untuk tabel `scans`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanRow {
    pub id: String,
    pub mode: String,
    pub started_at: i64,
    pub finished_at: i64,
    pub partial: bool,
    pub bytes_found: u64,
}

/// Simpan scan + findings atomik.
pub fn save_scan(
    connection: &mut Connection,
    scan: &ScanRow,
    findings: &[Finding],
) -> AppResult<()> {
    let transaction = connection
        .transaction()
        .map_err(|error| map("memulai transaksi", error))?;
    transaction
        .execute(
            "INSERT OR REPLACE INTO scans
             (id, mode, started_at, finished_at, partial, bytes_found)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                scan.id,
                scan.mode,
                scan.started_at,
                scan.finished_at,
                scan.partial as i64,
                scan.bytes_found as i64,
            ],
        )
        .map_err(|error| map("menyimpan scan", error))?;
    for finding in findings {
        transaction
            .execute(
                "INSERT OR REPLACE INTO findings
                 (id, scan_id, rule_id, path, bytes, item_count, risk, project,
                  excluded, excluded_set)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    finding.id,
                    finding.scan_id.as_str(),
                    finding.rule_id,
                    finding.path,
                    finding.bytes as i64,
                    finding.item_count as i64,
                    finding.risk.as_str(),
                    finding.project,
                    finding.excluded as i64,
                    finding.excluded_set as i64,
                ],
            )
            .map_err(|error| map("menyimpan finding", error))?;
    }
    transaction
        .commit()
        .map_err(|error| map("commit scan", error))
}

fn map(operation: &str, error: rusqlite::Error) -> AppError {
    AppError::new(ErrorCode::Internal, format!("{operation} gagal")).with_detail(error.to_string())
}
