use crate::models::{RuleInfo, ScanMode};
use crate::rules::{Category, Rule};

pub use super::catalog_data::RULES;

pub fn by_id(id: &str) -> Option<&'static Rule> {
    RULES.iter().find(|rule| rule.id == id)
}

/// Katalog ringkas untuk UI Setelan, urut category lalu id.
pub fn list_info() -> Vec<RuleInfo> {
    let mut infos: Vec<RuleInfo> = RULES.iter().map(RuleInfo::from_rule).collect();
    infos.sort_by(|a, b| (&a.category, &a.id).cmp(&(&b.category, &b.id)));
    infos
}

/// Quick: cache developer saja. Deep: cache + artefak build + media.
pub fn for_mode(mode: ScanMode) -> Vec<&'static Rule> {
    RULES
        .iter()
        .filter(|rule| match mode {
            ScanMode::Quick => rule.category == Category::DeveloperCache,
            ScanMode::Deep => {
                matches!(
                    rule.category,
                    Category::DeveloperCache | Category::Build | Category::Media
                )
            }
        })
        .collect()
}

/// Kedalaman maksimum yang perlu ditelusuri untuk menemukan kandidat.
pub fn max_depth(rules: &[&'static Rule]) -> usize {
    rules
        .iter()
        .map(|rule| rule.max_depth as usize)
        .max()
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_info_has_every_rule_sorted() {
        let infos = list_info();
        assert_eq!(infos.len(), RULES.len());
        let mut sorted = infos.clone();
        sorted.sort_by(|a, b| (&a.category, &a.id).cmp(&(&b.category, &b.id)));
        assert_eq!(infos, sorted);
    }

    #[test]
    fn list_info_maps_category_risk_delete_mode() {
        let gradle = list_info()
            .into_iter()
            .find(|r| r.id == "cache.gradle")
            .unwrap();
        assert_eq!(gradle.category, "developer-cache");
        assert_eq!(gradle.risk, "safe");
        assert_eq!(gradle.delete_mode, "trash");

        let node_modules = list_info()
            .into_iter()
            .find(|r| r.id == "build.node_modules")
            .unwrap();
        assert_eq!(node_modules.category, "build");
    }

    #[test]
    fn risk_and_delete_mode_strings_are_in_contract() {
        for info in list_info() {
            assert!(
                matches!(info.risk.as_str(), "safe" | "review" | "risky"),
                "risk tak dikenal: {}",
                info.risk
            );
            assert!(
                matches!(info.delete_mode.as_str(), "trash" | "permanent"),
                "delete_mode tak dikenal: {}",
                info.delete_mode
            );
            assert!(
                matches!(
                    info.category.as_str(),
                    "developer-cache" | "build" | "media" | "docker" | "system"
                ),
                "category tak dikenal: {}",
                info.category
            );
        }
    }

    #[test]
    fn rule_info_round_trips_serde() {
        let info = list_info().remove(0);
        let text = serde_json::to_string(&info).unwrap();
        let back: RuleInfo = serde_json::from_str(&text).unwrap();
        assert_eq!(back, info);
        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["category"], info.category);
    }
}
