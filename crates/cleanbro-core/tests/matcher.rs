mod common;

use cleanbro_core::Matcher;

#[test]
fn matcher_matches_known_cache() {
    let home = common::TempDir::new("matcher-cache");
    let matcher = Matcher::new(&home.path);
    let rule = matcher
        .match_rule(&home.path.join(".gradle/caches"))
        .expect("cocok");
    assert_eq!(rule.id, "cache.gradle");
}

#[test]
fn matcher_rejects_unknown_path() {
    let home = common::TempDir::new("matcher-unknown");
    let matcher = Matcher::new(&home.path);
    assert!(matcher.match_rule(&home.path.join("proyek/acak")).is_none());
}

#[test]
fn matcher_never_matches_denylist() {
    let home = common::TempDir::new("matcher-deny");
    let matcher = Matcher::new(&home.path);
    let deny = [
        ".ssh",
        ".gnupg",
        ".config",
        ".local/share/keyrings",
        ".password-store",
        "Documents",
        "Pictures",
        "Videos",
        ".hermes",
        ".claude",
        "proyek/data.db",
        "proyek/dump.sqlite",
        "proyek/skema.sql",
        "proyek/pgdata",
        "proyek/mysql",
        "proyek/.env",
    ];
    for relative in deny {
        assert!(
            matcher.match_rule(&home.path.join(relative)).is_none(),
            "deny-list tidak boleh cocok: {relative}"
        );
    }
}

#[test]
fn matcher_respects_rule_max_depth() {
    let home = common::TempDir::new("matcher-depth");
    let matcher = Matcher::new(&home.path);
    let inside = home.path.join("a/b/c/d/e/f/node_modules");
    assert!(matcher.match_rule(&inside).is_some());
    let too_deep = home.path.join("a/b/c/d/e/f/g/h/node_modules");
    assert!(matcher.match_rule(&too_deep).is_none());
}
