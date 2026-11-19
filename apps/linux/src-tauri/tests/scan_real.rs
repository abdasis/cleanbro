//! Verifikasi scan nyata pada home palsu dengan fixture berukuran terkontrol.
//! Jalankan eksplisit:
//! `cargo test -p sapu-bersih --test scan_real -- --ignored --nocapture`

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use cleanbro_core::store::{db, finding_repo};
use cleanbro_core::{CancellationToken, ScanMode};
use sapu_bersih::services::event::EventEmitter;
use sapu_bersih::services::scan_service::{execute, ScanContext, ScanRequest};
use serde_json::Value;

const MB: usize = 1024 * 1024;

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

struct TempHome {
    path: PathBuf,
}

impl TempHome {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "cleanbro-real-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for TempHome {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn write_file(path: &Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, vec![b'x'; bytes]).unwrap();
}

/// Jumlah byte persis seperti walker: `st_blocks * 512`, fallback `len()`.
fn entry_size(meta: &std::fs::Metadata) -> u64 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let blocks = meta.blocks().saturating_mul(512);
        if blocks > 0 {
            return blocks;
        }
    }
    meta.len()
}

fn expected_bytes(dir: &Path) -> u64 {
    let mut total = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else {
                continue;
            };
            if meta.is_dir() {
                stack.push(entry.path());
            } else if meta.is_file() {
                total += entry_size(&meta);
            }
        }
    }
    total
}

#[test]
#[ignore = "fixture 1 MB per berkas; jalankan eksplisit dengan --ignored"]
fn real_scan_reports_expected_findings_and_bytes() {
    let home = TempHome::new("home");
    let root = home.path.clone();

    // Cache developer.
    write_file(&root.join(".npm/_cacache/content/a.bin"), MB);
    write_file(&root.join(".npm/_cacache/content/b.bin"), MB);
    write_file(&root.join(".gradle/caches/berkas.bin"), MB);
    write_file(&root.join(".cache/pip/http/body.bin"), MB);

    // Build: node_modules tanpa penanda proyek, target dengan Cargo.toml.
    write_file(&root.join("proj/node_modules/pkg/index.js"), MB);
    write_file(&root.join("proj/Cargo.toml"), 64);
    write_file(&root.join("proj/target/debug/app"), MB);

    // Folder di luar katalog: tidak boleh menjadi finding.
    write_file(&root.join("outside/notes.txt"), MB);

    let db_path = root.join("state/state.db");
    let recorder = Recorder::new();
    let context = ScanContext {
        home: root.clone(),
        db_path: db_path.clone(),
    };
    // Mode Deep karena rule Build tidak termasuk katalog mode Quick.
    let request = ScanRequest {
        mode: ScanMode::Deep,
        categories: None,
    };
    let started = std::time::Instant::now();
    let outcome = execute(
        &context,
        request,
        recorder.clone(),
        &CancellationToken::new(),
    )
    .expect("scan selesai");
    let elapsed = started.elapsed();

    let mut per_rule: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for finding in &outcome.findings {
        let entry = per_rule.entry(finding.rule_id.clone()).or_insert((0, 0));
        entry.0 += finding.bytes;
        entry.1 += finding.item_count;
    }

    let expected = [
        ("cache.npm", root.join(".npm").join("_cacache")),
        ("cache.gradle", root.join(".gradle").join("caches")),
        ("cache.pip", root.join(".cache").join("pip")),
        ("build.node_modules", root.join("proj").join("node_modules")),
        ("build.rust_target", root.join("proj").join("target")),
    ];
    for (rule_id, dir) in expected {
        let (bytes, items) = per_rule
            .get(rule_id)
            .unwrap_or_else(|| panic!("finding {rule_id} tidak ada: {per_rule:?}"));
        assert!(*bytes > 0, "{rule_id} bytes nol");
        assert!(*items > 0, "{rule_id} item_count nol");
        assert_eq!(*bytes, expected_bytes(&dir), "{rule_id} bytes beda");
    }

    assert!(
        !per_rule.contains_key("build.dist"),
        "folder non-katalog ikut terdeteksi: {per_rule:?}"
    );
    assert!(
        outcome
            .findings
            .iter()
            .all(|finding| !finding.path.contains("/outside/")),
        "folder di luar katalog menjadi finding"
    );

    let names = recorder.names();
    assert_eq!(
        names
            .iter()
            .filter(|name| *name == "scan://finished")
            .count(),
        1,
        "{names:?}"
    );
    assert!(!outcome.partial, "scan fixture tidak boleh partial");

    let connection = db::open(&db_path).expect("state db");
    let stored = finding_repo::findings_for_scan(&connection, outcome.scan_id.as_str())
        .expect("membaca finding");
    assert_eq!(stored.len(), outcome.findings.len());
    assert!(!stored.is_empty(), "DB tanpa finding");

    println!(
        "scan_real: rules={per_rule:?} findings={} duration_ms={} elapsed_ms={}",
        stored.len(),
        outcome.duration_ms,
        elapsed.as_millis()
    );
}
