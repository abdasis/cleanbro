use rusqlite::{params, Connection};

use crate::error::{AppError, AppResult, ErrorCode};
use crate::models::{Finding, ScanId};
use crate::rules::RiskLevel;

/// Semua finding milik satu scan, urut path agar hasil deterministik.
pub fn findings_for_scan(connection: &Connection, scan_id: &str) -> AppResult<Vec<Finding>> {
    let mut statement = connection
        .prepare(
            "SELECT id, scan_id, rule_id, path, bytes, item_count, risk, project,
                    excluded, excluded_set
             FROM findings WHERE scan_id = ?1 ORDER BY path",
        )
        .map_err(|error| map("menyiapkan query finding", error))?;
    let rows = statement
        .query_map(params![scan_id], row_to_finding)
        .map_err(|error| map("membaca finding", error))?;
    let mut findings = Vec::new();
    for row in rows {
        findings.push(row.map_err(|error| map("membaca baris finding", error))?);
    }
    Ok(findings)
}

/// Ambil finding tertentu (milik scan yang sama) dengan urutan sesuai input.
pub fn findings_by_ids(
    connection: &Connection,
    scan_id: &str,
    item_ids: &[String],
) -> AppResult<Vec<Finding>> {
    let mut findings = findings_for_scan(connection, scan_id)?;
    findings.retain(|finding| item_ids.iter().any(|id| id == &finding.id));
    findings.sort_by_key(|finding| {
        item_ids
            .iter()
            .position(|id| id == &finding.id)
            .unwrap_or(usize::MAX)
    });
    Ok(findings)
}

/// Tandai status excluded pilihan pengguna; `excluded_set` mengunci nilainya.
pub fn set_finding_excluded(
    connection: &Connection,
    scan_id: &str,
    item_id: &str,
    excluded: bool,
) -> AppResult<()> {
    let changed = connection
        .execute(
            "UPDATE findings SET excluded = ?1, excluded_set = 1
             WHERE scan_id = ?2 AND id = ?3",
            params![excluded as i64, scan_id, item_id],
        )
        .map_err(|error| map("memperbarui finding", error))?;
    if changed == 0 {
        return Err(AppError::new(
            ErrorCode::NotFound,
            format!("item {item_id} tidak ada pada scan {scan_id}"),
        ));
    }
    Ok(())
}

/// Status efektif: pilihan pengguna menang; selain itu default berdasarkan risiko.
pub fn effective_excluded(finding: &Finding) -> bool {
    if finding.excluded_set {
        finding.excluded
    } else {
        finding.default_excluded()
    }
}

fn row_to_finding(row: &rusqlite::Row<'_>) -> rusqlite::Result<Finding> {
    let scan_id: String = row.get(1)?;
    let risk: String = row.get(6)?;
    let excluded: i64 = row.get(8)?;
    Ok(Finding {
        id: row.get(0)?,
        scan_id: ScanId::from(scan_id),
        rule_id: row.get(2)?,
        path: row.get(3)?,
        bytes: row.get::<_, i64>(4)? as u64,
        item_count: row.get::<_, i64>(5)? as u64,
        risk: risk_from_db(&risk),
        project: row.get(7)?,
        excluded: excluded != 0,
        excluded_set: row.get::<_, i64>(9)? != 0,
    })
}

fn risk_from_db(value: &str) -> RiskLevel {
    match value {
        "review" => RiskLevel::Review,
        "risky" => RiskLevel::Risky,
        _ => RiskLevel::Safe,
    }
}

fn map(operation: &str, error: rusqlite::Error) -> AppError {
    AppError::new(ErrorCode::Internal, format!("{operation} gagal")).with_detail(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_excluded_prefers_explicit_choice() {
        let mut finding = Finding::new(
            ScanId::new(),
            "build.dist",
            "/home/uji/proyek/dist",
            10,
            1,
            RiskLevel::Review,
            None,
        );
        assert!(effective_excluded(&finding));
        finding.excluded_set = true;
        finding.excluded = false;
        assert!(!effective_excluded(&finding));
    }
}
