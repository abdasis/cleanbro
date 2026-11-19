use serde::{Deserialize, Serialize};

/// Versi katalog rule yang dipakai kedua aplikasi.
pub const RULES_VERSION: &str = "rules-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Review,
    Risky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeleteMode {
    Trash,
    Permanent,
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
}
