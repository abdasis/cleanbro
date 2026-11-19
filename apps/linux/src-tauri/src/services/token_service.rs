use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use cleanbro_core::{AppError, AppResult, ErrorCode, TokenGrant};
use sha2::{Digest, Sha256};

/// Masa berlaku token konfirmasi.
pub const TOKEN_TTL_MS: u64 = 5 * 60 * 1000;

#[derive(Debug, Clone)]
struct Entry {
    scan_id: String,
    item_ids: Vec<String>,
    expires_at: u64,
}

/// Token server-side: hash dari (scan_id, item_ids terurut, action_hash) + secret.
#[derive(Debug)]
pub struct TokenStore {
    secret: String,
    entries: Mutex<HashMap<String, Entry>>,
}

impl Default for TokenStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenStore {
    pub fn new() -> Self {
        // uuid v4 dipakai sebagai sumber acak proses; tidak butuh CSPRNG khusus.
        let secret = format!("{}{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4());
        Self {
            secret,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Terbitkan token baru; entri kedaluwarsa lama dibersihkan.
    pub fn issue(&self, scan_id: &str, item_ids: &[String], now: u64) -> TokenGrant {
        let expires_at = now.saturating_add(TOKEN_TTL_MS);
        let token = self.digest(scan_id, item_ids);
        let mut entries = self.lock();
        entries.retain(|_, entry| entry.expires_at > now);
        entries.insert(
            token.clone(),
            Entry {
                scan_id: scan_id.to_owned(),
                item_ids: sorted(item_ids),
                expires_at,
            },
        );
        TokenGrant { token, expires_at }
    }

    /// Pakai token sekali; tolak kosong, salah, kedaluwarsa, atau beda aksi.
    pub fn consume(
        &self,
        scan_id: &str,
        item_ids: &[String],
        token: &str,
        now: u64,
    ) -> AppResult<()> {
        let entry = self.lock().remove(token);
        let Some(entry) = entry else {
            return Err(rejected("token konfirmasi tidak valid"));
        };
        if now >= entry.expires_at {
            return Err(rejected("token konfirmasi kedaluwarsa"));
        }
        if entry.scan_id != scan_id || entry.item_ids != sorted(item_ids) {
            return Err(rejected("token konfirmasi tidak cocok dengan aksi"));
        }
        Ok(())
    }

    fn digest(&self, scan_id: &str, item_ids: &[String]) -> String {
        let items = sorted(item_ids).join(",");
        let action_hash = hex::encode(Sha256::digest(items.as_bytes()));
        let mut hasher = Sha256::new();
        hasher.update(self.secret.as_bytes());
        hasher.update(b"|");
        hasher.update(scan_id.as_bytes());
        hasher.update(b"|");
        hasher.update(items.as_bytes());
        hasher.update(b"|");
        hasher.update(action_hash.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<String, Entry>> {
        self.entries
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

fn sorted(item_ids: &[String]) -> Vec<String> {
    let mut ids = item_ids.to_vec();
    ids.sort();
    ids.dedup();
    ids
}

fn rejected(message: &str) -> AppError {
    AppError::new(ErrorCode::GuardRejected, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> Vec<String> {
        vec!["b".to_owned(), "a".to_owned()]
    }

    #[test]
    fn token_is_valid_once_then_rejected() {
        let store = TokenStore::new();
        let grant = store.issue("scan", &ids(), 1_000);
        store
            .consume("scan", &ids(), &grant.token, 2_000)
            .expect("token sah sekali");
        assert_eq!(grant.expires_at, 1_000 + TOKEN_TTL_MS);
        let error = store
            .consume("scan", &ids(), &grant.token, 2_001)
            .unwrap_err();
        assert_eq!(error.code, ErrorCode::GuardRejected);
    }

    #[test]
    fn token_expires_after_five_minutes() {
        let store = TokenStore::new();
        let grant = store.issue("scan", &ids(), 0);
        let error = store
            .consume("scan", &ids(), &grant.token, TOKEN_TTL_MS)
            .unwrap_err();
        assert_eq!(error.code, ErrorCode::GuardRejected);
        assert!(error.message.contains("kedaluwarsa"));
    }

    #[test]
    fn token_rejects_wrong_hash() {
        let store = TokenStore::new();
        let grant = store.issue("scan", &ids(), 0);
        let error = store.consume("scan", &ids(), "deadbeef", 1).unwrap_err();
        assert_eq!(error.code, ErrorCode::GuardRejected);
        assert!(store.consume("scan", &ids(), &grant.token, 1).is_ok());
    }

    #[test]
    fn token_is_bound_to_item_set() {
        let store = TokenStore::new();
        let grant = store.issue("scan", &ids(), 0);
        let other = vec!["a".to_owned()];
        let error = store.consume("scan", &other, &grant.token, 1).unwrap_err();
        assert_eq!(error.code, ErrorCode::GuardRejected);
    }
}
