use std::fmt;

use serde::{Deserialize, Serialize};

use crate::rules::RiskLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanMode {
    Quick,
    Deep,
}

impl ScanMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quick => "quick",
            Self::Deep => "deep",
        }
    }
}

/// Identitas scan; dibangkitkan di server agar UI tidak menentukan id.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScanId(String);

impl ScanId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ScanId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for ScanId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for ScanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Satu artefak yang ditemukan; satu baris tabel `findings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub scan_id: ScanId,
    pub rule_id: String,
    pub path: String,
    pub bytes: u64,
    pub item_count: u64,
    pub risk: RiskLevel,
    pub project: Option<String>,
    pub excluded: bool,
    /// `true` bila pengguna pernah mengubah status excluded secara eksplisit.
    #[serde(default)]
    pub excluded_set: bool,
}

impl Finding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scan_id: ScanId,
        rule_id: impl Into<String>,
        path: impl Into<String>,
        bytes: u64,
        item_count: u64,
        risk: RiskLevel,
        project: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            scan_id,
            rule_id: rule_id.into(),
            path: path.into(),
            bytes,
            item_count,
            risk,
            project,
            excluded: false,
            excluded_set: false,
        }
    }

    /// Default UI: artefak berisiko tidak dipilih otomatis.
    pub fn default_excluded(&self) -> bool {
        self.risk != RiskLevel::Safe
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub scan_id: ScanId,
    pub scanned_files: u64,
    pub current_path: String,
    pub categories_done: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDone {
    pub scan_id: ScanId,
    pub rule_id: String,
    pub bytes: u64,
    pub item_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanFinished {
    pub scan_id: ScanId,
    pub partial: bool,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_mode_serializes_lowercase() {
        assert_eq!(serde_json::to_value(ScanMode::Quick).unwrap(), "quick");
        assert_eq!(serde_json::to_value(ScanMode::Deep).unwrap(), "deep");
    }

    #[test]
    fn scan_id_is_transparent_string() {
        let id = ScanId::new();
        assert!(!id.as_str().is_empty());
        assert_eq!(serde_json::to_value(&id).unwrap(), id.as_str());
        let back: ScanId = serde_json::from_str(&format!("\"{}\"", id)).unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn finding_round_trips() {
        let finding = Finding::new(
            ScanId::new(),
            "cache.gradle",
            "/home/user/.gradle/caches",
            1024,
            3,
            RiskLevel::Safe,
            None,
        );
        let text = serde_json::to_string(&finding).unwrap();
        let back: Finding = serde_json::from_str(&text).unwrap();
        assert_eq!(back, finding);
        assert!(!back.excluded);
    }
}
