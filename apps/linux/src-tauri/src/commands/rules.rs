use cleanbro_core::rules::catalog;
use cleanbro_core::RuleInfo;

/// Katalog rule untuk UI Setelan.
#[tauri::command]
pub fn list_rules() -> Vec<RuleInfo> {
    catalog::list_info()
}
