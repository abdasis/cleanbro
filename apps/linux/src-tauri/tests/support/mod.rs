#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use cleanbro_core::models::{Finding, ScanId};
use cleanbro_core::rules::RiskLevel;
use cleanbro_core::store::db;
use cleanbro_core::store::scan_repo::{save_scan, ScanRow};
use sapu_bersih::adapters::TrashGateway;

pub struct TempDir {
    pub path: PathBuf,
}

impl TempDir {
    pub fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "cleanbro-m3-{tag}-{}-{}",
            std::process::id(),
            cleanbro_core::models::ScanId::new().as_str()
        ));
        std::fs::create_dir_all(&path).expect("membuat temp dir");
        Self { path }
    }

    pub fn db(&self) -> PathBuf {
        self.path.join("state/state.db")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

pub fn write_file(path: &Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("membuat parent");
    }
    std::fs::write(path, vec![b'x'; bytes]).expect("menulis berkas");
}

pub fn finding(
    scan_id: &ScanId,
    rule_id: &str,
    path: &Path,
    bytes: u64,
    risk: RiskLevel,
) -> Finding {
    Finding::new(
        scan_id.clone(),
        rule_id,
        path.to_string_lossy(),
        bytes,
        1,
        risk,
        None,
    )
}

/// Simpan scan + findings ke SQLite, kembalikan scan_id.
pub fn seed_scan(home: &Path, findings: Vec<Finding>) -> ScanId {
    let existing = findings
        .first()
        .map(|finding| finding.scan_id.clone())
        .unwrap_or_default();
    let scan_id = existing.clone();
    let row = ScanRow {
        id: scan_id.as_str().to_owned(),
        mode: "quick".to_owned(),
        started_at: 1,
        finished_at: 2,
        partial: false,
        bytes_found: findings.iter().map(|finding| finding.bytes).sum(),
    };
    let mut connection = db::open(&home.join("state/state.db")).expect("state db");
    save_scan(&mut connection, &row, &findings).expect("simpan scan");
    scan_id
}

/// Trash palsu: berkas dipindah ke folder uji, bisa dipulihkan.
#[derive(Default)]
pub struct MockTrash {
    pub root: PathBuf,
    entries: Mutex<HashMap<String, PathBuf>>,
    next: AtomicU64,
}

impl MockTrash {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        std::fs::create_dir_all(&root).expect("membuat trash uji");
        Self {
            root,
            entries: Mutex::new(HashMap::new()),
            next: AtomicU64::new(1),
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.lock().unwrap().len()
    }
}

impl TrashGateway for MockTrash {
    fn trash(&self, path: &Path) -> Result<String, String> {
        let id = format!("mock-{}", self.next.fetch_add(1, Ordering::Relaxed));
        let destination = self.root.join(&id);
        std::fs::rename(path, &destination).map_err(|error| error.to_string())?;
        self.entries
            .lock()
            .unwrap()
            .insert(id.clone(), path.to_path_buf());
        Ok(id)
    }

    fn restore(&self, trash_ref: &str, target: &Path) -> Result<(), String> {
        let mut entries = self.entries.lock().unwrap();
        let original = entries
            .remove(trash_ref)
            .ok_or_else(|| "tidak ada di trash".to_owned())?;
        if original != target {
            return Err("path asal tidak cocok".to_owned());
        }
        std::fs::rename(self.root.join(trash_ref), target).map_err(|error| error.to_string())
    }

    fn exists(&self, trash_ref: &str) -> bool {
        self.entries.lock().unwrap().contains_key(trash_ref)
    }
}
