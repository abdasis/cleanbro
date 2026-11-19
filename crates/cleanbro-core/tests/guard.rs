mod common;

use cleanbro_core::rules::catalog::by_id;
use cleanbro_core::{ErrorCode, Guard};

#[test]
fn guard_blocks_ssh_dir() {
    let home = common::TempDir::new("guard-ssh");
    common::mkdir(&home.path.join(".ssh"));
    let guard = Guard::new(&home.path);
    let rule = by_id("cache.gradle").expect("rule ada");
    let error = guard.check(&home.path.join(".ssh"), rule).unwrap_err();
    assert_eq!(error.code, ErrorCode::GuardRejected);
}

#[test]
fn guard_blocks_dirty_git_worktree() {
    let home = common::TempDir::new("guard-git");
    let project = home.path.join("proyek");
    common::mkdir(&project.join("dist"));
    common::write_file(&project.join("package.json"), 2);
    let status = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(&project)
        .status()
        .expect("git tersedia");
    assert!(status.success());
    common::write_file(&project.join("belum-dicommit.txt"), 4);

    let guard = Guard::new(&home.path);
    let rule = by_id("build.dist").expect("rule ada");
    let error = guard.check(&project.join("dist"), rule).unwrap_err();
    assert_eq!(error.code, ErrorCode::GuardRejected);
    assert!(error.message.contains("kotor"));
}

#[test]
fn guard_blocks_path_outside_home() {
    let home = common::TempDir::new("guard-outside");
    let guard = Guard::new(&home.path);
    let rule = by_id("cache.gradle").expect("rule ada");
    let error = guard.check(std::path::Path::new("/tmp"), rule).unwrap_err();
    assert_eq!(error.code, ErrorCode::GuardRejected);
}

#[test]
fn guard_allows_gradle_cache() {
    let home = common::TempDir::new("guard-gradle");
    let cache = home.path.join(".gradle/caches");
    common::mkdir(&cache);
    let guard = Guard::new(&home.path);
    let rule = by_id("cache.gradle").expect("rule ada");
    guard.check(&cache, rule).expect("cache gradle diizinkan");
}

#[test]
fn guard_allows_clean_git_worktree() {
    let home = common::TempDir::new("guard-git-clean");
    let project = home.path.join("proyek");
    common::mkdir(&project.join("dist"));
    common::write_file(&project.join("package.json"), 2);
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(&project)
        .status()
        .expect("git tersedia");
    std::process::Command::new("git")
        .args(["add", "-A"])
        .current_dir(&project)
        .status()
        .expect("git add");
    std::process::Command::new("git")
        .args([
            "-c",
            "user.name=Uji",
            "-c",
            "user.email=uji@example.test",
            "commit",
            "-q",
            "-m",
            "awal",
        ])
        .current_dir(&project)
        .status()
        .expect("git commit");

    let guard = Guard::new(&home.path);
    let rule = by_id("build.dist").expect("rule ada");
    guard
        .check(&project.join("dist"), rule)
        .expect("worktree bersih boleh");
}

#[test]
fn guard_blocks_database_file_component() {
    let home = common::TempDir::new("guard-db");
    let nested = home.path.join("proyek/pgdata");
    common::mkdir(&nested);
    let guard = Guard::new(&home.path);
    let rule = by_id("build.node_modules").expect("rule ada");
    let error = guard.check(&nested, rule).unwrap_err();
    assert_eq!(error.code, ErrorCode::GuardRejected);
}
