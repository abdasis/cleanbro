mod support;

use std::sync::{Arc, Mutex};

use cleanbro_core::rules::RiskLevel;
use cleanbro_core::{CleanMode, ErrorCode, ScanId};
use sapu_bersih::services::event::EventEmitter;
use sapu_bersih::services::exec_service::{self, ExecContext, ExecRequest};
use sapu_bersih::services::token_service::TokenStore;
use serde_json::Value;

#[derive(Default)]
struct Recorder {
    events: Mutex<Vec<(String, Value)>>,
}

impl Recorder {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn names(&self) -> Vec<String> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl EventEmitter for Recorder {
    fn emit(&self, event: &str, payload: Value) {
        self.events
            .lock()
            .unwrap()
            .push((event.to_owned(), payload));
    }
}

#[test]
fn execute_moves_file_to_trash_and_records_journal() {
    let temp = support::TempDir::new("exec-trash");
    let path = temp.path.join(".gradle/caches");
    support::write_file(&path.join("berkas.bin"), 4096);
    let scan = ScanId::new();
    let finding = support::finding(&scan, "cache.gradle", &path, 4096, RiskLevel::Safe);
    let item_id = finding.id.clone();
    support::seed_scan(&temp.path, vec![finding]);

    let context = ExecContext {
        home: temp.path.clone(),
        db_path: temp.db(),
    };
    let gateway = support::MockTrash::new(temp.path.join("trash-root"));
    let recorder = Recorder::new();
    let outcome = exec_service::execute(
        &context,
        &TokenStore::new(),
        ExecRequest {
            scan_id: scan.as_str().to_owned(),
            item_ids: vec![item_id],
            mode: CleanMode::Trash,
            confirm_token: String::new(),
        },
        recorder.as_ref(),
        &gateway,
        exec_service::now_millis(),
    )
    .expect("eksekusi trash");

    assert_eq!(outcome.deleted, 1);
    assert_eq!(outcome.failed, 0);
    assert_eq!(outcome.freed_bytes, 4096);
    assert!(!path.exists());
    assert_eq!(gateway.entry_count(), 1);
    assert!(recorder.names().contains(&"clean://finished".to_owned()));

    let connection = cleanbro_core::store::db::open(&temp.db()).expect("state db");
    let actions: i64 = connection
        .query_row("SELECT COUNT(*) FROM actions", [], |row| row.get(0))
        .unwrap();
    let journal: i64 = connection
        .query_row("SELECT COUNT(*) FROM undo_journal", [], |row| row.get(0))
        .unwrap();
    assert_eq!(actions, 1);
    assert_eq!(journal, 1);
}

#[test]
fn execute_skips_path_rejected_by_guard() {
    let temp = support::TempDir::new("exec-guard");
    let path = temp.path.join("proyek/target");
    std::fs::create_dir_all(&path).expect("membuat target");
    let scan = ScanId::new();
    let finding = support::finding(&scan, "build.rust_target", &path, 10, RiskLevel::Safe);
    let item_id = finding.id.clone();
    support::seed_scan(&temp.path, vec![finding]);

    let context = ExecContext {
        home: temp.path.clone(),
        db_path: temp.db(),
    };
    let gateway = support::MockTrash::new(temp.path.join("trash-root"));
    let outcome = exec_service::execute(
        &context,
        &TokenStore::new(),
        ExecRequest {
            scan_id: scan.as_str().to_owned(),
            item_ids: vec![item_id],
            mode: CleanMode::Trash,
            confirm_token: String::new(),
        },
        Recorder::new().as_ref(),
        &gateway,
        exec_service::now_millis(),
    )
    .expect("eksekusi tetap selesai");

    assert_eq!(outcome.deleted, 0);
    assert_eq!(outcome.skipped, 1);
    assert!(path.exists(), "path yang ditolak guard tidak boleh dihapus");
    assert_eq!(gateway.entry_count(), 0);

    let connection = cleanbro_core::store::db::open(&temp.db()).expect("state db");
    let decision: String = connection
        .query_row(
            "SELECT decision FROM audit WHERE decision = 'skip' LIMIT 1",
            [],
            |row| row.get(0),
        )
        .expect("audit skip");
    assert_eq!(decision, "skip");
}

#[test]
fn confirm_token_required_above_threshold() {
    let temp = support::TempDir::new("exec-token");
    let path = temp.path.join(".npm/_cacache");
    let scan = ScanId::new();
    let finding = support::finding(&scan, "cache.npm", &path, 6_000_000_000, RiskLevel::Safe);
    let item_id = finding.id.clone();
    support::seed_scan(&temp.path, vec![finding]);

    let context = ExecContext {
        home: temp.path.clone(),
        db_path: temp.db(),
    };
    let gateway = support::MockTrash::new(temp.path.join("trash-root"));
    let tokens = TokenStore::new();
    let request = |token: String| ExecRequest {
        scan_id: scan.as_str().to_owned(),
        item_ids: vec![item_id.clone()],
        mode: CleanMode::Trash,
        confirm_token: token,
    };

    let error = exec_service::execute(
        &context,
        &tokens,
        request(String::new()),
        Recorder::new().as_ref(),
        &gateway,
        exec_service::now_millis(),
    )
    .unwrap_err();
    assert_eq!(error.code, ErrorCode::GuardRejected);

    let grant = exec_service::request_token(
        &context,
        &tokens,
        scan.as_str(),
        std::slice::from_ref(&item_id),
        exec_service::now_millis(),
    )
    .expect("token");
    exec_service::execute(
        &context,
        &tokens,
        request(grant.token),
        Recorder::new().as_ref(),
        &gateway,
        exec_service::now_millis(),
    )
    .expect("token valid diterima");
}
