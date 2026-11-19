use crate::models::ScanMode;
use crate::rules::{Category, Rule};

pub use super::catalog_data::RULES;

pub fn by_id(id: &str) -> Option<&'static Rule> {
    RULES.iter().find(|rule| rule.id == id)
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
