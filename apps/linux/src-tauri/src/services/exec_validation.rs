use std::path::Path;

use cleanbro_core::models::{CleanItemState, CleanMode, Finding};
use cleanbro_core::store::action_repo::{ActionItemRow, AuditRow};
use cleanbro_core::{DeleteMode, Guard, Matcher};

pub struct Validated {
    pub finding: Finding,
}

/// Fase 1: validasi ulang matcher + guard tepat sebelum aksi (TOCTOU).
pub fn validate(
    findings: &[Finding],
    matcher: &Matcher,
    guard: &Guard,
    mode: CleanMode,
    now: u64,
    scan_id: &str,
) -> (Vec<Validated>, Vec<ActionItemRow>, Vec<AuditRow>) {
    let mut valid = Vec::new();
    let mut skipped = Vec::new();
    let mut audit = Vec::new();
    for finding in findings {
        let path = Path::new(&finding.path);
        let reason = match matcher.match_rule(path) {
            None => Some("rule tidak lagi cocok".to_owned()),
            Some(rule) if rule.id != finding.rule_id => Some("rule berubah".to_owned()),
            Some(rule)
                if mode == CleanMode::Delete && rule.delete_mode != DeleteMode::Permanent =>
            {
                Some("rule tidak mendukung hapus permanen".to_owned())
            }
            Some(rule) => match guard.check(path, rule) {
                Ok(()) => {
                    audit.push(audit_row(now, scan_id, finding, "allow", None));
                    valid.push(Validated {
                        finding: finding.clone(),
                    });
                    continue;
                }
                Err(error) => Some(error.message),
            },
        };
        let reason = reason.unwrap_or_else(|| "ditolak guard".to_owned());
        audit.push(audit_row(
            now,
            scan_id,
            finding,
            "skip",
            Some(reason.clone()),
        ));
        skipped.push(ActionItemRow {
            id: uuid::Uuid::new_v4().to_string(),
            action_id: String::new(),
            path: finding.path.clone(),
            bytes: finding.bytes,
            state: CleanItemState::Skipped.as_str().to_owned(),
            reason: Some(reason),
            trash_ref: None,
        });
    }
    (valid, skipped, audit)
}

pub fn permanent_delete(path: &Path) -> Result<(), String> {
    let meta = std::fs::symlink_metadata(path).map_err(|error| error.to_string())?;
    if meta.file_type().is_dir() {
        std::fs::remove_dir_all(path).map_err(|error| error.to_string())
    } else {
        std::fs::remove_file(path).map_err(|error| error.to_string())
    }
}

pub fn unique_ids(item_ids: &[String]) -> Vec<String> {
    let mut ids = item_ids.to_vec();
    ids.sort();
    ids.dedup();
    ids
}

fn audit_row(
    now: u64,
    scan_id: &str,
    finding: &Finding,
    decision: &str,
    reason: Option<String>,
) -> AuditRow {
    AuditRow {
        ts: now as i64,
        scan_id: scan_id.to_owned(),
        item_id: finding.id.clone(),
        rule_id: finding.rule_id.clone(),
        decision: decision.to_owned(),
        reason,
    }
}
