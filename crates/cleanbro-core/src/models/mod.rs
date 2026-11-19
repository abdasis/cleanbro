use serde::{Deserialize, Serialize};

use crate::rules::RULES_VERSION;

mod action;
mod disk;
mod docker;
mod scan;

pub use action::{
    ActionStatus, ActionSummary, CleanFinished, CleanItemState, CleanMode, CleanPlan,
    CleanProgress, HistoryPage, PlanItem, TokenGrant, UndoEntry, UndoOutcome,
    CONFIRM_THRESHOLD_BYTES,
};
pub use disk::{DiskSummary, WarnLevel};
pub use docker::{DockerContainer, DockerDiskUsage, DockerImage, DockerPruneResult, DockerStatus};
pub use scan::{CategoryDone, Finding, ScanFinished, ScanId, ScanMode, ScanProgress};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Linux,
    Macos,
}

/// Identitas aplikasi yang dilaporkan ke UI lewat command `app_info`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub platform: Platform,
    pub rules_version: String,
}

impl AppInfo {
    pub fn new(name: &str, version: &str, platform: Platform) -> Self {
        Self {
            name: name.to_owned(),
            version: version.to_owned(),
            platform,
            rules_version: RULES_VERSION.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fills_rules_version() {
        let info = AppInfo::new("Sapu Bersih", "0.1.0", Platform::Linux);
        assert_eq!(info.name, "Sapu Bersih");
        assert_eq!(info.version, "0.1.0");
        assert_eq!(info.platform, Platform::Linux);
        assert_eq!(info.rules_version, "rules-v1");
    }

    #[test]
    fn serializes_platform_lowercase() {
        let info = AppInfo::new("CleanBro", "0.1.0", Platform::Macos);
        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["platform"], "macos");
        assert_eq!(json["rules_version"], "rules-v1");

        let linux = AppInfo::new("Sapu Bersih", "0.1.0", Platform::Linux);
        assert_eq!(serde_json::to_value(&linux).unwrap()["platform"], "linux");
    }

    #[test]
    fn macos_round_trips() {
        let info = AppInfo::new("CleanBro", "0.1.0", Platform::Macos);
        let text = serde_json::to_string(&info).unwrap();
        let back: AppInfo = serde_json::from_str(&text).unwrap();
        assert_eq!(back, info);
    }
}
