use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use cleanbro_core::store::db;
use cleanbro_core::{CancellationToken, ScanMode};
use sapu_bersih::services::event::EventEmitter;
use sapu_bersih::services::scan_service::{execute, ScanContext, ScanRequest};
use serde_json::Value;

struct Recorder {
    events: Mutex<Vec<(String, Value)>>,
}

impl Recorder {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            events: Mutex::new(Vec::new()),
        })
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

fn temp_dir(tag: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "cleanbro-scan-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn write_file(path: &std::path::Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, vec![b'x'; bytes]).unwrap();
}

#[test]
fn scan_service_emits_finished_event() {
    let home = temp_dir("home");
    write_file(&home.join(".gradle/caches/berkas.bin"), 4096);
    let db_path = home.join("state/state.db");

    let recorder = Recorder::new();
    let context = ScanContext {
        home: home.clone(),
        db_path: db_path.clone(),
    };
    let request = ScanRequest {
        mode: ScanMode::Quick,
        categories: None,
    };
    let outcome = execute(
        &context,
        request,
        recorder.clone(),
        &CancellationToken::new(),
    )
    .expect("scan selesai");

    let names = recorder.names();
    assert!(names.contains(&"scan://finished".to_owned()), "{names:?}");
    assert!(names.contains(&"scan://category".to_owned()), "{names:?}");
    assert!(!outcome.findings.is_empty());
    assert_eq!(outcome.findings[0].rule_id, "cache.gradle");

    let connection = db::open(&db_path).expect("state db");
    let scans: i64 = connection
        .query_row("SELECT COUNT(*) FROM scans", [], |row| row.get(0))
        .unwrap();
    let findings: i64 = connection
        .query_row("SELECT COUNT(*) FROM findings", [], |row| row.get(0))
        .unwrap();
    assert_eq!(scans, 1);
    assert_eq!(findings as usize, outcome.findings.len());

    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn scan_service_rejects_unknown_rule_filter() {
    let home = temp_dir("unknown");
    let context = ScanContext {
        home: home.clone(),
        db_path: home.join("state.db"),
    };
    let request = ScanRequest {
        mode: ScanMode::Quick,
        categories: Some(vec!["cache.tidak-ada".to_owned()]),
    };
    let error = execute(
        &context,
        request,
        Recorder::new(),
        &CancellationToken::new(),
    )
    .unwrap_err();
    assert_eq!(error.code, cleanbro_core::ErrorCode::NotFound);
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn scan_service_marks_partial_when_cancelled() {
    let home = temp_dir("cancel");
    write_file(&home.join(".gradle/caches/berkas.bin"), 64);
    let context = ScanContext {
        home: home.clone(),
        db_path: home.join("state.db"),
    };
    let token = CancellationToken::new();
    token.cancel();
    let outcome = execute(
        &context,
        ScanRequest {
            mode: ScanMode::Quick,
            categories: None,
        },
        Recorder::new(),
        &token,
    )
    .expect("scan tetap selesai");
    assert!(outcome.partial);
    let _ = std::fs::remove_dir_all(&home);
}
