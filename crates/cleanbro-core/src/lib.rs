//! Inti bersama Sapu Bersih (Linux) dan CleanBro (macOS).

pub mod engine;
pub mod error;
pub mod models;
pub mod rules;
pub mod store;

pub use engine::{
    discover, project_name, walk, CancellationToken, Candidate, WalkOptions, WalkResult,
};
pub use error::{AppError, AppResult, ErrorCode};
pub use models::{
    ActionStatus, ActionSummary, AppInfo, CategoryDone, CleanFinished, CleanItemState, CleanMode,
    CleanPlan, CleanProgress, DiskSummary, DockerContainer, DockerDiskUsage, DockerImage,
    DockerPruneResult, DockerStatus, Finding, HistoryPage, PlanItem, Platform, RuleInfo,
    ScanFinished, ScanId, ScanMode, ScanProgress, TokenGrant, UndoEntry, UndoOutcome, WarnLevel,
    CONFIRM_THRESHOLD_BYTES,
};
pub use rules::{Category, DeleteMode, Guard, Matcher, RiskLevel, Rule, RULES, RULES_VERSION};
