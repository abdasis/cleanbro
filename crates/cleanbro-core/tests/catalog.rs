use cleanbro_core::rules::catalog::{for_mode, max_depth};
use cleanbro_core::{Category, ScanMode, RULES};

#[test]
fn catalog_ids_are_unique() {
    let mut ids: Vec<_> = RULES.iter().map(|rule| rule.id).collect();
    let total = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), total);
    assert!(total >= 20, "katalog v1 harus lengkap");
}

#[test]
fn quick_mode_only_developer_cache() {
    let rules = for_mode(ScanMode::Quick);
    assert!(!rules.is_empty());
    assert!(rules
        .iter()
        .all(|rule| rule.category == Category::DeveloperCache));
    assert_eq!(max_depth(&rules), 5);
}

#[test]
fn deep_mode_includes_build_and_media() {
    let rules = for_mode(ScanMode::Deep);
    assert!(rules.iter().any(|rule| rule.category == Category::Build));
    assert!(rules.iter().any(|rule| rule.category == Category::Media));
    assert!(rules.iter().any(|rule| rule.id == "cache.gradle"));
}

#[test]
fn catalog_entries_have_notes_and_globs() {
    for rule in RULES {
        assert!(!rule.globs.is_empty(), "{} tanpa glob", rule.id);
        assert!(!rule.note.is_empty(), "{} tanpa catatan", rule.id);
        assert!(rule.max_depth > 0, "{} depth nol", rule.id);
    }
}

#[test]
fn catalog_rules_spec_covered() {
    for id in [
        "cache.gradle",
        "cache.npm",
        "cache.cargo.registry",
        "cache.chrome",
        "build.node_modules",
        "build.rust_target",
        "build.dist",
        "build.vendor",
        "media.render_out",
        "media.cache",
    ] {
        assert!(RULES.iter().any(|rule| rule.id == id), "{id} tidak ada");
    }
}
