//! Inti bersama Sapu Bersih (Linux) dan CleanBro (macOS).

pub mod error;
pub mod models;
pub mod rules;

pub use error::{AppError, AppResult, ErrorCode};
pub use models::{AppInfo, Platform};
pub use rules::{DeleteMode, RiskLevel, RULES_VERSION};
