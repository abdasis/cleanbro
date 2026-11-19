use serde::{Deserialize, Serialize};

pub mod catalog;
mod catalog_data;
pub mod guard;
pub mod matcher;

pub use catalog::RULES;
pub use guard::Guard;
pub use matcher::Matcher;

/// Versi katalog rule yang dipakai kedua aplikasi.
pub const RULES_VERSION: &str = "rules-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Review,
    Risky,
}

impl RiskLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Review => "review",
            Self::Risky => "risky",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeleteMode {
    Trash,
    Permanent,
}

impl DeleteMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trash => "trash",
            Self::Permanent => "permanent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    DeveloperCache,
    Docker,
    Build,
    Media,
    System,
}

impl Category {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeveloperCache => "developer_cache",
            Self::Docker => "docker",
            Self::Build => "build",
            Self::Media => "media",
            Self::System => "system",
        }
    }

    /// Slug kebab yang dipakai kontrak UI (`developer-cache`).
    pub fn slug(self) -> &'static str {
        match self {
            Self::DeveloperCache => "developer-cache",
            Self::Docker => "docker",
            Self::Build => "build",
            Self::Media => "media",
            Self::System => "system",
        }
    }
}

/// Prasyarat tambahan yang diperiksa guard sebelum artefak boleh dihapus.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GuardSet {
    /// Penanda root proyek yang harus ada di direktori induk artefak.
    pub project_markers: &'static [&'static str],
    /// Tolak bila direktori induk berada di worktree git yang kotor.
    pub reject_dirty_git: bool,
}

/// Definisi rule (RULES.md bagian 10). `globs` relatif terhadap `$HOME`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    pub id: &'static str,
    pub globs: &'static [&'static str],
    pub category: Category,
    pub risk: RiskLevel,
    pub delete_mode: DeleteMode,
    pub requires: GuardSet,
    pub max_depth: u8,
    pub note: &'static str,
}

impl Rule {
    pub fn delete_mode_str(&self) -> &'static str {
        self.delete_mode.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enums_serialize_lowercase() {
        assert_eq!(serde_json::to_value(RiskLevel::Review).unwrap(), "review");
        assert_eq!(
            serde_json::to_value(DeleteMode::Permanent).unwrap(),
            "permanent"
        );
        assert_eq!(RULES_VERSION, "rules-v1");
    }

    #[test]
    fn risk_as_str_matches_serde() {
        assert_eq!(RiskLevel::Safe.as_str(), "safe");
        assert_eq!(RiskLevel::Review.as_str(), "review");
    }
}
