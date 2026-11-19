use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use cleanbro_core::{AppError, AppResult, CancellationToken, ErrorCode, ScanId};
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager};

use crate::services::event::EventEmitter;
use crate::services::scan_service::{execute, validate_request, ScanContext, ScanRequest};

/// Token aktif per scan; dipakai `cancel_scan`.
#[derive(Default)]
pub struct ScanState {
    tokens: Mutex<HashMap<String, CancellationToken>>,
}

impl ScanState {
    pub fn register(&self, scan_id: &ScanId) -> CancellationToken {
        let token = CancellationToken::new();
        lock(&self.tokens).insert(scan_id.as_str().to_owned(), token.clone());
        token
    }

    pub fn cancel(&self, scan_id: &str) -> bool {
        match lock(&self.tokens).get(scan_id) {
            Some(token) => {
                token.cancel();
                true
            }
            None => false,
        }
    }

    pub fn remove(&self, scan_id: &str) {
        lock(&self.tokens).remove(scan_id);
    }
}

pub struct TauriEmitter {
    app: AppHandle,
}

/// Emitter Tauri untuk service yang butuh kanal event.
pub fn emitter_for(app: &AppHandle) -> Arc<dyn EventEmitter> {
    Arc::new(TauriEmitter { app: app.clone() })
}

impl EventEmitter for TauriEmitter {
    fn emit(&self, event: &str, payload: Value) {
        let _ = self.app.emit(event, payload);
    }
}

/// Path DB state aplikasi; core hanya menerima path, tidak menebaknya.
pub fn db_path(app: &AppHandle) -> AppResult<PathBuf> {
    let dir = app.path().app_data_dir().map_err(|error| {
        AppError::new(
            ErrorCode::Internal,
            "direktori data aplikasi tidak tersedia",
        )
        .with_detail(error.to_string())
    })?;
    Ok(dir.join("state.db"))
}

/// Jalankan scan di `spawn_blocking` dan lepas token setelah selesai.
pub fn spawn_scan(
    app: AppHandle,
    state: Arc<ScanState>,
    home: PathBuf,
    db_path: PathBuf,
    request: ScanRequest,
) -> AppResult<ScanId> {
    validate_request(&request)?;
    let scan_id = ScanId::new();
    let token = state.register(&scan_id);
    let emitter = emitter_for(&app);
    let context = ScanContext { home, db_path };
    let cleanup = Arc::clone(&state);
    let cleanup_id = scan_id.clone();
    tauri::async_runtime::spawn_blocking(move || {
        if let Err(error) = execute(&context, request, emitter, &token) {
            eprintln!("scan {} gagal: {error}", cleanup_id);
        }
        cleanup.remove(cleanup_id.as_str());
    });
    Ok(scan_id)
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_cancels_known_scan() {
        let state = ScanState::default();
        let scan_id = ScanId::new();
        let token = state.register(&scan_id);
        assert!(state.cancel(scan_id.as_str()));
        assert!(token.is_cancelled());
        state.remove(scan_id.as_str());
        assert!(!state.cancel(scan_id.as_str()));
    }
}
