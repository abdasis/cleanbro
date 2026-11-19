use std::path::Path;

pub mod docker;
pub mod trash_gateway;

pub use docker::{DockerCli, SystemDockerCli};
pub use trash_gateway::SystemTrash;

/// Abstraksi pindah/pulihkan trash agar service bisa dites tanpa trash OS.
pub trait TrashGateway: Send + Sync {
    /// Pindahkan path ke trash; hasilnya referensi stabil untuk undo.
    fn trash(&self, path: &Path) -> Result<String, String>;
    /// Pulihkan referensi trash ke path asal.
    fn restore(&self, trash_ref: &str, target: &Path) -> Result<(), String>;
    /// Apakah item masih tersedia di trash.
    fn exists(&self, trash_ref: &str) -> bool;
}
