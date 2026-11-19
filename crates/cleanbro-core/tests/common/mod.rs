#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// Direktori sementara yang dibersihkan otomatis.
pub struct TempDir {
    pub path: PathBuf,
}

impl TempDir {
    pub fn new(tag: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("cleanbro-test-{tag}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&path).expect("membuat temp dir");
        Self { path }
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

pub fn mkdir(path: &Path) {
    std::fs::create_dir_all(path).expect("membuat direktori");
}
