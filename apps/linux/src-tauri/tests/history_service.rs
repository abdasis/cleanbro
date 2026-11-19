mod support;

use cleanbro_core::rules::RiskLevel;
use cleanbro_core::store::action_repo::{self, ActionItemRow, ActionRow, UndoJournalRow};
use cleanbro_core::store::db;
use cleanbro_core::{CleanMode, ScanId};
use sapu_bersih::services::exec_service::{self, ExecContext, ExecRequest};
use sapu_bersih::services::history_service;
use sapu_bersih::services::token_service::TokenStore;

fn action(id: &str, started_at: i64) -> ActionRow {
    ActionRow {
        id: id.to_owned(),
        scan_id: "scan".to_owned(),
        mode: CleanMode::Trash.as_str().to_owned(),
        started_at,
        finished_at: Some(started_at + 1),
        freed_bytes: 10,
        status: "completed".to_owned(),
        confirm_token: None,
    }
}

fn item(action_id: &str) -> ActionItemRow {
    ActionItemRow {
        id: format!("{action_id}-item"),
        action_id: action_id.to_owned(),
        path: "/home/uji/.gradle/caches".to_owned(),
        bytes: 10,
        state: "done".to_owned(),
        reason: None,
        trash_ref: Some("mock-1".to_owned()),
    }
}

#[test]
fn history_lists_newest_first() {
    let temp = support::TempDir::new("history-order");
    let mut connection = db::open(&temp.db()).expect("state db");
    for (id, started) in [("lama", 100), ("baru", 200)] {
        action_repo::save_action(
            &mut connection,
            &action(id, started),
            &[item(id)],
            &[],
            None,
        )
        .expect("simpan aksi");
    }
    let context = ExecContext {
        home: temp.path.clone(),
        db_path: temp.db(),
    };
    let page = history_service::list_history(
        &context,
        10,
        0,
        &support::MockTrash::new(temp.path.join("trash")),
        1_000,
    )
    .expect("riwayat");

    assert_eq!(page.total, 2);
    assert_eq!(page.items[0].id, "baru");
    assert_eq!(page.items[1].id, "lama");
}

#[test]
fn history_marks_expired_journal_not_undoable() {
    let temp = support::TempDir::new("history-expired");
    let mut connection = db::open(&temp.db()).expect("state db");
    let journal = UndoJournalRow {
        action_id: "aksi".to_owned(),
        payload: "[]".to_owned(),
        created_at: 1,
        expires_at: 2,
    };
    action_repo::save_action(
        &mut connection,
        &action("aksi", 1),
        &[item("aksi")],
        &[],
        Some(&journal),
    )
    .expect("simpan aksi");
    drop(connection);

    let context = ExecContext {
        home: temp.path.clone(),
        db_path: temp.db(),
    };
    let page = history_service::list_history(
        &context,
        10,
        0,
        &support::MockTrash::new(temp.path.join("trash")),
        1_000,
    )
    .expect("riwayat");
    assert!(!page.items[0].undoable);
}

#[test]
fn undo_restores_file_to_original_path() {
    let temp = support::TempDir::new("undo-restore");
    let path = temp.path.join(".gradle/caches");
    support::write_file(&path.join("berkas.bin"), 128);
    let scan = ScanId::new();
    let finding = support::finding(&scan, "cache.gradle", &path, 128, RiskLevel::Safe);
    let item_id = finding.id.clone();
    support::seed_scan(&temp.path, vec![finding]);

    let context = ExecContext {
        home: temp.path.clone(),
        db_path: temp.db(),
    };
    let gateway = support::MockTrash::new(temp.path.join("trash"));
    let outcome = exec_service::execute(
        &context,
        &TokenStore::new(),
        ExecRequest {
            scan_id: scan.as_str().to_owned(),
            item_ids: vec![item_id],
            mode: CleanMode::Trash,
            confirm_token: String::new(),
        },
        &NoopEmitter,
        &gateway,
        exec_service::now_millis(),
    )
    .expect("eksekusi trash");
    assert!(!path.exists());

    let restored = history_service::undo_action(
        &context,
        &outcome.action_id,
        &gateway,
        exec_service::now_millis(),
    )
    .expect("undo");
    assert_eq!(restored.restored, 1);
    assert_eq!(restored.failed, 0);
    assert!(path.join("berkas.bin").is_file());

    let page = history_service::list_history(&context, 10, 0, &gateway, exec_service::now_millis())
        .expect("riwayat");
    assert!(!page.items[0].undoable);
}

struct NoopEmitter;

impl sapu_bersih::services::event::EventEmitter for NoopEmitter {
    fn emit(&self, _event: &str, _payload: serde_json::Value) {}
}
