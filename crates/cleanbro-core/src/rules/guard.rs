use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{AppError, AppResult, ErrorCode};
use crate::rules::matcher::normalize;
use crate::rules::Rule;

/// Direktori relatif `$HOME` yang tidak pernah boleh disentuh rule mana pun.
const DENY_RELATIVE: &[&str] = &[
    ".ssh",
    ".gnupg",
    ".config",
    ".local/share/keyrings",
    ".password-store",
    "Documents",
    "Pictures",
    "Videos",
    ".hermes",
    ".claude",
    ".cargo/bin",
];

/// Komponen path yang menandakan data penting di mana pun berada.
const DENY_COMPONENTS: &[&str] = &[".git", "pgdata", "mysql"];

const SYSTEM_PATHS: &[&str] = &["/boot", "/etc", "/usr", "/var/lib"];

pub struct Guard {
    home: PathBuf,
    home_uid: Option<u32>,
}

impl Guard {
    pub fn new(home: impl Into<PathBuf>) -> Self {
        let home = normalize(&home.into());
        let home_uid = owner_uid(&home);
        Self { home, home_uid }
    }

    /// Dua lapis: allowlist sudah dilakukan matcher, di sini deny-list +
    /// prasyarat rule. Path di luar `$HOME` selalu ditolak.
    pub fn check(&self, path: &Path, rule: &Rule) -> AppResult<()> {
        let path = normalize(path);
        if !path.starts_with(&self.home) {
            return Err(reject("path di luar $HOME"));
        }
        if let Some(reason) = deny_reason(&self.home, &path) {
            return Err(reject(reason));
        }
        self.check_ownership(&path)?;
        if !rule.requires.project_markers.is_empty()
            && !markers_present(&path, rule.requires.project_markers)
        {
            return Err(reject(format!(
                "penanda proyek tidak ditemukan untuk {}",
                rule.id
            )));
        }
        if rule.requires.reject_dirty_git {
            let parent = path.parent().ok_or_else(|| reject("path tanpa induk"))?;
            ensure_git_clean(parent)?;
        }
        Ok(())
    }

    fn check_ownership(&self, path: &Path) -> AppResult<()> {
        let Some(expected) = self.home_uid else {
            return Ok(());
        };
        match owner_uid(path) {
            Some(uid) if uid == expected => Ok(()),
            Some(_) => Err(reject("path bukan milik pengguna")),
            None => Err(AppError::new(ErrorCode::Io, "gagal membaca metadata path")),
        }
    }
}

fn reject(reason: impl Into<String>) -> AppError {
    AppError::new(ErrorCode::GuardRejected, reason)
}

fn deny_reason(home: &Path, path: &Path) -> Option<String> {
    for relative in DENY_RELATIVE {
        let denied = home.join(relative);
        if path == denied || path.starts_with(&denied) {
            return Some(format!("path dilindungi: ~/{relative}"));
        }
    }
    if path == Path::new("/") || path == Path::new("/home") {
        return Some("path sistem dilindungi".to_owned());
    }
    for system in SYSTEM_PATHS {
        if path.starts_with(system) {
            return Some(format!("path sistem dilindungi: {system}"));
        }
    }
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if DENY_COMPONENTS.contains(&name) {
                return Some(format!("komponen dilindungi: {name}"));
            }
        }
    }
    let name = path.file_name()?.to_str()?;
    if name == ".env"
        || name.ends_with(".db")
        || name.ends_with(".sqlite")
        || name.ends_with(".sql")
    {
        return Some(format!("berkas data dilindungi: {name}"));
    }
    None
}

fn markers_present(path: &Path, markers: &[&str]) -> bool {
    let Some(parent) = path.parent() else {
        return false;
    };
    markers.iter().any(|marker| marker_present(parent, marker))
}

fn marker_present(dir: &Path, marker: &str) -> bool {
    match marker.strip_suffix('*') {
        Some(prefix) => std::fs::read_dir(dir)
            .map(|entries| {
                entries.flatten().any(|entry| {
                    entry
                        .file_name()
                        .to_str()
                        .is_some_and(|name| name.starts_with(prefix))
                })
            })
            .unwrap_or(false),
        None => dir.join(marker).exists(),
    }
}

fn ensure_git_clean(dir: &Path) -> AppResult<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["status", "--porcelain"])
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output();
    match output {
        Ok(output) if output.status.success() => {
            if output.stdout.iter().any(|byte| !byte.is_ascii_whitespace()) {
                Err(reject("worktree git kotor; commit atau stash dulu"))
            } else {
                Ok(())
            }
        }
        // Bukan repo git: tidak ada perubahan yang bisa hilang.
        Ok(_) => Ok(()),
        Err(_) => Err(reject("git tidak tersedia untuk memeriksa worktree")),
    }
}

#[cfg(unix)]
fn owner_uid(path: &Path) -> Option<u32> {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(path).ok().map(|meta| meta.uid())
}

#[cfg(not(unix))]
fn owner_uid(_path: &Path) -> Option<u32> {
    None
}
