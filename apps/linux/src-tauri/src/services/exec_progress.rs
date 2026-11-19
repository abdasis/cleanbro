use std::sync::Mutex;
use std::time::{Duration, Instant};

use cleanbro_core::models::{CleanItemState, CleanProgress};
use serde_json::json;

use crate::services::event::EventEmitter;

const PROGRESS_INTERVAL: Duration = Duration::from_millis(250);

/// Emit progress paling banyak 4 kali per detik.
pub struct Progress {
    last: Mutex<Option<Instant>>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            last: Mutex::new(None),
        }
    }

    pub fn emit(
        &self,
        emitter: &dyn EventEmitter,
        action_id: &str,
        item_id: &str,
        state: CleanItemState,
        freed_bytes: u64,
    ) {
        let mut last = self
            .last
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let now = Instant::now();
        let ready = last
            .map(|seen| now.duration_since(seen) >= PROGRESS_INTERVAL)
            .unwrap_or(true);
        if !ready {
            return;
        }
        *last = Some(now);
        emitter.emit(
            "clean://progress",
            json!(CleanProgress {
                action_id: action_id.to_owned(),
                item_id: item_id.to_owned(),
                state,
                freed_bytes,
            }),
        );
    }
}
