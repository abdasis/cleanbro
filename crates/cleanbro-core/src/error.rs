use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Kode error stabil yang dikonsumsi UI lewat IPC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorCode {
    NotFound,
    PermissionDenied,
    InvalidPath,
    GuardRejected,
    Busy,
    Cancelled,
    Io,
    Internal,
}

/// Error aplikasi dengan bentuk JSON tetap: `{ code, message, detail }`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub detail: Option<String>,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

// Serialize manual agar field selalu lengkap (detail null bila kosong),
// bukan bergantung pada `skip_serializing_if` saat derive.
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AppError", 3)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("detail", &self.detail)?;
        state.end()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_stable_json_shape() {
        let err = AppError::new(ErrorCode::GuardRejected, "path dilindungi");
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "code": "GuardRejected",
                "message": "path dilindungi",
                "detail": null,
            })
        );
    }

    #[test]
    fn detail_is_optional() {
        let err = AppError::new(ErrorCode::Io, "gagal").with_detail("os error 13");
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["detail"], "os error 13");
    }

    #[test]
    fn display_uses_message() {
        let err = AppError::new(ErrorCode::NotFound, "tidak ada");
        assert_eq!(err.to_string(), "tidak ada");
    }
}
