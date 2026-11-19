use serde_json::Value;

/// Abstraksi emit agar service scan bisa dites tanpa runtime Tauri.
pub trait EventEmitter: Send + Sync {
    fn emit(&self, event: &str, payload: Value);
}

#[cfg(test)]
pub mod test_support {
    use std::sync::Mutex;

    use serde_json::Value;

    use super::EventEmitter;

    #[derive(Default)]
    pub struct MockEmitter {
        events: Mutex<Vec<(String, Value)>>,
    }

    impl MockEmitter {
        pub fn events(&self) -> Vec<(String, Value)> {
            self.events
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .clone()
        }

        pub fn names(&self) -> Vec<String> {
            self.events().into_iter().map(|(name, _)| name).collect()
        }
    }

    impl EventEmitter for MockEmitter {
        fn emit(&self, event: &str, payload: Value) {
            self.events
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push((event.to_owned(), payload));
        }
    }
}
