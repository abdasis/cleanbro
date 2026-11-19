use std::path::Path;

use rusqlite::Connection;

use crate::error::{AppError, AppResult, ErrorCode};

use super::{connect, migrations};

/// Buka state DB di path yang diberikan pemanggil, lalu jalankan migrasi.
pub fn open(path: &Path) -> AppResult<Connection> {
    connect(path)
}

pub fn open_in_memory() -> AppResult<Connection> {
    let mut connection =
        Connection::open_in_memory().map_err(|error| map("membuka state memori", error))?;
    migrations::migrate(&mut connection).map_err(|error| map("migrasi memori", error))?;
    Ok(connection)
}

fn map(operation: &str, error: rusqlite::Error) -> AppError {
    AppError::new(ErrorCode::Internal, format!("{operation} gagal")).with_detail(error.to_string())
}
