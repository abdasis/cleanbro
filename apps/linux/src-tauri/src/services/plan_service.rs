use std::path::PathBuf;

use cleanbro_core::models::CleanPlan;
use cleanbro_core::rules::catalog;
use cleanbro_core::store::db;
use cleanbro_core::store::finding_repo::{
    effective_excluded, findings_for_scan, set_finding_excluded,
};
use cleanbro_core::{AppResult, PlanItem};

/// Konteks service: home untuk label rule, path DB state.
#[derive(Debug, Clone)]
pub struct PlanContext {
    pub db_path: PathBuf,
}

impl PlanContext {
    pub fn new(db_path: impl Into<PathBuf>) -> Self {
        Self {
            db_path: db_path.into(),
        }
    }
}

/// Bangun plan dari findings scan; `rule_ids` kosong berarti semua kategori.
pub fn preview_selection(
    context: &PlanContext,
    scan_id: &str,
    rule_ids: &[String],
) -> AppResult<CleanPlan> {
    build_plan(context, scan_id, Some(rule_ids))
}

/// Ubah status excluded lalu kembalikan plan penuh scan tersebut.
pub fn set_item_excluded(
    context: &PlanContext,
    scan_id: &str,
    item_id: &str,
    excluded: bool,
) -> AppResult<CleanPlan> {
    let connection = db::open(&context.db_path)?;
    set_finding_excluded(&connection, scan_id, item_id, excluded)?;
    drop(connection);
    build_plan(context, scan_id, None)
}

fn build_plan(
    context: &PlanContext,
    scan_id: &str,
    rule_filter: Option<&[String]>,
) -> AppResult<CleanPlan> {
    let connection = db::open(&context.db_path)?;
    let findings = findings_for_scan(&connection, scan_id)?;
    let mut items: Vec<PlanItem> = findings
        .into_iter()
        .filter(|finding| match rule_filter {
            Some(ids) if !ids.is_empty() => ids.iter().any(|id| id == &finding.rule_id),
            _ => true,
        })
        .map(|finding| {
            let reason = catalog::by_id(&finding.rule_id)
                .map(|rule| rule.note.to_owned())
                .unwrap_or_default();
            let excluded = effective_excluded(&finding);
            PlanItem {
                item_id: finding.id,
                rule_id: finding.rule_id,
                path: finding.path,
                bytes: finding.bytes,
                item_count: finding.item_count,
                risk: finding.risk,
                excluded,
                reason,
            }
        })
        .collect();
    items.sort_by(|left, right| {
        right
            .bytes
            .cmp(&left.bytes)
            .then_with(|| left.path.cmp(&right.path))
    });
    Ok(CleanPlan::from_items(scan_id.to_owned(), items))
}
