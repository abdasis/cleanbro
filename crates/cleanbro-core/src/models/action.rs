use serde::{Deserialize, Serialize};

use crate::rules::RiskLevel;

/// Ambang wajib konfirmasi ulang (5 GB).
pub const CONFIRM_THRESHOLD_BYTES: u64 = 5_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CleanMode {
    Trash,
    Delete,
}

impl CleanMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trash => "trash",
            Self::Delete => "delete",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "trash" => Some(Self::Trash),
            "delete" => Some(Self::Delete),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionStatus {
    Completed,
    Partial,
    Failed,
    Undone,
}

impl ActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::Undone => "undone",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "completed" => Some(Self::Completed),
            "partial" => Some(Self::Partial),
            "failed" => Some(Self::Failed),
            "undone" => Some(Self::Undone),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CleanItemState {
    Done,
    Failed,
    Skipped,
}

impl CleanItemState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Done => "done",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }
}

/// Satu baris pratinjau; `item_id` adalah id finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanItem {
    pub item_id: String,
    pub rule_id: String,
    pub path: String,
    pub bytes: u64,
    pub item_count: u64,
    pub risk: RiskLevel,
    pub excluded: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanPlan {
    pub scan_id: String,
    pub total_bytes: u64,
    pub total_items: u64,
    pub requires_confirmation: bool,
    pub items: Vec<PlanItem>,
}

impl CleanPlan {
    /// Total hanya menghitung item yang tidak dikecualikan.
    pub fn from_items(scan_id: impl Into<String>, items: Vec<PlanItem>) -> Self {
        let total_bytes = items
            .iter()
            .filter(|item| !item.excluded)
            .map(|item| item.bytes)
            .sum();
        let total_items = items.iter().filter(|item| !item.excluded).count() as u64;
        Self {
            scan_id: scan_id.into(),
            total_bytes,
            total_items,
            requires_confirmation: total_bytes > CONFIRM_THRESHOLD_BYTES,
            items,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenGrant {
    pub token: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanProgress {
    pub action_id: String,
    pub item_id: String,
    pub state: CleanItemState,
    pub freed_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanFinished {
    pub action_id: String,
    pub deleted: u64,
    pub failed: u64,
    pub skipped: u64,
    pub freed_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionSummary {
    pub id: String,
    pub scan_id: String,
    pub mode: CleanMode,
    pub started_at: u64,
    pub finished_at: Option<u64>,
    pub freed_bytes: u64,
    pub status: ActionStatus,
    pub item_count: u64,
    pub undoable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryPage {
    pub items: Vec<ActionSummary>,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoOutcome {
    pub restored: u64,
    pub failed: u64,
}

/// Satu entri jurnal undo untuk item yang berhasil dipindah ke trash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoEntry {
    pub item_id: String,
    pub path: String,
    pub trash_ref: String,
    pub bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_requires_confirmation_above_threshold() {
        let item = PlanItem {
            item_id: "a".to_owned(),
            rule_id: "cache.gradle".to_owned(),
            path: "/home/uji/.gradle/caches".to_owned(),
            bytes: CONFIRM_THRESHOLD_BYTES,
            item_count: 1,
            risk: RiskLevel::Safe,
            excluded: false,
            reason: String::new(),
        };
        assert!(!CleanPlan::from_items("s", vec![item.clone()]).requires_confirmation);
        let mut bigger = item.clone();
        bigger.bytes += 1;
        assert!(CleanPlan::from_items("s", vec![bigger]).requires_confirmation);
    }

    #[test]
    fn plan_ignores_excluded_items_in_totals() {
        let mk = |id: &str, bytes: u64, excluded: bool| PlanItem {
            item_id: id.to_owned(),
            rule_id: "cache.gradle".to_owned(),
            path: format!("/home/uji/{id}"),
            bytes,
            item_count: 1,
            risk: RiskLevel::Safe,
            excluded,
            reason: String::new(),
        };
        let plan = CleanPlan::from_items("s", vec![mk("a", 10, false), mk("b", 90, true)]);
        assert_eq!(plan.total_bytes, 10);
        assert_eq!(plan.total_items, 1);
    }

    #[test]
    fn enums_use_stable_lowercase() {
        assert_eq!(serde_json::to_value(CleanMode::Delete).unwrap(), "delete");
        assert_eq!(
            serde_json::to_value(ActionStatus::Undone).unwrap(),
            "undone"
        );
        assert_eq!(
            serde_json::to_value(CleanItemState::Skipped).unwrap(),
            "skipped"
        );
        assert_eq!(
            ActionStatus::from_db("partial"),
            Some(ActionStatus::Partial)
        );
        assert_eq!(CleanMode::from_db("trash"), Some(CleanMode::Trash));
    }
}
