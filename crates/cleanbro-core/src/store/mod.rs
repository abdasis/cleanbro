use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::error::{AppError, AppResult, ErrorCode};

pub mod action_repo;
pub mod db;
pub mod finding_repo;
pub mod migrations;
pub mod scan_repo;

pub use db::{open, open_in_memory};

fn io_error(operation: &str, error: impl std::fmt::Display) -> AppError {
    AppError::new(ErrorCode::Internal, format!("{operation} gagal")).with_detail(error.to_string())
}

/// Pastikan direktori induk ada; path DB selalu ditentukan pemanggil.
pub fn ensure_parent(path: &Path) -> AppResult<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(parent).map_err(|error| io_error("membuat direktori state", error))
}

pub fn default_db_file(dir: PathBuf) -> PathBuf {
    dir.join("state.db")
}

pub fn connect(path: &Path) -> AppResult<Connection> {
    ensure_parent(path)?;
    let mut connection =
        Connection::open(path).map_err(|error| io_error("membuka state", error))?;
    migrations::migrate(&mut connection).map_err(|error| io_error("menjalankan migrasi", error))?;
    Ok(connection)
}
